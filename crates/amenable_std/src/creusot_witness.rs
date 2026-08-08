//! `CreusotWitness` impls for Rust standard-library carriers.
//!
//! This lives here, not alongside the proof functions in `amenable_creusot`,
//! because `creusot-rustc`'s translator sweeps every local item in a
//! `creusot-std`-dependent crate — including ones no `#[cfg(creusot)]` gate
//! protects, since Rust items don't need to *run* to be enumerated. Ordinary
//! infrastructure that's completely unremarkable to plain `rustc` turned out
//! to be unsupported there: a return-position `impl Trait` on a local `impl`
//! panicked its intrinsics-gathering pass outright (a real ICE, not a
//! hypothetical), and the `static` item `::inventory::submit!` expands to
//! hit "unsupported definition kind ... Static" the same way. So
//! `amenable_creusot` stays pure Pearlite proof-function content — the thing
//! `cargo creusot -- -p amenable_creusot` actually needs to translate — and
//! everything else about *finding* those proofs (the witness bridge, the
//! registry) moves to the crate that already owns the types being proved
//! about.
//!
//! This is legal under Rust's orphan rule for a reason distinct from every
//! other verifier backend's bridge (see `amenable_creusot::witness`'s own
//! doc comment for the usual justification: the verifier marker type is
//! local): here, `RustStdStandard<T>` — the `Self` type — is local to this
//! crate instead. The orphan rule only requires *one* of {trait, Self type,
//! trait's own type parameters} to be local to the implementing crate, not
//! any particular one, and a real 3-crate test confirmed a Self-type-local
//! justification compiles exactly the same as a trait-parameter-local one.
//!
//! One block per concrete type: a Creusot-checkable property doesn't
//! generalize across types the way provenance does, so there is no blanket
//! impl here — each type gets exactly the contract that's actually true of
//! it. The bridge to `Witness<CreusotVerifier>` is mechanical (delegates
//! straight to `CreusotWitness`), so it's generated per type by a macro
//! rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Creusot to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! contract content to add. `char` and `String` do carry a genuine
//! constraint, so their `proof()` also names the Creusot contract function
//! that checks it, alongside the same chain-derived provenance.
//!
//! Each type also registers a [`amenable_core::ProofRecord`] alongside its
//! `Witness` bridge, so `proof()`'s output is discoverable by name for
//! audit — see `amenable_core::chain::proof_chain`. The registered
//! `evidence` name is a hardcoded module-path literal matching
//! `RustStdStandard`'s own registration in `rust_std`, so both sides agree
//! on the same string without one computing it from the other.
//!
//! A "checked" carrier's [`CheckedProof::claim`] is the contract's own
//! verbatim source (`#[requires]`/`#[ensures]` included), captured via
//! [`amenable_derive::harness!`] *in `amenable_creusot`* — this crate only
//! imports the resulting `&'static str` constant, never the harness
//! function itself, so the claim can never drift from the real contract
//! without also touching the crate that's actually translated.

use std::alloc::{Layout, LayoutError, System};
use std::any::TypeId;
use std::array::{IntoIter, TryFromSliceError};
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
use std::fmt::{
    Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter, FromFn,
};
use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
};
use std::future::{Pending, PollFn, Ready};
use std::hash::{BuildHasherDefault, RandomState};
use std::io::{
    BufReader, BufWriter, Cursor, IntoInnerError, IoSlice, IoSliceMut, LineWriter, PipeReader,
    PipeWriter, SeekFrom, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, WriterPanicked,
};
use std::iter::{
    Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect, Map,
    MapWhile, OnceWith, Peekable, RepeatN, RepeatWith, Rev, Scan, Skip, SkipWhile, StepBy,
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
use std::path::{
    Ancestors, Component, Components, Path, PathBuf, Prefix, PrefixComponent, StripPrefixError,
};
use std::pin::Pin;
use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, CommandArgs, CommandEnvs, ExitCode,
    ExitStatus, Output, Stdio,
};
use std::ptr::NonNull;
use std::rc::Rc;
use std::slice::{
    ChunkBy, ChunkByMut, Chunks, ChunksExact, ChunksExactMut, ChunksMut, EscapeAscii,
    GetDisjointMutError, Iter, RChunks, RChunksExact, RChunksExactMut, RChunksMut, RSplitMut,
    RSplitNMut, SplitInclusiveMut, SplitMut, SplitNMut, Windows,
};
use std::str::{
    CharIndices, Chars, EncodeUtf16, MatchIndices, Matches, ParseBoolError, RMatchIndices,
    RMatches, RSplitTerminator, SplitAsciiWhitespace, SplitTerminator, SplitWhitespace, Utf8Chunk,
    Utf8Chunks, Utf8Error,
};
use std::string::{FromUtf8Error, FromUtf16Error};
use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering as AtomicOrdering,
};
use std::sync::mpsc::SyncSender;
use std::sync::{
    Arc, Barrier, BarrierWaitResult, LazyLock, OnceLock, OnceState, WaitTimeoutResult,
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};
use std::time::{Duration, Instant, SystemTime, SystemTimeError, TryFromFloatSecsError};
use std::vec::Vec;

use core::panic::{Location, PanicInfo, PanicMessage};

use amenable_core::{Ensures, Evidence, Provenance, Requires, Witness};
use amenable_creusot::{
    A_LESS_THAN_B_HOLDS_SRC, ARGV_EXTRA_HEADROOM_HOLDS_SRC, ARGV_INCLUDES_PROGRAM_PATH_SRC,
    ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC,
    BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC,
    BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC, BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC,
    BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC, CHAR_ROUNDTRIPS_SRC,
    COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, CreusotVerifier, CreusotWitness,
    DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC, FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC,
    HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
    HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC, INDEXING_AND_LENGTH_HOLDS_SRC,
    ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC, K1_LESS_THAN_K2_HOLDS_SRC,
    MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, NON_NUL_BYTE_HOLDS_SRC,
    NUL_ONLY_AT_THE_END_VALIDATES_SRC, OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
    SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
    SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC,
    SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
    SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC,
    STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC, TUPLE_FIELD_ACCESS_HOLDS_SRC,
    VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC,
    VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
    VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC,
    VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC,
    VERIFY_ATOMIC_I8_LOAD_STORE_SRC, VERIFY_ATOMIC_I16_LOAD_STORE_SRC,
    VERIFY_ATOMIC_I32_LOAD_STORE_SRC, VERIFY_ATOMIC_I64_LOAD_STORE_SRC,
    VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC, VERIFY_ATOMIC_PTR_LOAD_STORE_SRC,
    VERIFY_ATOMIC_U8_LOAD_STORE_SRC, VERIFY_ATOMIC_U16_LOAD_STORE_SRC,
    VERIFY_ATOMIC_U32_LOAD_STORE_SRC, VERIFY_ATOMIC_U64_LOAD_STORE_SRC,
    VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC,
    VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC,
    VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC,
    VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC,
    VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC, VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC,
    VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
    VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, VERIFY_CHAR_ROUNDTRIP_SRC,
    VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC,
    VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC,
    VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC,
    VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
    VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
    VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC,
    VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
    VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_SRC,
    VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
    VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC,
    VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
    VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
    VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
    VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
    VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC,
    VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC,
    VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC,
    VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
    VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC,
    VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
    VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC,
    VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC,
    VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC,
    VERIFY_NONZERO_I16_ROUNDTRIPS_SRC, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC,
    VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC,
    VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
    VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
    VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
    VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC,
    VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
    VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC,
    VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
    VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, VERIFY_PENDING_NEVER_RESOLVES_SRC,
    VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC,
    VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC,
    VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC,
    VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC, VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC,
    VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC,
    VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC,
    VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC,
    VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC,
    VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
    VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
    VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
    VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC,
    VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC, VERIFY_SLICE_INDEXING_AND_LENGTH_SRC,
    VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
    VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC,
    VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC,
    VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC, VERIFY_STRING_ROUNDTRIP_SRC,
    VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC,
    VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
    VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC, VERIFY_TUPLE_FIELD_ACCESS_SRC,
    VERIFY_VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC,
    VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC,
    VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
    VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC,
    VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC,
    VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
    YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC,
};

