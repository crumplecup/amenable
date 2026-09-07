use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;

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

impl Metadata for KaniExitStatusObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "a completed child exit status preserves the modeled exit code and success flag",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct Command::status path reaches the same unsupported spawn boundary as Command::spawn under Kani",
            ),
            OwnedEntry::new("exit_code", self.exit_code.to_string()),
        ]
    }
}

impl Provenance for KaniExitStatusObservation {}

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

impl Metadata for KaniOutputObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "Command::output preserves the completed exit status together with the captured stdout bytes",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct Command::output path reaches unsupported Stdio conversion machinery under Kani",
            ),
            OwnedEntry::new("exit_code", self.exit_code.to_string()),
            OwnedEntry::new("stdout_text", self.stdout_text.clone()),
        ]
    }
}

impl Provenance for KaniOutputObservation {}

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

impl Metadata for KaniStdioObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "stdio handle presence reflects the configured stdout policy for null versus piped children",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct Stdio configuration path reaches an unsupported C string literal construct under Kani",
            ),
            OwnedEntry::new(
                "null_stdout_handle_present",
                self.null_stdout_handle_present.to_string(),
            ),
            OwnedEntry::new(
                "piped_stdout_handle_present",
                self.piped_stdout_handle_present.to_string(),
            ),
        ]
    }
}

impl Provenance for KaniStdioObservation {}
