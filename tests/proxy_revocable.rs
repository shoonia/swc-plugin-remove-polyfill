mod common;
use common::run_test;

#[test]
fn proxy_revocable() {
    run_test(
        r#"var H = "undefined" != typeof Proxy && void 0 !== Proxy.revocable && "undefined" != typeof Reflect;"#,
        r#"var H = "undefined" != typeof Reflect;"#,
    );
}
