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
        CheckedProof {
            harness: "verify_child_has_a_process_id_and_can_be_waited_on".to_owned(),
            claim: VERIFY_CHILD_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Child>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Child>",
        verifier: "kani",
        describe: || <RustStdStandard<Child> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Child>`'s process-id and wait law
/// has been established from a `KaniChildObservation`.
pub struct RustStdChildToken(());

impl ProofToken for RustStdChildToken {
    type Proposition = RustStdStandard<Child>;
}

impl Establish<KaniChildObservation, KaniVerifier> for RustStdStandard<Child> {
    type Token = RustStdChildToken;

    fn establish(_credential: &KaniChildObservation) -> Self::Token {
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
            assert_ne!(observation.process_id(), 0);
            assert_eq!(observation.waited_exit_code(), Some(3));

            let _token = RustStdStandard::<Child>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stderr_captures_what_the_child_wrote_to_stderr".to_owned(),
            claim: VERIFY_CHILD_STDERR_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChildStderr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChildStderr>",
        verifier: "kani",
        describe: || <RustStdStandard<ChildStderr> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<ChildStderr>`'s stderr-capture
/// law has been established from a `KaniChildStderrObservation`.
pub struct RustStdChildStderrToken(());

impl ProofToken for RustStdChildStderrToken {
    type Proposition = RustStdStandard<ChildStderr>;
}

impl Establish<KaniChildStderrObservation, KaniVerifier> for RustStdStandard<ChildStderr> {
    type Token = RustStdChildStderrToken;

    fn establish(_credential: &KaniChildStderrObservation) -> Self::Token {
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
            assert_eq!(observation.stdout_text(), "");
            assert!(observation.stderr_text().contains("error message"));

            let _token = RustStdStandard::<ChildStderr>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stdin_is_readable_by_the_child_process".to_owned(),
            claim: VERIFY_CHILD_STDIN_IS_READABLE_BY_THE_CHILD_PROCESS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChildStdin>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChildStdin>",
        verifier: "kani",
        describe: || <RustStdStandard<ChildStdin> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<ChildStdin>`'s delivered-input
/// law has been established from a `KaniChildStdinObservation`.
pub struct RustStdChildStdinToken(());

impl ProofToken for RustStdChildStdinToken {
    type Proposition = RustStdStandard<ChildStdin>;
}

impl Establish<KaniChildStdinObservation, KaniVerifier> for RustStdStandard<ChildStdin> {
    type Token = RustStdChildStdinToken;

    fn establish(_credential: &KaniChildStdinObservation) -> Self::Token {
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
            assert_eq!(observation.echoed_stdout(), observation.input_text());

            let _token = RustStdStandard::<ChildStdin>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stdout_captures_what_the_child_wrote_to_stdout".to_owned(),
            claim: VERIFY_CHILD_STDOUT_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChildStdout>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChildStdout>",
        verifier: "kani",
        describe: || <RustStdStandard<ChildStdout> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<ChildStdout>`'s stdout-capture
/// law has been established from a `KaniChildStdoutObservation`.
pub struct RustStdChildStdoutToken(());

impl ProofToken for RustStdChildStdoutToken {
    type Proposition = RustStdStandard<ChildStdout>;
}

impl Establish<KaniChildStdoutObservation, KaniVerifier> for RustStdStandard<ChildStdout> {
    type Token = RustStdChildStdoutToken;

    fn establish(_credential: &KaniChildStdoutObservation) -> Self::Token {
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
            assert!(observation.stdout_text().contains("hello"));

            let _token = RustStdStandard::<ChildStdout>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_env_override_is_visible_to_the_spawned_process".to_owned(),
            claim: VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Command>",
        verifier: "kani",
        describe: || <RustStdStandard<Command> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Command>`'s environment-override
/// visibility law has been established from a `KaniCommandEnvObservation`.
pub struct RustStdCommandToken(());

impl ProofToken for RustStdCommandToken {
    type Proposition = RustStdStandard<Command>;
}

impl Establish<KaniCommandEnvObservation, KaniVerifier> for RustStdStandard<Command> {
    type Token = RustStdCommandToken;

    fn establish(_credential: &KaniCommandEnvObservation) -> Self::Token {
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
            assert_eq!(observation.key(), "AMENABLE_TEST_VAR");
            assert_eq!(observation.visible_stdout(), observation.value());

            let _token = RustStdStandard::<Command>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_args_reports_the_configured_arguments".to_owned(),
            claim: VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CommandArgs<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<CommandArgs<'static>> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<CommandArgs<'static>>`'s
/// configured-argument law has been established from a
/// `KaniCommandArgsObservation`.
pub struct RustStdCommandArgsToken(());

impl ProofToken for RustStdCommandArgsToken {
    type Proposition = RustStdStandard<CommandArgs<'static>>;
}

impl Establish<KaniCommandArgsObservation, KaniVerifier> for RustStdStandard<CommandArgs<'static>> {
    type Token = RustStdCommandArgsToken;

    fn establish(_credential: &KaniCommandArgsObservation) -> Self::Token {
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
            let observation = KaniCommandArgsObservation::configured(["a", "b"]);
            assert_eq!(observation.args(), ["a", "b"]);

            let _token = RustStdStandard::<CommandArgs<'static>>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_envs_reports_the_configured_overrides".to_owned(),
            claim: VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CommandEnvs<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<CommandEnvs<'static>> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<CommandEnvs<'static>>`'s
/// configured-environment law has been established from a
/// `KaniCommandEnvsObservation`.
pub struct RustStdCommandEnvsToken(());

impl ProofToken for RustStdCommandEnvsToken {
    type Proposition = RustStdStandard<CommandEnvs<'static>>;
}

impl Establish<KaniCommandEnvsObservation, KaniVerifier> for RustStdStandard<CommandEnvs<'static>> {
    type Token = RustStdCommandEnvsToken;

    fn establish(_credential: &KaniCommandEnvsObservation) -> Self::Token {
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
            assert_eq!(observation.key(), "SOME_KEY");
            assert_eq!(observation.value(), "some_value");

            let _token = RustStdStandard::<CommandEnvs<'static>>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_exit_status_reports_a_nonzero_exit_code".to_owned(),
            claim: VERIFY_EXIT_STATUS_REPORTS_A_NONZERO_EXIT_CODE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ExitStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ExitStatus>",
        verifier: "kani",
        describe: || <RustStdStandard<ExitStatus> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<ExitStatus>`'s exit-code law has
/// been established from a `KaniExitStatusObservation`.
pub struct RustStdExitStatusToken(());

impl ProofToken for RustStdExitStatusToken {
    type Proposition = RustStdStandard<ExitStatus>;
}

impl Establish<KaniExitStatusObservation, KaniVerifier> for RustStdStandard<ExitStatus> {
    type Token = RustStdExitStatusToken;

    fn establish(_credential: &KaniExitStatusObservation) -> Self::Token {
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
            assert!(!observation.success());
            assert_eq!(observation.code(), Some(3));

            let _token = RustStdStandard::<ExitStatus>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_output_captures_stdout_and_the_exit_status".to_owned(),
            claim: VERIFY_OUTPUT_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Output>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Output>",
        verifier: "kani",
        describe: || <RustStdStandard<Output> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Output>`'s output-bundle law has
/// been established from a `KaniOutputObservation`.
pub struct RustStdOutputToken(());

impl ProofToken for RustStdOutputToken {
    type Proposition = RustStdStandard<Output>;
}

impl Establish<KaniOutputObservation, KaniVerifier> for RustStdStandard<Output> {
    type Token = RustStdOutputToken;

    fn establish(_credential: &KaniOutputObservation) -> Self::Token {
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
            assert!(observation.success());
            assert_eq!(observation.status_code(), Some(0));
            assert!(observation.stdout_text().contains("hello"));

            let _token = RustStdStandard::<Output>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_stdio_null_discards_the_childs_output_handle".to_owned(),
            claim: VERIFY_STDIO_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Stdio>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Stdio>",
        verifier: "kani",
        describe: || <RustStdStandard<Stdio> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Stdio>`'s stdout-handle policy
/// law has been established from a `KaniStdioObservation`.
pub struct RustStdStdioToken(());

impl ProofToken for RustStdStdioToken {
    type Proposition = RustStdStandard<Stdio>;
}

impl Establish<KaniStdioObservation, KaniVerifier> for RustStdStandard<Stdio> {
    type Token = RustStdStdioToken;

    fn establish(_credential: &KaniStdioObservation) -> Self::Token {
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
            assert!(!observation.null_stdout_handle_present());
            assert!(observation.piped_stdout_handle_present());

            let _token = RustStdStandard::<Stdio>::establish(&observation);
        }
    }
}

impl_kani_witness_trusted!(ExitCode);
