mod common;
use common::run_test;

#[test]
fn void_test_1() {
    run_test(
        "let t = Math.abs == void 0 ? true : false;",
        "let t = false;",
    );
}

#[test]
fn void_test_2() {
    run_test(
        "let t = Math.abs != void 0 ? true : false;",
        "let t = true;",
    );
}

#[test]
fn void_test_3() {
    run_test(
        "let t = void 0 == Math.abs ? true : false;",
        "let t = false;",
    );
}

#[test]
fn void_test_4() {
    run_test(
        "let t = void 0 != Math.abs ? true : false;",
        "let t = true;",
    );
}
