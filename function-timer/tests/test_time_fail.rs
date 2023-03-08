#[test]
fn test_fail_wrong_token() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/fail.rs");
}

#[test]
fn test_fail_disable_impl() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/fail_disable_struct.rs");
}
