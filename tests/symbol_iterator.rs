mod common;

use common::run_test;

#[test]
fn transform_1() {
    run_test(
        r#"
      var ma = function(a) {
        var b = "undefined" != typeof Symbol && Symbol.iterator && a[Symbol.iterator];
        if (b) return b.call(a);
        if ("number" == typeof a.length) return {
          next: ba(a)
        };
        throw Error(String(a) + " is not an iterable or ArrayLike");
      }
        "#,
        r#"
        var ma = function (a) {
          var b = a[Symbol.iterator];
          if (b) return b.call(a);
          if ("number" == typeof a.length) return {
            next: ba(a)
          };
          throw Error(String(a) + " is not an iterable or ArrayLike");
        };
        "#,
    );
}
