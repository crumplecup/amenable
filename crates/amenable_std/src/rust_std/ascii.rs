//! `RustStdType` registrations for `core::ascii`.
//!
//! `EscapeDefault` is written fully-qualified: `core::char` and
//! `core::str` each define their own carrier by the same bare name — only
//! the qualified path disambiguates which one a given registration means
//! for tooling reading the registry (e.g. `elicit_doc`'s coverage report).

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    core::ascii::EscapeDefault,
    "core",
    "core::ascii",
    "https://doc.rust-lang.org/core/ascii/struct.EscapeDefault.html",
    "The EscapeDefault carrier lazily yields a byte's ASCII escape sequence, as in a Rust string literal."
);

register_rust_std_standard_evidence!(core::ascii::EscapeDefault);
