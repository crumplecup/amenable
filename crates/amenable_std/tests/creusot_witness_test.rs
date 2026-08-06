#![cfg(feature = "creusot")]

use std::alloc::{Layout, LayoutError, System};
use std::any::TypeId;
use std::array::{IntoIter as ArrayIntoIter, TryFromSliceError};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::borrow::Cow;
use std::boxed::Box;
use std::cell::{
    BorrowError, BorrowMutError, Cell, LazyCell, OnceCell, Ref, RefCell, RefMut, UnsafeCell,
};
use std::char::{
    CharTryFromError, DecodeUtf16, DecodeUtf16Error, ParseCharError, ToLowercase, ToUppercase,
    TryFromCharError,
};
use std::cmp::Reverse;
use std::collections::binary_heap::{
    Drain as BinaryHeapDrain, IntoIter as BinaryHeapIntoIter, Iter as BinaryHeapIter,
    PeekMut as BinaryHeapPeekMut,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::linked_list::{
    ExtractIf as LinkedListExtractIf, IntoIter as LinkedListIntoIter, Iter as LinkedListIter,
    IterMut as LinkedListIterMut,
};
use std::collections::vec_deque::{
    Drain as VecDequeDrain, IntoIter as VecDequeIntoIter, Iter as VecDequeIter,
    IterMut as VecDequeIterMut,
};
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, TryReserveError, VecDeque,
};
use std::convert::Infallible;
use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, VarError, Vars, VarsOs};
use std::ffi::os_str::Display as OsStrDisplay;
use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError};
use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError, OsStr, OsString};
use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
};
use std::fmt::{
    Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter, FromFn,
};
use std::future::{Pending, PollFn, Ready};
use std::hash::{BuildHasherDefault, RandomState};
use std::io::{
    BufReader, BufWriter, Cursor, IntoInnerError, IoSlice, IoSliceMut, LineWriter, PipeReader,
    PipeWriter, SeekFrom, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock,
    WriterPanicked,
};
use std::iter::{
    Cloned, Copied, Cycle, Empty, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect,
    Map, MapWhile, OnceWith, Peekable, RepeatN, RepeatWith, Rev, Scan, Skip, SkipWhile, StepBy,
    Successors, TakeWhile, Zip,
};
use std::marker::{PhantomData, PhantomPinned};
use std::mem::{Discriminant, ManuallyDrop};
use std::net::{
    AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4,
    SocketAddrV6, TcpListener, TcpStream, UdpSocket,
};
use std::num::{NonZero, Saturating, Wrapping};
use std::ops::{Bound, ControlFlow, Range, RangeFull, RangeTo};
use std::panic::{AssertUnwindSafe, PanicHookInfo};
use std::pin::Pin;
use std::ptr::NonNull;
use std::path::{
    Ancestors, Component, Components, Display as PathDisplay, Iter as PathIter, Path, PathBuf,
    Prefix, PrefixComponent, StripPrefixError,
};
use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, CommandArgs, CommandEnvs, ExitCode,
    ExitStatus, Output, Stdio,
};
use std::rc::Rc;
use std::slice::{
    ChunkBy, ChunkByMut, Chunks, ChunksExact, ChunksExactMut, ChunksMut, EscapeAscii,
    GetDisjointMutError, Iter, RChunks, RChunksExact, RChunksExactMut, RChunksMut, RSplitMut,
    RSplitNMut, SplitInclusiveMut, SplitMut, SplitNMut, Windows,
};
use std::str::{
    Bytes as StrBytes, CharIndices, Chars, EncodeUtf16, EscapeDebug as StrEscapeDebug,
    EscapeDefault as StrEscapeDefault, EscapeUnicode as StrEscapeUnicode, Lines as StrLines,
    MatchIndices, Matches, ParseBoolError, RMatchIndices, RMatches, RSplit as StrRSplit,
    RSplitN as StrRSplitN, RSplitTerminator, Split as StrSplit, SplitAsciiWhitespace,
    SplitInclusive as StrSplitInclusive, SplitN as StrSplitN, SplitTerminator, SplitWhitespace,
    Utf8Chunk, Utf8Chunks, Utf8Error,
};
use std::string::{FromUtf8Error, FromUtf16Error};
use std::sync::{
    Arc, Barrier, BarrierWaitResult, LazyLock, OnceLock, OnceState, WaitTimeoutResult,
};
use std::sync::mpsc::SyncSender;
use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering as AtomicOrdering,
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};
use std::time::{Duration, Instant, SystemTime, SystemTimeError};
use std::vec::{
    Drain as VecDrain, ExtractIf as VecExtractIf, IntoIter as VecIntoIter, Splice as VecSplice, Vec,
};

use core::panic::{Location, PanicInfo, PanicMessage};

use amenable_core::Witness;
use amenable_creusot::CreusotVerifier;
use amenable_std::{RustStdStandard, RustStdType};

#[expect(
    deprecated,
    reason = "SipHasher itself is stable, only deprecated as a recommendation to use DefaultHasher instead; covering it is a coverage-completeness question, not a call to use it"
)]
type SipHasherAlias = std::hash::SipHasher;

#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
type LinesAnyStatic = std::str::LinesAny<'static>;

macro_rules! assert_trusted_creusot_witness {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            assert_eq!(
                <RustStdStandard<$ty> as Witness<CreusotVerifier>>::proof(),
                <$ty as RustStdType>::provenance()
            );
        }
    };
}

#[test]
fn bool_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<bool> as Witness<CreusotVerifier>>::proof(),
        <bool as RustStdType>::provenance()
    );
}

#[test]
fn char_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<char> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_char_roundtrip");
    assert_eq!(proof.provenance, <char as RustStdType>::provenance());
}

macro_rules! assert_checked_atomic_creusot_witness {
    ($name:ident, $ty:ty, $harness:literal) => {
        #[test]
        fn $name() {
            let proof = <RustStdStandard<$ty> as Witness<CreusotVerifier>>::proof();

            assert_eq!(proof.harness, $harness);
            assert_eq!(proof.provenance, <$ty as RustStdType>::provenance());
        }
    };
}

