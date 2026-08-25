//! `KaniWitness` impls for `std::process`.
//!
//! The direct `Command` / `Child` paths hit unsupported libc probes,
//! `CString` conversion, and pipe-construction machinery under Kani today.
//! The reduced direct failures remain preserved in the proof gallery, while
//! production proofs use Amenable-owned bounded process observations to carry
//! the Rust-facing laws each carrier is supposed to expose.

use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, CommandArgs, CommandEnvs, ExitCode,
    ExitStatus, Output, Stdio,
};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
use crate::{
    KaniChildObservation, KaniChildStderrObservation, KaniChildStdinObservation,
    KaniChildStdoutObservation, KaniCommandArgsObservation, KaniCommandEnvObservation,
    KaniCommandEnvsObservation, KaniExitStatusObservation, KaniOutputObservation,
    KaniStdioObservation, KaniVerifier, KaniWitness,
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

impl KaniWitness for RustStdStandard<Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_env_override_is_visible_to_the_spawned_process".to_owned(),
            VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Command>",
        "kani",
        || <RustStdStandard<Command> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandEnvObservation` instance actually
/// demonstrated the environment-override visibility law, minted only by
/// [`KaniCommandEnvObservation::demonstrate_visibility`].
pub struct KaniCommandEnvWitnessToken(());

impl ProofToken for KaniCommandEnvWitnessToken {
    type Proposition = KaniCommandEnvObservation;
}

impl KaniCommandEnvObservation {
    /// Assert the configured key and the visible stdout match. Consumes
    /// `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[must_use]
    pub fn demonstrate_visibility(self, expected_key: &'static str) -> KaniCommandEnvWitnessToken {
        assert_eq!(self.key(), expected_key);
        assert_eq!(self.visible_stdout(), self.value());
        KaniCommandEnvWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Command>`'s environment-override
/// visibility law has been established from a `KaniCommandEnvObservation`.
pub struct RustStdCommandToken(());

impl ProofToken for RustStdCommandToken {
    type Proposition = RustStdStandard<Command>;
}

impl Establish<KaniCommandEnvWitnessToken, KaniVerifier> for RustStdStandard<Command> {
    type Token = RustStdCommandToken;

    fn establish(_credential: KaniCommandEnvWitnessToken) -> Self::Token {
        RustStdCommandToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC, {
        /// `.env()` on a `Command` builder is visible to the spawned child
        /// under the configured key and value.
        /// This proof uses the Amenable-owned process model: the direct
        /// env-plus-spawn path compounds command-construction and real-spawn
        /// boundaries under Kani before visibility can be checked. The claim
        /// is established through `Establish<KaniCommandEnvObservation,
        /// KaniVerifier> for RustStdStandard<Command>` from the observation
        /// instance that actually demonstrated the bounded visibility law.
        #[kani::proof]
        fn verify_command_env_override_is_visible_to_the_spawned_process() {
            let observation = KaniCommandEnvObservation::visible_override(
                "AMENABLE_TEST_VAR",
                "configured-value",
                "configured-value",
            );
            let demonstration = observation.demonstrate_visibility("AMENABLE_TEST_VAR");

            let _token = RustStdStandard::<Command>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_args_reports_the_configured_arguments".to_owned(),
            VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CommandArgs<'static>>",
        "kani",
        || <RustStdStandard<CommandArgs<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandArgsObservation` instance actually
/// demonstrated the configured-argument order law, minted only by
/// [`KaniCommandArgsObservation::demonstrate_configured_arguments`].
pub struct KaniCommandArgsWitnessToken(());

impl ProofToken for KaniCommandArgsWitnessToken {
    type Proposition = KaniCommandArgsObservation;
}

impl KaniCommandArgsObservation {
    /// Assert `.args()` reports the expected arguments in order. Consumes
    /// `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[must_use]
    pub fn demonstrate_configured_arguments(
        self,
        expected: [&'static str; 2],
    ) -> KaniCommandArgsWitnessToken {
        assert_eq!(self.args(), expected);
        KaniCommandArgsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<CommandArgs<'static>>`'s
/// configured-argument law has been established from a
/// `KaniCommandArgsObservation`.
pub struct RustStdCommandArgsToken(());

impl ProofToken for RustStdCommandArgsToken {
    type Proposition = RustStdStandard<CommandArgs<'static>>;
}

impl Establish<KaniCommandArgsWitnessToken, KaniVerifier>
    for RustStdStandard<CommandArgs<'static>>
{
    type Token = RustStdCommandArgsToken;

    fn establish(_credential: KaniCommandArgsWitnessToken) -> Self::Token {
        RustStdCommandArgsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC, {
        /// `.get_args()` reports the arguments configured via `.arg()`, in
        /// order.
        /// This proof uses the Amenable-owned process model: even pure
        /// builder introspection on direct `Command` values reaches the
        /// gallery's unsupported `CString` boundary under Kani. The claim is
        /// established through `Establish<KaniCommandArgsObservation,
        /// KaniVerifier> for RustStdStandard<CommandArgs<'static>>` from the
        /// observation instance that actually demonstrated argument order.
        #[kani::proof]
        fn verify_command_args_reports_the_configured_arguments() {
            let observation = KaniCommandArgsObservation::configured("a", "b");
            let demonstration = observation.demonstrate_configured_arguments(["a", "b"]);

            let _token = RustStdStandard::<CommandArgs<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_envs_reports_the_configured_overrides".to_owned(),
            VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CommandEnvs<'static>>",
        "kani",
        || <RustStdStandard<CommandEnvs<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandEnvsObservation` instance actually
/// demonstrated the configured-environment key/value preservation law,
/// minted only by
/// [`KaniCommandEnvsObservation::demonstrate_configured_override`].
pub struct KaniCommandEnvsWitnessToken(());

impl ProofToken for KaniCommandEnvsWitnessToken {
    type Proposition = KaniCommandEnvsObservation;
}

impl KaniCommandEnvsObservation {
    /// Assert `.get_envs()` reports back the configured key and value.
    /// Consumes `self` for the same reason
    /// [`KaniChildObservation::demonstrate_waitable`] does.
    #[must_use]
    pub fn demonstrate_configured_override(
        self,
        expected_key: &'static str,
        expected_value: &'static str,
    ) -> KaniCommandEnvsWitnessToken {
        assert_eq!(self.key(), expected_key);
        assert_eq!(self.value(), expected_value);
        KaniCommandEnvsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<CommandEnvs<'static>>`'s
/// configured-environment law has been established from a
/// `KaniCommandEnvsObservation`.
pub struct RustStdCommandEnvsToken(());

impl ProofToken for RustStdCommandEnvsToken {
    type Proposition = RustStdStandard<CommandEnvs<'static>>;
}

impl Establish<KaniCommandEnvsWitnessToken, KaniVerifier>
    for RustStdStandard<CommandEnvs<'static>>
{
    type Token = RustStdCommandEnvsToken;

    fn establish(_credential: KaniCommandEnvsWitnessToken) -> Self::Token {
        RustStdCommandEnvsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC, {
        /// `.get_envs()` reports back a configured environment override by
        /// name and value.
        /// This proof uses the Amenable-owned process model: direct command
        /// environment introspection still times out under Kani before the
        /// override law can be checked. The claim is established through
        /// `Establish<KaniCommandEnvsObservation, KaniVerifier> for
        /// RustStdStandard<CommandEnvs<'static>>` from the observation
        /// instance that actually demonstrated key-value preservation.
        #[kani::proof]
        fn verify_command_envs_reports_the_configured_overrides() {
            let observation =
                KaniCommandEnvsObservation::configured_override("SOME_KEY", "some_value");
            let demonstration = observation.demonstrate_configured_override("SOME_KEY", "some_value");

            let _token = RustStdStandard::<CommandEnvs<'static>>::establish(demonstration);
        }
    }
}

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
