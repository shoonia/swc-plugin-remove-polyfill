use crate::keys::{function_group, is_built_in_constructor, is_built_in_member};
use std::matches;
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::{BinaryOp, Expr, Lit, MemberExpr, MemberProp, UnaryOp};

pub struct Token {
    pub value: bool,
    pub ctxt: SyntaxContext,
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

const FUN: &str = "function";
const OBJ: &str = "object";

fn typeof_arg(expr: &Expr) -> Option<(&str, SyntaxContext)> {
    let Expr::Unary(unary) = expr else {
        return None;
    };

    if unary.op != UnaryOp::TypeOf {
        return None;
    }

    if let Some(m) = unary.arg.as_member() {
        return is_member_fn(m).map(|ctxt| (FUN, ctxt));
    }

    if let Some(i) = unary.arg.as_ident() {
        let name = i.sym.as_ref();

        if is_built_in_constructor(name) {
            return Some(((FUN), i.ctxt));
        }

        if is_built_in_member(name) {
            return Some((OBJ, i.ctxt));
        }
    }

    None
}

pub fn evaluate(node: &Expr) -> Option<Token> {
    match node {
        Expr::Member(member) => {
            if let Some(ctxt) = is_member_fn(member) {
                Some(Token { value: true, ctxt })
            } else {
                None
            }
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(res) = typeof_arg(&bin.left) {
                    if let Expr::Lit(lit) = &*bin.right {
                        if let Lit::Str(str_lit) = lit {
                            return Some(Token {
                                value: calc_eq(str_lit.value == res.0, bin.op),
                                ctxt: res.1,
                            });
                        }
                    }
                }

                if let Some(res) = typeof_arg(&bin.right) {
                    if let Expr::Lit(lit) = &*bin.left {
                        if let Lit::Str(str_lit) = lit {
                            return Some(Token {
                                value: calc_eq(str_lit.value == res.0, bin.op),
                                ctxt: res.1,
                            });
                        }
                    }
                }
            }
            None
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_ref();
            if is_built_in_constructor(name) || is_built_in_member(name) {
                Some(Token {
                    value: true,
                    ctxt: ident.ctxt,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