assert_checked_atomic_creusot_witness!(
    atomic_bool_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicBool,
    "verify_atomic_bool_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_i8_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicI8,
    "verify_atomic_i8_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_i16_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicI16,
    "verify_atomic_i16_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_i32_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicI32,
    "verify_atomic_i32_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_i64_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicI64,
    "verify_atomic_i64_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_isize_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicIsize,
    "verify_atomic_isize_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_u8_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicU8,
    "verify_atomic_u8_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_u16_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicU16,
    "verify_atomic_u16_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_u32_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicU32,
    "verify_atomic_u32_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_u64_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicU64,
    "verify_atomic_u64_load_store"
);
assert_checked_atomic_creusot_witness!(
    atomic_usize_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicUsize,
    "verify_atomic_usize_load_store"
);
#[test]
fn atomic_ordering_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<AtomicOrdering> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_relaxed_ordering_still_makes_a_store_observable"
    );
    assert_eq!(
        proof.provenance,
        <AtomicOrdering as RustStdType>::provenance()
    );
}

#[test]
fn system_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<System> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_system_allocates_and_deallocates_a_layout"
    );
    assert_eq!(proof.provenance, <System as RustStdType>::provenance());
}

#[test]
fn backtrace_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Backtrace> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_backtrace_force_capture_always_actually_captures"
    );
    assert_eq!(proof.provenance, <Backtrace as RustStdType>::provenance());
}

#[test]
fn backtrace_status_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BacktraceStatus> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_backtrace_status_reports_captured_after_force_capture"
    );
    assert_eq!(
        proof.provenance,
        <BacktraceStatus as RustStdType>::provenance()
    );
}

#[test]
fn seek_from_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<SeekFrom> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_seek_from_round_trips_each_variants_offset"
    );
    assert_eq!(proof.provenance, <SeekFrom as RustStdType>::provenance());
}

assert_trusted_creusot_witness!(
    buf_reader_witness_is_trusted_and_carries_chain_derived_provenance,
    BufReader<&'static [u8]>
);
assert_trusted_creusot_witness!(
    buf_writer_witness_is_trusted_and_carries_chain_derived_provenance,
    BufWriter<Vec<u8>>
);
assert_trusted_creusot_witness!(
    io_bytes_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Bytes<&'static [u8]>
);
assert_trusted_creusot_witness!(
    io_chain_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Chain<&'static [u8], &'static [u8]>
);
assert_trusted_creusot_witness!(
    cursor_witness_is_trusted_and_carries_chain_derived_provenance,
    Cursor<&'static [u8]>
);
assert_trusted_creusot_witness!(
    io_empty_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Empty
);
assert_trusted_creusot_witness!(
    io_error_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Error
);
assert_trusted_creusot_witness!(
    into_inner_error_witness_is_trusted_and_carries_chain_derived_provenance,
    IntoInnerError<BufWriter<Vec<u8>>>
);
assert_trusted_creusot_witness!(
    io_slice_witness_is_trusted_and_carries_chain_derived_provenance,
    IoSlice<'static>
);
assert_trusted_creusot_witness!(
    io_slice_mut_witness_is_trusted_and_carries_chain_derived_provenance,
    IoSliceMut<'static>
);
assert_trusted_creusot_witness!(
    line_writer_witness_is_trusted_and_carries_chain_derived_provenance,
    LineWriter<Vec<u8>>
);
assert_trusted_creusot_witness!(
    io_lines_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Lines<&'static [u8]>
);
assert_trusted_creusot_witness!(
    pipe_reader_witness_is_trusted_and_carries_chain_derived_provenance,
    PipeReader
);
assert_trusted_creusot_witness!(
    pipe_writer_witness_is_trusted_and_carries_chain_derived_provenance,
    PipeWriter
);
assert_trusted_creusot_witness!(
    io_repeat_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Repeat
);
assert_trusted_creusot_witness!(
    sink_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Sink
);
assert_trusted_creusot_witness!(
    io_split_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Split<&'static [u8]>
);
assert_trusted_creusot_witness!(
    stderr_witness_is_trusted_and_carries_chain_derived_provenance,
    Stderr
);
assert_trusted_creusot_witness!(
    stderr_lock_witness_is_trusted_and_carries_chain_derived_provenance,
    StderrLock<'static>
);
assert_trusted_creusot_witness!(
    stdin_witness_is_trusted_and_carries_chain_derived_provenance,
    Stdin
);
assert_trusted_creusot_witness!(
    stdin_lock_witness_is_trusted_and_carries_chain_derived_provenance,
    StdinLock<'static>
);
assert_trusted_creusot_witness!(
    stdout_witness_is_trusted_and_carries_chain_derived_provenance,
    Stdout
);
assert_trusted_creusot_witness!(
    stdout_lock_witness_is_trusted_and_carries_chain_derived_provenance,
    StdoutLock<'static>
);
assert_trusted_creusot_witness!(
    take_witness_is_trusted_and_carries_chain_derived_provenance,
    std::io::Take<&'static [u8]>
);
assert_trusted_creusot_witness!(
    writer_panicked_witness_is_trusted_and_carries_chain_derived_provenance,
    WriterPanicked
);

#[test]
fn shutdown_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Shutdown> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_shutdown_write_prevents_further_writes"
    );
    assert_eq!(proof.provenance, <Shutdown as RustStdType>::provenance());
}

assert_checked_atomic_creusot_witness!(
    atomic_ptr_i32_witness_is_checked_and_still_carries_chain_derived_provenance,
    AtomicPtr<i32>,
    "verify_atomic_ptr_load_store"
);

#[test]
fn string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<String> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_string_roundtrip");
    assert_eq!(proof.provenance, <String as RustStdType>::provenance());
}

#[test]
fn array_i32_3_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<[i32; 3]> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_array_indexing_and_length");
    assert_eq!(proof.provenance, <[i32; 3] as RustStdType>::provenance());
}

#[test]
fn slice_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<[i32]> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_slice_indexing_and_length");
    assert_eq!(proof.provenance, <[i32] as RustStdType>::provenance());
}

#[test]
fn slice_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::Iter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_slice_iter_yields_shared_references_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::IterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_slice_iter_mut_yields_mutable_references_that_write_through"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