use crate::{
    ArgvIncludesProgramPath, AsciiByte, DrainsTwoValuesInOrderAndEmpties, IndexingAndLength,
    IterYieldsValueOnceThenEnds, NonNulByte, NulOnlyAtTheEndValidates, RustLanguageProvenance,
    RustStdProvenance, RustStdStandard, ValidUnicodeScalar, YieldsTwoValuesInOrderThenEnds,
};

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

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}

macro_rules! impl_creusot_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreusotWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_creusot_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "creusot",
                    describe: || <RustStdStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_creusot_witness_trusted!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    (),
    Cell<i32>,
    RefCell<i32>,
    Ref<'static, i32>,
    RefMut<'static, i32>,
    OnceCell<i32>,
    UnsafeCell<i32>,
    LazyCell<i32, fn() -> i32>,
    BorrowError,
    BorrowMutError,
    Location<'static>,
    PanicInfo<'static>,
    PanicMessage<'static>,
    PanicHookInfo<'static>,
    Pin<Box<i32>>,
    NonNull<i32>,
    Chunks<'static, i32>,
    ChunksExact<'static, i32>,
    ChunksMut<'static, i32>,
    ChunksExactMut<'static, i32>,
    RChunks<'static, i32>,
    RChunksExact<'static, i32>,
    RChunksExactMut<'static, i32>,
    RChunksMut<'static, i32>,
    Windows<'static, i32>,
    ChunkBy<'static, i32, fn(&i32, &i32) -> bool>,
    ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>,
    std::slice::RSplit<'static, i32, fn(&i32) -> bool>,
    RSplitMut<'static, i32, fn(&i32) -> bool>,
    std::slice::RSplitN<'static, i32, fn(&i32) -> bool>,
    RSplitNMut<'static, i32, fn(&i32) -> bool>,
    std::slice::Split<'static, i32, fn(&i32) -> bool>,
    std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>,
    SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
    SplitMut<'static, i32, fn(&i32) -> bool>,
    std::slice::SplitN<'static, i32, fn(&i32) -> bool>,
    SplitNMut<'static, i32, fn(&i32) -> bool>,
    EscapeAscii<'static>,
    GetDisjointMutError,
    std::str::Bytes<'static>,
    CharIndices<'static>,
    Chars<'static>,
    EncodeUtf16<'static>,
    std::str::EscapeDebug<'static>,
    std::str::EscapeDefault<'static>,
    std::str::EscapeUnicode<'static>,
    std::str::Lines<'static>,
    SplitAsciiWhitespace<'static>,
    SplitWhitespace<'static>,
    Utf8Chunk<'static>,
    Utf8Chunks<'static>,
    ParseBoolError,
    Utf8Error,
    std::str::Split<'static, char>,
    std::str::RSplit<'static, char>,
    std::str::SplitN<'static, char>,
    std::str::RSplitN<'static, char>,
    std::str::SplitInclusive<'static, char>,
    SplitTerminator<'static, char>,
    RSplitTerminator<'static, char>,
    Matches<'static, char>,
    RMatches<'static, char>,
    MatchIndices<'static, char>,
    RMatchIndices<'static, char>,
    CharTryFromError,
    DecodeUtf16<std::array::IntoIter<u16, 1>>,
    DecodeUtf16Error,
    core::char::EscapeDebug,
    core::char::EscapeDefault,
    core::char::EscapeUnicode,
    ParseCharError,
    ToLowercase,
    ToUppercase,
    TryFromCharError,
    TypeId,
    TryFromFloatSecsError,
    Infallible,
    Layout,
    LayoutError,
    TryFromSliceError,
    IntoIter<i32, 3>,
    core::ascii::EscapeDefault,
    core::ffi::c_void,
    BuildHasherDefault<DefaultHasher>,
    Map<Range<i32>, fn(i32) -> i32>,
    std::iter::Chain<Range<i32>, Range<i32>>,
    Zip<Range<i32>, Range<i32>>,
    Cloned<Iter<'static, i32>>,
    Copied<Iter<'static, i32>>,
    Cycle<Range<i32>>,
    std::iter::Empty<i32>,
    Enumerate<Range<i32>>,
    Rev<Range<i32>>,
    Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>,
    FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>,
    FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>,
    Fuse<Range<i32>>,
    Inspect<Range<i32>, fn(&i32)>,
    Peekable<Range<i32>>,
    Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>,
    Skip<Range<i32>>,
    SkipWhile<Range<i32>, fn(&i32) -> bool>,
    StepBy<Range<i32>>,
    std::iter::Take<Range<i32>>,
    TakeWhile<Range<i32>, fn(&i32) -> bool>,
    MapWhile<Range<i32>, fn(i32) -> Option<i32>>,
    std::iter::Once<i32>,
    OnceWith<fn() -> i32>,
    std::iter::Repeat<i32>,
    RepeatWith<fn() -> i32>,
    RepeatN<i32>,
    BufReader<&'static [u8]>,
    BufWriter<Vec<u8>>,
    std::io::Bytes<&'static [u8]>,
    std::io::Chain<&'static [u8], &'static [u8]>,
    Cursor<&'static [u8]>,
    std::io::Empty,
    std::io::Error,
    IntoInnerError<BufWriter<Vec<u8>>>,
    IoSlice<'static>,
    IoSliceMut<'static>,
    LineWriter<Vec<u8>>,
    std::io::Lines<&'static [u8]>,
    PipeReader,
    PipeWriter,
    std::io::Repeat,
    std::io::Sink,
    std::io::Split<&'static [u8]>,
    Stderr,
    StderrLock<'static>,
    Stdin,
    StdinLock<'static>,
    Stdout,
    StdoutLock<'static>,
    std::io::Take<&'static [u8]>,
    WriterPanicked,
    Successors<i32, fn(&i32) -> Option<i32>>,
    PhantomData<i32>,
    PhantomPinned,
    std::fmt::Alignment,
    Arguments<'static>,
    std::fmt::Error,
    Formatter<'static>,
    DebugList<'static, 'static>,
    DebugMap<'static, 'static>,
    DebugSet<'static, 'static>,
    DebugStruct<'static, 'static>,
    DebugTuple<'static, 'static>,
    FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>,
    Discriminant<Option<i32>>,
    AddrParseError,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    Incoming<'static>,
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    TcpListener,
    TcpStream,
    UdpSocket,
    Ancestors<'static>,
    Component<'static>,
    Components<'static>,
    std::path::Display<'static>,
    std::path::Iter<'static>,
    Path,
    PathBuf,
    Prefix<'static>,
    PrefixComponent<'static>,
    StripPrefixError,
    Child,
    ChildStderr,
    ChildStdin,
    ChildStdout,
    Command,
    CommandArgs<'static>,
    CommandEnvs<'static>,
    ExitCode,
    ExitStatus,
    Output,
    Stdio,
    Instant,
    SystemTime,
    SystemTimeError,
    DirBuilder,
    DirEntry,
    File,
    FileTimes,
    FileType,
    Metadata,
    OpenOptions,
    Permissions,
    ReadDir,
    std::fs::TryLockError,
    Rc<i32>,
    std::rc::Weak<i32>,
    std::string::Drain<'static>,
    FromUtf16Error,
    FromUtf8Error,
    Arc<i32>,
    std::sync::Weak<i32>,
    Barrier,
    BarrierWaitResult,
    LazyLock<i32, fn() -> i32>,
    std::sync::Once,
    OnceLock<i32>,
    OnceState,
    WaitTimeoutResult,
    std::sync::mpsc::Iter<'static, i32>,
    std::sync::mpsc::Receiver<i32>,
    std::sync::mpsc::Sender<i32>,
    SyncSender<i32>,
    std::sync::mpsc::TryIter<'static, i32>,
    AccessError,
    Builder,
    JoinHandle<i32>,
    LocalKey<std::cell::Cell<i32>>,
    Scope<'static, 'static>,
    ScopedJoinHandle<'static, i32>,
    Thread,
    ThreadId,
    Vec<i32>,
    std::vec::Drain<'static, i32>,
    std::vec::IntoIter<i32>,
    std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    std::vec::Splice<'static, std::vec::IntoIter<i32>>
);

