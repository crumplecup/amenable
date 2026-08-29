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
mod error;
mod fd_model;
mod fmt_model;
mod fs_model;
mod gaap_ledger;
mod gallery;
mod hash_collections_model;
mod hash_model;
mod io_model;
mod ledger;
mod linked_list_extract_model;
mod mpsc_model;
mod net_model;
mod os_windows_model;
mod panic_model;
mod path_model;
mod pipe_model;
mod process_model;
mod registry;
mod runtime_model;
mod rust_std;
mod slice_escape_ascii_model;
mod slice_split_model;
mod stoplight;
mod str_pattern_model;
mod string_drain_model;
mod sync_lock_model;
mod utf8_model;
mod witness;

pub use backtrace_model::{KaniBacktrace, KaniBacktraceStatus};
pub use btree_model::{KaniBTreeMap, KaniBTreeSet};
pub use calculator::{AddEvidence, AddToken, CalculationProof, Credit, Debit, Sum, add};
#[cfg(kani)]
pub use compose::KaniCompose;
pub use env_model::KaniArgv;
pub use env_path_model::{
    KaniEnvPath, KaniEnvPathList, KaniEnvPaths, KaniJoinPathsError, KaniJoinedEnvPaths,
};
pub use error::{KaniModelError, KaniModelErrorKind};
pub use fd_model::{KaniBorrowedFd, KaniFd, KaniFile, NonNegativeFd};
pub use fmt_model::{KaniFmt, KaniFormatAtom, KaniFormatLabel, KaniRendered, KaniRenderedKind};
pub use fs_model::{
    KaniAlreadyExists, KaniAlreadyLocked, KaniCreateNewObservation, KaniDirEntryObservation,
    KaniFileContentObservation, KaniFileLenObservation, KaniFileSystem, KaniFileTimesObservation,
    KaniFileTypeObservation, KaniFsDirEntry, KaniFsLabel, KaniFsNodeKind, KaniFsPath,
    KaniLockObservation, KaniPermissionsObservation, KaniReadDirObservation,
    KaniRecursiveDirObservation,
};
pub use hash_collections_model::{KaniHashMap, KaniHashSet};
pub use hash_model::KaniRandomStateObservation;
pub use io_model::{
    KaniBufReadSplitObservation, KaniBufReadSplitObservationBuilder, KaniBufferedReadObservation,
    KaniFlushErrorObservation, KaniLineWriterObservation, KaniLinesObservation,
    KaniWriterPanickedObservation,
};
pub use linked_list_extract_model::KaniLinkedListExtractIf;
pub use mpsc_model::{KaniChannel, KaniRecvError, KaniRecvTimeoutError, KaniSendError};
pub use net_model::{
    KaniTcpClient, KaniTcpListener, KaniTcpServer, KaniUdpSocket, KaniWriteHalfClosed,
};
pub use os_windows_model::{
    KANI_INVALID_HANDLE_VALUE, KaniWindowsHandle, KaniWindowsHandleOrInvalid, KaniWindowsSocket,
    kani_encode_wide_bmp_char,
};
pub use panic_model::{KaniCallerLocationObservation, KaniPanicHookObservation};
pub use path_model::{KaniPathDisplayObservation, KaniWindowsPrefixObservation};
pub use pipe_model::{KaniPipe, KaniPipeReader, KaniPipeWriter};
pub use process_model::{
    KaniChildObservation, KaniChildStderrObservation, KaniChildStdinObservation,
    KaniChildStdoutObservation, KaniCommandArgsObservation, KaniCommandEnvObservation,
    KaniCommandEnvsObservation, KaniExitStatusObservation, KaniOutputObservation,
    KaniStdioObservation,
};
pub use registry::{
    KaniGalleryCase, KaniGalleryDisposition, KaniGalleryExpectation, KaniGalleryRegistration,
    KaniProof, KaniProofRegistration,
};
pub use runtime_model::{KaniCurrentThreadObservation, KaniInstantObservation};
pub use rust_std::{
    AccessorRecoversTheExpectedValue, AtomicLoadReflectsTheLastWrite, CheckedProof,
    CollectedSequenceMatchesExpected, DerefReflectsTheStoredValue, EmptiedContainerReportsEmpty,
    FallibleOperationReportsFailure, FallibleOperationReportsSuccess,
    FieldAccessRecoversTheStoredValue, FirstValueIsLessThanTheSecond, FourBytesAreEachAscii,
    GetterRecoversTheStoredReference, IndexRecoversTheStoredElement,
    IteratorMatchesReferenceStepByStep, IteratorYieldsAReferenceToTheStoredValue,
    IteratorYieldsNoneWhenExhausted, NonZeroGetRoundTrips, PeekRevealsTheStoredReference,
    PopRecoversTheStoredValue, RenderedKindMatchesTheBuildingOperation,
    SplitOperandsAreDistinctFromThePattern, StrongCountTracksLiveReferences,
    ThreeBytesAreEachAscii, ThreeSplitOperandsAreDistinctFromThePattern,
    ValueIsWithinInclusiveRange, VecLengthTracksPushesAndPops, WeakUpgradeReturnsNone,
};
pub use slice_escape_ascii_model::KaniEscapeAsciiObservation;
pub use slice_split_model::{
    KaniChunkByObservation, KaniSplitNObservation, KaniSplitNObservationBuilder,
    KaniSplitObservation,
};
pub use stoplight::{Established, GreenToken, RedToken, Stoplight, StoplightError, YellowToken};
pub use str_pattern_model::{
    KaniStrMatchObservation, KaniStrMatchObservationBuilder, KaniStrRSplitNObservation,
    KaniStrRSplitNObservationBuilder, KaniStrRSplitObservation, KaniStrSplitTerminatorObservation,
};
pub use string_drain_model::KaniStringDrainObservation;
pub use sync_lock_model::{
    KaniBarrierLeaderObservation, KaniMutexExclusionObservation, KaniMutexFailureObservation,
    KaniWaitTimeoutObservation,
};
pub use utf8_model::{
    KaniAssumedUtf8Validity, KaniFromUtf8Error, KaniUtf8, KaniUtf8Buffer, KaniUtf8BufferError,
    KaniUtf8BufferToken, KaniUtf8PositionError, KaniUtf8String,
};
pub use witness::{KaniVerifier, KaniVerifierMetadata, KaniWitness};