assert_trusted_creusot_witness!(
    str_bytes_witness_is_trusted_and_carries_chain_derived_provenance,
    StrBytes<'static>
);
assert_trusted_creusot_witness!(
    str_char_indices_witness_is_trusted_and_carries_chain_derived_provenance,
    CharIndices<'static>
);
assert_trusted_creusot_witness!(
    str_chars_witness_is_trusted_and_carries_chain_derived_provenance,
    Chars<'static>
);
assert_trusted_creusot_witness!(
    str_encode_utf16_witness_is_trusted_and_carries_chain_derived_provenance,
    EncodeUtf16<'static>
);
assert_trusted_creusot_witness!(
    str_escape_debug_witness_is_trusted_and_carries_chain_derived_provenance,
    StrEscapeDebug<'static>
);
assert_trusted_creusot_witness!(
    str_escape_default_witness_is_trusted_and_carries_chain_derived_provenance,
    StrEscapeDefault<'static>
);
assert_trusted_creusot_witness!(
    str_escape_unicode_witness_is_trusted_and_carries_chain_derived_provenance,
    StrEscapeUnicode<'static>
);
assert_trusted_creusot_witness!(
    str_lines_witness_is_trusted_and_carries_chain_derived_provenance,
    StrLines<'static>
);
assert_trusted_creusot_witness!(
    str_split_ascii_whitespace_witness_is_trusted_and_carries_chain_derived_provenance,
    SplitAsciiWhitespace<'static>
);
assert_trusted_creusot_witness!(
    str_split_whitespace_witness_is_trusted_and_carries_chain_derived_provenance,
    SplitWhitespace<'static>
);
assert_trusted_creusot_witness!(
    utf8_chunk_witness_is_trusted_and_carries_chain_derived_provenance,
    Utf8Chunk<'static>
);
assert_trusted_creusot_witness!(
    utf8_chunks_witness_is_trusted_and_carries_chain_derived_provenance,
    Utf8Chunks<'static>
);
assert_trusted_creusot_witness!(
    parse_bool_error_witness_is_trusted_and_carries_chain_derived_provenance,
    ParseBoolError
);
assert_trusted_creusot_witness!(
    utf8_error_witness_is_trusted_and_carries_chain_derived_provenance,
    Utf8Error
);
assert_trusted_creusot_witness!(
    lines_any_witness_is_trusted_and_carries_chain_derived_provenance,
    LinesAnyStatic
);
assert_trusted_creusot_witness!(
    str_split_witness_is_trusted_and_carries_chain_derived_provenance,
    StrSplit<'static, char>
);
assert_trusted_creusot_witness!(
    str_rsplit_witness_is_trusted_and_carries_chain_derived_provenance,
    StrRSplit<'static, char>
);
assert_trusted_creusot_witness!(
    str_split_n_witness_is_trusted_and_carries_chain_derived_provenance,
    StrSplitN<'static, char>
);
assert_trusted_creusot_witness!(
    str_rsplit_n_witness_is_trusted_and_carries_chain_derived_provenance,
    StrRSplitN<'static, char>
);
assert_trusted_creusot_witness!(
    str_split_inclusive_witness_is_trusted_and_carries_chain_derived_provenance,
    StrSplitInclusive<'static, char>
);
assert_trusted_creusot_witness!(
    str_split_terminator_witness_is_trusted_and_carries_chain_derived_provenance,
    SplitTerminator<'static, char>
);
assert_trusted_creusot_witness!(
    str_rsplit_terminator_witness_is_trusted_and_carries_chain_derived_provenance,
    RSplitTerminator<'static, char>
);
assert_trusted_creusot_witness!(
    str_matches_witness_is_trusted_and_carries_chain_derived_provenance,
    Matches<'static, char>
);
assert_trusted_creusot_witness!(
    str_rmatches_witness_is_trusted_and_carries_chain_derived_provenance,
    RMatches<'static, char>
);
assert_trusted_creusot_witness!(
    str_match_indices_witness_is_trusted_and_carries_chain_derived_provenance,
    MatchIndices<'static, char>
);
assert_trusted_creusot_witness!(
    str_rmatch_indices_witness_is_trusted_and_carries_chain_derived_provenance,
    RMatchIndices<'static, char>
);

#[test]
fn chunks_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Chunks<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <Chunks<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn chunks_exact_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ChunksExact<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <ChunksExact<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn chunks_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ChunksMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <ChunksMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn chunks_exact_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ChunksExactMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <ChunksExactMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn windows_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Windows<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <Windows<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn rchunks_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RChunks<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RChunks<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn rchunks_exact_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RChunksExact<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RChunksExact<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn rchunks_exact_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RChunksExactMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RChunksExactMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn rchunks_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RChunksMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RChunksMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn chunk_by_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <ChunkBy<'static, i32, fn(&i32, &i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn chunk_by_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <ChunkByMut<'static, i32, fn(&i32, &i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn rsplit_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::slice::RSplit<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn rsplit_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>> as Witness<CreusotVerifier>>::proof(),
        <RSplitMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn rsplit_n_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::slice::RSplitN<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn rsplit_n_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>> as Witness<CreusotVerifier>>::proof(),
        <RSplitNMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::slice::Split<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_inclusive_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_inclusive_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <SplitInclusiveMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>> as Witness<CreusotVerifier>>::proof(),
        <SplitMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_n_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::slice::SplitN<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn split_n_mut_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>> as Witness<CreusotVerifier>>::proof(),
        <SplitNMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn escape_ascii_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<EscapeAscii<'static>> as Witness<CreusotVerifier>>::proof(),
        <EscapeAscii<'static> as RustStdType>::provenance()
    );
}

#[test]
fn get_disjoint_mut_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<GetDisjointMutError> as Witness<CreusotVerifier>>::proof(),
        <GetDisjointMutError as RustStdType>::provenance()
    );
}

#[test]
fn str_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<str> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_str_byte_length_and_content");
    assert_eq!(proof.provenance, <str as RustStdType>::provenance());
}

#[test]
fn tuple_i32_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<(i32, i32)> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_tuple_field_access");
    assert_eq!(proof.provenance, <(i32, i32) as RustStdType>::provenance());
}

#[test]
fn fn_pointer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<fn(i32) -> i32> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_fn_pointer_calls_the_underlying_function"
    );
    assert_eq!(
        proof.provenance,
        <fn(i32) -> i32 as RustStdType>::provenance()
    );
}

#[test]
fn const_pointer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<*const i32> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_const_pointer_cast_preserves_the_address"
    );
    assert_eq!(proof.provenance, <*const i32 as RustStdType>::provenance());
}

#[test]
fn mut_pointer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<*mut i32> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_mut_pointer_cast_preserves_the_address"
    );
    assert_eq!(proof.provenance, <*mut i32 as RustStdType>::provenance());
}

#[test]
fn assert_unwind_safe_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<AssertUnwindSafe<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_assert_unwind_safe_derefs_transparently"
    );
    assert_eq!(
        proof.provenance,
        <AssertUnwindSafe<i32> as RustStdType>::provenance()
    );
}

