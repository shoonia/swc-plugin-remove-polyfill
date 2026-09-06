mod common;
use common::run_test;

#[test]
fn number_isnan() {
    run_test(
        r#"
        Number.isNaN = Number.isNaN || function(a) {
          return "number" == typeof a && a !== a;
        };
        "#,
        "Number.isNaN = Number.isNaN;",
    );
}
