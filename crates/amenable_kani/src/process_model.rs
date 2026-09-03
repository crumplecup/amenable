//! Kani-only accommodation models for focused `std::process` laws.
//!
//! The direct `std::process::Command` / `Child` paths reach several unsupported
//! libc and pipe-construction boundaries under Kani today. This module keeps
//! the smaller Rust-facing laws the production proofs actually claim so every
//! process carrier can still have explicit passing evidence.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

mod command;
mod completion;

pub use command::{
    KaniCommandArgsObservation, KaniCommandEnvObservation, KaniCommandEnvsObservation,
};
pub use completion::{KaniExitStatusObservation, KaniOutputObservation, KaniStdioObservation};

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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
