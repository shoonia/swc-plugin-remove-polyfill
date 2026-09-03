use crate::checkers::checker;
use swc_core::common::util::take::Take;
use swc_core::common::Mark;
use swc_core::ecma::ast::{BinaryOp, Expr};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct TransformVisitor {
    pub unresolved_mark: Mark,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let Expr::Bin(bin) = expr else {
            return;
        };

        if checker(&bin.left, self.unresolved_mark).is_none() {
            return;
        }

        match bin.op {
            BinaryOp::LogicalOr => {
                *expr = *bin.left.take();
            }
            _ => {}
        }
    }
}
