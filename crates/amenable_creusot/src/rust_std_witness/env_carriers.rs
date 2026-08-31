use super::CheckedProof;

use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, VarError, Vars, VarsOs};

use crate::{
    ARGV_EXTRA_HEADROOM_HOLDS_SRC, ARGV_INCLUDES_PROGRAM_PATH_SRC, CreusotVerifier, CreusotWitness,
    VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC,
    VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
    VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
    VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC,
    VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC,
    VERIFY_VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC,
};
use amenable_core::{Ensures, Evidence, Provenance, Requires, Witness};

use amenable_std::{ArgvIncludesProgramPath, RustStdProvenance, RustStdStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<Args> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_args_reports_at_least_the_program_path".to_string(),
            VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Args>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Args>",
        "creusot",
        || <RustStdStandard<Args> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<ArgsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_args_os_reports_at_least_the_program_path".to_string(),
            VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<ArgsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ArgsOs>",
        "creusot",
        || <RustStdStandard<ArgsOs> as CreusotWitness>::proof().to_string(),
    )
}

/// [`ArgvIncludesProgramPath`] reuses the `Args` harness rather than
/// adding a new Creusot proof — it names the precondition/postcondition
/// pair both `Args` and `ArgsOs` proofs already share, it doesn't prove
/// anything new.
impl CreusotWitness for ArgvIncludesProgramPath {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_args_reports_at_least_the_program_path".to_string(),
            VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ArgvIncludesProgramPath);

/// Returns `amenable_creusot::ARGV_EXTRA_HEADROOM_HOLDS_SRC` /
/// `ARGV_INCLUDES_PROGRAM_PATH_SRC` directly -- the verbatim,
/// `harness!`-captured source of the real `#[logic(open)]` fns both
/// `verify_args_reports_at_least_the_program_path` and
/// `verify_args_os_reports_at_least_the_program_path` call, not a
/// hand-retyped copy of their expressions. There is exactly one place
/// each of this precondition/postcondition pair's text exists in the
/// whole codebase.
impl Requires<CreusotVerifier> for ArgvIncludesProgramPath {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        ARGV_EXTRA_HEADROOM_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for ArgvIncludesProgramPath {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ARGV_INCLUDES_PROGRAM_PATH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::ArgvIncludesProgramPath",
        "creusot",
        "requires",
        || <ArgvIncludesProgramPath as Requires<CreusotVerifier>>::requires(()),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::ArgvIncludesProgramPath",
        "creusot",
        "ensures",
        || <ArgvIncludesProgramPath as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_join_paths_error_reports_an_unjoinable_path".to_string(),
            VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<JoinPathsError>",
        "creusot",
        || <RustStdStandard<JoinPathsError> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_paths_recovers_paths_joined_by_join_paths".to_string(),
            VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitPaths<'static>>",
        "creusot",
        || <RustStdStandard<SplitPaths<'static>> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<VarError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_var_error_distinguishes_not_present_from_not_unicode".to_string(),
            VERIFY_VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VarError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<VarError>",
        "creusot",
        || <RustStdStandard<VarError> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn var_error_distinguishes_not_present_from_not_unicode`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<VarError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<VarError>",
        "creusot",
        "ensures",
        || <RustStdStandard<VarError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<Vars> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<Vars>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Vars>",
        "creusot",
        || <RustStdStandard<Vars> as CreusotWitness>::proof().report().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<VarsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<VarsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<VarsOs>",
        "creusot",
        || <RustStdStandard<VarsOs> as CreusotWitness>::proof().report().to_string(),
    )
}
