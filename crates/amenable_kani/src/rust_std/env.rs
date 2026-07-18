//! `KaniWitness` impls for `std::env`.

use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, VarError, Vars, VarsOs};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Args> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_reports_at_least_the_program_path",
            claim: VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
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
            harness: "verify_args_os_reports_at_least_the_program_path",
            claim: VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
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
            harness: "verify_join_paths_error_reports_an_unjoinable_path",
            claim: VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC,
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
            harness: "verify_split_paths_recovers_paths_joined_by_join_paths",
            claim: VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC,
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

impl KaniWitness for RustStdStandard<VarError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_var_error_reports_an_unset_variable",
            claim: VERIFY_VAR_ERROR_REPORTS_AN_UNSET_VARIABLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VarError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarError>",
        verifier: "kani",
        describe: || <RustStdStandard<VarError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VAR_ERROR_REPORTS_AN_UNSET_VARIABLE_SRC, {
        /// `.var()` on a variable that was never set fails with
        /// `VarError::NotPresent`.
        #[kani::proof]
        fn verify_var_error_reports_an_unset_variable() {
            assert_eq!(
                std::env::var("AMENABLE_KANI_ENV_TEST_DEFINITELY_UNSET"),
                Err(VarError::NotPresent)
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Vars> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vars_reports_a_variable_set_via_set_var",
            claim: VERIFY_VARS_REPORTS_A_VARIABLE_SET_VIA_SET_VAR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Vars>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Vars>",
        verifier: "kani",
        describe: || <RustStdStandard<Vars> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VARS_REPORTS_A_VARIABLE_SET_VIA_SET_VAR_SRC, {
        /// A variable set via `set_var()` genuinely shows up in
        /// `.vars()`, as a `(String, String)` pair.
        #[kani::proof]
        fn verify_vars_reports_a_variable_set_via_set_var() {
            // SAFETY: single-threaded harness; no concurrent env access.
            unsafe {
                std::env::set_var("AMENABLE_KANI_ENV_TEST_VAR", "some-value");
            }
            let found = std::env::vars().find(|(key, _)| key == "AMENABLE_KANI_ENV_TEST_VAR");
            assert_eq!(
                found,
                Some(("AMENABLE_KANI_ENV_TEST_VAR".to_string(), "some-value".to_string()))
            );
            // SAFETY: single-threaded harness; no concurrent env access.
            unsafe {
                std::env::remove_var("AMENABLE_KANI_ENV_TEST_VAR");
            }
        }
    }
}

impl KaniWitness for RustStdStandard<VarsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vars_os_reports_a_variable_set_via_set_var",
            claim: VERIFY_VARS_OS_REPORTS_A_VARIABLE_SET_VIA_SET_VAR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VarsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarsOs>",
        verifier: "kani",
        describe: || <RustStdStandard<VarsOs> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VARS_OS_REPORTS_A_VARIABLE_SET_VIA_SET_VAR_SRC, {
        /// Same guarantee as `Vars`, in the raw `OsString` form.
        #[kani::proof]
        fn verify_vars_os_reports_a_variable_set_via_set_var() {
            // SAFETY: single-threaded harness; no concurrent env access.
            unsafe {
                std::env::set_var("AMENABLE_KANI_ENV_TEST_VAR_OS", "some-value");
            }
            let found = std::env::vars_os()
                .find(|(key, _)| key == std::ffi::OsStr::new("AMENABLE_KANI_ENV_TEST_VAR_OS"));
            assert!(found.is_some());
            // SAFETY: single-threaded harness; no concurrent env access.
            unsafe {
                std::env::remove_var("AMENABLE_KANI_ENV_TEST_VAR_OS");
            }
        }
    }
}
