//! `KaniWitness` impls for Rust's scalar primitives and `String`.
//!
//! `array`/`fn`/`pointer`/`reference`/`tuple` (the compound primitives, one
//! representative concrete instantiation each) get real checked proofs
//! below rather than the trusted disposition: each has a genuine, non-ZST
//! semantic property worth checking (indexing, field access, dereferencing,
//! calling). `pointer`'s proofs deliberately never dereference the raw
//! pointer -- only checked properties safe code can establish (address
//! reproducibility from a cast), never `unsafe`. `unit` (`()`) is the one
//! exception, trusted alongside the scalars: it has exactly one possible
//! value, nothing to check.
//!
//! [`scalars`] is the trusted-scalar registration and the numeric/hash
//! contract impls (`RustStdStandard<i32>`'s `checked_add` precondition
//! and so on) several *other* files across this crate share -- not
//! specific to any one compound-type family below, so it stands alone.
//! The rest split by compound-type family: [`char_string`], [`ascii`],
//! [`array_slice_str`], [`range_bounds`] (the four generic bound-check
//! markers), [`tuple_fn_ptr`], [`raw_pointers`], and [`references`].

mod array_slice_str;
mod ascii;
mod char_string;
mod range_bounds;
mod raw_pointers;
mod references;
mod scalars;
mod tuple_fn_ptr;

pub use ascii::{FourBytesAreEachAscii, ThreeBytesAreEachAscii, ValueIsWithinInclusiveRange};
pub use char_string::EmptinessTracksZeroLength;
pub use range_bounds::{ValueIsAtLeast, ValueIsBelow, ValueIsOutsideInclusiveRange};
pub use references::{
    DerefReflectsTheStoredValue, FieldAccessRecoversTheStoredValue, IndexRecoversTheStoredElement,
};