impl CreusotWitness for RustStdStandard<LinesAnyStatic> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<LinesAnyStatic>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LinesAny<'static>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinesAnyStatic> as CreusotWitness>::proof().report().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<SipHasherAlias> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<SipHasherAlias>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SipHasher>",
        verifier: "creusot",
        describe: || <RustStdStandard<SipHasherAlias> as CreusotWitness>::proof().report().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Flatten<IntoIter<Range<i32>>>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>> as CreusotWitness>::proof()
                .report()
                .to_string()
        },
    }
}

/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    pub claim: &'static str,
    /// The chain-derived provenance this claim still rests on.
    pub provenance: RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

impl CreusotWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        describe: || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::CHAR_ROUNDTRIPS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// char_roundtrips` the real site calls, not a hand-retyped copy of
/// its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<char> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        CHAR_ROUNDTRIPS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<char> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression CHAR_ROUNDTRIPS_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "char_roundtrips (c , result)",
        harnesses: &["verify_char_roundtrip"],
    }
}

/// The [`ValidUnicodeScalar`] contract type reuses `verify_char_roundtrip`
/// rather than adding a new Creusot proof — it names the postcondition the
/// harness already checks (`c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <=
/// 0x10FFFF)`), it doesn't prove anything new.
impl CreusotWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(ValidUnicodeScalar);

impl Ensures<CreusotVerifier> for ValidUnicodeScalar {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF)"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ValidUnicodeScalar",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <ValidUnicodeScalar as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &["verify_char_roundtrip"],
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::ValidUnicodeScalar",
        verifier: "creusot",
        describe: || <ValidUnicodeScalar as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_roundtrip",
            claim: VERIFY_STRING_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        describe: || <RustStdStandard<String> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn string_roundtrips_and_preserves_length` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<String> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<String> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "string_roundtrips_and_preserves_length (s , result)",
        harnesses: &["verify_string_roundtrip"],
    }
}

macro_rules! impl_creusot_atomic_checked_witness {
    ($ty:ty, $harness:literal, $claim:ident) => {
        impl CreusotWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = CheckedProof;

            fn proof() -> Self::ProofArtifact {
                CheckedProof {
                    harness: $harness,
                    claim: $claim,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_creusot_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "creusot",
                describe: || <RustStdStandard<$ty> as CreusotWitness>::proof().to_string(),
            }
        }
    };
}

impl_creusot_atomic_checked_witness!(
    AtomicBool,
    "verify_atomic_bool_load_store",
    VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI8,
    "verify_atomic_i8_load_store",
    VERIFY_ATOMIC_I8_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI16,
    "verify_atomic_i16_load_store",
    VERIFY_ATOMIC_I16_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI32,
    "verify_atomic_i32_load_store",
    VERIFY_ATOMIC_I32_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI64,
    "verify_atomic_i64_load_store",
    VERIFY_ATOMIC_I64_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicIsize,
    "verify_atomic_isize_load_store",
    VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU8,
    "verify_atomic_u8_load_store",
    VERIFY_ATOMIC_U8_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU16,
    "verify_atomic_u16_load_store",
    VERIFY_ATOMIC_U16_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU32,
    "verify_atomic_u32_load_store",
    VERIFY_ATOMIC_U32_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU64,
    "verify_atomic_u64_load_store",
    VERIFY_ATOMIC_U64_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicUsize,
    "verify_atomic_usize_load_store",
    VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC
);
// Bare `Ordering`, matching `amenable_std::rust_std::sync_atomic`'s own
// registration exactly: unlike `std::cmp::Ordering`, this evidence string
// intentionally stays unqualified so the checklist row resolves to the
// atomic carrier rather than the comparison carrier.
impl CreusotWitness for RustStdStandard<AtomicOrdering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_relaxed_ordering_still_makes_a_store_observable",
            claim: VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<AtomicOrdering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ordering>",
        verifier: "creusot",
        describe: || <RustStdStandard<AtomicOrdering> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<System> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_system_allocates_and_deallocates_a_layout",
            claim: VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<System>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<System>",
        verifier: "creusot",
        describe: || <RustStdStandard<System> as CreusotWitness>::proof().to_string(),
    }
}

/// A `Box` allocation serviced by the default `System` allocator
/// round-trips its value — the same claim Kani's own
/// `verify_system_allocates_and_deallocates_a_layout` harness checks via
/// `assert_eq!` (out of the contract-bound scanner's reach, since
/// `assert_eq!`'s comparands aren't parsed as a clause), named here once
/// for the Creusot side that does state it as an explicit `#[ensures]`.
impl Ensures<CreusotVerifier> for RustStdStandard<System> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result == value"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<System>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<System> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &["verify_system_allocates_and_deallocates_a_layout"],
    }
}

impl CreusotWitness for RustStdStandard<Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_backtrace_force_capture_always_actually_captures",
            claim: VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Backtrace>",
        verifier: "creusot",
        describe: || <RustStdStandard<Backtrace> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_backtrace_status_reports_captured_after_force_capture",
            claim: VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        verifier: "creusot",
        describe: || <RustStdStandard<BacktraceStatus> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_seek_from_round_trips_each_variants_offset",
            claim: VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        verifier: "creusot",
        describe: || <RustStdStandard<SeekFrom> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn seek_from_round_trips_each_variants_offset` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<SeekFrom> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<SeekFrom> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "seek_from_round_trips_each_variants_offset (start_offset , end_offset , current_offset , result)",
        harnesses: &["verify_seek_from_round_trips_each_variants_offset"],
    }
}

impl CreusotWitness for RustStdStandard<Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_shutdown_write_prevents_further_writes",
            claim: VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Shutdown>",
        verifier: "creusot",
        describe: || <RustStdStandard<Shutdown> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_default_hasher_is_deterministic_across_fresh_instances",
            claim: VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        verifier: "creusot",
        describe: || <RustStdStandard<DefaultHasher> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_random_state_gives_the_same_hasher_seed_across_calls",
            claim: VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RandomState>",
        verifier: "creusot",
        describe: || <RustStdStandard<RandomState> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_hash_map_insert_then_get_recovers_the_value",
            claim: VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<HashMap<i32, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn hash_map_insert_then_get_recovers_the_value` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<HashMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<HashMap<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "hash_map_insert_then_get_recovers_the_value (value , result)",
        harnesses: &["verify_hash_map_insert_then_get_recovers_the_value"],
    }
}

