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

struct Match {
    kind: &'static str,
    ctxt: SyntaxContext,
}

#[inline(always)]
fn is_equalities(op: &BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::EqEq | BinaryOp::EqEqEq | BinaryOp::NotEq | BinaryOp::NotEqEq
    )
}

#[inline(always)]
fn is_prototype_ident(prop: &MemberProp) -> bool {
    prop.as_ident().is_some_and(|i| i.sym == "prototype")
}

fn evaluate_member(member: &MemberExpr) -> Option<Match> {
    let MemberProp::Ident(prop) = &member.prop else {
        return None;
    };

    match member.obj.as_ref() {
        Expr::Ident(obj) => {
            let o = obj.sym.as_ref();
            let p = prop.sym.as_ref();

            if is_static_method(o, p) {
                return Some(Match {
                    kind: FUN,
                    ctxt: obj.ctxt,
                });
            }

            if is_well_known_symbol(o, p) {
                return Some(Match {
                    kind: SYM,
                    ctxt: obj.ctxt,
                });
            }
        }
        Expr::Member(memb) => {
            if is_prototype_ident(&memb.prop) {
                if let Expr::Ident(ident) = &*memb.obj {
                    return is_prototype_method(ident.sym.as_ref(), prop.sym.as_ref()).then_some(
                        Match {
                            kind: FUN,
                            ctxt: ident.ctxt,
                        },
                    );
                }
            }
        }
        _ => {}
    }

    None
}

fn evaluate_typeof(unary: &UnaryExpr) -> Option<Match> {
    if unary.op != UnaryOp::TypeOf {
        return None;
    }

    if let Some(memb) = unary.arg.as_member() {
        return evaluate_member(memb);
    }

    if let Some(i) = unary.arg.as_ident() {
        let name = i.sym.as_ref();

        if is_built_in_constructor(name) {
            return Some(Match {
                kind: FUN,
                ctxt: i.ctxt,
            });
        }

        if is_built_in_member(name) {
            return Some(Match {
                kind: OBJ,
                ctxt: i.ctxt,
            });
        }
    }

    None
}

fn evaluate_unary_comparison(unary: &UnaryExpr, other: &Expr, op: BinaryOp) -> Option<Token> {
    if let Some(mtc) = evaluate_typeof(unary) {
        return other.as_lit().and_then(Lit::as_str).map(|str| Token {
            value: if str.value == mtc.kind {
                op == BinaryOp::EqEq || op == BinaryOp::EqEqEq
            } else {
                op == BinaryOp::NotEq || op == BinaryOp::NotEqEq
            },
            ctxt: mtc.ctxt,
        });
    }

    if unary.op == UnaryOp::Void && unary.arg.as_lit().is_some_and(Lit::is_num) {
        if let Some(member) = other.as_member() {
            return evaluate_member(member).map(|mtc| Token {
                value: op == BinaryOp::NotEq || op == BinaryOp::NotEqEq,
                ctxt: mtc.ctxt,
            });
        }
    }

    None
}

pub fn evaluate(node: &Expr) -> Option<Token> {
    match node {
        Expr::Member(member) => {
            return evaluate_member(member).map(|mtc| Token {
                value: true,
                ctxt: mtc.ctxt,
            })
        }
        Expr::Bin(bin) => {
            if is_equalities(&bin.op) {
                if let Some(left) = bin.left.as_unary() {
                    return evaluate_unary_comparison(left, &bin.right, bin.op);
                }

                if let Some(right) = bin.right.as_unary() {
                    return evaluate_unary_comparison(right, &bin.left, bin.op);
                }
            } else if bin.op == BinaryOp::In {
                if let Some(key) = bin
                    .left
                    .as_lit()
                    .and_then(Lit::as_str)
                    .and_then(|str| str.value.as_str())
                {
                    if let Some(ident) = bin.right.as_ident() {
                        if is_static_method(ident.sym.as_ref(), key) {
                            return Some(Token {
                                value: true,
                                ctxt: ident.ctxt,
                            });
                        }
                    } else if let Some(memb) = bin.right.as_member() {
                        if is_prototype_ident(&memb.prop) {
                            if let Expr::Ident(ident) = &*memb.obj {
                                return is_prototype_method(ident.sym.as_ref(), key).then_some(
                                    Token {
                                        value: true,
                                        ctxt: ident.ctxt,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
        Expr::Unary(unary) => {
            if unary.op != UnaryOp::Bang {
                return None;
            }

            if let Some(memb) = unary.arg.as_member() {
                return evaluate_member(memb).map(|mtc| Token {
                    value: false,
                    ctxt: mtc.ctxt,
                });
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
