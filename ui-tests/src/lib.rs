#[test]
fn ui() {
    let fail = trybuild::TestCases::new();
    fail.compile_fail("fail/*.rs");
}
