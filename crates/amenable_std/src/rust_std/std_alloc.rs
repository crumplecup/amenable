//! `RustStdType` registrations for `std::alloc`.
//!
//! Unlike `alloc::alloc::Global`, `System` is stable — it predates the
//! generic `allocator_api` infrastructure.

use std::alloc::System;

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    System,
    "std",
    "std::alloc",
    "https://doc.rust-lang.org/std/alloc/struct.System.html",
    "The System carrier is a GlobalAlloc implementation that requests memory from the operating system's allocator."
);

register_rust_std_standard_evidence!(System);
