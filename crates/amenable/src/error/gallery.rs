//! Business-rule invariants for the Kani proof-gallery CLI
//! (`gallery.rs`) -- split out of the generic `Invariant` bucket.

use std::path::PathBuf;

use tracing::instrument;

use crate::AmenableError;
use crate::error::kind::AmenableErrorKind;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("no Kani proof-gallery cases are registered ({file}:{line})")]
struct NoCasesRegisteredSource {
    line: u32,
    file: String,
}

impl NoCasesRegisteredSource {
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

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{count} proof-gallery case(s) did not match their expected outcome ({file}:{line})")]
struct CasesMismatchedSource {
    #[error(ignore)]
    count: usize,
    line: u32,
    file: String,
}

impl CasesMismatchedSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new(count: usize) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            count,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("unknown Kani proof-gallery case ID: {id} ({file}:{line})")]
struct UnknownCaseIdSource {
    #[error(ignore)]
    id: String,
    line: u32,
    file: String,
}

impl UnknownCaseIdSource {
    #[track_caller]
    #[instrument(level = "debug", skip(id))]
    pub fn new(id: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            id: id.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid Kani proof-gallery ledger header in {} ({file}:{line})", path.display())]
struct InvalidGalleryLedgerHeaderSource {
    #[error(ignore)]
    path: PathBuf,
    line: u32,
    file: String,
}

impl InvalidGalleryLedgerHeaderSource {
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
    "invalid Kani proof-gallery ledger row {row} in {} ({file}:{line})",
    path.display()
)]
struct InvalidGalleryLedgerRowSource {
    #[error(ignore)]
    row: usize,
    #[error(ignore)]
    path: PathBuf,
    line: u32,
    file: String,
}

impl InvalidGalleryLedgerRowSource {
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

/// Umbrella kind for Kani proof-gallery CLI invariants, nested one
/// level under [`crate::error::kind::AmenableErrorKind`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
enum GalleryErrorKind {
    #[display("{_0}")]
    NoCasesRegistered(NoCasesRegisteredSource),
    #[display("{_0}")]
    CasesMismatched(CasesMismatchedSource),
    #[display("{_0}")]
    UnknownCaseId(UnknownCaseIdSource),
    #[display("{_0}")]
    InvalidLedgerHeader(InvalidGalleryLedgerHeaderSource),
    #[display("{_0}")]
    InvalidLedgerRow(InvalidGalleryLedgerRowSource),
}

/// Nests [`GalleryErrorKind`] under a field named `kind`, not `source`
/// -- see [`super::assessment::AssessmentSource`]'s doc comment for why
/// (identical reasoning, same pattern).
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{kind}")]
pub struct GallerySource {
    #[error(ignore)]
    kind: Box<GalleryErrorKind>,
    line: u32,
    file: String,
}

impl GallerySource {
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
    fn new(kind: GalleryErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Convenience constructors for `AmenableErrorKind::Gallery`.
impl AmenableError {
    /// `no Kani proof-gallery cases are registered`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn no_gallery_cases_registered() -> Self {
        Self::new(AmenableErrorKind::Gallery(GallerySource::new(
            GalleryErrorKind::NoCasesRegistered(NoCasesRegisteredSource::new()),
        )))
    }

    /// `{count} proof-gallery case(s) did not match their expected outcome`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn gallery_cases_mismatched(count: usize) -> Self {
        Self::new(AmenableErrorKind::Gallery(GallerySource::new(
            GalleryErrorKind::CasesMismatched(CasesMismatchedSource::new(count)),
        )))
    }

    /// `unknown Kani proof-gallery case ID: {id}`.
    #[track_caller]
    #[instrument(level = "debug", skip(id))]
    pub fn unknown_gallery_case_id(id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Gallery(GallerySource::new(
            GalleryErrorKind::UnknownCaseId(UnknownCaseIdSource::new(id)),
        )))
    }

    /// `invalid Kani proof-gallery ledger header in {path}`.
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn invalid_gallery_ledger_header(path: impl Into<PathBuf>) -> Self {
        Self::new(AmenableErrorKind::Gallery(GallerySource::new(
            GalleryErrorKind::InvalidLedgerHeader(InvalidGalleryLedgerHeaderSource::new(path)),
        )))
    }

    /// `invalid Kani proof-gallery ledger row {row} in {path}`.
    #[track_caller]
    #[instrument(level = "debug", skip(path))]
    pub fn invalid_gallery_ledger_row(row: usize, path: impl Into<PathBuf>) -> Self {
        Self::new(AmenableErrorKind::Gallery(GallerySource::new(
            GalleryErrorKind::InvalidLedgerRow(InvalidGalleryLedgerRowSource::new(row, path)),
        )))
    }

}
