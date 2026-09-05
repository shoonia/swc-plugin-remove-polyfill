use crate::evaluate::evaluate;
use swc_core::common::util::take::Take;
use swc_core::common::{Mark, Span, SyntaxContext};
use swc_core::ecma::ast::{BinaryOp, EmptyStmt, Expr, Stmt};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

#[inline(always)]
fn empty_stmt(span: Span) -> Stmt {
    EmptyStmt { span }.into()
}

pub struct TransformVisitor {
    pub unresolved_mark: Mark,
}

impl TransformVisitor {
    #[inline(always)]
    fn is_global(&self, ctxt: SyntaxContext) -> bool {
        ctxt.outer() == self.unresolved_mark
    }

    fn checker(&self, node: &Expr) -> Option<bool> {
        evaluate(node).and_then(|token| self.is_global(token.ctxt).then_some(token.value))
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_mut_children_with(self);

        match stmt {
            Stmt::Expr(value) => {
                if let Some(_) = self.checker(&value.expr) {
                    *stmt = empty_stmt(value.span);
                }
            }
            Stmt::If(if_stmt) => {
                if let Some(val) = self.checker(&if_stmt.test) {
                    *stmt = *if val {
                        if_stmt.cons.take()
                    } else if let Some(ref mut alt) = if_stmt.alt {
                        alt.take()
                    } else {
                        empty_stmt(if_stmt.span).into()
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        match expr {
            Expr::Bin(bin) => match bin.op {
                BinaryOp::LogicalOr | BinaryOp::NullishCoalescing => {
                    if let Some(val) = self.checker(&bin.left) {
                        *expr = *if val {
                            bin.left.take()
                        } else {
                            bin.right.take()
                        };
                    };
                }
                BinaryOp::LogicalAnd => {
                    if let Some(val) = self.checker(&bin.left) {
                        *expr = *if val {
                            bin.right.take()
                        } else {
                            bin.left.take()
                        };
                    };
                }
                _ => {}
            },
            Expr::Cond(cond) => {
                if let Some(val) = self.checker(&cond.test) {
                    *expr = *if val {
                        cond.cons.take()
                    } else {
                        cond.alt.take()
                    }
                }
            }
            _ => {}
        };
    }
}
