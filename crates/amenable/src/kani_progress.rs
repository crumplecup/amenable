//! Live progress tracking for `cargo kani`'s own stdout, driving an
//! `indicatif` progress bar instead of leaving the terminal silent for
//! the whole run.
//!
//! Kani's output is a fixed sequence of stage markers (confirmed by
//! direct observation across many real runs, not guessed): symbolic
//! execution, VCC generation (which states the total property count
//! up front, in `Generated N VCC(s), M remaining after simplification`),
//! SSA conversion, then a repeating per-property cycle (`Running
//! propositional reduction` -> `Solving with CaDiCaL` -> `N variables,
//! M clauses` -> `SAT checker:` -> `Runtime Solver:` -> `Runtime
//! decision procedure:`) that recurs once per property CBMC checks.
//! `Runtime decision procedure:` is the last line of that cycle, so a
//! property is counted complete there, not on the cycle's first line.

use regex::Regex;
use std::sync::OnceLock;
use tracing::instrument;

/// Kani/CBMC's own pipeline stage, as inferred from its stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KaniStage {
    /// Harness selected, nothing reported yet.
    #[default]
    Starting,
    /// `Kani Rust Verifier` banner or a `warning:`-prefixed rustc
    /// diagnostic line seen -- this phase genuinely streams
    /// incrementally (confirmed by direct, timestamped observation:
    /// cargo/rustc flush line-by-line here), unlike everything CBMC
    /// itself prints once verification starts, which arrives in one
    /// buffered burst at process exit regardless of how long the real
    /// work took.
    Compiling {
        /// A real, live-incrementing count of `warning:` lines seen so
        /// far, not an estimate.
        warnings_seen: u32,
    },
    /// `Starting Bounded Model Checking` seen.
    BoundedModelChecking,
    /// A bounded loop is being unwound.
    Unwinding {
        /// The most recent iteration reported, not a total -- Kani
        /// doesn't state the bound.
        iteration: u64,
    },
    /// `Generated N VCC(s), M remaining after simplification` seen.
    GeneratingVccs {
        /// `M`, the known total property count.
        total: usize,
    },
    /// `converting SSA` seen.
    ConvertingSsa,
    /// Inside the repeating per-property solving cycle.
    Solving {
        /// Number of `Runtime decision procedure:` lines seen so far.
        completed: usize,
        /// The known total property count, if a `GeneratingVccs` line
        /// reported one before solving started.
        total: Option<usize>,
    },
    /// A final `VERIFICATION:-` line was seen.
    Done {
        /// Whether that line reported `SUCCESSFUL` rather than `FAILED`.
        successful: bool,
    },
}

struct Patterns {
    kani_banner: Regex,
    warning_line: Regex,
    bmc_start: Regex,
    unwinding: Regex,
    generated_vcc: Regex,
    converting_ssa: Regex,
    decision_procedure: Regex,
    verification_result: Regex,
}

/// Builds every pattern, short-circuiting on the first failure. Only
/// reachable if a pattern literal above is itself malformed -- never
/// a runtime condition -- but this tracker exists purely for progress
/// UX, not correctness, so even that case degrades to "show nothing
/// recognized" rather than aborting the whole verification run.
#[instrument(level = "debug", err(level = "warn"))]
fn build_patterns() -> Result<Patterns, regex::Error> {
    Ok(Patterns {
        kani_banner: Regex::new(r"^Kani Rust Verifier")?,
        warning_line: Regex::new(r"^warning:")?,
        bmc_start: Regex::new(r"^Starting Bounded Model Checking")?,
        unwinding: Regex::new(r"^Unwinding loop .* iteration (\d+)")?,
        generated_vcc: Regex::new(
            r"^Generated \d+ VCC\(s\), (\d+) remaining after simplification",
        )?,
        converting_ssa: Regex::new(r"^converting SSA")?,
        decision_procedure: Regex::new(r"^Runtime decision procedure:")?,
        verification_result: Regex::new(r"^VERIFICATION:-\s*(SUCCESSFUL|FAILED)")?,
    })
}

#[instrument(level = "debug")]
fn patterns() -> Option<&'static Patterns> {
    static PATTERNS: OnceLock<Option<Patterns>> = OnceLock::new();
    PATTERNS
        .get_or_init(|| match build_patterns() {
            Ok(patterns) => Some(patterns),
            Err(error) => {
                tracing::error!(%error, "kani progress patterns failed to compile");
                None
            }
        })
        .as_ref()
}

/// Tracks `KaniStage` transitions across a `cargo kani` run's stdout,
/// one line at a time.
#[derive(Default, derive_getters::Getters)]
pub struct KaniProgressTracker {
    /// The current stage after all lines observed so far.
    #[getter(copy)]
    stage: KaniStage,
}

impl KaniProgressTracker {
    /// A fresh tracker, starting at `KaniStage::Starting`.
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed one line of `cargo kani`'s stdout, updating the tracked
    /// stage. Lines matching no known pattern leave the stage
    /// unchanged.
    #[instrument(level = "debug", skip(self))]
    pub fn observe_line(&mut self, line: &str) {
        let Some(patterns) = patterns() else {
            return;
        };

        if let Some(captures) = patterns.verification_result.captures(line) {
            self.stage = KaniStage::Done {
                successful: &captures[1] == "SUCCESSFUL",
            };
            return;
        }
        if patterns.kani_banner.is_match(line) {
            self.stage = KaniStage::Compiling { warnings_seen: 0 };
            return;
        }
        if patterns.warning_line.is_match(line) {
            let warnings_seen = match self.stage {
                KaniStage::Compiling { warnings_seen } => warnings_seen + 1,
                _ => 1,
            };
            self.stage = KaniStage::Compiling { warnings_seen };
            return;
        }
        if patterns.bmc_start.is_match(line) {
            self.stage = KaniStage::BoundedModelChecking;
            return;
        }
        if let Some(captures) = patterns.unwinding.captures(line) {
            let iteration = captures[1].parse().unwrap_or(0);
            self.stage = KaniStage::Unwinding { iteration };
            return;
        }
        if let Some(captures) = patterns.generated_vcc.captures(line) {
            let total = captures[1].parse().unwrap_or(0);
            self.stage = KaniStage::GeneratingVccs { total };
            return;
        }
        if patterns.converting_ssa.is_match(line) {
            self.stage = KaniStage::ConvertingSsa;
            return;
        }
        if patterns.decision_procedure.is_match(line) {
            let total = match self.stage {
                KaniStage::GeneratingVccs { total } => Some(total),
                KaniStage::Solving { total, .. } => total,
                _ => None,
            };
            let completed = match self.stage {
                KaniStage::Solving { completed, .. } => completed + 1,
                _ => 1,
            };
            self.stage = KaniStage::Solving { completed, total };
        }
    }
}