impl CreusotWitness for RustStdStandard<HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_hash_set_insert_then_contains_reports_membership",
            claim: VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<HashSet<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn hash_set_insert_then_contains_reports_membership`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<HashSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<HashSet<i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "hash_set_insert_then_contains_reports_membership (result)",
        harnesses: &["verify_hash_set_insert_then_contains_reports_membership"],
    }
}

impl_creusot_atomic_checked_witness!(
    AtomicPtr<i32>,
    "verify_atomic_ptr_load_store",
    VERIFY_ATOMIC_PTR_LOAD_STORE_SRC
);

impl CreusotWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_array_indexing_and_length",
            claim: VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        verifier: "creusot",
        describe: || <RustStdStandard<[i32; 3]> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_slice_indexing_and_length",
            claim: VERIFY_SLICE_INDEXING_AND_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32]>",
        verifier: "creusot",
        describe: || <RustStdStandard<[i32]> as CreusotWitness>::proof().to_string(),
    }
}

/// [`IndexingAndLength`] reuses the array-indexing harness rather than
/// adding a new Creusot proof — it names the postcondition both the
/// array and slice indexing/length proofs already share, it doesn't
/// prove anything new.
impl CreusotWitness for IndexingAndLength {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_array_indexing_and_length",
            claim: VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(IndexingAndLength);

/// Returns `amenable_creusot::INDEXING_AND_LENGTH_HOLDS_SRC` directly --
/// the verbatim, `harness!`-captured source of the real `#[logic(open)]
/// fn indexing_and_length_holds` both `verify_array_indexing_and_length`
/// and `verify_slice_indexing_and_length` call, not a hand-retyped copy
/// of its expression. There is exactly one place this postcondition's
/// text exists in the whole codebase.
impl Ensures<CreusotVerifier> for IndexingAndLength {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        INDEXING_AND_LENGTH_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IndexingAndLength",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <IndexingAndLength as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape both real sites now use, instead of the raw
// expression INDEXING_AND_LENGTH_HOLDS_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IndexingAndLength",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "indexing_and_length_holds (a , b , c , result)",
        harnesses: &["verify_array_indexing_and_length", "verify_slice_indexing_and_length"],
    }
}

impl CreusotWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_slice_iter_yields_shared_references_in_order",
            claim: VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<std::slice::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    }
}

/// Returns `amenable_creusot::SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn slice_iter_yields_shared_references_in_order` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<std::slice::Iter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<std::slice::Iter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "slice_iter_yields_shared_references_in_order (a , b , c , result)",
        harnesses: &["verify_slice_iter_yields_shared_references_in_order"],
    }
}

impl CreusotWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_slice_iter_mut_yields_mutable_references_that_write_through",
            claim: VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<std::slice::IterMut<'static, i32>> as CreusotWitness>::proof()
                .to_string()
        },
    }
}

/// Returns
/// `amenable_creusot::SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn slice_iter_mut_yields_mutable_references_that_
/// write_through` the real site calls, not a hand-retyped copy of its
/// expression.
impl Ensures<CreusotVerifier> for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<std::slice::IterMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC
// itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "slice_iter_mut_yields_mutable_references_that_write_through (a , b , updated_a , updated_b , result)",
        harnesses: &["verify_slice_iter_mut_yields_mutable_references_that_write_through"],
    }
}

impl CreusotWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_str_byte_length_and_content",
            claim: VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<str>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "creusot",
        describe: || <RustStdStandard<str> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn str_byte_length_and_content_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<str> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<str> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "str_byte_length_and_content_holds (byte , result)",
        harnesses: &["verify_str_byte_length_and_content"],
    }
}

/// [`AsciiByte`] reuses the same harness rather than adding a new Creusot
/// proof — it names the precondition the harness already requires, it
/// doesn't prove anything new.
impl CreusotWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_str_byte_length_and_content",
            claim: VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(AsciiByte);

impl Requires<CreusotVerifier> for AsciiByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "byte < 128u8"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "creusot",
        kind: "requires",
        fragment: || <AsciiByte as Requires<CreusotVerifier>>::requires(()),
        harnesses: &["verify_str_byte_length_and_content"],
    }
}

impl CreusotWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_tuple_field_access",
            claim: VERIFY_TUPLE_FIELD_ACCESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        verifier: "creusot",
        describe: || <RustStdStandard<(i32, i32)> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::TUPLE_FIELD_ACCESS_HOLDS_SRC` directly --
/// the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn tuple_field_access_holds` the real site calls,
/// not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<(i32, i32)> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        TUPLE_FIELD_ACCESS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<(i32, i32)> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression TUPLE_FIELD_ACCESS_HOLDS_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "tuple_field_access_holds (a , b , result)",
        harnesses: &["verify_tuple_field_access"],
    }
}

impl CreusotWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_fn_pointer_calls_the_underlying_function",
            claim: VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        verifier: "creusot",
        describe: || <RustStdStandard<fn(i32) -> i32> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn fn_pointer_calls_the_underlying_function` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<fn(i32) -> i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<fn(i32) -> i32> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "fn_pointer_calls_the_underlying_function (value , result)",
        harnesses: &["verify_fn_pointer_calls_the_underlying_function"],
    }
}

impl CreusotWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_const_pointer_cast_preserves_the_address",
            claim: VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*const i32>",
        verifier: "creusot",
        describe: || <RustStdStandard<*const i32> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mut_pointer_cast_preserves_the_address",
            claim: VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*mut i32>",
        verifier: "creusot",
        describe: || <RustStdStandard<*mut i32> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_assert_unwind_safe_derefs_transparently",
            claim: VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<AssertUnwindSafe<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn assert_unwind_safe_derefs_transparently` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<AssertUnwindSafe<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<AssertUnwindSafe<i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "assert_unwind_safe_derefs_transparently (value , updated , result)",
        harnesses: &["verify_assert_unwind_safe_derefs_transparently"],
    }
}

impl CreusotWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_shared_reference_dereferences_to_the_referent",
            claim: VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static i32>",
        verifier: "creusot",
        describe: || <RustStdStandard<&'static i32> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn shared_reference_dereferences_to_the_referent`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<&'static i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<&'static i32> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "shared_reference_dereferences_to_the_referent (value , result)",
        harnesses: &["verify_shared_reference_dereferences_to_the_referent"],
    }
}

impl CreusotWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mutable_reference_dereferences_to_and_updates_the_referent",
            claim: VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        verifier: "creusot",
        describe: || <RustStdStandard<&'static mut i32> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// mutable_reference_dereferences_to_and_updates_the_referent` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<&'static mut i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<&'static mut i32> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression
// MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "mutable_reference_dereferences_to_and_updates_the_referent (initial , next , result)",
        harnesses: &["verify_mutable_reference_dereferences_to_and_updates_the_referent"],
    }
}

// Bare `Cow<'static, i32>`, matching `amenable_std::rust_std::
// alloc_borrow`'s own registration exactly (confirmed against the
// checklist's own `evidence_name` column:
// `RustStdStandard<Cow<'static, i32>>`).
impl CreusotWitness for RustStdStandard<Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cow_destructure_recovers_the_wrapped_value",
            claim: VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Cow<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn cow_destructure_recovers_the_wrapped_value` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Cow<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<Cow<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "cow_destructure_recovers_the_wrapped_value (value , result)",
        harnesses: &["verify_cow_destructure_recovers_the_wrapped_value"],
    }
}

impl CreusotWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_map_iterates_in_key_order",
            claim: VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BTreeMap<i32, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::K1_LESS_THAN_K2_HOLDS_SRC` /
/// `BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC` directly -- the verbatim,
/// `harness!`-captured source of the real `#[logic(open)]` fns the real
/// site calls, not a hand-retyped copy of their expressions.
impl Requires<CreusotVerifier> for RustStdStandard<BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        K1_LESS_THAN_K2_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        kind: "requires",
        fragment: || <RustStdStandard<BTreeMap<i32, i32>> as Requires<CreusotVerifier>>::requires(
            (),
        ),
        harnesses: &[],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BTreeMap<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shapes the real site now uses, instead of the raw
