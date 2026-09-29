//! Interval and UTC-timeline ordering: start-precedes-end, non-negative duration, and total ordering of fixed instants.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
        UtcTimelineOrderingAppliesToFixedInstants,
    };

    use crate::CreusotVerifier;

    use super::{
        INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC, INTERVAL_START_PRECEDES_END_HOLDS_SRC,
        UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC,
        VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, VERIFY_INTERVAL_START_PRECEDES_END_SRC,
        VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC,
    };

    impl amenable_core::Witness<CreusotVerifier> for IntervalStartPrecedesEnd {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_interval_start_precedes_end".to_owned(),
                VERIFY_INTERVAL_START_PRECEDES_END_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for IntervalStartPrecedesEnd {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            INTERVAL_START_PRECEDES_END_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::IntervalStartPrecedesEnd",
            "creusot",
            || {
                <IntervalStartPrecedesEnd as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for IntervalDurationIsNonNegative {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_interval_duration_is_non_negative".to_owned(),
                VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for IntervalDurationIsNonNegative {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::IntervalDurationIsNonNegative",
            "creusot",
            || {
                <IntervalDurationIsNonNegative as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
                VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
            "creusot",
            || {
                <UtcTimelineOrderingAppliesToFixedInstants as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for IntervalStartPrecedesEnd {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for IntervalDurationIsNonNegative {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for UtcTimelineOrderingAppliesToFixedInstants
    {
    }
}

amenable_derive::harness! {
    creusot, INTERVAL_START_PRECEDES_END_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first endpoint is no later than its second on the relevant timeline: the outcome equals `start` preceding-or-equal `end` on the timeline.
        #[logic(open)]
        pub fn interval_start_precedes_end_holds(start: i32, end: i32, outcome: bool) -> bool {
            pearlite! { outcome == (start@ <= end@) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::interval_start_precedes_end_holds",
        "creusot",
        "ensures",
        || INTERVAL_START_PRECEDES_END_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_INTERVAL_START_PRECEDES_END_SRC, {
        /// `start <= end` satisfies the precedence spec, for every `i32` pair.
        #[requires(true)]
        #[ensures(interval_start_precedes_end_holds(start, end, result))]
        fn check_interval_start_precedes_end(start: i32, end: i32) -> bool {
            start <= end
        }
    }
}

amenable_derive::harness! {
    creusot, INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.8 — the span between an interval's endpoints is zero or positive, never negative: the outcome equals the `end@ - start@` span being non-negative.
        #[logic(open)]
        pub fn interval_duration_is_non_negative_holds(start: i32, end: i32, outcome: bool) -> bool {
            pearlite! { outcome == (end@ - start@ >= 0) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::interval_duration_is_non_negative_holds",
        "creusot",
        "ensures",
        || INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, {
        /// `start <= end` is the panic-free witness that the span is non-negative, for every `i32` pair.
        #[requires(true)]
        #[ensures(interval_duration_is_non_negative_holds(start, end, result))]
        fn check_interval_duration_is_non_negative(start: i32, end: i32) -> bool {
            start <= end
        }
    }
}

amenable_derive::harness! {
    creusot, UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC, {
        /// RFC 3339, 5.1 — two fixed instants are totally ordered by their position on the UTC timeline: the outcome equals `earlier` being at or before `later`.
        #[logic(open)]
        pub fn utc_timeline_ordering_applies_to_fixed_instants_holds(earlier: i32, later: i32, outcome: bool) -> bool {
            pearlite! { outcome == (earlier@ <= later@) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::utc_timeline_ordering_applies_to_fixed_instants_holds",
        "creusot",
        "ensures",
        || UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, UTC_TIMELINE_ORDERING_IS_A_TOTAL_ORDER_HOLDS_SRC, {
        /// `<=` on UTC timeline positions is a total order: reflexive,
        /// total, antisymmetric, and transitive. A genuinely distinct
        /// claim from [`utc_timeline_ordering_applies_to_fixed_instants_holds`]
        /// itself (which only states the relation, not that it has
        /// these four properties) — named so the compound check is a
        /// real, callable predicate rather than an inline restatement.
        #[logic(open)]
        pub fn utc_timeline_ordering_is_a_total_order_holds(a: i32, b: i32, c: i32) -> bool {
            pearlite! {
                a@ <= a@
                    && (a@ <= b@ || b@ <= a@)
                    && (a@ <= b@ && b@ <= a@ ==> a@ == b@)
                    && (a@ <= b@ && b@ <= c@ ==> a@ <= c@)
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::utc_timeline_ordering_is_a_total_order_holds",
        "creusot",
        "ensures",
        || UTC_TIMELINE_ORDERING_IS_A_TOTAL_ORDER_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC, {
        /// `a <= b` satisfies the ordering spec, and the relation is reflexive, total, antisymmetric, and transitive over `i32` instants.
        #[requires(true)]
        #[ensures(utc_timeline_ordering_applies_to_fixed_instants_holds(a, b, result))]
        #[ensures(utc_timeline_ordering_is_a_total_order_holds(a, b, c))]
        fn check_utc_timeline_ordering_applies_to_fixed_instants(a: i32, b: i32, c: i32) -> bool {
            a <= b
        }
    }
}
