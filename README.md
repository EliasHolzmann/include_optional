# `include_optional`

*MSRV: 1.88*

----

**Note**: This crate has a known bug in regards to proc-macro caching. If an
included file does not exist and a macro from this crate returns `None`, this
result is cached forever. The macro isn't reevaluated when the file is added
later. A fix for this is currently not possible in Stable Rust.

If you are running Nightly Rust, you can enable the `nightly` feature. With
this, this crate makes use of the unstable [`track_path`]
feature that is necessary in
order to fix this. Otherwise, you will have to call `cargo clean` after
adding an included file to force reevaluation.

[`track_path`]: https://github.com/rust-lang/rust/issues/99515]

----

This crate allows you to optionally include a file as a [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html). This crate supports the complete `include_X!` macro family ([`include_bytes!`](https://doc.rust-lang.org/nightly/core/macro.include_bytes.html), [`include!`](https://doc.rust-lang.org/nightly/core/macro.include.html) and [`include_str!`](https://doc.rust-lang.org/nightly/core/macro.include_str.html)).

## Installation

Add this to your `Cargo.toml`:
```lang-toml
[dependencies]
include_optional = "1.0"
```

## Example

This includes some metadata from a file, falling back to default metadata if the file is missing:
```rust
use include_optional::include_str_optional;
let metadata:  &'static str = include_str_optional!("./metadata_files/file_exists.txt" ).unwrap_or("default metadata string");
```