// expressions K1_LESS_THAN_K2_HOLDS_SRC/
// BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC themselves capture.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        kind: "requires",
        fragment: || "k1_less_than_k2_holds (k1 , k2)",
        harnesses: &["verify_btree_map_iterates_in_key_order"],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "btree_map_iterates_in_key_order_holds (k1 , k2 , v1 , v2 , result)",
        harnesses: &["verify_btree_map_iterates_in_key_order"],
    }
}

impl CreusotWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_set_iterates_in_sorted_order",
            claim: VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BTreeSet<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::A_LESS_THAN_B_HOLDS_SRC` /
/// `BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)]`
/// fns the real site calls, not a hand-retyped copy of their
/// expressions. `A_LESS_THAN_B_HOLDS_SRC` is shared with
/// `RustStdStandard<BinaryHeapPeekMut<'static, i32>>`'s own `Requires`
/// impl below -- the identical precondition, named once.
impl Requires<CreusotVerifier> for RustStdStandard<BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        A_LESS_THAN_B_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        kind: "requires",
        fragment: || <RustStdStandard<BTreeSet<i32>> as Requires<CreusotVerifier>>::requires(()),
        harnesses: &[],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BTreeSet<i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shapes both real sites (BTreeSet and BinaryHeapPeekMut)
// now use, instead of the raw expression A_LESS_THAN_B_HOLDS_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        kind: "requires",
        fragment: || "a_less_than_b_holds (a , b)",
        harnesses: &[
            "verify_btree_set_iterates_in_sorted_order",
            "verify_binary_heap_peek_mut_exposes_the_maximum",
        ],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "btree_set_iterates_in_sorted_order_holds (a , b , result)",
        harnesses: &["verify_btree_set_iterates_in_sorted_order"],
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_pop_yields_the_maximum_first",
            claim: VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeap<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeap<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn binary_heap_pop_yields_the_maximum_first_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeap<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BinaryHeap<i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "binary_heap_pop_yields_the_maximum_first_holds (a , b , result)",
        harnesses: &["verify_binary_heap_pop_yields_the_maximum_first"],
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeapDrain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_drain_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapDrain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeapDrain<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_drain_yields_every_pushed_element_once_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapDrain<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapDrain<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BinaryHeapDrain<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression
// BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapDrain<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "binary_heap_drain_yields_every_pushed_element_once_holds (a , b , result)",
        harnesses: &["verify_binary_heap_drain_yields_every_pushed_element_once"],
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeapIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_into_iter_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeapIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_into_iter_yields_every_pushed_element_once_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapIntoIter<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapIntoIter<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BinaryHeapIntoIter<i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression
// BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
// itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapIntoIter<i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "binary_heap_into_iter_yields_every_pushed_element_once_holds (a , b , result)",
        harnesses: &["verify_binary_heap_into_iter_yields_every_pushed_element_once"],
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeapIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_iter_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeapIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_iter_yields_every_pushed_element_once_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapIter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapIter<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<BinaryHeapIter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression
// BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC itself
// captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapIter<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "binary_heap_iter_yields_every_pushed_element_once_holds (a , b , result)",
        harnesses: &["verify_binary_heap_iter_yields_every_pushed_element_once"],
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_peek_mut_exposes_the_maximum",
            claim: VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapPeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns `amenable_creusot::A_LESS_THAN_B_HOLDS_SRC` /
/// `BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)]`
/// fns the real site calls, not a hand-retyped copy of their
/// expressions. `A_LESS_THAN_B_HOLDS_SRC` is the same shared
/// precondition `RustStdStandard<BTreeSet<i32>>`'s own `Requires` impl
/// above already names.
impl Requires<CreusotVerifier> for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        A_LESS_THAN_B_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapPeekMut<'static, i32>>",
        verifier: "creusot",
        kind: "requires",
        fragment: ||
            <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as Requires<CreusotVerifier>>::requires(()),
        harnesses: &[],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapPeekMut<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: ||
            <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC itself
// captures. (a_less_than_b_holds's own call-shape fragment is already
// registered, scoped to both harnesses, on
// RustStdStandard<BTreeSet<i32>> above.)
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeapPeekMut<'static, i32>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "binary_heap_peek_mut_exposes_the_maximum_holds (a , b , result)",
        harnesses: &["verify_binary_heap_peek_mut_exposes_the_maximum"],
    }
}

impl CreusotWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_is_fifo_through_back_and_front",
            claim: VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinkedList<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// [`DrainsTwoValuesInOrderAndEmpties`] reuses the `LinkedList` FIFO
/// harness rather than adding a new Creusot proof — it names the
/// postcondition both the `LinkedList` FIFO and `VecDeque::drain`
/// proofs already share, it doesn't prove anything new.
impl CreusotWitness for DrainsTwoValuesInOrderAndEmpties {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_is_fifo_through_back_and_front",
            claim: VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(DrainsTwoValuesInOrderAndEmpties);

/// Returns `amenable_creusot::DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn drains_two_values_in_order_and_empties` both real
/// sites call, not a hand-retyped copy of its expression. There is
/// exactly one place this postcondition's text exists in the whole
/// codebase.
impl Ensures<CreusotVerifier> for DrainsTwoValuesInOrderAndEmpties {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::DrainsTwoValuesInOrderAndEmpties",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <DrainsTwoValuesInOrderAndEmpties as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape both real sites now use, instead of the raw
// expression DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::DrainsTwoValuesInOrderAndEmpties",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "drains_two_values_in_order_and_empties (a , b , result)",
        harnesses: &[
            "verify_linked_list_is_fifo_through_back_and_front",
            "verify_vec_deque_drain_removes_and_yields_in_order",
        ],
    }
}

impl CreusotWitness for RustStdStandard<LinkedListIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_yields_references_in_order",
            claim: VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinkedListIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<LinkedListIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_mut_writes_through",
            claim: VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinkedListIterMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<LinkedListIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_into_iter_yields_owned_values_in_order",
            claim: VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinkedListIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    }
}

/// [`YieldsTwoValuesInOrderThenEnds`] reuses the `LinkedList::into_iter`
/// harness rather than adding a new Creusot proof — it names the
/// postcondition both the `LinkedList` and `VecDeque` `into_iter`
/// proofs already share, it doesn't prove anything new.
impl CreusotWitness for YieldsTwoValuesInOrderThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_into_iter_yields_owned_values_in_order",
            claim: VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(YieldsTwoValuesInOrderThenEnds);

/// Returns `amenable_creusot::YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn yields_two_values_in_order_then_ends` both real
/// sites call, not a hand-retyped copy of its expression. There is
/// exactly one place this postcondition's text exists in the whole
/// codebase.
impl Ensures<CreusotVerifier> for YieldsTwoValuesInOrderThenEnds {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::YieldsTwoValuesInOrderThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <YieldsTwoValuesInOrderThenEnds as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape both real sites now use, instead of the raw
// expression YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::YieldsTwoValuesInOrderThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "yields_two_values_in_order_then_ends (a , b , result)",
        harnesses: &[
            "verify_linked_list_into_iter_yields_owned_values_in_order",
            "verify_vec_deque_into_iter_yields_owned_values_in_order",
        ],
    }
}

impl CreusotWitness for RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_extract_if_partitions_by_the_predicate",
            claim: VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> as CreusotWitness>::proof()
                .to_string()
        },
    }
}

