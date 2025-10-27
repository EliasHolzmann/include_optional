use std::{
    fs::{self, File},
    path::Path,
};

mod absolute_path;
mod relative_path;

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/*.rs");
}

/// Rust caches proc-macro results. If a file is included via
/// an std `include_x!` macro (as is done by `include_optional`
/// macros), the cache is invalidated on file change. However,
/// if a file doesn't exist and [`None`] is returned by an
/// `include_optional` macro, the cache does never get invalidated,
/// as the corresponding `include_x!` function is never called.
///
/// On nightly Rust, there is an unstable function [`proc_macro::tracked_path::path`]
/// that allows to manually track paths. This is used if the `nightly`
/// feature is active. This test checks that the `tracked_path` handling
/// works as expected. To do so, it manipulates a crate in a subdirectory,
/// adds/deletes an optionally included file and checks that the macro result
/// changes as expected.
#[test]
fn macro_caching() {
    let file: &Path = "tests/macro_caching/the_file.txt".as_ref();
    let mut cargo_run = std::process::Command::new("cargo");
    cargo_run
        .arg("+nightly")
        .arg("run")
        .current_dir(fs::canonicalize(file.parent().unwrap()).unwrap());

    File::create(file).unwrap();
    assert!(cargo_run.status().unwrap().success());

    fs::remove_file(file).unwrap();
    assert!(!cargo_run.status().unwrap().success());

    File::create(file).unwrap();
    assert!(cargo_run.status().unwrap().success());

    fs::remove_file(file).unwrap();
}
