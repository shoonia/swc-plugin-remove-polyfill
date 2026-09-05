use crate::keys::*;
use std::matches;
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::{BinaryOp, Expr, Lit, MemberExpr, MemberProp, UnaryExpr, UnaryOp};

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

fn evaluate_member(memb: &MemberExpr) -> Option<(&str, SyntaxContext)> {
    let MemberProp::Ident(prop) = &memb.prop else {
        return None;
    };

    match memb.obj.as_ref() {
        Expr::Ident(obj) => {
            let o = obj.sym.as_ref();
            let p = prop.sym.as_ref();

            if function_group(o, p) {
                return Some((FUN, obj.ctxt));
            }

            if well_known_symbols(o, p) {
                return Some((SYM, obj.ctxt));
            }
        }
        Expr::Member(m) => {
            if m.prop.as_ident().is_some_and(|i| i.sym == "prototype") {
                if let Expr::Ident(idn) = &*m.obj {
                    return prototype_group(idn.sym.as_ref(), prop.sym.as_ref())
                        .then_some((FUN, idn.ctxt));
                }
            }
        }
        _ => {}
    }

    None
}

fn evaluate_typeof(unary: &UnaryExpr) -> Option<(&str, SyntaxContext)> {
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

fn evaluate_unary_comparison(unary: &UnaryExpr, other: &Expr, op: BinaryOp) -> Option<Token> {
    if let Some((kind, ctxt)) = evaluate_typeof(unary) {
        return other.as_lit().and_then(Lit::as_str).map(|str| Token {
            value: if str.value == kind {
                op == BinaryOp::EqEq || op == BinaryOp::EqEqEq
            } else {
                op == BinaryOp::NotEq || op == BinaryOp::NotEqEq
            },
            ctxt,
        });
    }

    if unary.op == UnaryOp::Void && unary.arg.as_lit().is_some_and(Lit::is_num) {
        if let Some(member) = other.as_member() {
            return evaluate_member(member).map(|(_, ctxt)| Token {
                value: op == BinaryOp::NotEq || op == BinaryOp::NotEqEq,
                ctxt,
            });
        }
    }

    None
}

pub fn evaluate(node: &Expr) -> Option<Token> {
    match node {
        Expr::Member(member) => {
            return evaluate_member(member).map(|(_, ctxt)| Token { value: true, ctxt })
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(left) = bin.left.as_unary() {
                    return evaluate_unary_comparison(left, &bin.right, bin.op);
                }

                if let Some(right) = bin.right.as_unary() {
                    return evaluate_unary_comparison(right, &bin.left, bin.op);
                }
            }
        }
        Expr::Unary(unary) => {
            if unary.op != UnaryOp::Bang {
                return None;
            }

            if let Some(memb) = unary.arg.as_member() {
                return evaluate_member(memb).map(|(_, ctxt)| Token { value: false, ctxt });
            }
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_ref();
            if is_built_in_constructor(name) || is_built_in_member(name) {
                return Some(Token {
                    value: true,
                    ctxt: ident.ctxt,
                });
            }
        }
        _ => {}
    }

    None
}
