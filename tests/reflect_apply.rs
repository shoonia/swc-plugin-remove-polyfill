mod common;
use common::run_test;

#[test]
fn reflect_apply() {
    run_test(
        r#"
        function a(e, n, t) {
          var i = t(91),
            a = Function.prototype,
            r = a.apply,
            o = a.call;
          e.exports = "object" == typeof Reflect && Reflect.apply || (i ? o.bind(r) : function() {
            return o.apply(r, arguments)
          })
        }
        "#,
        r#"
        function a(e, n, t) {
          var i = t(91),
            a = Function.prototype,
            r = a.apply,
            o = a.call;
          e.exports = Reflect.apply;
        }
        "#,
    );
}
