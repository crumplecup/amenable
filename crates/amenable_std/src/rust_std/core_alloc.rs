//! `RustStdType` registrations for `core::alloc`.
//!
//! `LayoutErr` is a deprecated `TypeAlias` for `LayoutError`, already
//! covered — nothing separate to impl. `AllocError` is deliberately not
//! covered — unstable (`allocator_api`), same as `alloc::alloc::Global`.

use core::alloc::{Layout, LayoutError};

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    Layout,
    "core",
    "core::alloc",
    "https://doc.rust-lang.org/core/alloc/struct.Layout.html",
    "The Layout carrier describes the size and alignment requirements for a memory allocation."
);

impl_rust_std_type!(
    LayoutError,
    "core",
    "core::alloc",
    "https://doc.rust-lang.org/core/alloc/struct.LayoutError.html",
    "The LayoutError carrier reports that computing a Layout would violate its size or alignment invariants."
);