#[test]
fn pin_box_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Pin<Box<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Pin<Box<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn non_null_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<NonNull<i32>> as Witness<CreusotVerifier>>::proof(),
        <NonNull<i32> as RustStdType>::provenance()
    );
}

#[test]
fn shared_reference_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<&'static i32> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_shared_reference_dereferences_to_the_referent"
    );
    assert_eq!(
        proof.provenance,
        <&'static i32 as RustStdType>::provenance()
    );
}

#[test]
fn mutable_reference_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<&'static mut i32> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_mutable_reference_dereferences_to_and_updates_the_referent"
    );
    assert_eq!(
        proof.provenance,
        <&'static mut i32 as RustStdType>::provenance()
    );
}

#[test]
fn unit_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<()> as Witness<CreusotVerifier>>::proof(),
        <() as RustStdType>::provenance()
    );
}

#[test]
fn location_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Location<'static>> as Witness<CreusotVerifier>>::proof(),
        <Location<'static> as RustStdType>::provenance()
    );
}

#[test]
fn panic_info_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<PanicInfo<'static>> as Witness<CreusotVerifier>>::proof(),
        <PanicInfo<'static> as RustStdType>::provenance()
    );
}

#[test]
fn panic_message_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<PanicMessage<'static>> as Witness<CreusotVerifier>>::proof(),
        <PanicMessage<'static> as RustStdType>::provenance()
    );
}

#[test]
fn cow_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Cow<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cow_destructure_recovers_the_wrapped_value"
    );
    assert_eq!(
        proof.provenance,
        <Cow<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn args_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Args> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_args_reports_at_least_the_program_path"
    );
    assert_eq!(proof.provenance, <Args as RustStdType>::provenance());
}

#[test]
fn args_os_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<ArgsOs> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_args_os_reports_at_least_the_program_path"
    );
    assert_eq!(proof.provenance, <ArgsOs as RustStdType>::provenance());
}

#[test]
fn join_paths_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<JoinPathsError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_join_paths_error_reports_an_unjoinable_path"
    );
    assert_eq!(
        proof.provenance,
        <JoinPathsError as RustStdType>::provenance()
    );
}

#[test]
fn split_paths_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<SplitPaths<'static>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_paths_recovers_paths_joined_by_join_paths"
    );
    assert_eq!(
        proof.provenance,
        <SplitPaths<'static> as RustStdType>::provenance()
    );
}

#[test]
fn var_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VarError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_var_error_distinguishes_not_present_from_not_unicode"
    );
    assert_eq!(proof.provenance, <VarError as RustStdType>::provenance());
}

#[test]
fn vars_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Vars> as Witness<CreusotVerifier>>::proof(),
        <Vars as RustStdType>::provenance()
    );
}

#[test]
fn vars_os_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VarsOs> as Witness<CreusotVerifier>>::proof(),
        <VarsOs as RustStdType>::provenance()
    );
}

