//! `RustStdType` registrations for `core::cmp`.

use std::cmp::{Ordering, Reverse};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, register_rust_std_standard_evidence,
};

impl_rust_std_type!(
    Ordering,
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

// Evidence registration is per concrete type (see `register_rust_std_
// standard_evidence!`'s own doc comment) — `Reverse<i32>` is the one
// concrete instantiation this module's proof batch covers.
register_rust_std_standard_evidence!(Ordering, Reverse<i32>);
