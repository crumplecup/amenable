//! `KaniWitness` impls for `std::process`.
//!
//! Every spawning harness runs a trivial command through the platform's
//! own shell (`cmd /C` on Windows, `sh -c` elsewhere), chosen at runtime
//! via `cfg!(windows)` rather than `#[cfg(windows)]` so the same harness
//! source is meaningful on either platform. `ExitCode` is the one
//! exception: it has no introspection API at all (it's only meaningful
//! once actually passed to process exit, which a proof harness can't do
//! without killing itself), so it stays "trusted."

use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, CommandArgs, CommandEnvs, ExitCode,
    ExitStatus, Output, Stdio,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<Child> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_has_a_process_id_and_can_be_waited_on",
            claim: VERIFY_CHILD_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CHILD_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC, {
        /// A freshly spawned `Child` has a nonzero process id, and
        /// `.wait()` reports the exit code it was told to return.
        #[kani::proof]
        fn verify_child_has_a_process_id_and_can_be_waited_on() {
            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let mut child = Command::new(shell).arg(flag).arg("exit 3").spawn().unwrap();
            assert_ne!(child.id(), 0, "a spawned child has a nonzero process id");

            let status = child.wait().unwrap();
            assert_eq!(status.code(), Some(3));
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stderr_captures_what_the_child_wrote_to_stderr",
            claim: VERIFY_CHILD_STDERR_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDERR_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC, {
        /// Piping a child's stderr captures exactly what it wrote there,
        /// separately from stdout.
        #[kani::proof]
        fn verify_child_stderr_captures_what_the_child_wrote_to_stderr() {
            use std::io::Read;

            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let mut child = Command::new(shell)
                .arg(flag)
                .arg("echo error message 1>&2")
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();

            let mut collected = String::new();
            child.stderr.take().unwrap().read_to_string(&mut collected).unwrap();
            child.wait().unwrap();

            assert!(collected.contains("error message"), "stderr was: {collected:?}");
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stdin_is_readable_by_the_child_process",
            claim: VERIFY_CHILD_STDIN_IS_READABLE_BY_THE_CHILD_PROCESS_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDIN_IS_READABLE_BY_THE_CHILD_PROCESS_SRC, {
        /// Bytes written to a piped `ChildStdin` are genuinely delivered
        /// to the child process, which a cat-like command then echoes
        /// back on its own stdout.
        #[kani::proof]
        fn verify_child_stdin_is_readable_by_the_child_process() {
            use std::io::Write;

            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let cat = if cfg!(windows) { "more" } else { "cat" };
            let mut child = Command::new(shell)
                .arg(flag)
                .arg(cat)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"hello, child\n")
                .unwrap();
            // Dropping the taken `ChildStdin` above closes the pipe, so
            // the child sees EOF and exits.
            let output = child.wait_with_output().unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("hello, child"), "stdout was: {stdout:?}");
        }
    }
}

impl KaniWitness for RustStdStandard<ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_child_stdout_captures_what_the_child_wrote_to_stdout",
            claim: VERIFY_CHILD_STDOUT_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CHILD_STDOUT_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC, {
        /// Piping a child's stdout captures exactly what it printed.
        #[kani::proof]
        fn verify_child_stdout_captures_what_the_child_wrote_to_stdout() {
            use std::io::Read;

            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let mut child = Command::new(shell)
                .arg(flag)
                .arg("echo hello")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut collected = String::new();
            child.stdout.take().unwrap().read_to_string(&mut collected).unwrap();
            child.wait().unwrap();

            assert!(collected.contains("hello"), "stdout was: {collected:?}");
        }
    }
}

