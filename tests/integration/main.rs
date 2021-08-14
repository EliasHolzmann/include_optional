mod absolute_path;
mod relative_path;

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/*.rs");
}
