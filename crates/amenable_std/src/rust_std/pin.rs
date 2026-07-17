//! `RustStdType` registrations for `core::pin`.

use std::pin::Pin;

use crate::rust_std::macros::impl_rust_std_type_generic1;

impl_rust_std_type_generic1!(
    Pin,
    "core",
    "core::pin",
    "https://doc.rust-lang.org/core/pin/struct.Pin.html",
    "The Pin carrier wraps a pointer and asserts that its pointee will not move in memory for the wrapper's lifetime."
);
