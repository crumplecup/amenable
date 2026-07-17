//! `RustStdType` registrations for `core::ops` (ranges, `Bound`, `ControlFlow`).
//!
//! `core::range::*` (the newer, still-unstable range types behind
//! `new_range_api`) is deliberately not covered here — it isn't nameable
//! from a stable toolchain, and amenable_std stays on stable.

use std::ops::{Bound, ControlFlow, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    register_rust_std_standard_evidence,
};

impl_rust_std_type_generic1!(
    Range,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.Range.html",
    "The Range carrier stores a half-open start..end interval."
);

impl_rust_std_type_generic1!(
    RangeFrom,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.RangeFrom.html",
    "The RangeFrom carrier stores an unbounded-above start.. interval."
);

impl_rust_std_type_generic1!(
    RangeTo,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.RangeTo.html",
    "The RangeTo carrier stores an unbounded-below ..end interval."
);

impl_rust_std_type_generic1!(
    RangeToInclusive,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html",
    "The RangeToInclusive carrier stores an unbounded-below, end-inclusive ..=end interval."
);

impl_rust_std_type_generic1!(
    RangeInclusive,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html",
    "The RangeInclusive carrier stores a closed start..=end interval."
);

impl_rust_std_type!(
    core::ops::RangeFull,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/struct.RangeFull.html",
    "The RangeFull carrier represents the unbounded .. interval over an entire collection."
);

impl_rust_std_type_generic1!(
    Bound,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/enum.Bound.html",
    "The Bound carrier names one endpoint of a range as Included, Excluded, or Unbounded."
);

impl_rust_std_type_generic2!(
    ControlFlow,
    "core",
    "core::ops",
    "https://doc.rust-lang.org/core/ops/enum.ControlFlow.html",
    "The ControlFlow carrier represents an early-exit decision as Continue or Break."
);

// Evidence registration is per concrete type (see `register_rust_std_
// standard_evidence!`'s own doc comment) — `i32` is the one concrete
// element type this module's proof batch covers.
register_rust_std_standard_evidence!(
    Range<i32>,
    RangeFrom<i32>,
    RangeTo<i32>,
    RangeToInclusive<i32>,
    RangeInclusive<i32>,
    RangeFull,
    Bound<i32>,
    ControlFlow<i32, i32>,
);
