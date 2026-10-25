//! Foreign errors from IO and JSON parsing.

use std::path::PathBuf;

use tracing::instrument;
/// Preserved `std::io::Error` source, naming the artifact path that failed.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("IO error at {}: {source}", path.display())]
pub struct IoSource {
    /// The preserved `std::io::Error`.
    source: std::io::Error,
    /// The artifact path the failing operation targeted.
    path: PathBuf,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl IoSource {
    /// Preserve an IO error alongside the artifact path it failed on and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, path))]
    pub fn new(source: std::io::Error, path: impl Into<PathBuf>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            path: path.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `serde_json::Error` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct SerdeSource {
    /// The preserved `serde_json::Error`.
    source: serde_json::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl SerdeSource {
    /// Preserve a serde JSON error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: serde_json::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `serde_json::Error` source for one line of a JSON Lines
/// artifact, naming the artifact path and the 1-indexed line that failed
/// to parse. Distinct from [`SerdeSource`] (used for JSON errors with no
/// meaningful path/line, e.g. `assessment::print_json`): the two variants
/// this backs, `AmenableErrorKind::Serde`/`JsonLine`, are genuinely
/// different claims, not the same shape restated.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid JSON on line {json_line} in {}: {source}", path.display())]
pub struct JsonLineSource {
    /// The preserved `serde_json::Error`.
    source: serde_json::Error,
    /// The JSON Lines artifact path.
    path: PathBuf,
    /// The 1-indexed line number that failed to parse.
    json_line: usize,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JsonLineSource {
    /// Preserve a serde JSON error alongside the artifact path, the
    /// 1-indexed JSON line that failed, and the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, path))]
    pub fn new(source: serde_json::Error, path: impl Into<PathBuf>, json_line: usize) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            path: path.into(),
            json_line,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `std::io::Error` source from spawning `cargo kani` for one
/// registered proof, naming the proof ID the failing spawn targeted.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("could not start cargo kani for proof {proof_id}: {source}")]
pub struct KaniSpawnFailedSource {
    /// The preserved `std::io::Error`.
    source: std::io::Error,
    /// The proof ID whose `cargo kani` invocation failed to spawn.
    #[error(ignore)]
    proof_id: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl KaniSpawnFailedSource {
    /// Preserve a spawn failure alongside the proof ID it targeted and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, proof_id))]
    pub fn new(source: std::io::Error, proof_id: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            proof_id: proof_id.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
