use crate::keys::function_group;
use std::matches;
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::{BinaryOp, Expr, Lit, MemberExpr, MemberProp, UnaryOp};

pub struct BoolToken {
    pub value: bool,
    pub ctxt: SyntaxContext,
}

pub enum EvalToken {
    Bool(BoolToken),
    Empty,
}

#[inline(always)]
fn is_equalities(op: &BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::EqEq | BinaryOp::EqEqEq | BinaryOp::NotEq | BinaryOp::NotEqEq
    )
}

fn calc_eq(val: bool, op: BinaryOp) -> bool {
    if val {
        op == BinaryOp::EqEq || op == BinaryOp::EqEqEq
    } else {
        op == BinaryOp::NotEq || op == BinaryOp::NotEqEq
    }
}

fn is_member_fn(m: &MemberExpr) -> Option<SyntaxContext> {
    let Expr::Ident(obj) = &*m.obj else {
        return None;
    };

    let MemberProp::Ident(prop) = &m.prop else {
        return None;
    };

    function_group(obj.sym.as_ref(), prop.sym.as_ref()).then_some(obj.ctxt)
}

fn typeof_arg(expr: &Expr) -> Option<SyntaxContext> {
    let Expr::Unary(unary) = expr else {
        return None;
    };

    if unary.op != UnaryOp::TypeOf {
        return None;
    }

    if let Some(m) = unary.arg.as_member() {
        return is_member_fn(m);
    }

    None
}

pub fn evaluate(node: &Expr) -> EvalToken {
    match node {
        Expr::Member(member) => {
            if let Some(ctxt) = is_member_fn(member) {
                EvalToken::Bool(BoolToken { value: true, ctxt })
            } else {
                EvalToken::Empty
            }
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(ctxt) = typeof_arg(&bin.left) {
                    if let Expr::Lit(lit) = &*bin.right {
                        if let Lit::Str(str_lit) = lit {
                            return EvalToken::Bool(BoolToken {
                                value: calc_eq(str_lit.value == "function", bin.op),
                                ctxt: ctxt,
                            });
                        }
                    }
                }

                if let Some(ctxt) = typeof_arg(&bin.right) {
                    if let Expr::Lit(lit) = &*bin.left {
                        if let Lit::Str(str_lit) = lit {
                            return EvalToken::Bool(BoolToken {
                                value: calc_eq(str_lit.value == "function", bin.op),
                                ctxt: ctxt,
                            });
                        }
                    }
                }
            }
            EvalToken::Empty
        }
        _ => EvalToken::Empty,
    }
}
