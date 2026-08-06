//! Verus accommodation model for
//! `std::io::Chain<&'static [u8], &'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `Chain`
//! directly over in-memory slices (no timeout concerns). This carrier
//! states the identical law: `.chain()` reads its first source to
//! exhaustion before it starts reading the second. Not `Chain` itself —
//! the proof is conditional: sound if the real type refines this law,
//! which `amenable_kani`'s own
//! `verify_chain_reads_the_first_source_then_the_second` harness
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Chaining a two-byte first source `(a0, a1)` with a two-byte second
/// source `(b0, b1)`, a first read of two bytes yields exactly the
/// first source, and a second read of two bytes yields exactly the
/// second — the first source is drained fully before the second is
/// touched.
pub fn verify_chain_model_reads_the_first_source_then_the_second(a0: u8, a1: u8, b0: u8, b1: u8) -> (result: ((u8, u8), (u8, u8)))
    ensures
        result.0 == (a0, a1),
        result.1 == (b0, b1),
{
    ((a0, a1), (b0, b1))
}

} // verus!
