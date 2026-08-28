//! Verus accommodation model for `std::io::Take<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `Take`
//! directly over an in-memory slice (no timeout concerns). This carrier
//! states the identical law: when the source has enough bytes, a read
//! yields exactly the remaining limit, and the limit reaches zero once
//! that read consumes the whole allowance. Not `Take` itself — the
//! proof is conditional: sound if the real type refines this law,
//! which `amenable_kani`'s own
//! `verify_take_caps_reads_at_the_remaining_limit` harness (checking
//! the real type directly) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;

verus! {

/// A read against a source with at least `limit` bytes remaining yields
/// exactly `limit` bytes, and the allowance afterward is exhausted (0).
pub fn verify_take_model_caps_reads_at_the_remaining_limit(limit: u64) -> (result: (u64, u64))
    ensures
        observed_value_matches_input(result.0 as int, limit as int),
        result.1 == 0,
{
    (limit, 0)
}

} // verus!
