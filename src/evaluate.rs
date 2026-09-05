use crate::keys::*;
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

impl Token {
    pub fn some(value: bool, ctxt: SyntaxContext) -> Option<Self> {
        Some(Self { value, ctxt })
    }
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
                Token::some(true, ctxt)
            } else {
                None
            }
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(res) = evaluate_typeof(&bin.left) {
                    if let Expr::Lit(lit) = &*bin.right {
                        if let Lit::Str(str_lit) = lit {
                            return Token::some(calc_eq(str_lit.value == res.0, bin.op), res.1);
                        }
                    }
                }

                if let Some(res) = evaluate_typeof(&bin.right) {
                    if let Expr::Lit(lit) = &*bin.left {
                        if let Lit::Str(str_lit) = lit {
                            return Token::some(calc_eq(str_lit.value == res.0, bin.op), res.1);
                        }
                    }
                }
            }
            None
        }
        Expr::Unary(unary) => {
            if unary.op != UnaryOp::Bang {
                return None;
            }

            if let Some(memb) = unary.arg.as_member() {
                if let Some((_, ctxt)) = evaluate_member(memb) {
                    return Token::some(false, ctxt);
                }
            }

            if let Some(i) = unary.arg.as_ident() {
                let name = i.sym.as_ref();

                if is_built_in_constructor(name) || is_built_in_member(name) {
                    return Token::some(false, i.ctxt);
                }
            }

            None
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_ref();
            if is_built_in_constructor(name) || is_built_in_member(name) {
                Token::some(true, ident.ctxt)
            } else {
                None
            }
        }
        _ => None,
    }
}
