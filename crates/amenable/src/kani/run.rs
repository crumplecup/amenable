//! Runs a single registered Kani proof as a child process and turns
//! its outcome into a `ProofRun`, rendering live progress from its
//! stdout along the way. Split out of `kani.rs` as its own named
//! layer once that file grew past the point a human reader could
//! take it in as one unit -- CLI argument parsing, ledger
//! persistence, and proof selection on one side, process execution
//! and progress rendering on the other.

use std::{
    collections::BTreeMap,
    fs, io,
    process::{Child, ChildStderr, ChildStdout, ExitStatus, Stdio},
};

use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::{ProofStatus, first_diagnostic_line, is_kani_timeout, kani_command};
use crate::{AmenableError, AmenableResult, KaniProof};

/// Separate from the verification-status ledger in `kani.rs` on
/// purpose -- this records a progress-bar UX hint (how many `Runtime
/// decision procedure:` events a harness actually produced last
/// time), not a verification result, and mixing the two would mean
/// every consumer of the status ledger's CSV schema has to account
/// for a column it never cared about.
#[instrument(level = "debug")]
fn default_chapters_cache_path() -> std::path::PathBuf {
    crate::paths::artifacts_directory().join("kani-progress-chapters.json")
}

#[derive(derive_getters::Getters, derive_new::new)]
pub(super) struct ProofRun {
    #[getter(copy)]
    status: ProofStatus,
    message: Option<String>,
}

impl ProofRun {
    #[instrument(level = "debug")]
    fn passed() -> Self {
        Self::new(ProofStatus::Passed, None)
    }
}

/// Persisted, per-harness record of the real `Runtime decision
/// procedure:` event count a run actually observed -- a progress-bar
/// UX hint, not a verification result (see `default_chapters_cache_
/// path`'s own doc comment for why this is a separate file, never a
/// column on `Ledger`). JSON, not CSV: this is a flat key-value map,
/// not a row-per-event history.
#[derive(Debug, Default, Serialize, Deserialize)]
struct ChaptersCache {
    chapters: BTreeMap<String, usize>,
}

impl ChaptersCache {
    /// Loads the cache from `path`, or an empty one if it doesn't
    /// exist yet or fails to parse (a UX hint is worth losing
    /// silently; it is never worth failing a verification run over).
    #[instrument(level = "debug", skip(path))]
    fn load(path: &std::path::Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    /// The cached chapter count for `proof_id`, if any run has ever
    /// recorded one.
    #[instrument(level = "trace", skip(self))]
    fn get(&self, proof_id: &str) -> Option<usize> {
        self.chapters.get(proof_id).copied()
    }

    /// Records `chapters` as the latest observed count for `proof_id`.
    #[instrument(level = "trace", skip(self))]
    fn set(&mut self, proof_id: String, chapters: usize) {
        self.chapters.insert(proof_id, chapters);
    }

    /// Writes the cache to `path`.
    #[instrument(level = "debug", skip(self, path))]
    fn save(&self, path: &std::path::Path) -> AmenableResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents).map_err(|error| AmenableError::io(path, error))
    }
}

