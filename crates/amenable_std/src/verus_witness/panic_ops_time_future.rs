//! `std::panic`'s `AssertUnwindSafe`/`Location`, the remaining `std::ops`
//! range/control-flow types, `std::time`, and Option/Result's own iterator
//! family (via their own macros) plus `std::future`'s `Pending`/`Ready`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::sync_net_task::VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::panic::AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_assert_unwind_safe_model_derefs_transparently".to_owned(),
            VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::panic::AssertUnwindSafe<i32>>",
        "verus",
        || {
            <RustStdStandard<std::panic::AssertUnwindSafe<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/panic_carrier.rs");

impl VerusWitness for RustStdStandard<core::panic::Location<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_location_model_caller_reflects_the_immediate_call_site".to_owned(),
            VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::panic::Location<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::panic::Location<'static>>",
        "verus",
        || {
            <RustStdStandard<core::panic::Location<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<core::panic::Location<'static>>,
    "amenable_std::rust_std::RustStdStandard<core::panic::Location<'static>>",
    "values_are_distinct"
);

const VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_range_to_model_contains_matches_bound".to_owned(),
            VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::RangeTo<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::RangeTo<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_range_full_model_contains_everything".to_owned(),
            VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeFull>",
        "verus",
        || {
            <RustStdStandard<std::ops::RangeFull> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_bound_model_round_trips_its_endpoint".to_owned(),
            VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::Bound<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_control_flow_model_continue_and_break_are_disjoint".to_owned(),
            VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::ControlFlow<i32, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_instant_model_is_monotonically_nondecreasing".to_owned(),
            VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Instant>",
        "verus",
        || {
            <RustStdStandard<std::time::Instant> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_model_duration_since_computes_the_elapsed_span".to_owned(),
            VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTime>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTime> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_error_model_recovers_how_far_backward_it_went".to_owned(),
            VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTimeError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_duration_model_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "verus",
        || {
            <RustStdStandard<std::time::Duration> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::time::Duration>,
    "amenable_std::rust_std::RustStdStandard<Duration>",
    "duration_new_secs_headroom_holds"
);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::time::Duration>,
    "amenable_std::rust_std::RustStdStandard<Duration>",
    "duration_new_result_matches"
);

const VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/option_result_iter_carrier.rs");

macro_rules! impl_option_result_into_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_into_iter_model_yields_zero_or_one_owned_value".to_owned(),
                    VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_option_result_into_iter_verus_witness!(core::option::IntoIter<i32>);
impl_option_result_into_iter_verus_witness!(core::result::IntoIter<i32>);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::option::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::option::IntoIter<i32>>",
    "into_iter_yields_zero_or_one_owned_value"
);

const VERIFY_ITER_MODEL_YIELDS_ZERO_OR_ONE_REFERENCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/option_result_iter_carrier.rs");

macro_rules! impl_option_result_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_iter_model_yields_zero_or_one_reference".to_owned(),
                    VERIFY_ITER_MODEL_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_option_result_iter_verus_witness!(core::option::Iter<'static, i32>);
impl_option_result_iter_verus_witness!(core::result::Iter<'static, i32>);

const VERIFY_ITER_MUT_MODEL_WRITES_THROUGH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/option_result_iter_carrier.rs");

macro_rules! impl_option_result_iter_mut_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_iter_mut_model_writes_through".to_owned(),
                    VERIFY_ITER_MUT_MODEL_WRITES_THROUGH_SRC.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_option_result_iter_mut_verus_witness!(core::option::IterMut<'static, i32>);
impl_option_result_iter_mut_verus_witness!(core::result::IterMut<'static, i32>);

const VERIFY_PENDING_MODEL_NEVER_RESOLVES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/future_carrier.rs");

impl VerusWitness for RustStdStandard<std::future::Pending<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_pending_model_never_resolves".to_owned(),
            VERIFY_PENDING_MODEL_NEVER_RESOLVES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        "verus",
        || {
            <RustStdStandard<std::future::Pending<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/future_carrier.rs");

impl VerusWitness for RustStdStandard<std::future::Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ready_model_resolves_immediately_with_its_value".to_owned(),
            VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        "verus",
        || {
            <RustStdStandard<std::future::Ready<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_POLL_FN_MODEL_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/future_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_poll_fn_model_dispatches_through_to_its_closure".to_owned(),
            VERIFY_POLL_FN_MODEL_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

pub(super) const VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

pub(super) const VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");
