mod common;

use common::run_test;

#[test]
fn string_prototype_endswith() {
    run_test(
        r#"
          String.prototype.endsWith || Object.defineProperty(String.prototype, "endsWith", {
            value: function(e, t) {
                return (void 0 === t || t > this.length) && (t = this.length),
                this.substring(t - e.length, t) === e
            },
            writable: !0,
            configurable: !0
          })
        "#,
        ";",
    );
}
