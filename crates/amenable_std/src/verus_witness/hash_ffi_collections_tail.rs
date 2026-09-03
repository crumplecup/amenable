//! Hashing (`DefaultHasher`/`RandomState`), OS strings (`OsStr`/`OsString`/
//! their `Display`), `HashMap`/`HashSet`, `Pin`/`NonNull`, the global
//! allocator, backtraces, panic-hook info, and `VecDeque::Drain`. The
//! primitive/pointer carriers are in `primitives_and_pointers`; the
//! Windows raw-handle wrappers are in `os_windows_handles`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

const VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_default_hasher_model_is_deterministic_across_fresh_instances".to_owned(),
            VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        "verus",
        || {
            <RustStdStandard<std::hash::DefaultHasher> as VerusWitness>::proof().to_string()
        },
    )
}

// `DefaultHasher::default()`'s own real postcondition -- named once,
// called from `hash_carrier.rs`'s own `assume_specification` rather than
// restated inline.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::hash::DefaultHasher>,
    "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
    "default_hasher_new_view_is_empty"
);

const VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_random_state_model_gives_the_same_hasher_seed_across_calls".to_owned(),
            VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RandomState>",
        "verus",
        || {
            <RustStdStandard<std::hash::RandomState> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_str_model_valid_utf8_content_round_trips_through_to_str".to_owned(),
            VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsStr>",
        "verus",
        || { <RustStdStandard<std::ffi::OsStr> as VerusWitness>::proof().to_string() },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::ffi::OsStr>,
    "amenable_std::rust_std::RustStdStandard<OsStr>",
    "os_str_len_fits_the_two_byte_buffer"
);

const VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_string_model_push_appends_to_the_existing_content".to_owned(),
            VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsString>",
        "verus",
        || {
            <RustStdStandard<std::ffi::OsString> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::os_str::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_str_display_model_renders_valid_utf8_content_unchanged".to_owned(),
            VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::os_str::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        "verus",
        || {
            <RustStdStandard<std::ffi::os_str::Display<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_hash_map_model_insert_then_get_recovers_the_value".to_owned(),
            VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::HashMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_hash_set_model_insert_then_contains_reports_membership".to_owned(),
            VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::HashSet<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::pin::Pin<Box<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_pin_model_derefs_and_get_mut_round_trip".to_owned(),
            VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::pin::Pin<Box<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pin<Box<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::pin::Pin<Box<i32>>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::ptr::NonNull<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_non_null_model_rejects_the_null_pointer".to_owned(),
            VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ptr::NonNull<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonNull<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ptr::NonNull<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::alloc::System> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_model_allocates_and_deallocates_a_layout".to_owned(),
            VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::alloc::System>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<System>",
        "verus",
        || { <RustStdStandard<std::alloc::System> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::backtrace::Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_backtrace_model_force_capture_always_actually_captures".to_owned(),
            VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Backtrace>",
        "verus",
        || {
            <RustStdStandard<std::backtrace::Backtrace> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::backtrace::BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_backtrace_model_force_capture_always_actually_captures".to_owned(),
            VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        "verus",
        || {
            <RustStdStandard<std::backtrace::BacktraceStatus> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::panic::PanicHookInfo<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_panic_hook_info_model_reports_the_panics_own_message".to_owned(),
            VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::PanicHookInfo<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PanicHookInfo<'static>>",
        "verus",
        || {
            <RustStdStandard<std::panic::PanicHookInfo<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_drain_model_removes_and_yields_in_order".to_owned(),
            VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}