impl CreusotWitness for RustStdStandard<TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_reserve_rejects_an_impossible_capacity",
            claim: VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        verifier: "creusot",
        describe: || <RustStdStandard<TryReserveError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_pushes_and_pops_from_both_ends",
            claim: VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<VecDeque<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VecDequeIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_into_iter_yields_owned_values_in_order",
            claim: VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<VecDequeIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VecDequeDrain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_drain_removes_and_yields_in_order",
            claim: VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeDrain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<VecDequeDrain<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VecDequeIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_yields_references_in_order",
            claim: VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<VecDequeIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VecDequeIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_mut_writes_through",
            claim: VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<VecDequeIterMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Args> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_reports_at_least_the_program_path",
            claim: VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Args>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Args>",
        verifier: "creusot",
        describe: || <RustStdStandard<Args> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<ArgsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_os_reports_at_least_the_program_path",
            claim: VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<ArgsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ArgsOs>",
        verifier: "creusot",
        describe: || <RustStdStandard<ArgsOs> as CreusotWitness>::proof().to_string(),
    }
}

/// [`ArgvIncludesProgramPath`] reuses the `Args` harness rather than
/// adding a new Creusot proof — it names the precondition/postcondition
/// pair both `Args` and `ArgsOs` proofs already share, it doesn't prove
/// anything new.
impl CreusotWitness for ArgvIncludesProgramPath {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_args_reports_at_least_the_program_path",
            claim: VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(ArgvIncludesProgramPath);

/// Returns `amenable_creusot::ARGV_EXTRA_HEADROOM_HOLDS_SRC` /
/// `ARGV_INCLUDES_PROGRAM_PATH_SRC` directly -- the verbatim,
/// `harness!`-captured source of the real `#[logic(open)]` fns both
/// `verify_args_reports_at_least_the_program_path` and
/// `verify_args_os_reports_at_least_the_program_path` call, not a
/// hand-retyped copy of their expressions. There is exactly one place
/// each of this precondition/postcondition pair's text exists in the
/// whole codebase.
impl Requires<CreusotVerifier> for ArgvIncludesProgramPath {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        ARGV_EXTRA_HEADROOM_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for ArgvIncludesProgramPath {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ARGV_INCLUDES_PROGRAM_PATH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArgvIncludesProgramPath",
        verifier: "creusot",
        kind: "requires",
        fragment: || <ArgvIncludesProgramPath as Requires<CreusotVerifier>>::requires(()),
        harnesses: &[],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArgvIncludesProgramPath",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <ArgvIncludesProgramPath as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shapes both real sites now use, instead of the raw
// expressions ARGV_EXTRA_HEADROOM_HOLDS_SRC/ARGV_INCLUDES_PROGRAM_PATH_SRC
// themselves capture.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArgvIncludesProgramPath",
        verifier: "creusot",
        kind: "requires",
        fragment: || "argv_extra_headroom_holds (extra)",
        harnesses: &[
            "verify_args_reports_at_least_the_program_path",
            "verify_args_os_reports_at_least_the_program_path",
        ],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArgvIncludesProgramPath",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "argv_includes_program_path (extra , result)",
        harnesses: &[
            "verify_args_reports_at_least_the_program_path",
            "verify_args_os_reports_at_least_the_program_path",
        ],
    }
}

impl CreusotWitness for RustStdStandard<JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_join_paths_error_reports_an_unjoinable_path",
            claim: VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<JoinPathsError>",
        verifier: "creusot",
        describe: || <RustStdStandard<JoinPathsError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_paths_recovers_paths_joined_by_join_paths",
            claim: VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitPaths<'static>>",
        verifier: "creusot",
        describe: || <RustStdStandard<SplitPaths<'static>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VarError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_var_error_distinguishes_not_present_from_not_unicode",
            claim: VERIFY_VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<VarError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarError>",
        verifier: "creusot",
        describe: || <RustStdStandard<VarError> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn var_error_distinguishes_not_present_from_not_unicode`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<VarError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarError>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<VarError> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC
// itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarError>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "var_error_distinguishes_not_present_from_not_unicode (result)",
        harnesses: &["verify_var_error_distinguishes_not_present_from_not_unicode"],
    }
}

impl CreusotWitness for RustStdStandard<Vars> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<Vars>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Vars>",
        verifier: "creusot",
        describe: || <RustStdStandard<Vars> as CreusotWitness>::proof().report().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<VarsOs> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<VarsOs>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VarsOs>",
        verifier: "creusot",
        describe: || <RustStdStandard<VarsOs> as CreusotWitness>::proof().report().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_os_str_valid_utf8_content_round_trips_through_to_str",
            claim: VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsStr>",
        verifier: "creusot",
        describe: || <RustStdStandard<OsStr> as CreusotWitness>::proof().to_string(),
    }
}

/// Returns
/// `amenable_creusot::OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn os_str_valid_utf8_content_round_trips_through_to_str`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<OsStr> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsStr>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<OsStr> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape the real site now uses, instead of the raw
// expression OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC
// itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsStr>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "os_str_valid_utf8_content_round_trips_through_to_str (result)",
        harnesses: &["verify_os_str_valid_utf8_content_round_trips_through_to_str"],
    }
}

impl CreusotWitness for RustStdStandard<OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_os_string_push_appends_to_the_existing_content",
            claim: VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsString>",
        verifier: "creusot",
        describe: || <RustStdStandard<OsString> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<OsStrDisplay<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_os_str_display_renders_valid_utf8_content_unchanged",
            claim: VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<OsStrDisplay<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        verifier: "creusot",
        describe: || <RustStdStandard<OsStrDisplay<'static>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstring_excludes_the_terminator_and_rejects_interior_nul",
            claim: VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<CString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CString>",
        verifier: "creusot",
        describe: || <RustStdStandard<CString> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<FromVecWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromVecWithNulError>",
        verifier: "creusot",
        describe: || <RustStdStandard<FromVecWithNulError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_into_string_error_recovers_the_original_cstring",
            claim: VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IntoStringError>",
        verifier: "creusot",
        describe: || <RustStdStandard<IntoStringError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nul_error_reports_the_interior_nuls_position",
            claim: VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<NulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NulError>",
        verifier: "creusot",
        describe: || <RustStdStandard<NulError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes",
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CStr>",
        verifier: "creusot",
        describe: || <RustStdStandard<CStr> as CreusotWitness>::proof().to_string(),
    }
}

/// [`NonNulByte`] reuses the same harness rather than adding a new
/// Creusot proof — it names the precondition every `CStr`/`CString`-family
/// proof in this crate already requires, it doesn't prove anything new.
impl CreusotWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes",
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(NonNulByte);

/// Returns `amenable_creusot::NON_NUL_BYTE_HOLDS_SRC` directly — the
/// verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// non_nul_byte_holds` every site in that cluster now calls, not a
/// hand-retyped copy of its expression. There is exactly one place this
/// precondition's text exists in the whole codebase.
impl Requires<CreusotVerifier> for NonNulByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        NON_NUL_BYTE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NonNulByte",
        verifier: "creusot",
        kind: "requires",
        fragment: || <NonNulByte as Requires<CreusotVerifier>>::requires(()),
        harnesses: &[],
    }
}

// The real call shape every site in the CStr/CString cluster now uses,
// instead of the raw expression `NON_NUL_BYTE_HOLDS_SRC` itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NonNulByte",
        verifier: "creusot",
        kind: "requires",
        fragment: || "non_nul_byte_holds (byte)",
        harnesses: &[
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul",
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            "verify_nul_error_reports_the_interior_nuls_position",
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes",
            "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere",
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end",
        ],
    }
}