assert_trusted_creusot_witness!(
    incoming_witness_is_trusted_and_carries_chain_derived_provenance,
    Incoming<'static>
);
assert_trusted_creusot_witness!(
    tcp_listener_witness_is_trusted_and_carries_chain_derived_provenance,
    TcpListener
);
assert_trusted_creusot_witness!(
    tcp_stream_witness_is_trusted_and_carries_chain_derived_provenance,
    TcpStream
);
assert_trusted_creusot_witness!(
    udp_socket_witness_is_trusted_and_carries_chain_derived_provenance,
    UdpSocket
);
assert_trusted_creusot_witness!(
    ancestors_witness_is_trusted_and_carries_chain_derived_provenance,
    Ancestors<'static>
);
assert_trusted_creusot_witness!(
    component_witness_is_trusted_and_carries_chain_derived_provenance,
    Component<'static>
);
assert_trusted_creusot_witness!(
    components_witness_is_trusted_and_carries_chain_derived_provenance,
    Components<'static>
);
assert_trusted_creusot_witness!(
    path_display_witness_is_trusted_and_carries_chain_derived_provenance,
    PathDisplay<'static>
);
assert_trusted_creusot_witness!(
    path_iter_witness_is_trusted_and_carries_chain_derived_provenance,
    PathIter<'static>
);
assert_trusted_creusot_witness!(
    path_witness_is_trusted_and_carries_chain_derived_provenance,
    Path
);
assert_trusted_creusot_witness!(
    path_buf_witness_is_trusted_and_carries_chain_derived_provenance,
    PathBuf
);
assert_trusted_creusot_witness!(
    prefix_witness_is_trusted_and_carries_chain_derived_provenance,
    Prefix<'static>
);
assert_trusted_creusot_witness!(
    prefix_component_witness_is_trusted_and_carries_chain_derived_provenance,
    PrefixComponent<'static>
);
assert_trusted_creusot_witness!(
    strip_prefix_error_witness_is_trusted_and_carries_chain_derived_provenance,
    StripPrefixError
);
assert_trusted_creusot_witness!(
    instant_witness_is_trusted_and_carries_chain_derived_provenance,
    Instant
);
assert_trusted_creusot_witness!(
    system_time_witness_is_trusted_and_carries_chain_derived_provenance,
    SystemTime
);
assert_trusted_creusot_witness!(
    system_time_error_witness_is_trusted_and_carries_chain_derived_provenance,
    SystemTimeError
);
assert_trusted_creusot_witness!(
    panic_hook_info_witness_is_trusted_and_carries_chain_derived_provenance,
    PanicHookInfo<'static>
);
assert_trusted_creusot_witness!(
    barrier_witness_is_trusted_and_carries_chain_derived_provenance,
    Barrier
);
assert_trusted_creusot_witness!(
    barrier_wait_result_witness_is_trusted_and_carries_chain_derived_provenance,
    BarrierWaitResult
);
assert_trusted_creusot_witness!(
    lazy_lock_witness_is_trusted_and_carries_chain_derived_provenance,
    LazyLock<i32, fn() -> i32>
);
assert_trusted_creusot_witness!(
    once_witness_is_trusted_and_carries_chain_derived_provenance,
    std::sync::Once
);
assert_trusted_creusot_witness!(
    once_lock_witness_is_trusted_and_carries_chain_derived_provenance,
    OnceLock<i32>
);
assert_trusted_creusot_witness!(
    once_state_witness_is_trusted_and_carries_chain_derived_provenance,
    OnceState
);
assert_trusted_creusot_witness!(
    wait_timeout_result_witness_is_trusted_and_carries_chain_derived_provenance,
    WaitTimeoutResult
);
assert_trusted_creusot_witness!(
    mpsc_iter_witness_is_trusted_and_carries_chain_derived_provenance,
    std::sync::mpsc::Iter<'static, i32>
);
assert_trusted_creusot_witness!(
    mpsc_receiver_witness_is_trusted_and_carries_chain_derived_provenance,
    std::sync::mpsc::Receiver<i32>
);
assert_trusted_creusot_witness!(
    mpsc_sender_witness_is_trusted_and_carries_chain_derived_provenance,
    std::sync::mpsc::Sender<i32>
);
assert_trusted_creusot_witness!(
    mpsc_sync_sender_witness_is_trusted_and_carries_chain_derived_provenance,
    SyncSender<i32>
);
assert_trusted_creusot_witness!(
    mpsc_try_iter_witness_is_trusted_and_carries_chain_derived_provenance,
    std::sync::mpsc::TryIter<'static, i32>
);
assert_trusted_creusot_witness!(
    thread_access_error_witness_is_trusted_and_carries_chain_derived_provenance,
    AccessError
);
assert_trusted_creusot_witness!(
    thread_builder_witness_is_trusted_and_carries_chain_derived_provenance,
    Builder
);
assert_trusted_creusot_witness!(
    thread_join_handle_witness_is_trusted_and_carries_chain_derived_provenance,
    JoinHandle<i32>
);
assert_trusted_creusot_witness!(
    thread_local_key_witness_is_trusted_and_carries_chain_derived_provenance,
    LocalKey<Cell<i32>>
);
assert_trusted_creusot_witness!(
    thread_scope_witness_is_trusted_and_carries_chain_derived_provenance,
    Scope<'static, 'static>
);
assert_trusted_creusot_witness!(
    thread_scoped_join_handle_witness_is_trusted_and_carries_chain_derived_provenance,
    ScopedJoinHandle<'static, i32>
);
assert_trusted_creusot_witness!(
    thread_witness_is_trusted_and_carries_chain_derived_provenance,
    Thread
);
assert_trusted_creusot_witness!(
    thread_id_witness_is_trusted_and_carries_chain_derived_provenance,
    ThreadId
);
assert_trusted_creusot_witness!(
    child_witness_is_trusted_and_carries_chain_derived_provenance,
    Child
);
assert_trusted_creusot_witness!(
    child_stderr_witness_is_trusted_and_carries_chain_derived_provenance,
    ChildStderr
);
assert_trusted_creusot_witness!(
    child_stdin_witness_is_trusted_and_carries_chain_derived_provenance,
    ChildStdin
);
assert_trusted_creusot_witness!(
    child_stdout_witness_is_trusted_and_carries_chain_derived_provenance,
    ChildStdout
);
assert_trusted_creusot_witness!(
    command_witness_is_trusted_and_carries_chain_derived_provenance,
    Command
);
assert_trusted_creusot_witness!(
    command_args_witness_is_trusted_and_carries_chain_derived_provenance,
    CommandArgs<'static>
);
assert_trusted_creusot_witness!(
    command_envs_witness_is_trusted_and_carries_chain_derived_provenance,
    CommandEnvs<'static>
);
assert_trusted_creusot_witness!(
    exit_code_witness_is_trusted_and_carries_chain_derived_provenance,
    ExitCode
);
assert_trusted_creusot_witness!(
    exit_status_witness_is_trusted_and_carries_chain_derived_provenance,
    ExitStatus
);
assert_trusted_creusot_witness!(
    output_witness_is_trusted_and_carries_chain_derived_provenance,
    Output
);
assert_trusted_creusot_witness!(
    stdio_witness_is_trusted_and_carries_chain_derived_provenance,
    Stdio
);
assert_trusted_creusot_witness!(
    dir_builder_witness_is_trusted_and_carries_chain_derived_provenance,
    DirBuilder
);
assert_trusted_creusot_witness!(
    dir_entry_witness_is_trusted_and_carries_chain_derived_provenance,
    DirEntry
);
assert_trusted_creusot_witness!(
    file_witness_is_trusted_and_carries_chain_derived_provenance,
    File
);
assert_trusted_creusot_witness!(
    file_times_witness_is_trusted_and_carries_chain_derived_provenance,
    FileTimes
);
assert_trusted_creusot_witness!(
    file_type_witness_is_trusted_and_carries_chain_derived_provenance,
    FileType
);
assert_trusted_creusot_witness!(
    metadata_witness_is_trusted_and_carries_chain_derived_provenance,
    Metadata
);
assert_trusted_creusot_witness!(
    open_options_witness_is_trusted_and_carries_chain_derived_provenance,
    OpenOptions
);
assert_trusted_creusot_witness!(
    permissions_witness_is_trusted_and_carries_chain_derived_provenance,
    Permissions
);
assert_trusted_creusot_witness!(
    read_dir_witness_is_trusted_and_carries_chain_derived_provenance,
    ReadDir
);
assert_trusted_creusot_witness!(
    fs_try_lock_error_witness_is_trusted_and_carries_chain_derived_provenance,
    std::fs::TryLockError
);

#[test]
fn os_str_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<OsStr> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_os_str_valid_utf8_content_round_trips_through_to_str"
    );
    assert_eq!(proof.provenance, <OsStr as RustStdType>::provenance());
}

#[test]
fn os_string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<OsString> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_os_string_push_appends_to_the_existing_content"
    );
    assert_eq!(proof.provenance, <OsString as RustStdType>::provenance());
}

