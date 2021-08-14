use include_optional::include_optional;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Metadata {
    foo: &'static str,
    do_bar: bool,
    baz_count: u32,
}

static DEFAULT_METADATA: Metadata = Metadata {
    foo: "default metadata",
    do_bar: false,
    baz_count: 42,
};

fn main() {
    let metadata_file_exists: Metadata =
        include_optional!("./metadata_files/file_exists.rs").unwrap_or(DEFAULT_METADATA);
    let metadata_file_missing: Metadata =
        include_optional!("./metadata_files/file_missing.rs").unwrap_or(DEFAULT_METADATA);

    assert_eq!(
        metadata_file_exists,
        Metadata {
            foo: "Metadata from file",
            do_bar: true,
            baz_count: 42 * 42
        }
    );
    assert_eq!(metadata_file_missing, DEFAULT_METADATA);
}
