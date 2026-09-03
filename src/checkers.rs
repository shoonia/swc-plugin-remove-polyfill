use swc_core::common::{Mark, Span};
use swc_core::ecma::ast::{Expr, MemberProp};
use swc_core::ecma::atoms::Atom;

pub struct MemberToken {
    pub obj: Atom,
    pub prop: Atom,
    pub span: Span,
}

enum EvalToken {
    Member(MemberToken),
    Empty,
}

fn evaluate(node: &Expr, unresolved_mark: Mark) -> EvalToken {
    match node {
        Expr::Member(member) => {
            let Expr::Ident(obj_ident) = &*member.obj else {
                return EvalToken::Empty;
            };

            if obj_ident.ctxt.outer() != unresolved_mark {
                return EvalToken::Empty;
            }

            let MemberProp::Ident(prop_ident) = &member.prop else {
                return EvalToken::Empty;
            };

            EvalToken::Member(MemberToken {
                obj: obj_ident.sym.clone(),
                prop: prop_ident.sym.clone(),
                span: member.span,
            })
        }
        _ => EvalToken::Empty,
    }
}

pub fn checker(node: &Expr, unresolved_mark: Mark) -> Option<bool> {
    match evaluate(node, unresolved_mark) {
        EvalToken::Member(member) => {
            if member.obj == "Object" && member.prop == "assign" {
                Some(true)
            } else {
                None
            }
        }
        EvalToken::Empty => None,
    }
}