/// [`NulOnlyAtTheEndValidates`] reuses the `FromVecWithNulError` harness
/// rather than adding a new Creusot proof — it names the postcondition
/// both `from_vec_with_nul`/`from_bytes_with_nul` proofs already share,
/// it doesn't prove anything new.
impl CreusotWitness for NulOnlyAtTheEndValidates {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(NulOnlyAtTheEndValidates);

/// Returns `amenable_creusot::NUL_ONLY_AT_THE_END_VALIDATES_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn nul_only_at_the_end_validates` both
/// `verify_from_vec_with_nul_requires_the_nul_only_at_the_end` and
/// `verify_from_bytes_with_nul_requires_the_nul_only_at_the_end` call,
/// not a hand-retyped copy of its expression. There is exactly one
/// place this postcondition's text exists in the whole codebase.
impl Ensures<CreusotVerifier> for NulOnlyAtTheEndValidates {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        NUL_ONLY_AT_THE_END_VALIDATES_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NulOnlyAtTheEndValidates",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <NulOnlyAtTheEndValidates as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shape both real sites now use, instead of the raw
// expression NUL_ONLY_AT_THE_END_VALIDATES_SRC itself captures.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NulOnlyAtTheEndValidates",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "nul_only_at_the_end_validates (result)",
        harnesses: &[
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end",
        ],
    }
}

impl CreusotWitness for RustStdStandard<FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere",
            claim: VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<FromBytesUntilNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromBytesUntilNulError>",
        verifier: "creusot",
        describe: || <RustStdStandard<FromBytesUntilNulError> as CreusotWitness>::proof()
            .to_string(),
    }
}

impl CreusotWitness for RustStdStandard<FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromBytesWithNulError>",
        verifier: "creusot",
        describe: || <RustStdStandard<FromBytesWithNulError> as CreusotWitness>::proof()
            .to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_box_new_preserves_the_wrapped_value",
            claim: VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Box<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Box<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_duration_new_normalizes_nanos_and_carries_into_secs",
            claim: VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Duration>",
        verifier: "creusot",
        describe: || <RustStdStandard<Duration> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_range_to_contains_matches_bound",
            claim: VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RangeTo<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<RangeTo<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_range_full_contains_everything",
            claim: VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RangeFull>",
        verifier: "creusot",
        describe: || <RustStdStandard<RangeFull> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_bound_round_trips_its_endpoint",
            claim: VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Bound<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_control_flow_continue_and_break_are_disjoint",
            claim: VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<ControlFlow<i32, i32>> as CreusotWitness>::proof().to_string()
        },
    }
}

impl CreusotWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i16_roundtrips",
            claim: VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "creusot",
        describe: || <RustStdStandard<NonZero<i16>> as CreusotWitness>::proof().to_string(),
    }
}

/// `#[trusted]`'s two `#[ensures]` clauses each get their own
/// `ContractRecord` here — the same bound `Ensures<KaniVerifier>`
/// (`amenable_kani::rust_std::num`, `value != 0`) and
/// `Ensures<VerusVerifier>` (`amenable_std::verus_witness`, split into
/// `value != 0 ==> result`/`value == 0 ==> !result`) already name, restated
/// once more in Creusot's own `match`-expression form. Fragment text is
/// the literal normalized (tokenize, don't parse-as-`syn::Expr`) form
/// `elicit_doc`'s scanner computes, not hand-formatted — Pearlite content
/// like this isn't valid plain-Rust expression grammar to begin with.
impl Ensures<CreusotVerifier> for RustStdStandard<NonZero<i16>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "match result { Some (_) => value != 0i16 , None => value == 0i16 , }"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <RustStdStandard<NonZero<i16>> as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &["verify_nonzero_i16_roundtrips"],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "match result { Some (nz) => nonzero_i16_get (& nz) == value , None => true , }",
        harnesses: &["verify_nonzero_i16_roundtrips"],
    }
}

// Fully qualified, matching `amenable_kani::rust_std::cmp` and
// `amenable_std::rust_std::cmp`'s own registration exactly: there's also
// a `core::sync::atomic::Ordering`, so the evidence string must say
// `std::cmp::Ordering`, not the bare name, or alias resolution won't
// match this proof to the checklist row.
impl CreusotWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ordering_reverse_swaps_less_and_greater",
            claim: VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        verifier: "creusot",
        describe: || <RustStdStandard<std::cmp::Ordering> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_wrapping_i32_add_wraps",
            claim: VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Wrapping<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_saturating_i32_add_clamps",
            claim: VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Saturating<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::IntErrorKind, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::IntErrorKind>`).
impl CreusotWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_int_error_kind_classifies_parse_failures",
            claim: VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::num::IntErrorKind> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::TryFromIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::TryFromIntError>`).
impl CreusotWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_from_int_error_occurs_exactly_when_out_of_range",
            claim: VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::TryFromIntError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::ParseIntError>`).
impl CreusotWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_parse_int_error_reports_the_kind_of_the_failure",
            claim: VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::ParseIntError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::FpCategory, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::FpCategory>`).
//
// `#[trusted]`, like `NonZero<i16>` above: `f64` has no `View` impl in
// `creusot-std`, and a bare float literal in Pearlite panics
// `creusot-rustc` outright — both confirmed real blockers, not a
// convenience shortcut; see `amenable_std::creusot_gallery`'s
// `f64_has_no_view_impl_at_all`/`float_literals_in_pearlite_ice_the_compiler`
// findings.
impl CreusotWitness for RustStdStandard<core::num::FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_fp_category_matches_the_value_it_classifies",
            claim: VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::FpCategory> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseFloatError, ...)`, confirmed against the checklist's
// own `evidence_name` column: `RustStdStandard<core::num::ParseFloatError>`).
//
// `#[trusted]`: a real extern_spec for `FromStr for f64` translates
// cleanly but `why3find prove` doesn't discharge the harness's own goal
// against it — confirmed reproducible, not a convenience shortcut; see
// `amenable_std::creusot_gallery`'s
// `parse_float_error_extern_spec_translates_but_wont_discharge` finding.
impl CreusotWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_parse_float_error_occurs_only_for_unparseable_input",
            claim: VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::ParseFloatError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Bare `Reverse<i32>`, matching `amenable_std::rust_std::cmp`'s own
// registration exactly (`register_rust_std_standard_evidence!(std::cmp::
// Ordering, Reverse<i32>)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Reverse<i32>>`).
impl CreusotWitness for RustStdStandard<Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_reverse_inverts_comparison",
            claim: VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Reverse<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Option<i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Option<i32>>`).
impl CreusotWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_some_and_none_are_disjoint",
            claim: VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Option<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Result<i32, i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Result<i32, i32>>`).
impl CreusotWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_ok_and_err_are_disjoint",
            claim: VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Result<i32, i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified to distinguish `core::option::Iter` from the many other
// std carriers also named `Iter` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::option::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_yields_zero_or_one_reference",
            claim: VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::option::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::option::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::option::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    }
}

// Fully qualified to distinguish `core::option::IterMut` from the many other
// std carriers also named `IterMut` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::option::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_mut_writes_through_to_the_option",
            claim: VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::option::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::option::IterMut<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::option::IterMut<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    }
}

// Fully qualified to distinguish `core::result::Iter` from the many other
// std carriers also named `Iter` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::result::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_yields_a_reference_to_the_ok_value",
            claim: VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::result::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::result::Iter<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::result::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    }
}

// Fully qualified to distinguish `core::result::IterMut` from the many other
// std carriers also named `IterMut` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::result::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_mut_writes_through_to_the_result",
            claim: VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::result::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::result::IterMut<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::result::IterMut<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    }
}

