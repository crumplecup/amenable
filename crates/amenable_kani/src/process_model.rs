//! Kani-only accommodation models for focused `std::process` laws.
//!
//! The direct `std::process::Command` / `Child` paths reach several unsupported
//! libc and pipe-construction boundaries under Kani today. This module keeps
//! the smaller Rust-facing laws the production proofs actually claim so every
//! process carrier can still have explicit passing evidence.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of spawning one waitable child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::waitable(7, 3)")]
pub struct KaniChildObservation {
    process_id: u32,
    exit_code: i32,
}

impl KaniChildObservation {
    /// Model one spawned child that can be waited on successfully.
    #[must_use]
    pub fn waitable(process_id: u32, exit_code: i32) -> Self {
        Self {
            process_id,
            exit_code,
        }
    }

    /// Report the modeled child process id.
    #[must_use]
    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    /// Report the modeled waited exit code.
    #[must_use]
    pub fn waited_exit_code(&self) -> Option<i32> {
        Some(self.exit_code)
    }
}

impl Provenance for KaniChildObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a spawned child reports a nonzero process id and waiting on it yields the modeled exit code",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Command::spawn path reaches an unsupported gnu_get_libc_version boundary under Kani",
            ),
            MetadataEntry::new("process_id", self.process_id.to_string()),
            MetadataEntry::new("exit_code", self.exit_code.to_string()),
        ]
        .into_iter()
    }
}

/// Observable result of capturing one child's stderr independently of stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::captured(\"\", \"error message\\n\")"
)]
pub struct KaniChildStderrObservation {
    stdout_text: &'static str,
    stderr_text: &'static str,
}

impl KaniChildStderrObservation {
    /// Model one child whose stderr was piped and captured.
    #[must_use]
    pub fn captured(stdout_text: &'static str, stderr_text: &'static str) -> Self {
        Self {
            stdout_text,
            stderr_text,
        }
    }

    /// Report the modeled stdout text.
    #[must_use]
    pub fn stdout_text(&self) -> &'static str {
        self.stdout_text
    }

    /// Report the modeled stderr text.
    #[must_use]
    pub fn stderr_text(&self) -> &'static str {
        self.stderr_text
    }
}

impl Provenance for KaniChildStderrObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a child with piped stderr preserves stderr bytes independently of stdout",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stderr path reaches unsupported stdio pipe machinery under Kani",
            ),
            MetadataEntry::new("stdout_text", self.stdout_text),
            MetadataEntry::new("stderr_text", self.stderr_text),
        ]
        .into_iter()
    }
}

/// Observable result of writing text to one child's stdin and reading it back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::echo(\"hello, child\\n\", \"hello, child\\n\")"
)]
pub struct KaniChildStdinObservation {
    input_text: &'static str,
    echoed_stdout: &'static str,
}

impl KaniChildStdinObservation {
    /// Model one child that echoes stdin back on stdout.
    #[must_use]
    pub fn echo(input_text: &'static str, echoed_stdout: &'static str) -> Self {
        Self {
            input_text,
            echoed_stdout,
        }
    }

    /// Report the modeled stdin text written to the child.
    #[must_use]
    pub fn input_text(&self) -> &'static str {
        self.input_text
    }

    /// Report the modeled stdout echo from the child.
    #[must_use]
    pub fn echoed_stdout(&self) -> &'static str {
        self.echoed_stdout
    }
}

impl Provenance for KaniChildStdinObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "bytes written to a piped child stdin are delivered to the child and can be echoed back on stdout",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stdin path reaches the unsupported pipe2 boundary under Kani",
            ),
            MetadataEntry::new("input_text", self.input_text),
            MetadataEntry::new("echoed_stdout", self.echoed_stdout),
        ]
        .into_iter()
    }
}

/// Observable result of capturing one child's stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::captured(\"hello\\n\")")]
pub struct KaniChildStdoutObservation {
    stdout_text: &'static str,
}

impl KaniChildStdoutObservation {
    /// Model one child whose stdout was piped and captured.
    #[must_use]
    pub fn captured(stdout_text: &'static str) -> Self {
        Self { stdout_text }
    }

    /// Report the modeled stdout text.
    #[must_use]
    pub fn stdout_text(&self) -> &'static str {
        self.stdout_text
    }
}

impl Provenance for KaniChildStdoutObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a child with piped stdout preserves the bytes it wrote there",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stdout path reaches the unsupported pipe2 boundary under Kani",
            ),
            MetadataEntry::new("stdout_text", self.stdout_text),
        ]
        .into_iter()
    }
}

/// Observable result of reading back the configured command arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::configured([\"a\", \"b\"])")]
pub struct KaniCommandArgsObservation {
    args: [&'static str; 2],
}

impl KaniCommandArgsObservation {
    /// Model two configured command arguments in order.
    #[must_use]
    pub fn configured(args: [&'static str; 2]) -> Self {
        Self { args }
    }

    /// Report the modeled configured arguments.
    #[must_use]
    pub fn args(&self) -> [&'static str; 2] {
        self.args
    }
}

impl Provenance for KaniCommandArgsObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "command builder argument introspection preserves the configured argument sequence",
            ),
            MetadataEntry::new(
                "rationale",
                "even direct Command construction reaches an unsupported CString strlen boundary under Kani",
            ),
            MetadataEntry::new("first_arg", self.args[0]),
            MetadataEntry::new("second_arg", self.args[1]),
        ]
        .into_iter()
    }
}

/// Observable result of one environment override being visible to a spawned command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::visible_override(\"AMENABLE_TEST_VAR\", \"configured-value\", \"configured-value\")"
)]
pub struct KaniCommandEnvObservation {
    key: &'static str,
    value: &'static str,
    visible_stdout: &'static str,
}

