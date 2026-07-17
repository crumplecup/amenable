//! `RustStdType` registrations for `core::mem`.
//!
//! `core::mem::type_info::*` (the reflection API) and `core::mem::Alignment`
//! (unstable `ptr_alignment_type`, rust-lang/rust#102070) are deliberately
//! not covered here — nightly-only, not part of amenable_std's
//! stable-toolchain scope.

use std::mem::{Discriminant, ManuallyDrop};

use crate::rust_std::macros::impl_rust_std_type_generic1;

impl_rust_std_type_generic1!(
    ManuallyDrop,
    "core",
    "core::mem",
    "https://doc.rust-lang.org/core/mem/struct.ManuallyDrop.html",
    "The ManuallyDrop carrier wraps a value and suppresses running its Drop implementation."
);

impl_rust_std_type_generic1!(
    Discriminant,
    "core",
    "core::mem",
    "https://doc.rust-lang.org/core/mem/struct.Discriminant.html",
    "The Discriminant carrier identifies which variant of an enum a value holds, without holding the value itself."
);
