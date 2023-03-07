#[test]
fn test_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/fail.rs");
}
