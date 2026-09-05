//! `RustStdType` registrations for `alloc::sync`.
//!
//! `UniqueArc<T>` is deliberately not covered here — unstable
//! (`unique_rc_arc`). As with `Box`, `Arc<T>`/`Weak<T>` cover only the
//! default-allocator case.

use std::sync::{Arc, Weak};

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    Arc,
    "alloc",
    "alloc::sync",
    "https://doc.rust-lang.org/alloc/sync/struct.Arc.html",
    "The Arc carrier is a thread-safe reference-counted, shared owner of its contents."
);

impl_rust_std_type_generic1!(
    Weak,
    "alloc",
    "alloc::sync",
    "https://doc.rust-lang.org/alloc/sync/struct.Weak.html",
    "The Weak carrier is a non-owning reference to an Arc's contents that does not keep them alive."
);

// Written as the fully-qualified `std::sync::Weak<i32>`, not the bare
// `Weak` imported above: `alloc::rc::Weak` shares the same bare name, and
// `register_rust_std_standard_evidence!`'s `stringify!`-derived evidence
// string is the only thing that disambiguates the two for tooling reading
// the registry (e.g. `cordial`'s coverage report) — a bare `Weak<i32>`
// here would collide with `alloc_rc.rs`'s registration.
register_rust_std_standard_evidence!(Arc<i32>, std::sync::Weak<i32>);
