mod common;

use common::run_test;

#[test]
fn array_is_array_basic_or_polyfill() {
    run_test(
        r#"
        var isArray = Array.isArray || function (e) {
          return "[object Array]" === Object.prototype.toString.call(e);
        };
        "#,
        "var isArray = Array.isArray;",
    );
}
