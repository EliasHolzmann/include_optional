#[test]
fn file_exists() {
    assert_eq!(
        Some(include_str!("../../src/lib.rs")),
        include_optional::include_str_optional!("../../src/lib.rs")
    );

    assert_eq!(
        include_str!("../../src/lib.rs"),
        include_optional::include_str_optional!("../../src/lib.rs").unwrap()
    );
}

#[test]
fn directory_does_not_exist() {
    let content: Option<&'static str> =
        include_optional::include_str_optional!("./directory_does_not_exist/some-file.txt");
    assert!(content.is_none());
}

#[test]
fn file_does_not_exist() {
    let content: Option<&'static str> =
        include_optional::include_str_optional!("./file-does-not-exist.txt");
    assert!(content.is_none());
}
