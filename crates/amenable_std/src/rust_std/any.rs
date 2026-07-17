//! `RustStdType` registrations for `core::any`.

use core::any::TypeId;

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    TypeId,
    "core",
    "core::any",
    "https://doc.rust-lang.org/core/any/struct.TypeId.html",
    "The TypeId carrier is an opaque, globally unique identifier for a 'static Rust type."
);
