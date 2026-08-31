//! The `AtomicPtr`/`atomic::Ordering` stragglers left over from the atomic
//! family, and the `std::process` types.

use super::io_and_sync_atomic::{
    ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT,
    VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC,
};
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::sync::atomic::AtomicPtr<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_atomic_ptr_model_load_store_swap_and_compare_exchange".to_owned(),
            VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::AtomicPtr<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::sync::atomic::AtomicPtr<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
    "verify_atomic_ptr_model_load_store_swap_and_compare_exchange"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::atomic::AtomicPtr<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        "verus",
        "ensures",
        || ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT,
    )
}

const VERIFY_ATOMIC_ORDERING_MODEL_RELAXED_STORE_IS_OBSERVABLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_atomic_ordering_carrier.rs");

// Bare `Ordering`, matching `amenable_std::rust_std::sync_atomic`'s own
// registration and the Kani/Creusot witnesses' evidence strings for the
// same type — this is `core::sync::atomic::Ordering`, not
// `std::cmp::Ordering` (see `ordering_carrier.rs`/that type's own
// witness above for the comparison-result enum).
impl VerusWitness for RustStdStandard<std::sync::atomic::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_atomic_ordering_model_relaxed_store_is_observable".to_owned(),
            VERIFY_ATOMIC_ORDERING_MODEL_RELAXED_STORE_IS_OBSERVABLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "verus",
        || {
            <RustStdStandard<std::sync::atomic::Ordering> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Child> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_model_has_a_process_id_and_can_be_waited_on".to_owned(),
            VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Child>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Child>",
        "verus",
        || {
            <RustStdStandard<std::process::Child> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::process::Child>,
    "amenable_std::rust_std::RustStdStandard<std::process::Child>",
    "process_id_is_nonzero"
);

// Same predicate, reused as a real `ensures` claim too (the process id
// is nonzero both before and after waiting) -- a separate registration
// since Kani/Creusot/Verus's `(verifier, kind)` lookup is keyed
// separately for `requires` vs `ensures` clauses.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::process::Child>,
    "amenable_std::rust_std::RustStdStandard<std::process::Child>",
    "process_id_is_nonzero"
);

const VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stderr_model_captures_what_the_child_wrote_to_stderr".to_owned(),
            VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStderr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStderr>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStderr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stdin_model_is_readable_by_the_child_process".to_owned(),
            VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdin>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStdin>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStdin> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stdout_model_captures_what_the_child_wrote_to_stdout".to_owned(),
            VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStdout>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStdout> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_model_env_override_is_visible_to_the_spawned_process".to_owned(),
            VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Command>",
        "verus",
        || {
            <RustStdStandard<std::process::Command> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_args_model_reports_the_configured_arguments".to_owned(),
            VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::CommandArgs<'static>>",
        "verus",
        || {
            <RustStdStandard<std::process::CommandArgs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_envs_model_reports_the_configured_overrides".to_owned(),
            VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::CommandEnvs<'static>>",
        "verus",
        || {
            <RustStdStandard<std::process::CommandEnvs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_exit_status_model_reports_a_nonzero_exit_code".to_owned(),
            VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ExitStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ExitStatus>",
        "verus",
        || {
            <RustStdStandard<std::process::ExitStatus> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::process::ExitStatus>,
    "amenable_std::rust_std::RustStdStandard<std::process::ExitStatus>",
    "exit_code_is_nonzero"
);

const VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_output_model_captures_stdout_and_the_exit_status".to_owned(),
            VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Output>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Output>",
        "verus",
        || {
            <RustStdStandard<std::process::Output> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: this fixed example's exit code is always 0
// (success).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::process::Output>,
    "amenable_std::rust_std::RustStdStandard<std::process::Output>",
    "output_exit_code_is_success"
);

const VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/process_stdio_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_stdio_model_null_discards_the_childs_output_handle".to_owned(),
            VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Stdio>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Stdio>",
        "verus",
        || {
            <RustStdStandard<std::process::Stdio> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/path_ancestors_carrier.rs");
