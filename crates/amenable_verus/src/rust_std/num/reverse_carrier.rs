//! Verus spec for `std::cmp::Reverse<i32>`.
//!
//! Not the comparison-inversion claim Kani/Creusot check (`Reverse<T>`'s
//! `.cmp()` swaps `T`'s ordering) — genuinely blocked under Verus for the
//! same structural reason as `wrapping_carrier.rs`'s `Add` claim (see
//! `amenable_std::verus_gallery`'s `wrapping_add_operator_blocked_by_
//! coherence` finding), just through `Ord` instead of `Add`: `Reverse`'s
//! `Ord` impl routes through vstd's own `OrdSpecImpl` extension-trait
//! machinery, which only vstd itself can implement for a foreign type.
//! Confirmed directly (not assumed from the `Add` case): a fully generic
//! `assume_specification` on `cmp` gets past the signature-match stage
//! this time, but with no way to state a real `ensures` for a bare
//! generic `T` (the same `obeys_*_spec` regress the `Add` case hits),
//! calling it leaves the postcondition genuinely unproved. Real, narrower
//! coverage lands the same way as `Wrapping`/`Saturating`: the tuple
//! constructor/field roundtrip.

use std::cmp::Reverse;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
pub struct ExReverse<T>(Reverse<T>);

/// `Reverse(value).0` recovers exactly `value` — the real tuple
/// constructor/field access, not a claim about the `Ord` impl.
pub fn verify_reverse_field_roundtrips_the_constructed_value(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    Reverse(value).0
}

} // verus!
