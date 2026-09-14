//! Real Creusot proof content for `jiff::SignedDuration`'s
//! normalization property (`ext::jiff::signed_duration` holds the
//! `CreusotWitness` bridge) — the same claim `amenable_kani::ext::jiff::
//! signed_duration`'s real Kani harness checks by symbolic execution,
//! restated as a real Creusot postcondition.
//!
//! `jiff::SignedDuration` is uncontracted everywhere (no `creusot-std`/
//! `elicitation` prior art — a third-party type). `as_secs`/
//! `subsec_nanos` read private fields, so each gets an opaque logic
//! accessor (the same shape `offset.rs`'s `offset_seconds_value` uses),
//! and `new`'s postcondition is stated in terms of those same two
//! accessors so both extern_specs agree on what the constructed value
//! actually is. `new` can panic (it's `-> SignedDuration`, not a
//! `Result`) when the nanosecond carry overflows `i64` — the
//! `#[requires]` below rules that out the same way the real Kani
//! harness's precondition does, confirmed to be the identical bound.
//! The `i64`-only nanos-preserved reformulation (rather than a
//! straightforward `i128` multiplication) mirrors the real Kani
//! harness's own fix for a genuine CBMC timeout at the `i128` version —
//! not needed for Creusot's own solving cost, but kept identical across
//! both backends so the same claim reads the same way in both places.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, extern_spec, logic, requires, trusted};

    #[trusted]
    #[logic(opaque)]
    pub(super) fn signed_duration_secs_value(_d: &jiff::SignedDuration) -> i64 {
        dead
    }

    #[trusted]
    #[logic(opaque)]
    pub(super) fn signed_duration_subsec_nanos_value(_d: &jiff::SignedDuration) -> i32 {
        dead
    }
}
#[cfg(creusot)]
use mirror::{
    ensures, extern_spec, logic, requires, signed_duration_secs_value,
    signed_duration_subsec_nanos_value,
};

#[cfg(creusot)]
extern_spec! {
    impl jiff::SignedDuration {
        #[ensures(result == signed_duration_secs_value(self))]
        fn as_secs(&self) -> i64;

        #[ensures(result == signed_duration_subsec_nanos_value(self))]
        fn subsec_nanos(&self) -> i32;

        #[requires(secs > i64::MIN + 3i64 && secs < i64::MAX - 3i64)]
        #[ensures(
            (nanos as i64) - (signed_duration_subsec_nanos_value(&result) as i64)
                == (signed_duration_secs_value(&result) - secs) * 1_000_000_000i64
        )]
        #[ensures(
            signed_duration_subsec_nanos_value(&result) < 1_000_000_000i32
                && signed_duration_subsec_nanos_value(&result) > -1_000_000_000i32
        )]
        #[ensures(
            signed_duration_secs_value(&result) == 0i64
                || signed_duration_subsec_nanos_value(&result) == 0i32
                || (signed_duration_secs_value(&result) > 0i64)
                    == (signed_duration_subsec_nanos_value(&result) > 0i32)
        )]
        fn new(secs: i64, nanos: i32) -> jiff::SignedDuration;
    }
}

amenable_derive::harness! {
    creusot, SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::SignedDuration>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it. States the same three facts
        /// the `extern_spec!` above already axiomatizes on `new`
        /// itself, applied to the harness's own return value.
        #[logic(open)]
        fn signed_duration_new_normalizes_nanos_and_carries_into_secs_holds(
            secs: i64,
            nanos: i32,
            outcome: (i64, i32),
        ) -> bool {
            pearlite! {
                (nanos as i64) - (outcome.1 as i64) == (outcome.0 - secs) * 1_000_000_000i64
                    && outcome.1 < 1_000_000_000i32 && outcome.1 > -1_000_000_000i32
                    && (outcome.0 == 0i64 || outcome.1 == 0i32 || (outcome.0 > 0i64) == (outcome.1 > 0i32))
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `SignedDuration::new` preserves the total nanosecond count
        /// while normalizing — the same claim `amenable_kani::ext::
        /// jiff::signed_duration`'s real Kani harness checks by
        /// symbolic execution, resting on the `extern_spec!` above.
        #[requires(secs > i64::MIN + 3i64 && secs < i64::MAX - 3i64)]
        #[ensures(signed_duration_new_normalizes_nanos_and_carries_into_secs_holds(secs, nanos, result))]
        fn verify_signed_duration_new_normalizes_nanos_and_carries_into_secs(
            secs: i64,
            nanos: i32,
        ) -> (i64, i32) {
            let d = jiff::SignedDuration::new(secs, nanos);
            (d.as_secs(), d.subsec_nanos())
        }
    }
}
