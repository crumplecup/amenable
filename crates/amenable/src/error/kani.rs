//! Business-rule invariants for the Kani verification CLI (`kani.rs`,
//! `kani/ledger.rs`, `kani/run.rs`) -- split out of the generic
//! `Invariant` bucket.

use std::path::PathBuf;

use tracing::instrument;

use crate::AmenableError;
use crate::error::kind::AmenableErrorKind;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{unsuccessful} Kani proof(s) did not pass ({file}:{line})")]
struct ProofsDidNotPassSource {
    #[error(ignore)]
    unsuccessful: usize,
    line: u32,
    file: String,
}

impl ProofsDidNotPassSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new(unsuccessful: usize) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            unsuccessful,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("unknown Kani proof ID: {proof} ({file}:{line})")]
struct UnknownKaniProofIdSource {
    #[error(ignore)]
    proof: String,
    line: u32,
    file: String,
}

impl UnknownKaniProofIdSource {
    #[track_caller]
    #[instrument(level = "debug", skip(proof))]
    pub fn new(proof: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            proof: proof.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid Kani result ledger header in {} ({file}:{line})", path.display())]
struct InvalidResultLedgerHeaderSource {
    #[error(ignore)]
    path: PathBuf,
    line: u32,
    file: String,
}

impl InvalidResultLedgerHeaderSource {
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            path: path.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "invalid Kani result ledger row {row} in {} ({file}:{line})",
    path.display()
)]
struct InvalidResultLedgerRowSource {
    #[error(ignore)]
    row: usize,
    #[error(ignore)]
    path: PathBuf,
    line: u32,
    file: String,
}

impl InvalidResultLedgerRowSource {
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn new(row: usize, path: impl Into<PathBuf>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            row,
            path: path.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("cargo kani's stdout/stderr were not piped as requested ({file}:{line})")]
struct StdoutStderrNotPipedSource {
    line: u32,
    file: String,
}

impl StdoutStderrNotPipedSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        let loc = std::panic::Location::caller();
        Self {
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Umbrella kind for Kani verification CLI invariants, nested one
/// level under [`crate::error::kind::AmenableErrorKind`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
enum KaniErrorKind {
    #[display("{_0}")]
    ProofsDidNotPass(ProofsDidNotPassSource),
    #[display("{_0}")]
    UnknownProofId(UnknownKaniProofIdSource),
    #[display("{_0}")]
    InvalidResultLedgerHeader(InvalidResultLedgerHeaderSource),
    #[display("{_0}")]
    InvalidResultLedgerRow(InvalidResultLedgerRowSource),
    #[display("{_0}")]
    StdoutStderrNotPiped(StdoutStderrNotPipedSource),
}

/// Nests [`KaniErrorKind`] under a field named `kind`, not `source` --
/// see [`super::assessment::AssessmentSource`]'s doc comment for why
/// (identical reasoning, same pattern).
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{kind}")]
pub struct KaniSource {
    #[error(ignore)]
    kind: Box<KaniErrorKind>,
    line: u32,
    file: String,
}

impl KaniSource {
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
    fn new(kind: KaniErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Convenience constructors for `AmenableErrorKind::Kani`.
impl AmenableError {
    /// `{unsuccessful} Kani proof(s) did not pass`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn kani_proofs_did_not_pass(unsuccessful: usize) -> Self {
        Self::new(AmenableErrorKind::Kani(KaniSource::new(
            KaniErrorKind::ProofsDidNotPass(ProofsDidNotPassSource::new(unsuccessful)),
        )))
    }

    /// `unknown Kani proof ID: {proof}`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof))]
    pub fn unknown_kani_proof_id(proof: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Kani(KaniSource::new(
            KaniErrorKind::UnknownProofId(UnknownKaniProofIdSource::new(proof)),
        )))
    }

    /// `invalid Kani result ledger header in {path}`.
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn invalid_result_ledger_header(path: impl Into<PathBuf>) -> Self {
        Self::new(AmenableErrorKind::Kani(KaniSource::new(
            KaniErrorKind::InvalidResultLedgerHeader(InvalidResultLedgerHeaderSource::new(path)),
        )))
    }

    /// `invalid Kani result ledger row {row} in {path}`.
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn invalid_result_ledger_row(row: usize, path: impl Into<PathBuf>) -> Self {
        Self::new(AmenableErrorKind::Kani(KaniSource::new(
            KaniErrorKind::InvalidResultLedgerRow(InvalidResultLedgerRowSource::new(row, path)),
        )))
    }

    /// `cargo kani's stdout/stderr were not piped as requested`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn kani_stdout_stderr_not_piped() -> Self {
        Self::new(AmenableErrorKind::Kani(KaniSource::new(
            KaniErrorKind::StdoutStderrNotPiped(StdoutStderrNotPipedSource::new()),
        )))
    }

}
