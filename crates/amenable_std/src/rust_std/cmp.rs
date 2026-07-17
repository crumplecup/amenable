//! `RustStdType` registrations for `core::cmp`.

use std::cmp::Reverse;

use crate::rust_std::macros::{impl_rust_std_type, impl_rust_std_type_generic1};

impl_rust_std_type!(
    core::cmp::Ordering,
    "core",
    "core::cmp",
    "https://doc.rust-lang.org/core/cmp/enum.Ordering.html",
    "The three-way comparison carrier admits exactly Less, Equal, or Greater."
);

impl_rust_std_type_generic1!(
    Reverse,
    "core",
    "core::cmp",
    "https://doc.rust-lang.org/core/cmp/struct.Reverse.html",
    "The Reverse carrier wraps a value and inverts its Ord/PartialOrd comparison direction."
);
