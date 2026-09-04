mod common;
use common::run_test;

#[test]
fn symbol_asynciterator() {
    run_test(
        r#"
        if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
        "#,
        ";",
    );
}
