//! Verus accommodation model for `jiff::SignedDuration::new`'s
//! normalization.
//!
//! Same zero-`vstd`-coverage gap `offset.rs`/`error.rs` document.
//! Reproduces jiff's real two-step algorithm (confirmed by reading
//! jiff 0.2.35's real source directly, not assumed): first, any
//! whole-second excess in `nanos` (`|nanos| >= 1_000_000_000`) carries
//! into `secs`; then, if the (possibly-carried) `secs` and the
//! remaining `nanos` end up with *different* signs, one more `±1`
//! shifts between them so the two share a sign (or either is zero).
//! That second step is the genuinely easy-to-miss part — unlike
//! `std::time::Duration`, `subsec_nanos()` is not always non-negative
//! here, confirmed empirically (not assumed) before modeling it: e.g.
//! `SignedDuration::new(-5, i32::MAX).subsec_nanos()` is `-852516353`.
//! The same claim `amenable_kani::ext::jiff::signed_duration`'s real
//! Kani harness and `amenable_creusot::ext_jiff::signed_duration`'s
//! real `extern_spec!` both independently confirm against jiff's
//! actual API.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The normalization law this model establishes, stated the same way
/// the real Kani/Creusot proofs for the identical claim do: the total
/// nanosecond count is preserved (via an `i64`-only reformulation that
/// avoids `i128`, matching the real Kani harness's own fix for a
/// genuine CBMC timeout at the straightforward version), the remainder
/// stays within one second in magnitude, and the two components end up
/// sharing a sign (or either is zero).
pub open spec fn signed_duration_new_model_normalizes(
    secs: i64,
    nanos: i32,
    secs_out: i64,
    subsec_out: i32,
) -> bool {
    (nanos as int) - (subsec_out as int) == (secs_out as int - secs as int) * 1_000_000_000
        && subsec_out < 1_000_000_000 && subsec_out > -1_000_000_000
        && (secs_out == 0 || subsec_out == 0 || (secs_out > 0) == (subsec_out > 0))
}

/// A model of `(SignedDuration::new(secs, nanos).as_secs(),
/// SignedDuration::new(secs, nanos).subsec_nanos())`, reproducing
/// jiff's real carry-then-sign-fix algorithm — the same claim
/// `amenable_kani::ext::jiff::signed_duration::
/// verify_signed_duration_new_normalizes_nanos_and_carries_into_secs`
/// and `amenable_creusot::ext_jiff::signed_duration::
/// verify_signed_duration_new_normalizes_nanos_and_carries_into_secs`
/// check against jiff's real API.
pub fn verify_signed_duration_new_model_normalizes_nanos_and_carries_into_secs(
    secs: i64,
    nanos: i32,
) -> (result: (i64, i32))
    requires
        secs > i64::MIN + 3, secs < i64::MAX - 3,
    ensures
        signed_duration_new_model_normalizes(secs, nanos, result.0, result.1),
{
    // Step 1: carry any whole-second excess in `nanos` into `secs`.
    let (mut secs, mut nanos) = if nanos <= -1_000_000_000 || nanos >= 1_000_000_000 {
        let addsecs = nanos / 1_000_000_000;
        (secs + addsecs as i64, nanos % 1_000_000_000)
    } else {
        (secs, nanos)
    };
    // Step 2: if `secs`/`nanos` now have different signs, shift `±1`
    // between them so they share a sign (or either is zero).
    if nanos != 0 && secs != 0 {
        if secs < 0 && nanos > 0 {
            secs += 1;
            nanos -= 1_000_000_000;
        } else if secs > 0 && nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
    }
    (secs, nanos)
}

} // verus!
