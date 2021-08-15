# include_optional

----

**Note: This currently only works on nightly Rust.** This crate depends on [`proc_macro::SourceFile::path()`](https://doc.rust-lang.org/nightly/proc_macro/struct.SourceFile.html#method.path), which is [not yet stabilized](https://github.com/rust-lang/rust/issues/54725) (additionally, there is a dependency on the unstable function [`std::path::Path::try_exists`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html#method.try_exists), however, this dependency could probably be removed without too much hassle).

If you are able to somehow avoid this dependency, pull requests are welcome!

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

fn main() {
    let metadata:  &'static str = include_str_optional!("./metadata_files/file_exists.txt" ).unwrap_or("default metadata string");
    //...
}
```
