//! `KaniWitness` impls for `std::env`.
//!
//! The direct `Args` / `ArgsOs` path depends on Kani's synthetic process
//! state, which can admit an empty argv even though a real process exposes its
//! own program slot. Production proofs therefore use an Amenable-owned argv
//! model instead; the direct std path remains preserved in the gallery as a
//! false trail.

use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, VarError, Vars, VarsOs};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ValueIsAtLeast;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};

impl KaniWitness for RustStdStandard<Args> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_args_reports_at_least_the_program_path".to_owned(),
            VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Args>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Args>",
        "kani",
        || <RustStdStandard<Args> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// The process's own argv always has at least one element — the
        /// program's own slot — so `.args()` never yields an empty sequence.
        /// This proof uses the Amenable-owned argv accommodation model: if the
        /// real process argv refines these modeled laws, the Rust-facing claim
        /// follows.
        #[kani::proof]
        fn verify_args_reports_at_least_the_program_path() {
            let argv = <crate::KaniArgv as crate::KaniCompose>::kani_any();
            assert!(ValueIsAtLeast::ensures((argv.args_count(), 1)));
            assert!(RustStdStandard::<usize>::ensures((
                argv.args_count(),
                1 + usize::from(argv.extra_count())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<ArgsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_args_os_reports_at_least_the_program_path".to_owned(),
            VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ArgsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ArgsOs>",
        "kani",
        || <RustStdStandard<ArgsOs> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// Same guarantee as `Args`, in the raw `OsString` form.
        #[kani::proof]
        fn verify_args_os_reports_at_least_the_program_path() {
            let argv = <crate::KaniArgv as crate::KaniCompose>::kani_any();
            assert!(ValueIsAtLeast::ensures((argv.args_os_count(), 1)));
            assert!(RustStdStandard::<usize>::ensures((
                argv.args_os_count(),
                1 + usize::from(argv.extra_count())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_join_paths_error_reports_an_unjoinable_path".to_owned(),
            VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<JoinPathsError>",
        "kani",
        || <RustStdStandard<JoinPathsError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC, {
        /// Amenable models the PATH-style helper boundary directly:
        /// if the real `std::env::join_paths()` path refines this
        /// bounded separator law, then a path that falls outside the
        /// modeled joinable subset is rejected as unjoinable.
        #[kani::proof]
        fn verify_join_paths_error_reports_an_unjoinable_path() {
            let bad_path = if cfg!(windows) { "a\"b" } else { "a:b" }.to_owned();
            let err = crate::KaniEnvPath::new(bad_path.clone()).unwrap_err();
            assert!(DerefReflectsTheStoredValue::ensures((
                err.offending_path().clone(),
                bad_path.clone()
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                err.into_offending_path(),
                bad_path
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_paths_recovers_paths_joined_by_join_paths".to_owned(),
            VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitPaths<'static>>",
        "kani",
        || <RustStdStandard<SplitPaths<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<SplitPaths<'static>>,
    "amenable_std::rust_std::RustStdStandard<SplitPaths<'static>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC, {
        /// Amenable models a bounded separator-free PATH-style subset
        /// directly: if the real `join_paths()` / `split_paths()`
        /// helpers refine these modeled laws on that subset, then the
        /// round trip recovers exactly the joined paths in order.
        #[kani::proof]
        fn verify_split_paths_recovers_paths_joined_by_join_paths() {
            let one = crate::KaniEnvPath::new("one".to_owned())
                .expect("separator-free path stays inside the modeled subset");
            let two = crate::KaniEnvPath::new("two".to_owned())
                .expect("separator-free path stays inside the modeled subset");
            let three = crate::KaniEnvPath::new("three".to_owned())
                .expect("separator-free path stays inside the modeled subset");
            let paths = crate::KaniEnvPathList::new(vec![one, two, three]);
            let joined = crate::KaniEnvPaths::join_semantic(paths);
            let split = crate::KaniEnvPaths::split_semantic(joined);

            assert!(RustStdStandard::<SplitPaths<'static>>::ensures((split.len(), 3)));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                split.paths()[0].as_str(),
                "one"
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                split.paths()[1].as_str(),
                "two"
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                split.paths()[2].as_str(),
                "three"
            )));
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
impl_kani_witness_trusted!(VarError, Vars, VarsOs);