#[test]
fn hash_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<HashMap<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_hash_map_insert_then_get_recovers_the_value"
    );
    assert_eq!(
        proof.provenance,
        <HashMap<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn hash_set_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<HashSet<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_hash_set_insert_then_contains_reports_membership"
    );
    assert_eq!(
        proof.provenance,
        <HashSet<i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BTreeMap<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_btree_map_iterates_in_key_order");
    assert_eq!(
        proof.provenance,
        <BTreeMap<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_set_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BTreeSet<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_btree_set_iterates_in_sorted_order");
    assert_eq!(
        proof.provenance,
        <BTreeSet<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BinaryHeap<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_pop_yields_the_maximum_first"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeap<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<BinaryHeapDrain<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_drain_yields_every_pushed_element_once"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BinaryHeapIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_into_iter_yields_every_pushed_element_once"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<BinaryHeapIter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_iter_yields_every_pushed_element_once"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapIter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_peek_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_peek_mut_exposes_the_maximum"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapPeekMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<LinkedList<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_is_fifo_through_back_and_front"
    );
    assert_eq!(
        proof.provenance,
        <LinkedList<i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<LinkedListIter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_iter_yields_references_in_order"
    );
    assert_eq!(
        proof.provenance,
        <LinkedListIter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<LinkedListIterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_linked_list_iter_mut_writes_through");
    assert_eq!(
        proof.provenance,
        <LinkedListIterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<LinkedListIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_into_iter_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <LinkedListIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_extract_if_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_extract_if_partitions_by_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn try_reserve_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<TryReserveError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_reserve_rejects_an_impossible_capacity"
    );
    assert_eq!(
        proof.provenance,
        <TryReserveError as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDeque<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_pushes_and_pops_from_both_ends"
    );
    assert_eq!(
        proof.provenance,
        <VecDeque<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_into_iter_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeDrain<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_drain_removes_and_yields_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeIter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_iter_yields_references_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeIter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<VecDequeIterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_vec_deque_iter_mut_writes_through");
    assert_eq!(
        proof.provenance,
        <VecDequeIterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Vec<i32>> as Witness<CreusotVerifier>>::proof(),
        <Vec<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_drain_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecDrain<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <VecDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_into_iter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecIntoIter<i32>> as Witness<CreusotVerifier>>::proof(),
        <VecIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_extract_if_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <VecExtractIf<'static, i32, fn(&mut i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn vec_splice_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecSplice<'static, VecIntoIter<i32>>> as Witness<CreusotVerifier>>::proof(
        ),
        <VecSplice<'static, VecIntoIter<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Cell<i32>> as Witness<CreusotVerifier>>::proof(),
        <Cell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RefCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <RefCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Ref<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <Ref<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_mut_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RefMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RefMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn once_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<OnceCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <OnceCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn unsafe_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<UnsafeCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <UnsafeCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn lazy_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<LazyCell<i32, fn() -> i32>> as Witness<CreusotVerifier>>::proof(),
        <LazyCell<i32, fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn borrow_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<BorrowError> as Witness<CreusotVerifier>>::proof(),
        <BorrowError as RustStdType>::provenance()
    );
}

#[test]
fn borrow_mut_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<BorrowMutError> as Witness<CreusotVerifier>>::proof(),
        <BorrowMutError as RustStdType>::provenance()
    );
}

#[test]
fn char_try_from_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<CharTryFromError> as Witness<CreusotVerifier>>::proof(),
        <CharTryFromError as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <DecodeUtf16<std::array::IntoIter<u16, 1>> as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DecodeUtf16Error> as Witness<CreusotVerifier>>::proof(),
        <DecodeUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_debug_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeDebug> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeDebug as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_default_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeDefault> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_unicode_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeUnicode> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeUnicode as RustStdType>::provenance()
    );
}

#[test]
fn parse_char_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ParseCharError> as Witness<CreusotVerifier>>::proof(),
        <ParseCharError as RustStdType>::provenance()
    );
}

#[test]
fn to_lowercase_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ToLowercase> as Witness<CreusotVerifier>>::proof(),
        <ToLowercase as RustStdType>::provenance()
    );
}

#[test]
fn to_uppercase_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ToUppercase> as Witness<CreusotVerifier>>::proof(),
        <ToUppercase as RustStdType>::provenance()
    );
}

#[test]
fn try_from_char_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TryFromCharError> as Witness<CreusotVerifier>>::proof(),
        <TryFromCharError as RustStdType>::provenance()
    );
}

#[test]
fn type_id_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TypeId> as Witness<CreusotVerifier>>::proof(),
        <TypeId as RustStdType>::provenance()
    );
}

#[test]
fn layout_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Layout> as Witness<CreusotVerifier>>::proof(),
        <Layout as RustStdType>::provenance()
    );
}

#[test]
fn try_from_slice_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TryFromSliceError> as Witness<CreusotVerifier>>::proof(),
        <TryFromSliceError as RustStdType>::provenance()
    );
}

#[test]
fn array_into_iter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ArrayIntoIter<i32, 3>> as Witness<CreusotVerifier>>::proof(),
        <ArrayIntoIter<i32, 3> as RustStdType>::provenance()
    );
}

#[test]
fn ascii_escape_default_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::ascii::EscapeDefault> as Witness<CreusotVerifier>>::proof(),
        <core::ascii::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn rc_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Rc<i32>> as Witness<CreusotVerifier>>::proof(),
        <Rc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn rc_weak_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::rc::Weak<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::rc::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn string_drain_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::string::Drain<'static>> as Witness<CreusotVerifier>>::proof(),
        <std::string::Drain<'static> as RustStdType>::provenance()
    );
}

#[test]
fn from_utf16_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<FromUtf16Error> as Witness<CreusotVerifier>>::proof(),
        <FromUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn from_utf8_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<FromUtf8Error> as Witness<CreusotVerifier>>::proof(),
        <FromUtf8Error as RustStdType>::provenance()
    );
}

#[test]
fn arc_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Arc<i32>> as Witness<CreusotVerifier>>::proof(),
        <Arc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn arc_weak_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::sync::Weak<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::sync::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn infallible_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Infallible> as Witness<CreusotVerifier>>::proof(),
        <Infallible as RustStdType>::provenance()
    );
}

#[test]
fn layout_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<LayoutError> as Witness<CreusotVerifier>>::proof(),
        <LayoutError as RustStdType>::provenance()
    );
}

#[test]
fn addr_parse_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<AddrParseError> as Witness<CreusotVerifier>>::proof(),
        <AddrParseError as RustStdType>::provenance()
    );
}

#[test]
fn ip_addr_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<IpAddr> as Witness<CreusotVerifier>>::proof(),
        <IpAddr as RustStdType>::provenance()
    );
}

#[test]
fn ipv4_addr_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Ipv4Addr> as Witness<CreusotVerifier>>::proof(),
        <Ipv4Addr as RustStdType>::provenance()
    );
}

