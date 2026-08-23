//! Verus spec for `std::num::Saturating<i32>`.
//!
//! Same shape and same real blocker as `wrapping_carrier.rs`: the
//! Add-operator claim Kani/Creusot check (`Saturating<T>`'s `+` saturates
//! exactly like the inner type's `saturating_add`) is genuinely
//! unreachable under Verus from this crate — `Add` is under vstd's own
//! `external_trait_extension`, and only vstd can implement its
//! `AddSpecImpl` extension trait for a foreign type. See
//! `amenable_std::verus_gallery`'s `wrapping_add_operator_blocked_by_
//! coherence` finding; the identical mechanism applies here (confirmed
//! against the real toolchain below, not assumed). Real, narrower
//! coverage lands the same way: the tuple constructor/field roundtrip.

use std::num::Saturating;

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
pub struct ExSaturating<T>(Saturating<T>);

/// `Saturating(value).0` recovers exactly `value` — the real tuple
/// constructor/field access, not a claim about the `Add` impl.
pub fn verify_saturating_field_roundtrips_the_constructed_value(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    Saturating(value).0
}

} // verus!
