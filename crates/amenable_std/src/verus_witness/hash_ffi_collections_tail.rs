//! The remaining generic/function-pointer-parameterized standard-library
//! carriers, `str` itself, hashing, OS strings, `HashMap`/`HashSet`,
//! `Pin`/`NonNull`, the global allocator, backtraces, panic hook info, and (on
//! Windows) the raw-handle wrapper types.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::panic_ops_time_future::{
    VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC,
    VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC,
};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_model_indexing_and_length".to_owned(),
            VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "verus",
        || { <RustStdStandard<[i32; 3]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_slice_model_indexing_and_length".to_owned(),
            VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "verus",
        || { <RustStdStandard<[i32]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_model_byte_length_and_content".to_owned(),
            VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<str>);

// The real Verus proof sites across the `str`, `path`, `process`, `env`,
// and panic carriers all call the shared `text_view_matches_expected`
// spec fn directly.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<str>,
    "amenable_std::rust_std::RustStdStandard<str>",
    "text_view_matches_expected"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "verus",
        || { <RustStdStandard<str> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tuple_model_field_access".to_owned(),
            VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "verus",
        || { <RustStdStandard<(i32, i32)> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fn_pointer_model_calls_the_underlying_function".to_owned(),
            VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "verus",
        || { <RustStdStandard<fn(i32) -> i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_const_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "verus",
        || { <RustStdStandard<*const i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mut_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "verus",
        || { <RustStdStandard<*mut i32> as VerusWitness>::proof().to_string() },
    )
}

impl VerusWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shared_reference_model_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "verus",
        || { <RustStdStandard<&'static i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mutable_reference_model_dereferences_to_and_updates_the_referent".to_owned(),
            VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "verus",
        || { <RustStdStandard<&'static mut i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

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

/// `std::os::windows::*` witnesses, one `#[cfg(windows)]` gate on this
/// `mod` instead of scattered per-item ones -- mirroring `rust_std::
/// os_windows`'s own gating. Unlike every other `VerusWitness` impl in
/// this file, `amenable_verus::rust_std::os_windows_carrier` (the
/// `claim` these `include_str!` in) has never been checked by `verus`
/// on this crate's primary development host (Linux) — only the
/// `verus-windows` GitHub Actions workflow (`workflow_dispatch`,
/// `windows-latest`) can. See that carrier's own module doc comment for
/// the full reasoning. Nothing here is `pub`, so the whole section
/// collapses into one private nested module with no re-export needed:
/// trait impls are visible crate-wide regardless of which module
/// defines them.
///
/// `EncodeWide`/`BorrowedHandle`/`BorrowedSocket`/`HandleOrInvalid`/
/// `OwnedHandle`/`OwnedSocket` need their own real `use` here -- a real,
/// pre-existing gap this module had before this consolidation (`cannot
/// find type` for all six, confirmed via a genuine `cross check --target
/// x86_64-pc-windows-gnu` run, not previously caught since the
/// `verus-windows` workflow is `workflow_dispatch`-only).
#[cfg(windows)]
mod windows_witnesses {
    use super::{Evidence, RustStdStandard, VerusCheckedProof, VerusWitness, bridge_verus_witness};
    use std::os::windows::ffi::EncodeWide;
    use std::os::windows::io::{
        BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
    };

    const VERIFY_ENCODE_WIDE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<EncodeWide<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<EncodeWide<'_> as Iterator>::next".to_owned(),
                VERIFY_ENCODE_WIDE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<EncodeWide<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
            "verus",
            || { <RustStdStandard<EncodeWide<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<EncodeWide<'static>>,
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "encode_wide_next_matches"
    );

    const VERIFY_BORROWED_HANDLE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<BorrowedHandle<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<BorrowedHandle<'_> as AsRawHandle>::as_raw_handle".to_owned(),
                VERIFY_BORROWED_HANDLE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<BorrowedHandle<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
            "verus",
            || { <RustStdStandard<BorrowedHandle<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<BorrowedHandle<'static>>,
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "as_raw_handle_addr_matches"
    );

    const VERIFY_BORROWED_SOCKET_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<BorrowedSocket<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<BorrowedSocket<'_> as AsRawSocket>::as_raw_socket".to_owned(),
                VERIFY_BORROWED_SOCKET_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<BorrowedSocket<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
            "verus",
            || { <RustStdStandard<BorrowedSocket<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<BorrowedSocket<'static>>,
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "as_raw_socket_matches"
    );

    const VERIFY_HANDLE_OR_INVALID_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<HandleOrInvalid> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedHandle as TryFrom<HandleOrInvalid>>::try_from".to_owned(),
                VERIFY_HANDLE_OR_INVALID_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<HandleOrInvalid>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
            "verus",
            || { <RustStdStandard<HandleOrInvalid> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<HandleOrInvalid>,
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "handle_or_invalid_try_from_matches"
    );

    const VERIFY_OWNED_HANDLE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<OwnedHandle> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedHandle as AsRawHandle>::as_raw_handle".to_owned(),
                VERIFY_OWNED_HANDLE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<OwnedHandle>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
            "verus",
            || { <RustStdStandard<OwnedHandle> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<OwnedHandle>,
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "owned_as_raw_handle_addr_matches"
    );

    const VERIFY_OWNED_SOCKET_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<OwnedSocket> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedSocket as AsRawSocket>::as_raw_socket".to_owned(),
                VERIFY_OWNED_SOCKET_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<OwnedSocket>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
            "verus",
            || { <RustStdStandard<OwnedSocket> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<OwnedSocket>,
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "owned_as_raw_socket_matches"
    );
}

/// `windows_witnesses`'s own 6 `verus_ensures_predicate!` calls generate
/// real `ContractRecord` registrations naming each real `open spec fn` --
/// but the whole module they live in is `#[cfg(windows)]`, so on this
/// project's own Linux dev/CI host (where `amenable dump-registry` always
/// runs) those registrations never fire, and cordial's own
/// `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001` check -- which reads that
/// registry dump, not the source directly -- sees 6 real, named `ensures`
/// clauses in `amenable_verus::rust_std::os_windows_carrier` as if they
/// were unnamed raw equations. `ContractRecord::new` takes only string/fn
/// -pointer data (`evidence: &'static str, verifier: &'static str, kind:
/// &'static str, fragment: fn() -> &'static str`) -- no dependency on the
/// real Windows types at all -- so the fix is the same one
/// `amenable_kani::os_windows_model`'s own doc comment already
/// established for `ProofRecord` on the Kani side: hand-write the
/// registration, with `fragment` a verbatim copy of the real spec fn's
/// own signature and body (`os_windows_carrier.rs`, not paraphrased), so
/// the registry is honest about what's already true and documented in
/// the real source, on the one platform this tooling actually runs on.
/// `#[cfg(not(windows))]`: the real `#[cfg(windows)]` registrations above
/// already cover a genuine Windows build; this is purely the Linux-side
/// fallback, never both at once.
#[cfg(not(windows))]
mod windows_contract_bounds_linux_fallback {
    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn encode_wide_next_matches(before: Seq<u16>, after: Seq<u16>, result: Option<u16>) -> bool { (before.len() == 0 ==> result is None && after == before) && (before.len() > 0 ==> result == Some(before[0]) && after == before.subrange(1, before.len() as int)) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn as_raw_handle_addr_matches(result: RawHandle, h: BorrowedHandle) -> bool { result.addr() == borrowed_handle_addr_spec(h) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn as_raw_socket_matches(result: RawSocket, s: BorrowedSocket) -> bool { result == borrowed_socket_value_spec(s) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
            "verus",
            "ensures",
            || "pub open spec fn handle_or_invalid_try_from_matches(handle_or_invalid: HandleOrInvalid, result: Result<OwnedHandle, <OwnedHandle as core::convert::TryFrom<HandleOrInvalid>>::Error>) -> bool { (handle_or_invalid_addr_spec(handle_or_invalid) == usize::MAX <==> result is Err) && (result is Ok ==> owned_handle_addr_spec(result->Ok_0) == handle_or_invalid_addr_spec(handle_or_invalid)) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
            "verus",
            "ensures",
            || "pub open spec fn owned_as_raw_handle_addr_matches(result: RawHandle, h: OwnedHandle) -> bool { result.addr() == owned_handle_addr_spec(h) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
            "verus",
            "ensures",
            || "pub open spec fn owned_as_raw_socket_matches(result: RawSocket, s: OwnedSocket) -> bool { result == owned_socket_value_spec(s) }",
        )
    }
}
