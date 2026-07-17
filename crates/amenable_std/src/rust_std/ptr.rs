//! `RustStdType` registrations for `core::ptr`.
//!
//! `core::ptr::Alignment` is a type alias to `core::mem::Alignment`, which
//! is unstable (see `rust_std::mem`'s doc comment) — nothing to cover here.
//! `DynMetadata` is deliberately not covered either — unstable
//! (`ptr_metadata`, rust-lang/rust#81513), not nameable from a stable
//! toolchain.

use std::ptr::NonNull;

use crate::rust_std::macros::impl_rust_std_type_generic1;

impl_rust_std_type_generic1!(
    NonNull,
    "core",
    "core::ptr",
    "https://doc.rust-lang.org/core/ptr/struct.NonNull.html",
    "The NonNull carrier stores a raw pointer statically known to never be null."
);
