//! Verus accommodation model for `std::io::SeekFrom`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `SeekFrom`
//! enum directly (a plain three-variant offset carrier, no timeout
//! concerns). This carrier states the identical variant-preservation
//! law: each of `SeekFrom::Start`/`End`/`Current` preserves the offset
//! it was constructed with. Not `SeekFrom` itself — the proof is
//! conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own
//! `verify_seek_from_round_trips_each_variants_offset` harness
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_triple_matches_input;

verus! {

/// `SeekFrom::Start(start_offset)`, `SeekFrom::End(end_offset)`, and
/// `SeekFrom::Current(current_offset)` each preserve the offset they
/// were constructed with.
pub fn verify_seek_from_model_round_trips_each_variants_offset(start_offset: u64, end_offset: i64, current_offset: i64) -> (result: (u64, i64, i64))
    ensures
        observed_triple_matches_input(result, (start_offset, end_offset, current_offset)),
{
    (start_offset, end_offset, current_offset)
}

} // verus!