#[test]
fn ipv6_addr_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Ipv6Addr> as Witness<CreusotVerifier>>::proof(),
        <Ipv6Addr as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SocketAddr> as Witness<CreusotVerifier>>::proof(),
        <SocketAddr as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_v4_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SocketAddrV4> as Witness<CreusotVerifier>>::proof(),
        <SocketAddrV4 as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_v6_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SocketAddrV6> as Witness<CreusotVerifier>>::proof(),
        <SocketAddrV6 as RustStdType>::provenance()
    );
}

#[test]
fn cstring_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<CString> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
    );
    assert_eq!(proof.provenance, <CString as RustStdType>::provenance());
}

#[test]
fn os_str_display_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<OsStrDisplay<'static>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_os_str_display_renders_valid_utf8_content_unchanged"
    );
    assert_eq!(
        proof.provenance,
        <OsStrDisplay<'static> as RustStdType>::provenance()
    );
}

#[test]
fn from_vec_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromVecWithNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_vec_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <FromVecWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn into_string_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<IntoStringError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_into_string_error_recovers_the_original_cstring"
    );
    assert_eq!(
        proof.provenance,
        <IntoStringError as RustStdType>::provenance()
    );
}

#[test]
fn nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<NulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_nul_error_reports_the_interior_nuls_position"
    );
    assert_eq!(proof.provenance, <NulError as RustStdType>::provenance());
}

#[test]
fn cstr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<CStr> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstr_excludes_the_terminating_nul_from_to_bytes"
    );
    assert_eq!(proof.provenance, <CStr as RustStdType>::provenance());
}

#[test]
fn from_bytes_until_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromBytesUntilNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"
    );
    assert_eq!(
        proof.provenance,
        <FromBytesUntilNulError as RustStdType>::provenance()
    );
}

#[test]
fn from_bytes_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromBytesWithNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <FromBytesWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn c_void_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::ffi::c_void> as Witness<CreusotVerifier>>::proof(),
        <core::ffi::c_void as RustStdType>::provenance()
    );
}

#[test]
fn build_hasher_default_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<BuildHasherDefault<DefaultHasher>> as Witness<CreusotVerifier>>::proof(),
        <BuildHasherDefault<DefaultHasher> as RustStdType>::provenance()
    );
}

#[test]
fn default_hasher_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<DefaultHasher> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_default_hasher_is_deterministic_across_fresh_instances"
    );
    assert_eq!(
        proof.provenance,
        <DefaultHasher as RustStdType>::provenance()
    );
}

#[test]
fn random_state_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<RandomState> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_random_state_gives_the_same_hasher_seed_across_calls"
    );
    assert_eq!(proof.provenance, <RandomState as RustStdType>::provenance());
}

#[test]
fn iter_map_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Map<Range<i32>, fn(i32) -> i32>> as Witness<CreusotVerifier>>::proof(),
        <Map<Range<i32>, fn(i32) -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_chain_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <std::iter::Chain<Range<i32>, Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_cloned_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Cloned<Iter<'static, i32>>> as Witness<CreusotVerifier>>::proof(),
        <Cloned<Iter<'static, i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_copied_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Copied<Iter<'static, i32>>> as Witness<CreusotVerifier>>::proof(),
        <Copied<Iter<'static, i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_cycle_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Cycle<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Cycle<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_empty_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Empty<i32>> as Witness<CreusotVerifier>>::proof(),
        <Empty<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_enumerate_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Enumerate<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Enumerate<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_filter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_filter_map_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<
            FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>,
        > as Witness<CreusotVerifier>>::proof(),
        <FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_flat_map_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<
            FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>,
        > as Witness<CreusotVerifier>>::proof(),
        <FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_flatten_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <Flatten<std::vec::IntoIter<Range<i32>>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_zip_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Zip<Range<i32>, Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Zip<Range<i32>, Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_rev_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Rev<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Rev<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_fuse_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Fuse<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Fuse<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_inspect_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Inspect<Range<i32>, fn(&i32)>> as Witness<CreusotVerifier>>::proof(),
        <Inspect<Range<i32>, fn(&i32)> as RustStdType>::provenance()
    );
}

#[test]
fn iter_peekable_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Peekable<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Peekable<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_scan_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_skip_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Skip<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Skip<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_skip_while_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <SkipWhile<Range<i32>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_step_by_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<StepBy<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <StepBy<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_take_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::iter::Take<Range<i32>>> as Witness<CreusotVerifier>>::proof(),
        <std::iter::Take<Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_take_while_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <TakeWhile<Range<i32>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_map_while_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <MapWhile<Range<i32>, fn(i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_once_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::iter::Once<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::iter::Once<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_once_with_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<OnceWith<fn() -> i32>> as Witness<CreusotVerifier>>::proof(),
        <OnceWith<fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::iter::Repeat<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::iter::Repeat<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_with_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RepeatWith<fn() -> i32>> as Witness<CreusotVerifier>>::proof(),
        <RepeatWith<fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_n_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RepeatN<i32>> as Witness<CreusotVerifier>>::proof(),
        <RepeatN<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_successors_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <Successors<i32, fn(&i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn sip_hasher_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<SipHasherAlias> as Witness<CreusotVerifier>>::proof(),
        <SipHasherAlias as RustStdType>::provenance()
    );
}

#[test]
fn phantom_data_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<PhantomData<i32>> as Witness<CreusotVerifier>>::proof(),
        <PhantomData<i32> as RustStdType>::provenance()
    );
}

#[test]
fn phantom_pinned_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<PhantomPinned> as Witness<CreusotVerifier>>::proof(),
        <PhantomPinned as RustStdType>::provenance()
    );
}

#[test]
fn fmt_alignment_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::fmt::Alignment> as Witness<CreusotVerifier>>::proof(),
        <core::fmt::Alignment as RustStdType>::provenance()
    );
}

#[test]
fn fmt_arguments_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Arguments<'static>> as Witness<CreusotVerifier>>::proof(),
        <Arguments<'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::fmt::Error> as Witness<CreusotVerifier>>::proof(),
        <core::fmt::Error as RustStdType>::provenance()
    );
}

#[test]
fn fmt_formatter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Formatter<'static>> as Witness<CreusotVerifier>>::proof(),
        <Formatter<'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_list_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DebugList<'static, 'static>> as Witness<CreusotVerifier>>::proof(),
        <DebugList<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_map_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DebugMap<'static, 'static>> as Witness<CreusotVerifier>>::proof(),
        <DebugMap<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_set_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DebugSet<'static, 'static>> as Witness<CreusotVerifier>>::proof(),
        <DebugSet<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_struct_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DebugStruct<'static, 'static>> as Witness<CreusotVerifier>>::proof(),
        <DebugStruct<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_tuple_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DebugTuple<'static, 'static>> as Witness<CreusotVerifier>>::proof(),
        <DebugTuple<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_from_fn_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result> as RustStdType>::provenance()
    );
}

