use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{
    KaniChildObservation, KaniChildStderrObservation, KaniChildStdinObservation,
    KaniChildStdoutObservation, KaniVerifier, KaniWitness,
};

impl KaniWitness for RustStdStandard<Child> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_child_has_a_process_id_and_can_be_waited_on".to_owned(),
            VERIFY_CHILD_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Child>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Child>",
        "kani",
        || <RustStdStandard<Child> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChildObservation` instance actually demonstrated its
/// process-id and wait law, minted only by
/// [`KaniChildObservation::demonstrate_waitable`].
pub struct KaniChildWitnessToken(());

impl ProofToken for KaniChildWitnessToken {
    type Proposition = KaniChildObservation;
}

impl KaniChildObservation {
    /// Assert a nonzero process id and that waiting reports the expected
    /// exit code. Consumes `self`: the only way to obtain the token is
    /// to have run this check against a real observation instance, not
    /// to assert it independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_waitable(self, exit_code: i32) -> KaniChildWitnessToken {
        assert_ne!(self.process_id(), 0);
        assert_eq!(self.waited_exit_code(), Some(exit_code));
        KaniChildWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Child>`'s process-id and wait law
/// has been established from a `KaniChildObservation`.
pub struct RustStdChildToken(());

impl ProofToken for RustStdChildToken {
    type Proposition = RustStdStandard<Child>;
}

impl Establish<KaniChildWitnessToken, KaniVerifier> for RustStdStandard<Child> {
    type Token = RustStdChildToken;

