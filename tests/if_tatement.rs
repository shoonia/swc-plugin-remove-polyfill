mod common;
use common::run_test;

#[test]
fn alternate_is_removed() {
    run_test(
        "if (Date.now) console.log(1); else console.log(2);",
        "console.log(1);",
    );
}

#[test]
fn consequent_is_removed() {
    run_test(
        "if (!Date.now) console.log(1); else console.log(2);",
        "console.log(2);",
    );
}

#[test]
fn remove_if() {
    run_test("if (!Date.now) console.log(1);", ";");
}

#[test]
fn should_be_block_stmt_alternate() {
    run_test(
        "if (!Date.now) console.log(1); else { console.log(2); console.log(3); }",
        "{ console.log(2); console.log(3); }",
    );
}

#[test]
fn should_be_block_stmt_consequent() {
    run_test(
        "if (Date.now) { console.log(1); console.log(2); } else console.log(3);",
        "{ console.log(1); console.log(2); }",
    );
}