#[test]
fn box_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Box<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_box_new_preserves_the_wrapped_value");
    assert_eq!(proof.provenance, <Box<i32> as RustStdType>::provenance());
}

#[test]
fn duration_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Duration> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_duration_new_normalizes_nanos_and_carries_into_secs"
    );
    assert_eq!(proof.provenance, <Duration as RustStdType>::provenance());
}

#[test]
fn range_to_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<RangeTo<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_range_to_contains_matches_bound");
    assert_eq!(
        proof.provenance,
        <RangeTo<i32> as RustStdType>::provenance()
    );
}

#[test]
fn range_full_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<RangeFull> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_range_full_contains_everything");
    assert_eq!(proof.provenance, <RangeFull as RustStdType>::provenance());
}

#[test]
fn bound_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Bound<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_bound_round_trips_its_endpoint");
    assert_eq!(proof.provenance, <Bound<i32> as RustStdType>::provenance());
}

#[test]
fn control_flow_i32_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<ControlFlow<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_control_flow_continue_and_break_are_disjoint"
    );
    assert_eq!(
        proof.provenance,
        <ControlFlow<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn nonzero_i16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<NonZero<i16>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_nonzero_i16_roundtrips");
    assert_eq!(
        proof.provenance,
        <NonZero<i16> as RustStdType>::provenance()
    );
}

#[test]
fn ordering_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cmp::Ordering> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordering_reverse_swaps_less_and_greater"
    );
    assert_eq!(
        proof.provenance,
        <std::cmp::Ordering as RustStdType>::provenance()
    );
}

#[test]
fn wrapping_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Wrapping<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_wrapping_i32_add_wraps");
    assert_eq!(
        proof.provenance,
        <Wrapping<i32> as RustStdType>::provenance()
    );
}

#[test]
fn saturating_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Saturating<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_saturating_i32_add_clamps");
    assert_eq!(
        proof.provenance,
        <Saturating<i32> as RustStdType>::provenance()
    );
}

#[test]
fn int_error_kind_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::IntErrorKind> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_int_error_kind_classifies_parse_failures"
    );
    assert_eq!(
        proof.provenance,
        <core::num::IntErrorKind as RustStdType>::provenance()
    );
}

#[test]
fn try_from_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::TryFromIntError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_from_int_error_occurs_exactly_when_out_of_range"
    );
    assert_eq!(
        proof.provenance,
        <core::num::TryFromIntError as RustStdType>::provenance()
    );
}

#[test]
fn parse_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseIntError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_int_error_reports_the_kind_of_the_failure"
    );
    assert_eq!(
        proof.provenance,
        <core::num::ParseIntError as RustStdType>::provenance()
    );
}

#[test]
fn fp_category_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::FpCategory> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_fp_category_matches_the_value_it_classifies"
    );
    assert_eq!(
        proof.provenance,
        <core::num::FpCategory as RustStdType>::provenance()
    );
}

#[test]
fn parse_float_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseFloatError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_float_error_occurs_only_for_unparseable_input"
    );
    assert_eq!(
        proof.provenance,
        <core::num::ParseFloatError as RustStdType>::provenance()
    );
}

#[test]
fn reverse_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Reverse<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_reverse_inverts_comparison");
    assert_eq!(
        proof.provenance,
        <Reverse<i32> as RustStdType>::provenance()
    );
}

#[test]
fn option_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Option<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_option_some_and_none_are_disjoint");
    assert_eq!(proof.provenance, <Option<i32> as RustStdType>::provenance());
}

#[test]
fn result_i32_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Result<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_result_ok_and_err_are_disjoint");
    assert_eq!(
        proof.provenance,
        <Result<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn option_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::option::Iter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_option_iter_yields_zero_or_one_reference"
    );
    assert_eq!(
        proof.provenance,
        <core::option::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn option_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::option::IterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_option_iter_mut_writes_through_to_the_option"
    );
    assert_eq!(
        proof.provenance,
        <core::option::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn result_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::result::Iter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_result_iter_yields_a_reference_to_the_ok_value"
    );
    assert_eq!(
        proof.provenance,
        <core::result::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn result_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::result::IterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_result_iter_mut_writes_through_to_the_result"
    );
    assert_eq!(
        proof.provenance,
        <core::result::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn pending_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Pending<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_pending_never_resolves");
    assert_eq!(
        proof.provenance,
        <Pending<i32> as RustStdType>::provenance()
    );
}

#[test]
fn poll_fn_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> as Witness<
        CreusotVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_poll_fn_dispatches_through_to_its_closure"
    );
    assert_eq!(
        proof.provenance,
        <PollFn<fn(&mut Context<'_>) -> Poll<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn ready_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Ready<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ready_resolves_immediately_with_its_value"
    );
    assert_eq!(proof.provenance, <Ready<i32> as RustStdType>::provenance());
}

#[test]
fn context_static_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Context<'static>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_context_from_waker_exposes_the_same_waker"
    );
    assert_eq!(
        proof.provenance,
        <Context<'static> as RustStdType>::provenance()
    );
}

#[test]
fn poll_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Poll<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_poll_ready_and_pending_are_disjoint");
    assert_eq!(proof.provenance, <Poll<i32> as RustStdType>::provenance());
}

assert_trusted_creusot_witness!(
    raw_waker_witness_is_trusted_and_carries_chain_derived_provenance,
    RawWaker
);
assert_trusted_creusot_witness!(
    raw_waker_vtable_witness_is_trusted_and_carries_chain_derived_provenance,
    RawWakerVTable
);

#[test]
fn waker_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Waker> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_waker_wake_by_ref_invokes_the_wake_impl"
    );
    assert_eq!(proof.provenance, <Waker as RustStdType>::provenance());
}

#[test]
fn manually_drop_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<ManuallyDrop<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_manually_drop_derefs_and_into_inner_round_trip"
    );
    assert_eq!(
        proof.provenance,
        <ManuallyDrop<i32> as RustStdType>::provenance()
    );
}

#[test]
fn discriminant_option_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Discriminant<Option<i32>>> as Witness<CreusotVerifier>>::proof(),
        <Discriminant<Option<i32>> as RustStdType>::provenance()
    );
}
