//! Kani verifier backend for the `amenable` constitutional trait family.
//!
//! `KaniVerifier` is defined *here*, not in `amenable_core` — there is only
//! one verifier Kani works with, Kani, so the marker belongs with the crate
//! that means it. That locality is what makes
//! `impl amenable_core::Witness<KaniVerifier> for amenable_std::RustStdStandard<T>`
//! legal under Rust's orphan rule, one concrete type at a time: the rule
//! requires *some* type in `Witness<KaniVerifier>`'s type list to be local,
//! and `KaniVerifier` now is. A blanket impl over a bare type parameter
//! still isn't legal (the parameter itself is never "covered"), which is
//! why each type gets its own [`KaniWitness`] impl plus a one-line
//! mechanical bridge, rather than one generic impl for all of them — see
//! `rust_std.rs`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate self as amenable_kani;

mod backtrace_model;
mod btree_model;
mod calculator;
mod compose;
mod env_model;
mod env_path_model;
mod fd_model;
mod fmt_model;
mod fs_model;
mod gallery;
mod io_model;
mod linked_list_extract_model;
mod mpsc_model;
mod pipe_model;
mod registry;
mod rust_std;
mod slice_split_model;
mod stoplight;
mod sync_lock_model;
mod utf8_model;
mod witness;

pub use backtrace_model::{KaniBacktrace, KaniBacktraceStatus};
pub use btree_model::{KaniBTreeMap, KaniBTreeSet};
pub use calculator::{AddEvidence, AddToken, CalculationProof, Credit, Debit, Sum, add};
pub use compose::KaniCompose;
pub use env_model::KaniArgv;
pub use env_path_model::{
    KaniEnvPath, KaniEnvPathList, KaniEnvPaths, KaniJoinPathsError, KaniJoinedEnvPaths,
};
pub use fd_model::{KaniBorrowedFd, KaniFd, KaniFile};
pub use fmt_model::{KaniFmt, KaniFormatAtom, KaniFormatLabel, KaniRendered, KaniRenderedKind};
pub use fs_model::{
    KaniAlreadyExists, KaniAlreadyLocked, KaniCreateNewObservation, KaniDirEntryObservation,
    KaniFileContentObservation, KaniFileLenObservation, KaniFileSystem, KaniFileTimesObservation,
    KaniFileTypeObservation, KaniFsDirEntry, KaniFsLabel, KaniFsNodeKind, KaniFsPath,
    KaniLockObservation, KaniPermissionsObservation, KaniReadDirObservation,
    KaniRecursiveDirObservation,
};
pub use io_model::{
    KaniBufReadSplitObservation, KaniBufferedReadObservation, KaniFlushErrorObservation,
    KaniLineWriterObservation, KaniLinesObservation, KaniWriterPanickedObservation,
};
pub use linked_list_extract_model::KaniLinkedListExtractIf;
pub use mpsc_model::{KaniChannel, KaniRecvError, KaniSendError};
pub use pipe_model::{KaniPipe, KaniPipeReader, KaniPipeWriter};
pub use registry::{
    KaniGalleryCase, KaniGalleryDisposition, KaniGalleryExpectation, KaniGalleryRegistration,
    KaniProof, KaniProofRegistration,
};
pub use rust_std::CheckedProof;
pub use slice_split_model::{KaniSplitNObservation, KaniSplitObservation};
pub use stoplight::{
    Color, Established, Green, GreenToken, Red, RedToken, SequentialCycle, Stoplight, Yellow,
    YellowToken, next,
};
pub use sync_lock_model::{
    KaniBarrierLeaderObservation, KaniMutexExclusionObservation, KaniMutexFailureObservation,
    KaniWaitTimeoutObservation,
};
pub use utf8_model::{
    KaniAssumedUtf8Validity, KaniFromUtf8Error, KaniUtf8, KaniUtf8Buffer, KaniUtf8BufferError,
    KaniUtf8BufferToken, KaniUtf8PositionError, KaniUtf8String,
};
pub use witness::{KaniVerifier, KaniVerifierMetadata, KaniWitness};
