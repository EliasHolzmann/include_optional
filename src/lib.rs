#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(clippy::nursery)]
#![cfg_attr(feature = "nightly", feature(track_path))]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

// this is a (heavily) modified version from the `resolve_path` function rustc uses to implement `include_X!` – see https://github.com/rust-lang/rust/blob/c6094fc7/compiler/rustc_expand/src/base.rs#L1097
fn resolve_path(file: &LitStr) -> PathBuf {
    let path: PathBuf = file.value().into();

    // Relative paths are resolved relative to the file in which they are found
    // after macro expansion (that is, they are unhygienic).
    if path.is_relative() {
        // rustc uses `rustc_span::source_map:SourceMap::span_to_filename()` here,
        // which in turn uses `source_map.lookup_char_pos(sp.lo()).file.name.clone()`,
        // which is also what `proc_macro::Span::local_file()` does (via
        // `rustc_expand::server::Span::local_file`). Therefore, use this over
        // `proc_nacro::Span::file()`.
        let mut result = proc_macro::Span::call_site()
            .local_file()
            // error message adapted from https://github.com/rust-lang/rust/blob/28880985/compiler/rustc_expand/messages.ftl#L191-L192
            .expect("Cannot resolve relative path in non-file source");
        result.pop();
        result.push(path);
        result
    } else {
        path
    }
}

enum FileExists {
    Exists,
    NoSuchFile,
    Error(std::io::Error),
}

fn get_file_exists(lit_file: &LitStr) -> FileExists {
    let complete_path = resolve_path(lit_file);

    match complete_path.try_exists() {
        Ok(true) => FileExists::Exists,
        Ok(false) => FileExists::NoSuchFile,
        Err(e) => FileExists::Error(e),
    }
}

macro_rules! gen_include_optional_macro {
    (
        $new_macro:ident!
        using $original_macro:ident!
        with example file ending $ending:literal
    ) => {
        gen_include_optional_macro!(
            $new_macro!
            using $original_macro!
            with example file ending $ending
            and example file content concat!(
                "```ignore\n",
                include_str!(
                    concat!(
                        "../examples/metadata_files/file_exists",
                        $ending
                    )
                ), "\n```"
            )
        );
    };
    (
        $new_macro:ident!
        using $original_macro:ident!
        with example file ending $ending:literal
        inline
    ) => {
        gen_include_optional_macro!(
            $new_macro!
            using $original_macro!
            with example file ending $ending
            and example file content concat!(
                "`",
                include_str!(
                    concat!(
                        "../examples/metadata_files/file_exists",
                        $ending
                    )
                ), "`. "
            )
        );
    };
    (
        $new_macro:ident!
        using $original_macro:ident!
        with example file ending $ending:literal
        and syntax highlighting $syntax_highlighting:literal
    ) => {
        gen_include_optional_macro!(
            $new_macro!
            using $original_macro!
            with example file ending $ending
            and example file content concat!(
                "```",
                $syntax_highlighting,
                "\n",
                include_str!(
                    concat!(
                        "../examples/metadata_files/file_exists",
                        $ending
                    )
                ), "\n```"
            )
        );
    };
    (
        $new_macro:ident!
        using $original_macro:ident!
        with example file ending $ending:literal
        and example file content $($example_file_content:tt)*
    ) => {
        #[doc = concat!("Wraps [`", stringify!($original_macro), "!`](core::", stringify!($original_macro), ") inside [`Option`](core::option::Option).")]
        ///
        #[doc = concat!("You should call this macro as `", stringify!($new_macro), "!(\"./path/to/file", $ending, "\")` (with either an absolute or a relative path).")]
        ///
        /// The macro checks whether a file exists under the given path:
        #[doc = concat!("- If the file **does exist**, the macro emits `Some(", stringify!($original_macro), "!(\"./path/to/file", $ending, "\"))`.")]
        /// - If the file **does not exist**, the macro emits `None`.
        /// - If trying to check the existence of the file **resulted in an error** (for example, because it is in a directory that the current user does not have read permissions for or because of hardware failure), compilation fails.
        ///
        /// # Examples
        /// You can use this macro to include metadata from a file, falling back to some default value if the file does not exist.
        ///
        #[doc = concat!("Consider a file `./metadata_files/file_exists", $ending, "` with the content ")]
        #[doc = $($example_file_content)*]
        #[doc = concat!("The file `./metadata_files/file_missing", $ending, "` does not exist.")]
        #[doc = concat!("```\n", include_str!(concat!("../examples/", stringify!($new_macro), ".rs")), "\n```")]
        #[proc_macro]
        pub fn $new_macro(input: TokenStream) -> TokenStream {
            let file_lit = parse_macro_input!(input as LitStr);
            match get_file_exists(&file_lit) {
                FileExists::Exists => quote! {
                    ::core::option::Option::Some($original_macro!(#file_lit))
                },
                FileExists::NoSuchFile => {
                    #[cfg(feature = "nightly")]
                    {
                        proc_macro::tracked_path::path(file_lit.value());
                    }

                    quote! {
                        ::core::option::Option::None
                    }
                },
                FileExists::Error(e) => {
                    let compile_error = format!("Couldn't access {}: {}", file_lit.value(), e);
                    let file_lit_span = file_lit.span();
                    quote_spanned! {
                        file_lit_span =>
                        ::core::compile_error!(#compile_error)
                    }
                }
            }.into()
        }
    }
}

gen_include_optional_macro!(include_optional! using include! with example file ending ".rs");
gen_include_optional_macro!(include_str_optional! using include_str! with example file ending ".txt" inline);
gen_include_optional_macro!(include_bytes_optional! using include_bytes! with example file ending ".bin" and example file content "`0xDEADBEEF`. ");
