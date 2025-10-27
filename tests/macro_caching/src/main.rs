fn main() {
    assert_eq!(
        include_optional::include_str_optional!("../the_file.txt"),
        Some("")
    )
}
