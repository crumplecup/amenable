//! Kani-only accommodation models for focused `std::process` laws.
//!
//! The direct `std::process::Command` / `Child` paths reach several unsupported
//! libc and pipe-construction boundaries under Kani today. This module keeps
//! the smaller Rust-facing laws the production proofs actually claim so every
//! process carrier can still have explicit passing evidence.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of spawning one waitable child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::waitable(7, 3)")]
pub struct KaniChildObservation {
    /// The modeled child process id.
    #[getter(copy)]
    process_id: u32,
    /// The modeled waited exit code -- only exposed wrapped, via
    /// [`KaniChildObservation::waited_exit_code`].
    #[getter(skip)]
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

    /// Report the modeled waited exit code.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn waited_exit_code(&self) -> Option<i32> {
        Some(self.exit_code)
    }
}

impl Provenance for KaniChildObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Observable result of capturing one child's stderr independently of stdout.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::captured(\"\", \"error message\\n\")"
)]
pub struct KaniChildStderrObservation {
    /// The modeled stdout text.
    stdout_text: String,
    /// The modeled stderr text.
    stderr_text: String,
}

impl KaniChildStderrObservation {
    /// Model one child whose stderr was piped and captured.
    #[must_use]
    pub fn captured(stdout_text: impl Into<String>, stderr_text: impl Into<String>) -> Self {
        Self {
            stdout_text: stdout_text.into(),
            stderr_text: stderr_text.into(),
        }
    }
}

impl Provenance for KaniChildStderrObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a child with piped stderr preserves stderr bytes independently of stdout",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stderr path reaches unsupported stdio pipe machinery under Kani",
            ),
            MetadataEntry::new("stdout_text", self.stdout_text.clone()),
            MetadataEntry::new("stderr_text", self.stderr_text.clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of writing text to one child's stdin and reading it back.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::echo(\"hello, child\\n\", \"hello, child\\n\")"
)]
pub struct KaniChildStdinObservation {
    /// The modeled stdin text written to the child.
    input_text: String,
    /// The modeled stdout echo from the child.
    echoed_stdout: String,
}

impl KaniChildStdinObservation {
    /// Model one child that echoes stdin back on stdout.
    #[must_use]
    pub fn echo(input_text: impl Into<String>, echoed_stdout: impl Into<String>) -> Self {
        Self {
            input_text: input_text.into(),
            echoed_stdout: echoed_stdout.into(),
        }
    }
}

impl Provenance for KaniChildStdinObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "bytes written to a piped child stdin are delivered to the child and can be echoed back on stdout",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stdin path reaches the unsupported pipe2 boundary under Kani",
            ),
            MetadataEntry::new("input_text", self.input_text.clone()),
            MetadataEntry::new("echoed_stdout", self.echoed_stdout.clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of capturing one child's stdout.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::captured(\"hello\\n\")")]
pub struct KaniChildStdoutObservation {
    /// The modeled stdout text.
    stdout_text: String,
}

impl KaniChildStdoutObservation {
    /// Model one child whose stdout was piped and captured.
    #[must_use]
    pub fn captured(stdout_text: impl Into<String>) -> Self {
        Self {
            stdout_text: stdout_text.into(),
        }
    }
}

impl Provenance for KaniChildStdoutObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a child with piped stdout preserves the bytes it wrote there",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct piped-child stdout path reaches the unsupported pipe2 boundary under Kani",
            ),
            MetadataEntry::new("stdout_text", self.stdout_text.clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of reading back the configured command arguments.
#[derive(Debug, Clone, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::configured(\"a\", \"b\")")]
pub struct KaniCommandArgsObservation {
    args: [String; 2],
}

impl KaniCommandArgsObservation {
    /// Model two configured command arguments in order.
    #[must_use]
    pub fn configured(first: impl Into<String>, second: impl Into<String>) -> Self {
        Self {
            args: [first.into(), second.into()],
        }
    }

    /// Report the modeled configured arguments.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn args(&self) -> [&str; 2] {
        [self.args[0].as_str(), self.args[1].as_str()]
    }
}

impl Provenance for KaniCommandArgsObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "command builder argument introspection preserves the configured argument sequence",
            ),
            MetadataEntry::new(
                "rationale",
                "even direct Command construction reaches an unsupported CString strlen boundary under Kani",
            ),
            MetadataEntry::new("first_arg", self.args[0].clone()),
            MetadataEntry::new("second_arg", self.args[1].clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of one environment override being visible to a spawned command.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::visible_override(\"AMENABLE_TEST_VAR\", \"configured-value\", \"configured-value\")"
)]
pub struct KaniCommandEnvObservation {
    /// The modeled override key.
    key: String,
    /// The modeled override value.
    value: String,
    /// The modeled child-visible stdout text.
    visible_stdout: String,
}

impl KaniCommandEnvObservation {
    /// Model one command environment override that is visible in child output.
    #[must_use]
    pub fn visible_override(
        key: impl Into<String>,
        value: impl Into<String>,
        visible_stdout: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            visible_stdout: visible_stdout.into(),
        }
    }
}

impl Provenance for KaniCommandEnvObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a command environment override is visible to the spawned child under the same key and value",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct env-plus-spawn path combines unsupported command construction and real spawn boundaries under Kani",
            ),
            MetadataEntry::new("key", self.key.clone()),
            MetadataEntry::new("value", self.value.clone()),
            MetadataEntry::new("visible_stdout", self.visible_stdout.clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of reading back one configured environment override.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::configured_override(\"SOME_KEY\", \"some_value\")"
)]
pub struct KaniCommandEnvsObservation {
    /// The modeled override key.
    key: String,
    /// The modeled override value.
    value: String,
}

impl KaniCommandEnvsObservation {
    /// Model one configured environment override stored on a command builder.
    #[must_use]
    pub fn configured_override(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Provenance for KaniCommandEnvsObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "command builder environment introspection preserves configured key-value overrides",
            ),
            MetadataEntry::new(
                "rationale",
                "direct Command environment introspection still times out under Kani before the override law can be checked",
            ),
            MetadataEntry::new("key", self.key.clone()),
            MetadataEntry::new("value", self.value.clone()),
        ]
        .into_iter()
        })
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn code(&self) -> Option<i32> {
        Some(self.exit_code)
    }

    /// Report whether the modeled exit status is successful.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

impl Provenance for KaniExitStatusObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Observable result of collecting one command's output bundle.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::captured(0, \"hello\\n\")")]
pub struct KaniOutputObservation {
    /// The modeled exit status code -- only exposed wrapped, via
    /// [`KaniOutputObservation::status_code`]/[`KaniOutputObservation::success`].
    #[getter(skip)]
    exit_code: i32,
    /// The modeled stdout text.
    stdout_text: String,
}

impl KaniOutputObservation {
    /// Model one completed output bundle with its stdout text.
    #[must_use]
    pub fn captured(exit_code: i32, stdout_text: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout_text: stdout_text.into(),
        }
    }

    /// Report the modeled exit status code.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn status_code(&self) -> Option<i32> {
        Some(self.exit_code)
    }

    /// Report whether the modeled output status is successful.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

impl Provenance for KaniOutputObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
            MetadataEntry::new("stdout_text", self.stdout_text.clone()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of choosing between `Stdio::null()` and `Stdio::piped()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::stdout_handle_policy(false, true)")]
pub struct KaniStdioObservation {
    /// Whether `Stdio::null()` leaves a stdout handle present.
    #[getter(copy)]
    null_stdout_handle_present: bool,
    /// Whether `Stdio::piped()` leaves a stdout handle present.
    #[getter(copy)]
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
}

impl Provenance for KaniStdioObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}
