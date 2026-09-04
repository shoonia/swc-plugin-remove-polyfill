use crate::keys::{
    function_group, is_built_in_constructor, is_built_in_member, well_known_symbols,
};
use std::matches;
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::{BinaryOp, Expr, Lit, MemberExpr, MemberProp, UnaryOp};

const FUN: &str = "function";
const OBJ: &str = "object";
const SYM: &str = "symbol";

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

fn evaluate_member(memb: &MemberExpr) -> Option<(&str, SyntaxContext)> {
    let Expr::Ident(obj) = &*memb.obj else {
        return None;
    };

    let MemberProp::Ident(prop) = &memb.prop else {
        return None;
    };

    let o = obj.sym.as_ref();
    let p = prop.sym.as_ref();

    if function_group(o, p) {
        return Some((FUN, obj.ctxt));
    }

    if well_known_symbols(o, p) {
        return Some((SYM, obj.ctxt));
    }

    None
}

fn evaluate_typeof(expr: &Expr) -> Option<(&str, SyntaxContext)> {
    let Expr::Unary(unary) = expr else {
        return None;
    };

    if unary.op != UnaryOp::TypeOf {
        return None;
    }

    if let Some(memb) = unary.arg.as_member() {
        return evaluate_member(memb);
    }

    if let Some(i) = unary.arg.as_ident() {
        let name = i.sym.as_ref();

        if is_built_in_constructor(name) {
            return Some((FUN, i.ctxt));
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
            if let Some((_, ctxt)) = evaluate_member(member) {
                Some(Token { value: true, ctxt })
            } else {
                None
            }
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(res) = evaluate_typeof(&bin.left) {
                    if let Expr::Lit(lit) = &*bin.right {
                        if let Lit::Str(str_lit) = lit {
                            return Some(Token {
                                value: calc_eq(str_lit.value == res.0, bin.op),
                                ctxt: res.1,
                            });
                        }
                    }
                }

                if let Some(res) = evaluate_typeof(&bin.right) {
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
