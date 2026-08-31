//! Verus spec for `core::array::TryFromSliceError`.
//!
//! Reaching the real, matching `assume_specification` signature took
//! several rounds — the durable lesson (see
//! `amenable_std::verus_gallery`'s `try_from_slice_signature_needs_a_
//! phantom_lifetime_binder` finding): `verus` prints this kind of impl
//! as TWO separate generic binder groups (`for<'_0, T, N> for<'_>`), and
//! only the second one governs the argument's actual lifetime — the
//! impl's own lifetime parameter has to appear in the trait reference
//! (`TryFrom<&'a [T]>`) while the argument itself stays elided (bare
//! `&[T]`).
//!
//! A second, separate lesson: a plain `match try_from(...) { Ok(x) =>
//! ..., Err(_) => ... }` expression doesn't let `verus` connect the
//! scrutinee call's postcondition to the match at all (every fact about
//! `x` reads as unknown inside the `Ok` arm, even ones the call's own
//! `ensures` states plainly) — binding the call's result to a `let`
//! first, then asserting its shape, then `.unwrap()`-ing it, does.

#[cfg(verus_keep_ghost)]
use std::array::TryFromSliceError;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::{does_not_have_length, has_length};

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExTryFromSliceError(TryFromSliceError);

/// `<[T; N]>::try_from`'s whole postcondition: succeeds and round-trips
/// the elements when the slice's length matches `N` exactly, otherwise
/// fails.
pub open spec fn try_from_slice_result_matches<T: Copy, const N: usize>(slice: Seq<T>, result: Result<[T; N], TryFromSliceError>) -> bool {
    (slice.len() == N ==> (result is Ok && result->Ok_0@ == slice))
        && (slice.len() != N ==> result is Err)
}

pub assume_specification<'a, T: Copy, const N: usize> [<[T; N] as core::convert::TryFrom<&'a [T]>>::try_from] (slice: &[T]) -> (result: Result<[T; N], TryFromSliceError>)
    ensures
        try_from_slice_result_matches(slice@, result),
;

/// `<[T; N]>::try_from` succeeds only when the slice's length matches
/// `N` exactly, and round-trips the elements when it does — the same
/// claim the Kani harness checks. Takes `matching`/`mismatched` as
/// `requires`-constrained parameters (the same "parameter, not inline
/// literal" shape as `option_carrier.rs`) rather than the literal
/// arrays Kani constructs inline.
pub fn verify_try_from_slice_rejects_a_length_mismatch(matching: &[i32], mismatched: &[i32]) -> (result: (bool, bool))
    requires
        has_length(matching@, 2),
        does_not_have_length(mismatched@, 2),
    ensures
        result.0,
        result.1,
{
    let converted = <[i32; 2]>::try_from(matching);
    assert(converted is Ok);
    let arr = converted.expect("proven Ok by the assert immediately above");
    let round_trips = arr[0] == matching[0] && arr[1] == matching[1];

    let rejected_result = <[i32; 2]>::try_from(mismatched);
    assert(rejected_result is Err);
    let rejected = rejected_result.is_err();

    (round_trips, rejected)
}

} // verus!
