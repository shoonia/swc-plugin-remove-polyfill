mod common;

use common::run_test;

#[test]
fn global_this_1() {
    run_test(
        r#"
        function Xo() {
          return "undefined" != typeof globalThis ? globalThis : "undefined" != typeof window ? window : void 0 !== n.g ? n.g : "undefined" != typeof self ? self : Yo
        }
        "#,
        r#"
        function Xo() {
          return globalThis;
        }
        "#,
    );
}

#[test]
fn global_this_2() {
    run_test(
        r#"
        var K3 = function() {
          return "undefined" != typeof window ? window : "undefined" != typeof globalThis ? globalThis : void 0 !== n.g ? n.g : "undefined" != typeof WorkerGlobalScope ? WorkerGlobalScope : G3
        }
        "#,
        r#"
        var K3 = function () {
          return "undefined" != typeof window ? window : globalThis;
        };
        "#,
    );
}
