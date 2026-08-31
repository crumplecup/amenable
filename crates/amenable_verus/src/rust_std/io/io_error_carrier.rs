//! Verus accommodation model for `std::io::Error`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks a representative,
//! bounded-exhaustive subset of `ErrorKind` variants (`ErrorKind` has
//! no `kani::Arbitrary` impl — it's a large foreign enum with no way to
//! derive one), indexed `0..4`; this carrier models the same four-way
//! choice as a plain `u8` tag standing in for the real enum, rather
//! than naming `ErrorKind` (which `vstd` has no spec support for at
//! all). The law: `Error::from(kind).kind()` recovers exactly the given
//! kind. Not `std::io::Error` itself — the proof is conditional: sound
//! if the real type refines this law, which `amenable_kani`'s own
//! `verify_error_from_error_kind_preserves_the_kind` harness (checking
//! the real type directly, over the same four representative kinds)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_value_matches_input;

verus! {

/// A singleton claim: this model represents exactly the four
/// representative `ErrorKind` variants (`NotFound`/`PermissionDenied`/
/// `AlreadyExists`/`InvalidInput`) as a tag `0..4`. Named, not inlined,
/// so the assumption has an explicit source even though nothing else
/// calls it.
pub open spec fn error_kind_index_is_representative(index: u8) -> bool {
    index < 4
}

/// `Error::from(kind).kind()` recovers exactly the given kind, over any
/// of the four representative kinds (modeled as a tag `0..4`, standing
/// in for `NotFound`/`PermissionDenied`/`AlreadyExists`/`InvalidInput`).
pub fn verify_error_model_from_error_kind_preserves_the_kind(kind_index: u8) -> (result: u8)
    requires
        error_kind_index_is_representative(kind_index),
    ensures
        observed_value_matches_input(result as int, kind_index as int),
{
    kind_index
}

} // verus!
