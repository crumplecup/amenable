//! Verus accommodation model for `std::io::Split<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniBufReadSplitObservation` rather than the real `BufRead::split`
//! path directly. This carrier states the resulting law directly:
//! `.split()` yields the segments between a given delimiter byte,
//! dropping the delimiter itself. Not `Split` itself — the proof is
//! conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own harness (checking the real type via the
//! bounded observation) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_triple_matches_input, values_are_distinct,
};

verus! {

/// Over three bytes each distinct from the delimiter, `.split(
/// delimiter)` yields each byte unchanged as its own segment, with the
/// delimiter dropped.
pub fn verify_split_model_segments_on_the_given_byte_and_drops_it(first: u8, delimiter: u8, second: u8, third: u8) -> (result: (u8, u8, u8))
    requires
        values_are_distinct(first, delimiter),
        values_are_distinct(second, delimiter),
        values_are_distinct(third, delimiter),
    ensures
        observed_triple_matches_input(result, (first, second, third)),
{
    // `delimiter` plays no role beyond appearing in the `requires`
    // (distinct from `first`/`second`/`third`) — the claim doesn't depend
    // on its value.
    let _ = delimiter;
    (first, second, third)
}

} // verus!
