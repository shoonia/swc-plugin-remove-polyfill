use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::{Expr, MemberProp};
use swc_core::ecma::atoms::Atom;

pub struct MemberToken {
    pub obj: Atom,
    pub prop: Atom,
    pub ctxt: SyntaxContext,
}

pub enum EvalToken {
    Member(MemberToken),
    Empty,
}

pub fn evaluate(node: &Expr) -> EvalToken {
    match node {
        Expr::Member(member) => {
            let Expr::Ident(obj) = &*member.obj else {
                return EvalToken::Empty;
            };

            let MemberProp::Ident(prop) = &member.prop else {
                return EvalToken::Empty;
            };

            EvalToken::Member(MemberToken {
                obj: obj.sym.clone(),
                prop: prop.sym.clone(),
                ctxt: obj.ctxt,
            })
        }
        _ => EvalToken::Empty,
    }
}
