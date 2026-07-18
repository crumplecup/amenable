//! `RustStdType` registrations for `alloc::rc`.
//!
//! `UniqueRc<T>` is deliberately not covered here — unstable
//! (`unique_rc_arc`). As with `Box`, `Rc<T>`/`Weak<T>` cover only the
//! default-allocator case.

use std::rc::{Rc, Weak};

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    Rc,
    "alloc",
    "alloc::rc",
    "https://doc.rust-lang.org/alloc/rc/struct.Rc.html",
    "The Rc carrier is a single-threaded reference-counted, shared owner of its contents."
);

impl_rust_std_type_generic1!(
    Weak,
    "alloc",
    "alloc::rc",
    "https://doc.rust-lang.org/alloc/rc/struct.Weak.html",
    "The Weak carrier is a non-owning reference to an Rc's contents that does not keep them alive."
);

// Written as the fully-qualified `std::rc::Weak<i32>`, not the bare `Weak`
// imported above: `alloc::sync::Weak` shares the same bare name, and
// `register_rust_std_standard_evidence!`'s `stringify!`-derived evidence
// string is the only thing that disambiguates the two for tooling reading
// the registry (e.g. `elicit_doc`'s coverage report) — a bare `Weak<i32>`
// here would collide with `alloc_sync.rs`'s registration.
register_rust_std_standard_evidence!(Rc<i32>, std::rc::Weak<i32>);
