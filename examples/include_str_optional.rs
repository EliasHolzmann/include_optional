use include_optional::include_str_optional;

static DEFAULT_METADATA: &'static str = "default metadata string";

fn main() {
    let metadata_file_exists: &'static str =
        include_str_optional!("./metadata_files/file_exists.txt").unwrap_or(DEFAULT_METADATA);
    let metadata_file_missing: &'static str =
        include_str_optional!("./metadata_files/file_missing.txt").unwrap_or(DEFAULT_METADATA);

    assert_eq!(metadata_file_exists, "metadata string from file");
    assert_eq!(metadata_file_missing, DEFAULT_METADATA);
}
