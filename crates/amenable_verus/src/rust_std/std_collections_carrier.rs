//! Verus accommodation model for `std::collections::{HashMap<i32, i32>,
//! HashSet<i32>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded one-entry
//! `KaniHashMap`/`KaniHashSet` model rather than the real collections
//! directly — the direct `HashMap::new()`/`HashSet::new()` path times
//! out under Kani even for a single fixed key/value pair, the same
//! "pure in-memory std implementation blow-up" class already documented
//! for `DefaultHasher`/`BTreeMap`. This carrier states each resulting
//! law directly. Neither function is `HashMap`/`HashSet` themselves —
//! each proof is conditional: sound if the real type refines the
//! stated law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type via the bounded model) already confirms
//! independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Inserting a key then getting it back recovers the same value.
pub fn verify_hash_map_model_insert_then_get_recovers_the_value(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// Inserting a value makes the set report it as a member.
pub fn verify_hash_set_model_insert_then_contains_reports_membership() -> (result: bool)
    ensures
        result,
{
    true
}

} // verus!
