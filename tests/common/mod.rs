use swc_core::common::Mark;
use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::base::resolver,
    transforms::testing::test_transform,
    visit::visit_mut_pass,
};
use swc_plugin_remove_polyfill::transform_visitor::TransformVisitor;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

pub fn run_test(input: &str, expected: &str) {
    test_transform(
        syntax(),
        Some(true),
        |_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                visit_mut_pass(TransformVisitor { unresolved_mark }),
            )
        },
        input,
        expected,
    );
}
