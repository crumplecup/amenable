//! `RustStdType` registrations for `alloc::rc`.
//!
//! `UniqueRc<T>` is deliberately not covered here — unstable
//! (`unique_rc_arc`). As with `Box`, `Rc<T>`/`Weak<T>` cover only the
//! default-allocator case.

use std::rc::{Rc, Weak as RcWeak};

use crate::rust_std::macros::impl_rust_std_type_generic1;

impl_rust_std_type_generic1!(
    Rc,
    "alloc",
    "alloc::rc",
    "https://doc.rust-lang.org/alloc/rc/struct.Rc.html",
    "The Rc carrier is a single-threaded reference-counted, shared owner of its contents."
);

impl_rust_std_type_generic1!(
    RcWeak,
    "alloc",
    "alloc::rc",
    "https://doc.rust-lang.org/alloc/rc/struct.Weak.html",
    "The Weak carrier is a non-owning reference to an Rc's contents that does not keep them alive."
);
