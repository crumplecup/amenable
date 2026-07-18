//! `RustStdType` registrations for `alloc::sync`.
//!
//! `UniqueArc<T>` is deliberately not covered here — unstable
//! (`unique_rc_arc`). As with `Box`, `Arc<T>`/`Weak<T>` cover only the
//! default-allocator case.

use std::sync::{Arc, Weak as ArcWeak};

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    Arc,
    "alloc",
    "alloc::sync",
    "https://doc.rust-lang.org/alloc/sync/struct.Arc.html",
    "The Arc carrier is a thread-safe reference-counted, shared owner of its contents."
);

impl_rust_std_type_generic1!(
    ArcWeak,
    "alloc",
    "alloc::sync",
    "https://doc.rust-lang.org/alloc/sync/struct.Weak.html",
    "The Weak carrier is a non-owning reference to an Arc's contents that does not keep them alive."
);

register_rust_std_standard_evidence!(Arc<i32>, ArcWeak<i32>);
