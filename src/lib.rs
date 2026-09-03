pub mod checkers;
pub mod transform_visitor;

use crate::transform_visitor::TransformVisitor;
use swc_core::ecma::{ast::Program, visit::visit_mut_pass};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(&mut TransformVisitor {
        unresolved_mark: metadata.unresolved_mark,
    }))
}
