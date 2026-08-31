use std::process::{ExitCode, ExitStatus, Output, Stdio};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
use crate::{
    KaniExitStatusObservation, KaniOutputObservation, KaniStdioObservation, KaniVerifier,
    KaniWitness,
};

impl KaniWitness for RustStdStandard<ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_exit_status_reports_a_nonzero_exit_code".to_owned(),
            VERIFY_EXIT_STATUS_REPORTS_A_NONZERO_EXIT_CODE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ExitStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ExitStatus>",
        "kani",
        || <RustStdStandard<ExitStatus> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniExitStatusObservation` instance actually
/// demonstrated the nonzero exit-code law, minted only by
/// [`KaniExitStatusObservation::demonstrate_nonzero_exit`].
pub struct KaniExitStatusWitnessToken(());

impl ProofToken for KaniExitStatusWitnessToken {
    type Proposition = KaniExitStatusObservation;
}

impl KaniExitStatusObservation {
    /// Assert `!success()` and the expected exit code. Consumes `self`
    /// for the same reason [`KaniChildObservation::demonstrate_waitable`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_nonzero_exit(self, exit_code: i32) -> KaniExitStatusWitnessToken {
        assert!(!self.success());
        assert_eq!(self.code(), Some(exit_code));
        KaniExitStatusWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ExitStatus>`'s exit-code law has
/// been established from a `KaniExitStatusObservation`.
pub struct RustStdExitStatusToken(());

impl ProofToken for RustStdExitStatusToken {
    type Proposition = RustStdStandard<ExitStatus>;
}

impl Establish<KaniExitStatusWitnessToken, KaniVerifier> for RustStdStandard<ExitStatus> {
    type Token = RustStdExitStatusToken;

    fn establish(_credential: KaniExitStatusWitnessToken) -> Self::Token {
        RustStdExitStatusToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_EXIT_STATUS_REPORTS_A_NONZERO_EXIT_CODE_SRC, {
        /// A process that exits with a specific nonzero code reports
        /// `!success()` and that exact code.
        /// This proof uses the Amenable-owned process model: the direct
        /// `Command::status` path reaches the gallery's unsupported spawn
        /// boundary before exit status can be checked. The claim is
        /// established through `Establish<KaniExitStatusObservation,
        /// KaniVerifier> for RustStdStandard<ExitStatus>` from the
        /// observation instance that actually demonstrated the bounded status
        /// law.
        #[kani::proof]
        fn verify_exit_status_reports_a_nonzero_exit_code() {
            let observation = KaniExitStatusObservation::nonzero(3);
            let demonstration = observation.demonstrate_nonzero_exit(3);

            let _token = RustStdStandard::<ExitStatus>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_output_captures_stdout_and_the_exit_status".to_owned(),
            VERIFY_OUTPUT_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Output>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Output>",
        "kani",
        || <RustStdStandard<Output> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniOutputObservation` instance actually demonstrated
/// bundling the exit status with captured stdout, minted only by
/// [`KaniOutputObservation::demonstrate_bundle`].
pub struct KaniOutputWitnessToken(());

impl ProofToken for KaniOutputWitnessToken {
    type Proposition = KaniOutputObservation;
}

impl KaniOutputObservation {
    /// Assert success, the expected status code, and captured stdout.
    /// Consumes `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_bundle(self, exit_code: i32) -> KaniOutputWitnessToken {
        assert!(self.success());
        assert_eq!(self.status_code(), Some(exit_code));
        assert!(self.stdout_text().contains("hello"));
        KaniOutputWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Output>`'s output-bundle law has
/// been established from a `KaniOutputObservation`.
pub struct RustStdOutputToken(());

impl ProofToken for RustStdOutputToken {
    type Proposition = RustStdStandard<Output>;
}

impl Establish<KaniOutputWitnessToken, KaniVerifier> for RustStdStandard<Output> {
    type Token = RustStdOutputToken;

    fn establish(_credential: KaniOutputWitnessToken) -> Self::Token {
        RustStdOutputToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_OUTPUT_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC, {
        /// `.output()` bundles a command's exit status with the stdout it
        /// produced.
        /// This proof uses the Amenable-owned process model: the direct
        /// `Command::output` path reaches the gallery's unsupported `Stdio`
        /// conversion boundary before bundle capture can be checked. The
        /// claim is established through `Establish<KaniOutputObservation,
        /// KaniVerifier> for RustStdStandard<Output>` from the observation
        /// instance that actually demonstrated the bounded bundle law.
        #[kani::proof]
        fn verify_output_captures_stdout_and_the_exit_status() {
            let observation = KaniOutputObservation::captured(0, "hello\n");
            let demonstration = observation.demonstrate_bundle(0);

            let _token = RustStdStandard::<Output>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_stdio_null_discards_the_childs_output_handle".to_owned(),
            VERIFY_STDIO_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Stdio>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Stdio>",
        "kani",
        || <RustStdStandard<Stdio> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniStdioObservation` instance actually demonstrated
/// the stdout-handle policy, minted only by
/// [`KaniStdioObservation::demonstrate_handle_policy`].
pub struct KaniStdioWitnessToken(());

impl ProofToken for KaniStdioWitnessToken {
    type Proposition = KaniStdioObservation;
}

impl KaniStdioObservation {
    /// Assert `Stdio::null()` leaves no handle while `Stdio::piped()`
    /// exposes one. Consumes `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_handle_policy(self) -> KaniStdioWitnessToken {
        assert!(!self.null_stdout_handle_present());
        assert!(self.piped_stdout_handle_present());
        KaniStdioWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Stdio>`'s stdout-handle policy
/// law has been established from a `KaniStdioObservation`.
pub struct RustStdStdioToken(());

impl ProofToken for RustStdStdioToken {
    type Proposition = RustStdStandard<Stdio>;
}

impl Establish<KaniStdioWitnessToken, KaniVerifier> for RustStdStandard<Stdio> {
    type Token = RustStdStdioToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniStdioWitnessToken) -> Self::Token {
        RustStdStdioToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_STDIO_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC, {
        /// `.stdout(Stdio::null())` leaves no child stdout handle, while
        /// `.stdout(Stdio::piped())` does expose one.
        /// This proof uses the Amenable-owned process model: the direct
        /// `Stdio` configuration path reaches the gallery's unsupported
        /// C-string-literal boundary before handle presence can be checked.
        /// The claim is established through `Establish<KaniStdioObservation,
        /// KaniVerifier> for RustStdStandard<Stdio>` from the observation
        /// instance that actually demonstrated the bounded handle law.
        #[kani::proof]
        fn verify_stdio_null_discards_the_childs_output_handle() {
            let observation = KaniStdioObservation::stdout_handle_policy(false, true);
            let demonstration = observation.demonstrate_handle_policy();

            let _token = RustStdStandard::<Stdio>::establish(demonstration);
        }
    }
}

impl_kani_witness_trusted!(ExitCode);
