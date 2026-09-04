mod common;
use common::run_test;

#[test]
fn object_defineproperties() {
    run_test(
        r#"
        var ca = "function" == typeof Object.defineProperties ? Object.defineProperty : function(a, b, c) {
          if (a == Array.prototype || a == Object.prototype) return a;
          a[b] = c.value;
          return a
        };
        "#,
        "var ca = Object.defineProperty;",
    );
}

#[test]
fn object_defineproperties_drop_else() {
    run_test(
        r#"
      if (Object.defineProperties) {
        var d = Object.getOwnPropertyDescriptor(b, c);
        d && Object.defineProperty(a, c, d)
      } else a[c] = b[c];
      "#,
        r#"
      if (true) {
        var d = Object.getOwnPropertyDescriptor(b, c);
        d && Object.defineProperty(a, c, d);
      }
      "#,
    );
}
