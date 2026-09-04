use crate::checkers::{evaluate, EvalToken};
use swc_core::common::util::take::Take;
use swc_core::common::{Mark, SyntaxContext};
use swc_core::ecma::ast::{BinaryOp, EmptyStmt, Expr, Stmt};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct TransformVisitor {
    pub unresolved_mark: Mark,
}

impl TransformVisitor {
    fn is_global(&self, ctxt: SyntaxContext) -> bool {
        ctxt.outer() == self.unresolved_mark
    }

    fn checker(&self, node: &Expr) -> Option<bool> {
        match evaluate(node) {
            EvalToken::Bool(token) => {
                if self.is_global(token.ctxt) {
                    Some(token.value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_mut_children_with(self);

        if let Some(value) = stmt.as_expr() {
            if let Some(_) = self.checker(&value.expr) {
                *stmt = EmptyStmt { span: value.span }.into();
            }
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        match expr {
            Expr::Bin(bin) => {
                let Some(val) = self.checker(&bin.left) else {
                    return;
                };

                match bin.op {
                    BinaryOp::LogicalOr | BinaryOp::NullishCoalescing => {
                        if val {
                            *expr = *bin.left.take();
                        } else {
                            *expr = *bin.right.take();
                        }
                    }
                    BinaryOp::LogicalAnd => {
                        if val {
                            *expr = *bin.right.take();
                        } else {
                            *expr = *bin.left.take();
                        }
                    }
                    _ => {}
                }
            }
            Expr::Cond(cond) => {
                if let Some(val) = self.checker(&cond.test) {
                    if val {
                        *expr = *cond.cons.take();
                    } else {
                        *expr = *cond.alt.take();
                    }
                }
            }
            _ => {}
        };
    }
}
