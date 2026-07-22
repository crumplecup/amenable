//! `KaniWitness` impls for `std::env`.

use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, Vars, VarsOs};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<Args> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_reports_at_least_the_program_path".to_owned(),
            claim: VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Args>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Args>",
        verifier: "kani",
        describe: || <RustStdStandard<Args> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// The process's own argv always has at least one element — the
        /// program's own path — so `.args()` never yields an empty
        /// sequence.
        #[kani::proof]
        fn verify_args_reports_at_least_the_program_path() {
            assert!(std::env::args().count() >= 1);
        }
    }
}

impl KaniWitness for RustStdStandard<ArgsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_os_reports_at_least_the_program_path".to_owned(),
            claim: VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ArgsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ArgsOs>",
        verifier: "kani",
        describe: || <RustStdStandard<ArgsOs> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// Same guarantee as `Args`, in the raw `OsString` form.
        #[kani::proof]
        fn verify_args_os_reports_at_least_the_program_path() {
            assert!(std::env::args_os().count() >= 1);
        }
    }
}

impl KaniWitness for RustStdStandard<JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_join_paths_error_reports_an_unjoinable_path".to_owned(),
            claim: VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<JoinPathsError>",
        verifier: "kani",
        describe: || <RustStdStandard<JoinPathsError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC, {
        /// `.join_paths()` fails for a path that can't be represented
        /// unambiguously in the platform's PATH-style form: a literal
        /// list separator on Unix, or an unbalanced quote on Windows
        /// (Windows can otherwise just quote a path containing its own
        /// separator).
        #[kani::proof]
        fn verify_join_paths_error_reports_an_unjoinable_path() {
            let bad_path = if cfg!(windows) { "a\"b" } else { "a:b" };
            assert!(std::env::join_paths([bad_path]).is_err());
        }
    }
}

impl KaniWitness for RustStdStandard<SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_paths_recovers_paths_joined_by_join_paths".to_owned(),
            claim: VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitPaths<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitPaths<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC, {
        /// `.split_paths()` inverts `.join_paths()`, recovering exactly
        /// the paths that were joined — checked via the round trip
        /// itself, so it holds regardless of the platform's own
        /// separator convention.
        #[kani::proof]
        fn verify_split_paths_recovers_paths_joined_by_join_paths() {
            let joined = std::env::join_paths(["one", "two", "three"]).unwrap();
            let split: Vec<std::path::PathBuf> = std::env::split_paths(&joined).collect();
            assert_eq!(
                split,
                vec![
                    std::path::PathBuf::from("one"),
                    std::path::PathBuf::from("two"),
                    std::path::PathBuf::from("three"),
                ]
            );
        }
    }
}

// `Vars` and `VarsOs` expose process-global state. Their previously
// generated harnesses mutated that global state through `set_var`, which is
// unsafe on current Rust and prohibited by this crate. There is no local,
// type-level invariant to check without that mutation, so retain their Kani
// witness as provenance-only rather than weakening the safety boundary.
// `VarError` likewise depends on ambient process state: a harness cannot
// establish that an arbitrary external variable is absent without that same
// unsafe mutation. Keep it provenance-only as well.
impl_kani_witness_trusted!(std::env::VarError, Vars, VarsOs);