impl KaniWitness for RustStdStandard<Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_env_override_is_visible_to_the_spawned_process",
            claim: VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC, {
        /// `.env()` on the `Command` builder is genuinely propagated to
        /// the spawned child's environment.
        #[kani::proof]
        fn verify_command_env_override_is_visible_to_the_spawned_process() {
            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let echo_var = if cfg!(windows) {
                "echo %AMENABLE_TEST_VAR%"
            } else {
                "echo $AMENABLE_TEST_VAR"
            };
            let output = Command::new(shell)
                .arg(flag)
                .arg(echo_var)
                .env("AMENABLE_TEST_VAR", "configured-value")
                .output()
                .unwrap();

            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("configured-value"), "stdout was: {stdout:?}");
        }
    }
}

impl KaniWitness for RustStdStandard<CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_args_reports_the_configured_arguments",
            claim: VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC, {
        /// `.get_args()` reports back exactly the arguments configured
        /// via `.arg()`, in order — no spawning needed, since this is
        /// pure builder introspection.
        #[kani::proof]
        fn verify_command_args_reports_the_configured_arguments() {
            let mut command = Command::new("prog");
            command.arg("a").arg("b");
            let args: Vec<&std::ffi::OsStr> = command.get_args().collect();
            assert_eq!(
                args,
                vec![std::ffi::OsStr::new("a"), std::ffi::OsStr::new("b")]
            );
        }
    }
}

impl KaniWitness for RustStdStandard<CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_command_envs_reports_the_configured_overrides",
            claim: VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC, {
        /// `.get_envs()` reports back a configured environment override
        /// by name — pure builder introspection, no spawning needed.
        #[kani::proof]
        fn verify_command_envs_reports_the_configured_overrides() {
            let mut command = Command::new("prog");
            command.env("SOME_KEY", "some_value");
            let found = command
                .get_envs()
                .find(|(key, _)| *key == std::ffi::OsStr::new("SOME_KEY"))
                .unwrap();
            assert_eq!(found.1, Some(std::ffi::OsStr::new("some_value")));
        }
    }
}

impl KaniWitness for RustStdStandard<ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_exit_status_reports_a_nonzero_exit_code",
            claim: VERIFY_EXIT_STATUS_REPORTS_A_NONZERO_EXIT_CODE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_EXIT_STATUS_REPORTS_A_NONZERO_EXIT_CODE_SRC, {
        /// A process that exits with a specific nonzero code reports
        /// `!success()` and that exact code.
        #[kani::proof]
        fn verify_exit_status_reports_a_nonzero_exit_code() {
            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let status = Command::new(shell).arg(flag).arg("exit 3").status().unwrap();
            assert!(!status.success());
            assert_eq!(status.code(), Some(3));
        }
    }
}

impl KaniWitness for RustStdStandard<Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_output_captures_stdout_and_the_exit_status",
            claim: VERIFY_OUTPUT_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_OUTPUT_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC, {
        /// `.output()` bundles a successfully-run command's exit status
        /// with the stdout it produced.
        #[kani::proof]
        fn verify_output_captures_stdout_and_the_exit_status() {
            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
            let output = Command::new(shell).arg(flag).arg("echo hello").output().unwrap();
            assert!(output.status.success());
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("hello"), "stdout was: {stdout:?}");
        }
    }
}

impl KaniWitness for RustStdStandard<Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_stdio_null_discards_the_childs_output_handle",
            claim: VERIFY_STDIO_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_STDIO_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC, {
        /// `.stdout(Stdio::null())` leaves `Child::stdout` as `None`,
        /// while `.stdout(Stdio::piped())` gives back `Some` — the
        /// handle's presence directly reflects the `Stdio` configured.
        #[kani::proof]
        fn verify_stdio_null_discards_the_childs_output_handle() {
            let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };

            let mut null_child = Command::new(shell)
                .arg(flag)
                .arg("echo discarded")
                .stdout(Stdio::null())
                .spawn()
                .unwrap();
            assert!(null_child.stdout.is_none());
            null_child.wait().unwrap();

            let mut piped_child = Command::new(shell)
                .arg(flag)
                .arg("echo captured")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            assert!(piped_child.stdout.is_some());
            piped_child.wait().unwrap();
        }
    }
}

impl_kani_witness_trusted!(ExitCode);
