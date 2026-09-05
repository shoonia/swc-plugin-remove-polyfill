mod common;
use common::run_test;

#[test]
fn in_test_1() {
    run_test(
        r#"
        let A = "object" == typeof globalThis && "fromEntries"in Object && "flatMap"in Array.prototype && "trimEnd"in String.prototype && "allSettled"in Promise && "matchAll"in String.prototype && "replaceAll"in String.prototype && "any"in Promise && "at"in String.prototype && "at"in Array.prototype && "hasOwn"in Object
      "#,
        r#"
        let A = "hasOwn" in Object;
        "#,
    );
}
