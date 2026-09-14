//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::SignedDuration>` —
//! a real, checked normalization property over jiff's actual public API
//! (`SignedDuration::new`/`as_secs`/`subsec_nanos`), not a trusted stub.
//!
//! `SignedDuration` is semantically the signed counterpart of
//! `std::time::Duration`, which already has a real Kani proof in this
//! codebase for a similarly-shaped claim
//! (`amenable_kani::rust_std::time::
//! verify_duration_new_normalizes_nanos_and_carries_into_secs`) — but
//! the *sign* convention genuinely differs, confirmed empirically (not
//! assumed) before writing this harness: `std::time::Duration::
//! subsec_nanos()` is always non-negative, but `SignedDuration::
//! subsec_nanos()` is **not** — e.g. `SignedDuration::new(-5,
//! i32::MAX).subsec_nanos()` is `-852516353`. The real invariant is
//! that `as_secs()`/`subsec_nanos()` share the same sign (or either is
//! zero), not that `subsec_nanos()` is non-negative. A first version of
//! this harness stated the `std::time::Duration`-shaped claim and the
//! real Kani run failed it for real (not a timeout) — corrected by
//! probing jiff's actual behavior directly before restating the
//! property, not by guessing again.
//!
//! A second version, correct but stated via `i128` arithmetic (`secs as
//! i128 * 1_000_000_000 + nanos as i128`), timed out at 3 minutes over
//! the full `i64 x i32` domain — a real scale wall, not a logic bug
//! this time. Confirmed by direct reformulation, not guessed: `(nanos -
//! subsec_out) as i64 == (secs_out - secs) * 1_000_000_000i64` states
//! the identical fact using only `i64` arithmetic (the carry `secs_out
//! - secs` is always small), and verifies well inside the timeout.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::SignedDuration> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_signed_duration_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::SignedDuration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::SignedDuration>",
        "kani",
        || <ExtStandard<jiff::SignedDuration> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::SignedDuration>,
    "amenable_ext::ExtStandard<jiff::SignedDuration>",
    (i64, i32),
    |(secs, nanos)| {
        if secs <= i64::MIN + 3 || secs >= i64::MAX - 3 {
            true
        } else {
            let d = jiff::SignedDuration::new(secs, nanos);
            let secs_out = d.as_secs();
            let subsec_out = d.subsec_nanos();
            // Equivalent to `secs*1e9 + nanos == secs_out*1e9 + subsec_out`
            // (confirmed empirically before writing this), but entirely
            // in `i64` -- `secs_out - secs` is small (the carry is at
            // most a few seconds), so no `i128` widening is needed, and
            // this checks meaningfully faster under Kani than the
            // straightforward `i128`-multiplication version did (which
            // timed out at 3 minutes over the full `i64 x i32` domain).
            let nanos_preserved =
                (nanos as i64) - (subsec_out as i64) == (secs_out - secs) * 1_000_000_000i64;
            let same_sign_or_zero = secs_out == 0
                || subsec_out == 0
                || secs_out.signum() == i64::from(subsec_out.signum());
            nanos_preserved && subsec_out.unsigned_abs() < 1_000_000_000 && same_sign_or_zero
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `SignedDuration::new` does not require `nanos` to already be
        /// within one second — it normalizes: the total nanosecond
        /// count (`secs * 1e9 + nanos`, checked via an `i64`-only
        /// reformulation that avoids `i128` widening — see this
        /// module's own `ensures` closure) is preserved exactly,
        /// `subsec_nanos()` stays within one second in magnitude, and
        /// `as_secs()`/
        /// `subsec_nanos()` end up sharing a sign (or either is zero) —
        /// unlike `std::time::Duration`, `subsec_nanos()` is not always
        /// non-negative here, confirmed against jiff's real behavior
        /// before stating this claim (see this module's own doc
        /// comment). Checked for every `(i64, i32)` pair a few seconds
        /// away from the `i64` extremes (where the carry would panic on
        /// overflow, per `SignedDuration::new`'s own documented
        /// behavior).
        #[kani::proof]
        fn verify_signed_duration_new_normalizes_nanos_and_carries_into_secs() {
            let secs: i64 = kani::any();
            let nanos: i32 = kani::any();
            assert!(
                ExtStandard::<jiff::SignedDuration>::ensures((secs, nanos)),
                "SignedDuration::new must preserve the total nanosecond count and normalize \
                 as_secs()/subsec_nanos() to share a sign (or either zero)"
            );
        }
    }
}