/// [`IterYieldsValueOnceThenEnds`] reuses the `Option::iter` harness
/// rather than adding a new Creusot proof — it names the postcondition
/// all four `Option`/`Result` value-iterator proofs already share, it
/// doesn't prove anything new.
impl CreusotWitness for IterYieldsValueOnceThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_yields_zero_or_one_reference",
            claim: VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(IterYieldsValueOnceThenEnds);

/// Returns `amenable_creusot::ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn iter_yields_value_once_then_ends` both
/// `Option`-shaped sites call, not a hand-retyped copy of its
/// expression. There is exactly one place each shape's text exists in
/// the whole codebase (the `Result`-shaped sibling
/// `ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC` is registered as a
/// supplementary fragment below, alongside its own two call sites).
impl Ensures<CreusotVerifier> for IterYieldsValueOnceThenEnds {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IterYieldsValueOnceThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || <IterYieldsValueOnceThenEnds as Ensures<CreusotVerifier>>::ensures(()),
        harnesses: &[],
    }
}

// The real call shapes all four sites now use, instead of the raw
// expressions ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC/
// ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC themselves capture.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IterYieldsValueOnceThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "iter_yields_value_once_then_ends (value , value , result)",
        harnesses: &["verify_option_iter_yields_zero_or_one_reference"],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IterYieldsValueOnceThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "iter_yields_value_once_then_ends (value , updated , result)",
        harnesses: &["verify_option_iter_mut_writes_through_to_the_option"],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IterYieldsValueOnceThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "iter_yields_ok_value_once_then_ends (value , value , result)",
        harnesses: &["verify_result_iter_yields_a_reference_to_the_ok_value"],
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IterYieldsValueOnceThenEnds",
        verifier: "creusot",
        kind: "ensures",
        fragment: || "iter_yields_ok_value_once_then_ends (value , updated , result)",
        harnesses: &["verify_result_iter_mut_writes_through_to_the_result"],
    }
}

// Bare `Pending<i32>`, matching `amenable_std::rust_std::future`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Pending<i32>>`).
impl CreusotWitness for RustStdStandard<Pending<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pending_never_resolves",
            claim: VERIFY_PENDING_NEVER_RESOLVES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Pending<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `PollFn<fn(&mut Context<'_>) -> Poll<i32>>`, matching
// `amenable_std::rust_std::future`'s own registration exactly.
impl CreusotWitness for RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_poll_fn_dispatches_through_to_its_closure",
            claim: VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        verifier: "creusot",
        describe: || <RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> as CreusotWitness>::proof()
            .to_string(),
    }
}

// Bare `Ready<i32>`, matching `amenable_std::rust_std::future`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Ready<i32>>`).
impl CreusotWitness for RustStdStandard<Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ready_resolves_immediately_with_its_value",
            claim: VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Ready<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Context<'static>`, matching `amenable_std::rust_std::task`'s own
// representative lifetime choice exactly.
impl CreusotWitness for RustStdStandard<Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_context_from_waker_exposes_the_same_waker",
            claim: VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Context<'static>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Context<'static>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Poll<i32>`, matching `amenable_std::rust_std::task`'s own
// representative instantiation exactly.
impl CreusotWitness for RustStdStandard<Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_poll_ready_and_pending_are_disjoint",
            claim: VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Poll<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// `RawWaker` and `RawWakerVTable` stay trusted in Creusot for the same
// reason they do in Kani: constructing or exercising the concrete vtable
// semantics requires `unsafe fn` entries, which this proof stack does not
// admit.
impl_creusot_witness_trusted!(RawWaker, RawWakerVTable);

impl CreusotWitness for RustStdStandard<Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_waker_wake_by_ref_invokes_the_wake_impl",
            claim: VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Waker>",
        verifier: "creusot",
        describe: || <RustStdStandard<Waker> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `ManuallyDrop<i32>`, matching `amenable_std::rust_std::mem`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<ManuallyDrop<i32>>`).
impl CreusotWitness for RustStdStandard<ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_manually_drop_derefs_and_into_inner_round_trip",
            claim: VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ManuallyDrop<i32>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<ManuallyDrop<i32>> as CreusotWitness>::proof().to_string()
        },
    }
}

// `std::os::windows::{ffi::EncodeWide, io::{BorrowedHandle, BorrowedSocket,
// HandleOrInvalid, OwnedHandle, OwnedSocket}}`: unlike every other type in
// this file, these can never get a real `impl CreusotWitness for
// RustStdStandard<T>` here, on any platform -- not because the type can't
// be named (on Windows, `amenable_std` names it just fine, `#[cfg(windows)]`,
// same as the `verus_witness` bridge does), but because `creusot-rustc`
// itself has no Windows target and cannot run natively on Windows the way
// `verus` does (see `amenable_verus::rust_std::os_windows_carrier`'s
// module doc comment for that contrast). There is no host, ever, on which
// `cargo creusot` could check a claim about the real type. So this bypasses
// `CreusotWitness`/`bridge_creusot_witness!` entirely, exactly like
// `amenable_kani::os_windows_model` bypasses `KaniWitness` for the
// identical reason (Kani/CBMC also never run on Windows): each harness in
// `amenable_creusot::rust_std`'s windows cluster proves a property of a
// synthetic, real-type-agnostic model (plain `isize`/`u64`/`u32` values,
// checked for real by `cargo creusot prove`), and the `evidence` string
// below connects that proof to the real type's evidence entry by name
// only -- `amenable_core::ProofRecord`'s `evidence` field is just a
// string, never required to come from naming a type that actually
// compiled here. Unconditional (no `#[cfg(windows)]`), for the same
// reason `os_windows_model.rs`'s own bypass is unconditional: nothing
// here ever names the real type in Rust syntax, only in a string literal.

fn windows_provenance(
    source_module: &str,
    url: &str,
    type_name: &str,
    summary: &str,
) -> RustStdProvenance {
    RustStdProvenance::new(
        RustLanguageProvenance::for_source("std", source_module),
        url,
        type_name,
        summary,
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value",
            claim: VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html",
                "std::os::windows::io::BorrowedHandle<'static>",
                "The BorrowedHandle carrier borrows a raw Windows HANDLE without taking ownership of it.",
            ),
        }
        .to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value",
            claim: VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html",
                "std::os::windows::io::OwnedHandle",
                "The OwnedHandle carrier owns a raw Windows HANDLE, closing it on drop.",
            ),
        }
        .to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_windows_handle_or_invalid_rejects_only_the_sentinel",
            claim: VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC,
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html",
                "std::os::windows::io::HandleOrInvalid",
                "The HandleOrInvalid carrier owns a Windows HANDLE that may be the sentinel INVALID_HANDLE_VALUE, deferring that check to conversion time.",
            ),
        }
        .to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value",
            claim: VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html",
                "std::os::windows::io::BorrowedSocket<'static>",
                "The BorrowedSocket carrier borrows a raw Windows SOCKET without taking ownership of it.",
            ),
        }
        .to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value",
            claim: VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedSocket.html",
                "std::os::windows::io::OwnedSocket",
                "The OwnedSocket carrier owns a raw Windows SOCKET, closing it on drop.",
            ),
        }
        .to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        verifier: "creusot",
        describe: || CheckedProof {
            harness: "verify_encode_wide_model_encodes_a_bmp_code_point_as_one_code_unit",
            claim: VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_SRC,
            provenance: windows_provenance(
                "std::os::windows::ffi",
                "https://doc.rust-lang.org/std/os/windows/ffi/struct.EncodeWide.html",
                "std::os::windows::ffi::EncodeWide<'static>",
                "The EncodeWide carrier lazily encodes an OsStr as UTF-16 code units, as Windows APIs expect.",
            ),
        }
        .to_string(),
    }
}