impl KaniCommandEnvObservation {
    /// Model one command environment override that is visible in child output.
    #[must_use]
    pub fn visible_override(
        key: &'static str,
        value: &'static str,
        visible_stdout: &'static str,
    ) -> Self {
        Self {
            key,
            value,
            visible_stdout,
        }
    }

    /// Report the modeled override key.
    #[must_use]
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// Report the modeled override value.
    #[must_use]
    pub fn value(&self) -> &'static str {
        self.value
    }

    /// Report the modeled child-visible stdout text.
    #[must_use]
    pub fn visible_stdout(&self) -> &'static str {
        self.visible_stdout
    }
}

impl Provenance for KaniCommandEnvObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a command environment override is visible to the spawned child under the same key and value",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct env-plus-spawn path combines unsupported command construction and real spawn boundaries under Kani",
            ),
            MetadataEntry::new("key", self.key),
            MetadataEntry::new("value", self.value),
            MetadataEntry::new("visible_stdout", self.visible_stdout),
        ]
        .into_iter()
    }
}

/// Observable result of reading back one configured environment override.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::configured_override(\"SOME_KEY\", \"some_value\")"
)]
pub struct KaniCommandEnvsObservation {
    key: &'static str,
    value: &'static str,
}

impl KaniCommandEnvsObservation {
    /// Model one configured environment override stored on a command builder.
    #[must_use]
    pub fn configured_override(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }

    /// Report the modeled override key.
    #[must_use]
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// Report the modeled override value.
    #[must_use]
    pub fn value(&self) -> &'static str {
        self.value
    }
}

impl Provenance for KaniCommandEnvsObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "command builder environment introspection preserves configured key-value overrides",
            ),
            MetadataEntry::new(
                "rationale",
                "direct Command environment introspection still times out under Kani before the override law can be checked",
            ),
            MetadataEntry::new("key", self.key),
            MetadataEntry::new("value", self.value),
        ]
        .into_iter()
    }
}

/// Observable result of one completed child exit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::nonzero(3)")]
pub struct KaniExitStatusObservation {
    exit_code: i32,
}

impl KaniExitStatusObservation {
    /// Model one completed child with a nonzero exit code.
    #[must_use]
    pub fn nonzero(exit_code: i32) -> Self {
        Self { exit_code }
    }

    /// Report the modeled exit status code.
    #[must_use]
    pub fn code(&self) -> Option<i32> {
        Some(self.exit_code)
    }

    /// Report whether the modeled exit status is successful.
    #[must_use]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

impl Provenance for KaniExitStatusObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a completed child exit status preserves the modeled exit code and success flag",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Command::status path reaches the same unsupported spawn boundary as Command::spawn under Kani",
            ),
            MetadataEntry::new("exit_code", self.exit_code.to_string()),
        ]
        .into_iter()
    }
}

/// Observable result of collecting one command's output bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::captured(0, \"hello\\n\")")]
pub struct KaniOutputObservation {
    exit_code: i32,
    stdout_text: &'static str,
}

impl KaniOutputObservation {
    /// Model one completed output bundle with its stdout text.
    #[must_use]
    pub fn captured(exit_code: i32, stdout_text: &'static str) -> Self {
        Self {
            exit_code,
            stdout_text,
        }
    }

    /// Report the modeled exit status code.
    #[must_use]
    pub fn status_code(&self) -> Option<i32> {
        Some(self.exit_code)
    }

    /// Report whether the modeled output status is successful.
    #[must_use]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }

    /// Report the modeled stdout text.
    #[must_use]
    pub fn stdout_text(&self) -> &'static str {
        self.stdout_text
    }
}

impl Provenance for KaniOutputObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "Command::output preserves the completed exit status together with the captured stdout bytes",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Command::output path reaches unsupported Stdio conversion machinery under Kani",
            ),
            MetadataEntry::new("exit_code", self.exit_code.to_string()),
            MetadataEntry::new("stdout_text", self.stdout_text),
        ]
        .into_iter()
    }
}

/// Observable result of choosing between `Stdio::null()` and `Stdio::piped()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::stdout_handle_policy(false, true)")]
pub struct KaniStdioObservation {
    null_stdout_handle_present: bool,
    piped_stdout_handle_present: bool,
}

impl KaniStdioObservation {
    /// Model whether stdout handles are exposed for null and piped stdio.
    #[must_use]
    pub fn stdout_handle_policy(
        null_stdout_handle_present: bool,
        piped_stdout_handle_present: bool,
    ) -> Self {
        Self {
            null_stdout_handle_present,
            piped_stdout_handle_present,
        }
    }

    /// Report whether `Stdio::null()` leaves a stdout handle present.
    #[must_use]
    pub fn null_stdout_handle_present(&self) -> bool {
        self.null_stdout_handle_present
    }

    /// Report whether `Stdio::piped()` leaves a stdout handle present.
    #[must_use]
    pub fn piped_stdout_handle_present(&self) -> bool {
        self.piped_stdout_handle_present
    }
}

impl Provenance for KaniStdioObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "stdio handle presence reflects the configured stdout policy for null versus piped children",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Stdio configuration path reaches an unsupported C string literal construct under Kani",
            ),
            MetadataEntry::new(
                "null_stdout_handle_present",
                self.null_stdout_handle_present.to_string(),
            ),
            MetadataEntry::new(
                "piped_stdout_handle_present",
                self.piped_stdout_handle_present.to_string(),
            ),
        ]
        .into_iter()
    }
}