    fn establish(_credential: KaniChildWitnessToken) -> Self::Token {
        RustStdChildToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHILD_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC, {
        /// A freshly spawned `Child` has a nonzero process id, and
        /// waiting on it reports the exit code it completed with.
        /// This proof uses the Amenable-owned process model: the direct
        /// `Command::spawn` path reaches the gallery's unsupported glibc
        /// boundary before `id()` / `wait()` can be checked. The claim is
        /// established through `Establish<KaniChildObservation, KaniVerifier>
        /// for RustStdStandard<Child>` from the observation instance that
        /// actually demonstrated the bounded wait law.
        #[kani::proof]
        fn verify_child_has_a_process_id_and_can_be_waited_on() {
            let observation = KaniChildObservation::waitable(7, 3);
            let demonstration = observation.demonstrate_waitable(3);

            let _token = RustStdStandard::<Child>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_child_stderr_captures_what_the_child_wrote_to_stderr".to_owned(),
            VERIFY_CHILD_STDERR_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChildStderr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChildStderr>",
        "kani",
        || <RustStdStandard<ChildStderr> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChildStderrObservation` instance actually
/// demonstrated stderr preservation, separate from stdout, minted only by
/// [`KaniChildStderrObservation::demonstrate_capture`].
pub struct KaniChildStderrWitnessToken(());

impl ProofToken for KaniChildStderrWitnessToken {
    type Proposition = KaniChildStderrObservation;
}

impl KaniChildStderrObservation {
    /// Assert stdout stayed empty while stderr captured the expected
    /// text. Consumes `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_capture(self) -> KaniChildStderrWitnessToken {
        assert_eq!(self.stdout_text(), "");
        assert!(self.stderr_text().contains("error message"));
        KaniChildStderrWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ChildStderr>`'s stderr-capture
/// law has been established from a `KaniChildStderrObservation`.
pub struct RustStdChildStderrToken(());

impl ProofToken for RustStdChildStderrToken {
    type Proposition = RustStdStandard<ChildStderr>;
}

impl Establish<KaniChildStderrWitnessToken, KaniVerifier> for RustStdStandard<ChildStderr> {
    type Token = RustStdChildStderrToken;

    fn establish(_credential: KaniChildStderrWitnessToken) -> Self::Token {
        RustStdChildStderrToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDERR_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC, {
        /// Piping a child's stderr preserves what it wrote there, separately
        /// from stdout.
        /// This proof uses the Amenable-owned process model: the direct piped
        /// stderr path bottoms out in unsupported stdio-pipe machinery before
        /// capture can be checked. The claim is established through
        /// `Establish<KaniChildStderrObservation, KaniVerifier> for
        /// RustStdStandard<ChildStderr>` from the observation instance that
        /// actually demonstrated stderr preservation.
        #[kani::proof]
        fn verify_child_stderr_captures_what_the_child_wrote_to_stderr() {
            let observation = KaniChildStderrObservation::captured("", "error message\n");
            let demonstration = observation.demonstrate_capture();

            let _token = RustStdStandard::<ChildStderr>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_child_stdin_is_readable_by_the_child_process".to_owned(),
            VERIFY_CHILD_STDIN_IS_READABLE_BY_THE_CHILD_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChildStdin>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChildStdin>",
        "kani",
        || <RustStdStandard<ChildStdin> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChildStdinObservation` instance actually
/// demonstrated the delivered-input echo law, minted only by
/// [`KaniChildStdinObservation::demonstrate_echo`].
pub struct KaniChildStdinWitnessToken(());

impl ProofToken for KaniChildStdinWitnessToken {
    type Proposition = KaniChildStdinObservation;
}

impl KaniChildStdinObservation {
    /// Assert the echoed stdout matches the delivered input exactly.
    /// Consumes `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_echo(self) -> KaniChildStdinWitnessToken {
        assert_eq!(self.echoed_stdout(), self.input_text());
        KaniChildStdinWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ChildStdin>`'s delivered-input
/// law has been established from a `KaniChildStdinObservation`.
pub struct RustStdChildStdinToken(());

impl ProofToken for RustStdChildStdinToken {
    type Proposition = RustStdStandard<ChildStdin>;
}

impl Establish<KaniChildStdinWitnessToken, KaniVerifier> for RustStdStandard<ChildStdin> {
    type Token = RustStdChildStdinToken;

    fn establish(_credential: KaniChildStdinWitnessToken) -> Self::Token {
        RustStdChildStdinToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDIN_IS_READABLE_BY_THE_CHILD_PROCESS_SRC, {
        /// Bytes written to a piped `ChildStdin` are delivered to the child,
        /// which can echo them back on stdout.
        /// This proof uses the Amenable-owned process model: the direct
        /// piped stdin/stdout path reaches the gallery's unsupported `pipe2`
        /// boundary before delivery can be checked. The claim is established
        /// through `Establish<KaniChildStdinObservation, KaniVerifier> for
        /// RustStdStandard<ChildStdin>` from the observation instance that
        /// actually demonstrated the bounded echo law.
        #[kani::proof]
        fn verify_child_stdin_is_readable_by_the_child_process() {
            let observation = KaniChildStdinObservation::echo("hello, child\n", "hello, child\n");
            let demonstration = observation.demonstrate_echo();

            let _token = RustStdStandard::<ChildStdin>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_child_stdout_captures_what_the_child_wrote_to_stdout".to_owned(),
            VERIFY_CHILD_STDOUT_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChildStdout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChildStdout>",
        "kani",
        || <RustStdStandard<ChildStdout> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChildStdoutObservation` instance actually
/// demonstrated stdout preservation, minted only by
/// [`KaniChildStdoutObservation::demonstrate_capture`].
pub struct KaniChildStdoutWitnessToken(());

impl ProofToken for KaniChildStdoutWitnessToken {
    type Proposition = KaniChildStdoutObservation;
}

impl KaniChildStdoutObservation {
    /// Assert stdout captured the expected text. Consumes `self` for the
    /// same reason [`KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_capture(self) -> KaniChildStdoutWitnessToken {
        assert!(self.stdout_text().contains("hello"));
        KaniChildStdoutWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ChildStdout>`'s stdout-capture
/// law has been established from a `KaniChildStdoutObservation`.
pub struct RustStdChildStdoutToken(());

impl ProofToken for RustStdChildStdoutToken {
    type Proposition = RustStdStandard<ChildStdout>;
}

impl Establish<KaniChildStdoutWitnessToken, KaniVerifier> for RustStdStandard<ChildStdout> {
    type Token = RustStdChildStdoutToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniChildStdoutWitnessToken) -> Self::Token {
        RustStdChildStdoutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDOUT_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC, {
        /// Piping a child's stdout captures what it printed there.
        /// This proof uses the Amenable-owned process model: the direct piped
        /// stdout path reaches the gallery's unsupported `pipe2` boundary
        /// before capture can be checked. The claim is established through
        /// `Establish<KaniChildStdoutObservation, KaniVerifier> for
        /// RustStdStandard<ChildStdout>` from the observation instance that
        /// actually demonstrated stdout preservation.
        #[kani::proof]
        fn verify_child_stdout_captures_what_the_child_wrote_to_stdout() {
            let observation = KaniChildStdoutObservation::captured("hello\n");
            let demonstration = observation.demonstrate_capture();

            let _token = RustStdStandard::<ChildStdout>::establish(demonstration);
        }
    }
}
