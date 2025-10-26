# `include_optional`

----

*MSRV: 1.88*

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
