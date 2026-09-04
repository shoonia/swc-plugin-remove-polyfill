mod common;

use common::run_test;

#[test]
fn transform_1() {
    run_test(
        r#"
        function a(t) {
          var n = "function" == typeof Symbol && t[Symbol.iterator],
            r = 0;
            return n ? n.call(t) : {
              next: function() {
                return t && r >= t.length && (t = void 0), {
                  value: t && t[r++],
                  done: !t
                }
              }
            }
          }
        "#,
        r#"
        function a(t) {
          var n = t[Symbol.iterator],
          r = 0;
          return n ? n.call(t) : {
            next: function () {
              return t && r >= t.length && (t = void 0), {
                value: t && t[r++],
                done: !t
              };
            }
          };
        "#,
    );
}
