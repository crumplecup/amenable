//! `RustStdType` registrations for `alloc::string`.
//!
//! `IntoChars` is deliberately not covered here — unstable
//! (`string_into_chars`). `ParseError` is a `TypeAlias` for
//! `core::convert::Infallible`, already covered — nothing separate to impl.

use std::string::{Drain, FromUtf8Error, FromUtf16Error};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type_lifetime0!(
    Drain,
    "alloc",
    "alloc::string",
    "https://doc.rust-lang.org/alloc/string/struct.Drain.html",
    "The Drain carrier removes and yields a range of a String's chars in order."
);

impl_rust_std_type!(
    FromUtf16Error,
    "alloc",
    "alloc::string",
    "https://doc.rust-lang.org/alloc/string/struct.FromUtf16Error.html",
    "The FromUtf16Error carrier reports that a u16 slice was not valid UTF-16 when converting it into a String."
);

impl_rust_std_type!(
    FromUtf8Error,
    "alloc",
    "alloc::string",
    "https://doc.rust-lang.org/alloc/string/struct.FromUtf8Error.html",
    "The FromUtf8Error carrier reports that a byte vector was not valid UTF-8 when converting it into a String, and returns the original bytes."
);

// `Drain` is written fully-qualified: its bare name collides with
// `alloc::vec::Drain`/`alloc::collections::{binary_heap,vec_deque}::Drain`,
// and only the qualified form disambiguates for tooling reading the
// registry (e.g. `cordial`'s coverage report).
register_rust_std_standard_evidence!(std::string::Drain<'static>, FromUtf16Error, FromUtf8Error);
