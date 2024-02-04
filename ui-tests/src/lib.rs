#[test]
fn ui() {
    let version = rustc_version::version().unwrap();
    assert_eq!(version.major, 1);

    let fail = trybuild::TestCases::new();
    if version.minor <= 65 {
        fail.compile_fail("fail-1.65/*.rs");
    } else if version.minor <= 77 {
        fail.compile_fail("fail-1.72/*.rs");
    } else {
        fail.compile_fail("fail-1.78/*.rs");
    }
}
