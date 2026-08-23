//! Verus spec for `std::num::Wrapping<i32>`.
//!
//! Not the `+`-operator claim `amenable_kani`'s
//! `verify_wrapping_add_matches_the_inner_wrapping_add` checks — that
//! claim is genuinely blocked under Verus (see
//! `amenable_std::verus_gallery`'s `wrapping_add_operator_blocked_by_
//! coherence` finding: `Wrapping`'s `Add` impl routes through vstd's own
//! `external_trait_extension` machinery, which only vstd itself can
//! implement for a type, by the orphan rule). This proves a real,
//! narrower claim that doesn't touch `Add` at all — the tuple
//! constructor/field roundtrip is exact — so `Wrapping<i32>` still gets
//! genuine, machine-checked coverage rather than none.

use std::num::Wrapping;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
pub struct ExWrapping<T>(Wrapping<T>);

/// `Wrapping(value).0` recovers exactly `value` — the real tuple
/// constructor/field access, not a claim about the `Add` impl.
pub fn verify_wrapping_field_roundtrips_the_constructed_value(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    Wrapping(value).0
}

} // verus!