/// Runs `record`, rendering live progress from `cargo kani`'s own
/// stdout (see `crate::kani_progress`) instead of leaving the terminal
/// silent for the run's whole duration. stdout drives the progress
/// bar on the main thread; stderr is drained on its own thread to
/// avoid a two-pipe deadlock, matching the standard pattern for
/// capturing both streams of a child process the caller also wants to
/// react to while it runs.
///
/// The honest shape, not a fabricated one: a harness's own property
/// *count* (from e.g. `--show-properties`) is not the right
/// denominator for progress measured in `Runtime decision procedure:`
/// events -- confirmed directly, a 316-property harness produced only
/// 7 such events, since CBMC batches an unknown, variable number of
/// properties per incremental solve, not one-for-one. There is no
/// upfront signal for that batch count; it is only knowable by
/// actually observing a run. So: the first time this harness is seen,
/// show only a spinner (no bar, no invented total). Once a run
/// completes, cache the real, observed event count against this
/// harness's own ID (`ChaptersCache`) -- every run after that gets a
/// real, stable, honest bar from its very first frame, seeded with
/// what this exact harness actually produced last time.
#[instrument(level = "info", skip(record))]
pub(super) fn run_proof(record: &KaniProof, harness_timeout: &str) -> ProofRun {
    use std::io::{BufReader, Read};

    let cache_path = default_chapters_cache_path();
    let mut cache = ChaptersCache::load(&cache_path);
    let known_chapters = cache.get(record.id());

    let (mut child, stdout, stderr) = match spawn_piped(record, harness_timeout) {
        Ok(parts) => parts,
        Err(error) => return ProofRun::new(ProofStatus::Failed, Some(error.to_string())),
    };

    let stderr_thread = std::thread::spawn(move || {
        let mut buffer = String::new();
        let _ = BufReader::new(stderr).read_to_string(&mut buffer);
        buffer
    });

    let (stdout_text, observed_chapters) = drive_progress_bar(stdout, known_chapters);

    let stderr_text = stderr_thread.join().unwrap_or_default();
    let wait_result = child.wait();

    // Record what this run actually produced, regardless of outcome --
    // a failed or timed-out run still observed a real chapter count,
    // and the next run (success or not) benefits from knowing it.
    if observed_chapters > 0 {
        cache.set(record.id().to_owned(), observed_chapters);
        let _ = cache.save(&cache_path);
    }

    interpret_wait_result(wait_result, &stdout_text, &stderr_text)
}

/// Starts `cargo kani` for `record` with stdout/stderr piped, splitting
/// both pipes out of the `Child` immediately so the caller can drain
/// each on its own thread without borrowing `child` twice.
#[instrument(level = "debug", skip(record))]
fn spawn_piped(
    record: &KaniProof,
    harness_timeout: &str,
) -> Result<(Child, ChildStdout, ChildStderr), AmenableError> {
    let mut child = kani_command(record, harness_timeout)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| AmenableError::kani_spawn_failed(record.id(), error))?;

    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        // Unreachable for a real `Command`: both were just requested
        // as piped above. Still a real, returned failure, not a
        // panic, since nothing about this path is worth asserting
        // via an abort when the existing `Err` arm just above already
        // shows the honest shape for "this run could not proceed".
        return Err(AmenableError::kani_stdout_stderr_not_piped());
    };
    Ok((child, stdout, stderr))
}

/// Streams `stdout` line by line, driving an indicatif progress bar
/// from each observed `KaniStage`, and returns the full captured text
/// alongside the last `Solving`-stage chapter count observed (0 if the
/// run never reached that stage). Seeds a real, length-based bar
/// immediately when `known_chapters` (a prior run's own observed
/// count for this exact harness) is already known; otherwise stays an
/// honest spinner until this run's own `Generated N VCC(s), M
/// remaining after simplification` line arrives, if it ever does.
#[instrument(level = "debug", skip(stdout))]
fn drive_progress_bar(stdout: ChildStdout, known_chapters: Option<usize>) -> (String, usize) {
    use std::io::{BufRead, BufReader};

    let bar = indicatif::ProgressBar::new_spinner();
    // CBMC's own verification output is fully buffered internally and
    // arrives as one burst right at process exit (confirmed by direct,
    // timestamped observation -- a ~33s real gap produced zero lines),
    // so a line-driven-only redraw would leave the spinner frozen for
    // however long that silent stretch runs. A steady tick animates it
    // regardless, so the bar still shows the process is alive, not hung.
    bar.enable_steady_tick(std::time::Duration::from_millis(120));
    let mut has_real_bar = false;
    if let Some(total) = known_chapters {
        bar.set_length(total as u64);
        if let Ok(style) = indicatif::ProgressStyle::with_template("{bar:40} {pos}/{len} {msg}") {
            bar.set_style(style);
        }
        has_real_bar = true;
    }
    let mut tracker = crate::kani_progress::KaniProgressTracker::new();
    let mut stdout_text = String::new();
    let mut observed_chapters: usize = 0;

    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        tracker.observe_line(&line);
        stdout_text.push_str(&line);
        stdout_text.push('\n');
        if let crate::kani_progress::KaniStage::Solving { completed, .. } = tracker.stage() {
            observed_chapters = completed;
        }
        render_stage(&bar, tracker.stage(), &mut has_real_bar);
    }
    bar.finish_and_clear();
    (stdout_text, observed_chapters)
}

