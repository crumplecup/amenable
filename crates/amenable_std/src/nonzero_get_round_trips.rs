//! The `NonZero<T>::get()` round-trip postcondition, named once per
//! numeric width instead of restated per proof.
//!
//! `core::num::NonZero<T>::get()` returns exactly the value the carrier
//! was constructed with — independently checked, identically, in each of
//! `amenable_kani::rust_std::num`'s twelve `verify_nonzero_*` harnesses
//! (one per numeric width Rust supports `NonZero` over). Each width needs
//! its own type here rather than one generic wrapper: `RustStdStandard<
//! NonZero<T>>` already carries a *different* `Ensures<KaniVerifier>`
//! impl for the same crate (`NonZero::new`'s own `value != 0`
//! construction precondition), and a trait's associated types can only
//! be claimed once per concrete `Self` — this is a second, distinct
//! claim (the accessor's postcondition), so it needs its own `Self` per
//! width.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// Defines one `NonZero{Width}GetRoundTrips` wrapper type per numeric
/// width: a value already known to be what `NonZero<T>::get()` returned
/// after being constructed with it.
///
/// `#[doc = concat!(...)]` (not a plain `///`) is what lets this macro
/// give each generated type its own real doc comment naming its width —
/// unlike `#[standard(basis = ...)]` below, which needs a literal string
/// per width because `standard` is a third-party derive attribute parsed
/// directly as a string literal, not expanded through `concat!`/
/// `stringify!` the way the compiler's own built-in `#[doc]` attribute is.
macro_rules! nonzero_get_round_trips {
    ($(($ty:ty, $name:ident, $basis:literal, $basis_ctor:literal, $provenance:literal)),* $(,)?) => {
        $(
            #[doc = concat!(
                "A `", stringify!($ty), "` already known to be `NonZero<",
                stringify!($ty), ">::get()`'s round-tripped result.\n\n",
                "A derived claim about `", stringify!($ty), "`, not a fresh root ",
                "authority — its evidence chain rests on `", stringify!($ty), "`'s ",
                "own already-registered standard-library provenance ",
                "([`RustStdStandard<", stringify!($ty), ">`])."
            )]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
            #[standard(
                basis = $basis,
                basis_ctor = $basis_ctor,
                provenance = $provenance,
                provenance_type = "RustStdProvenance"
            )]
            pub struct $name {
                value: $ty,
            }

            impl $name {
                #[doc = concat!(
                    "Wrap a `", stringify!($ty), "` already known to be the ",
                    "round-tripped result."
                )]
                pub const fn new(value: $ty) -> Self {
                    Self { value }
                }

                #[doc = "The wrapped value."]
                pub const fn value(&self) -> $ty {
                    self.value
                }
            }
        )*
    };
}

nonzero_get_round_trips! {
    (i8, NonZeroI8GetRoundTrips, "RustStdStandard<i8>", "RustStdStandard::<i8>::new()", "<i8 as RustStdType>::provenance()"),
    (i16, NonZeroI16GetRoundTrips, "RustStdStandard<i16>", "RustStdStandard::<i16>::new()", "<i16 as RustStdType>::provenance()"),
    (i32, NonZeroI32GetRoundTrips, "RustStdStandard<i32>", "RustStdStandard::<i32>::new()", "<i32 as RustStdType>::provenance()"),
    (i64, NonZeroI64GetRoundTrips, "RustStdStandard<i64>", "RustStdStandard::<i64>::new()", "<i64 as RustStdType>::provenance()"),
    (i128, NonZeroI128GetRoundTrips, "RustStdStandard<i128>", "RustStdStandard::<i128>::new()", "<i128 as RustStdType>::provenance()"),
    (isize, NonZeroIsizeGetRoundTrips, "RustStdStandard<isize>", "RustStdStandard::<isize>::new()", "<isize as RustStdType>::provenance()"),
    (u8, NonZeroU8GetRoundTrips, "RustStdStandard<u8>", "RustStdStandard::<u8>::new()", "<u8 as RustStdType>::provenance()"),
    (u16, NonZeroU16GetRoundTrips, "RustStdStandard<u16>", "RustStdStandard::<u16>::new()", "<u16 as RustStdType>::provenance()"),
    (u32, NonZeroU32GetRoundTrips, "RustStdStandard<u32>", "RustStdStandard::<u32>::new()", "<u32 as RustStdType>::provenance()"),
    (u64, NonZeroU64GetRoundTrips, "RustStdStandard<u64>", "RustStdStandard::<u64>::new()", "<u64 as RustStdType>::provenance()"),
    (u128, NonZeroU128GetRoundTrips, "RustStdStandard<u128>", "RustStdStandard::<u128>::new()", "<u128 as RustStdType>::provenance()"),
    (usize, NonZeroUsizeGetRoundTrips, "RustStdStandard<usize>", "RustStdStandard::<usize>::new()", "<usize as RustStdType>::provenance()"),
}
