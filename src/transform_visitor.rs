use crate::checkers::{evaluate, EvalToken};
use swc_core::common::util::take::Take;
use swc_core::common::{Mark, SyntaxContext};
use swc_core::ecma::ast::{BinaryOp, Expr};
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
            EvalToken::Member(member) => {
                if member.obj == "Object" && member.prop == "assign" && self.is_global(member.ctxt)
                {
                    Some(true)
                } else {
                    None
                }
            }
            EvalToken::Empty => None,
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        match expr {
            Expr::Bin(bin) => {
                if self.checker(&bin.left).is_none() {
                    return;
                }

                match bin.op {
                    BinaryOp::LogicalOr | BinaryOp::NullishCoalescing => {
                        *expr = *bin.left.take();
                    }
                    BinaryOp::LogicalAnd => {
                        *expr = *bin.right.take();
                    }
                    _ => {}
                }
            }
            Expr::Cond(cond) => {
                let Some(val) = self.checker(&cond.test) else {
                    return;
                };

                if val {
                    *expr = *cond.cons.take();
                } else {
                    *expr = *cond.alt.take();
                }
            }
            _ => return,
        };
    }
}