/// Turns the child's exit status, plus its captured stdout/stderr,
/// into the `ProofRun` the rest of the CLI reports.
#[instrument(level = "debug", skip(wait_result, stdout_text, stderr_text))]
fn interpret_wait_result(
    wait_result: io::Result<ExitStatus>,
    stdout_text: &str,
    stderr_text: &str,
) -> ProofRun {
    match wait_result {
        Ok(status) if status.success() => ProofRun::passed(),
        Ok(_) => {
            let diagnostics = format!("{stdout_text}{stderr_text}");
            let status = if is_kani_timeout(&diagnostics) {
                ProofStatus::Timeout
            } else {
                ProofStatus::Failed
            };
            ProofRun::new(status, first_diagnostic_line(&diagnostics))
        }
        Err(error) => ProofRun::new(
            ProofStatus::Failed,
            Some(format!("could not wait on cargo kani: {error}")),
        ),
    }
}

/// Reflects one `KaniStage` onto `bar`. Starts as an indeterminate
/// spinner; switches to a real length-based bar either immediately
/// (a cached prior-run chapter count already seeded it before this
/// function is ever called) or, failing that, the first time this
/// run's own `Generated N VCC(s), M remaining after simplification`
/// line arrives -- which, given the buffering this module's own docs
/// describe, is rarely useful in practice, but is still the honest
/// fallback for a harness's very first run.
#[instrument(level = "debug", skip(bar, stage))]
fn render_stage(
    bar: &indicatif::ProgressBar,
    stage: crate::kani_progress::KaniStage,
    has_real_bar: &mut bool,
) {
    use crate::kani_progress::KaniStage;

    match stage {
        KaniStage::Starting => bar.set_message("starting..."),
        KaniStage::Compiling { warnings_seen } => {
            bar.set_message(if warnings_seen == 0 {
                "compiling...".to_owned()
            } else {
                format!("compiling... ({warnings_seen} warnings so far)")
            });
        }
        KaniStage::BoundedModelChecking => bar.set_message("bounded model checking..."),
        KaniStage::Unwinding { iteration } => {
            bar.set_message(format!("unwinding loop (iteration {iteration})..."));
        }
        KaniStage::GeneratingVccs { total } => {
            bar.set_message(format!(
                "{total} propert{} to check",
                if total == 1 { "y" } else { "ies" }
            ));
        }
        KaniStage::ConvertingSsa => bar.set_message("converting SSA..."),
        KaniStage::Solving { completed, .. } => {
            // `*has_real_bar` is only ever set by a cached, prior-run
            // chapter count seeded before the loop starts (`run_proof`)
            // -- deliberately never by this run's own tracked property
            // total (`Generated N VCC(s)...`), confirmed to be the
            // wrong denominator for this metric. No cache yet means
            // an honest, position-less message for the whole run.
            if *has_real_bar {
                bar.set_position(completed as u64);
                bar.set_message("solving");
            } else {
                bar.set_message(format!("solving: property {completed}"));
            }
        }
        KaniStage::Done { successful } => {
            bar.set_message(if successful {
                "verified"
            } else {
                "not verified"
            });
        }
    }
}
