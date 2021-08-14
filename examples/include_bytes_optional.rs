use include_optional::include_bytes_optional;

#[derive(Debug)]
struct Metadata {
    foo: &'static str,
    do_bar: bool,
    baz_count: u32,
}

static DEFAULT_METADATA: [u8; 4] = [0xAB, 0xAD, 0x1D, 0xEA];

fn main() {
    let metadata_file_exists: &[u8] =
        include_bytes_optional!("./metadata_files/file_exists.bin").unwrap_or(&DEFAULT_METADATA);
    let metadata_file_missing: &[u8] =
        include_bytes_optional!("./metadata_files/file_missing.bin").unwrap_or(&DEFAULT_METADATA);

    assert_eq!(metadata_file_exists, &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(metadata_file_missing, &DEFAULT_METADATA);
}
