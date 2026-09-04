mod common;
use common::run_test;

#[test]
fn promise_prototype_finally() {
    run_test(
        r#"
        Promise.prototype.finally || (Promise.prototype.finally = function(e) {
            if ("function" != typeof e) return this.then(e, e);
            var t = this.constructor || Promise;
            return this.then((function(n) {
                return t.resolve(e()).then((function() {
                    return n
                }
                ))
            }), (function(n) {
                return t.resolve(e()).then((function() {
                    throw n
                }))
            }))
        });
        "#,
        ";",
    );
}
