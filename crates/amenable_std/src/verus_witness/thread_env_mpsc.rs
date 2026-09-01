//! The remaining `std::thread` types, `std::env`'s path-joining/splitting
//! types, and `std::sync::mpsc`'s receiver/iterator pair.

use super::fs::VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_local_key_model_with_reads_the_initialized_value".to_owned(),
            VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

// Singleton contract: the fixed example's initial value (5) and its
// value after one mutation (42).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>",
    "local_key_observes_initial_then_updated"
);

const VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/thread_current_carrier.rs");

impl VerusWitness for RustStdStandard<std::thread::Thread> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_thread_current_model_is_stable_across_repeated_calls".to_owned(),
            VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::Thread>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::Thread>",
        "verus",
        || {
            <RustStdStandard<std::thread::Thread> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::thread::ThreadId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_thread_current_model_is_stable_across_repeated_calls".to_owned(),
            VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::ThreadId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::ThreadId>",
        "verus",
        || {
            <RustStdStandard<std::thread::ThreadId> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ARGS_MODEL_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/env_carrier.rs");

const ARGS_MODEL_COUNT_MATCHES_PROGRAM_PLUS_EXTRA_VERUS_FRAGMENT: &str = r#"pub open spec fn args_model_count_matches_program_plus_extra(
    extra_count: u8,
    result: u32,
) -> bool {
    result >= 1 && result == 1 + extra_count as u32
}"#;

macro_rules! impl_env_args_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_args_model_reports_at_least_the_program_path".to_owned(),
                    VERIFY_ARGS_MODEL_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
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

        ::inventory::submit! {
            ::amenable_core::ContractRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                "ensures",
                || ARGS_MODEL_COUNT_MATCHES_PROGRAM_PLUS_EXTRA_VERUS_FRAGMENT,
            )
        }
    };
}

impl_env_args_verus_witness!(std::env::Args);
impl_env_args_verus_witness!(std::env::ArgsOs);

const VERIFY_JOIN_PATHS_ERROR_MODEL_REPORTS_AN_UNJOINABLE_PATH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/env_carrier.rs");

impl VerusWitness for RustStdStandard<std::env::JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_join_paths_error_model_reports_an_unjoinable_path".to_owned(),
            VERIFY_JOIN_PATHS_ERROR_MODEL_REPORTS_AN_UNJOINABLE_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::env::JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::env::JoinPathsError>",
        "verus",
        || {
            <RustStdStandard<std::env::JoinPathsError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/env_carrier.rs");

impl VerusWitness for RustStdStandard<std::env::SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_split_paths_model_recovers_paths_joined_by_join_paths".to_owned(),
            VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::env::SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::env::SplitPaths<'static>>",
        "verus",
        || {
            <RustStdStandard<std::env::SplitPaths<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_channel_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_channel_model_delivers_to_the_paired_receiver".to_owned(),
                    VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
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

impl_mpsc_channel_verus_witness!(std::sync::mpsc::Sender<i32>);
impl_mpsc_channel_verus_witness!(std::sync::mpsc::SyncSender<i32>);

const VERIFY_RECEIVER_MODEL_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_mpsc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::mpsc::Receiver<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_receiver_model_fails_once_every_sender_is_dropped".to_owned(),
            VERIFY_RECEIVER_MODEL_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Receiver<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::mpsc::Receiver<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_channel_iter_model_yields_sent_values_then_stops".to_owned(),
                    VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
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

impl_mpsc_iter_verus_witness!(std::sync::mpsc::IntoIter<i32>);
impl_mpsc_iter_verus_witness!(std::sync::mpsc::Iter<'static, i32>);

const VERIFY_TRY_ITER_MODEL_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_mpsc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_iter_model_does_not_block_on_an_empty_open_channel".to_owned(),
            VERIFY_TRY_ITER_MODEL_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

pub(super) const VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_once_carrier.rs");
