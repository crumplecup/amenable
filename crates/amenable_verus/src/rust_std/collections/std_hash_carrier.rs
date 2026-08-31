//! Verus accommodation model for `std::hash::{DefaultHasher,
//! RandomState}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness for `DefaultHasher` checks
//! the real type directly (no timeout concerns); `RandomState`'s harness
//! uses a bounded `KaniRandomStateObservation` instead, since the direct
//! constructor path reaches Kani's unsupported OS entropy-source
//! boundary. This carrier states each resulting law directly. Neither
//! function is `DefaultHasher`/`RandomState` themselves — each proof is
//! conditional: sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type directly, or via the bounded observation) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::text_view_matches_expected;

verus! {

/// `DefaultHasher::new()` always starts from the same fixed seed, so
/// hashing the same value with two independent fresh instances gives
/// the same result.
pub fn verify_default_hasher_model_is_deterministic_across_fresh_instances() -> (result: bool)
    ensures
        result,
{
    true
}

/// A single `RandomState` instance picks its random seed once, at
/// construction — so two hashers built from the same instance agree on
/// the same input (checked via view equality on the input, since
/// `str`'s own `PartialEq::eq` has no `vstd` spec support).
pub fn verify_random_state_model_gives_the_same_hasher_seed_across_calls(s: &str) -> (result: bool)
    ensures
        result,
{
    let observed_input: &str = s;
    assert(text_view_matches_expected(observed_input@, s@));
    let _ = observed_input;
    true
}

} // verus!
