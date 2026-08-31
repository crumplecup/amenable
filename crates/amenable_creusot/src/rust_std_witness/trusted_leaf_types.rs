use std::alloc::{Layout, LayoutError};
use std::any::TypeId;
use std::array::{IntoIter, TryFromSliceError};
use std::boxed::Box;
use std::cell::{
    BorrowError, BorrowMutError, Cell, LazyCell, OnceCell, Ref, RefCell, RefMut, UnsafeCell,
};
use std::char::{
    CharTryFromError, DecodeUtf16, DecodeUtf16Error, ParseCharError, ToLowercase, ToUppercase,
    TryFromCharError,
};
use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::fmt::{
    Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter, FromFn,
};
use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
};
use std::hash::BuildHasherDefault;
use std::io::{
    BufReader, BufWriter, Cursor, IntoInnerError, IoSlice, IoSliceMut, LineWriter, PipeReader,
    PipeWriter, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, WriterPanicked,
};
use std::iter::{
    Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Fuse, Inspect, Map, MapWhile,
    OnceWith, Peekable, RepeatN, RepeatWith, Rev, Scan, Skip, SkipWhile, StepBy, Successors,
    TakeWhile, Zip,
};
use std::marker::{PhantomData, PhantomPinned};
use std::mem::Discriminant;
use std::net::{
    AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
    TcpListener, TcpStream, UdpSocket,
};
use std::ops::Range;
use std::panic::PanicHookInfo;
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
use std::sync::mpsc::SyncSender;
use std::sync::{
    Arc, Barrier, BarrierWaitResult, LazyLock, OnceLock, OnceState, WaitTimeoutResult,
};
use std::task::{RawWaker, RawWakerVTable};
use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};
use std::time::{Instant, SystemTime, SystemTimeError, TryFromFloatSecsError};
use std::vec::Vec;

use core::panic::{Location, PanicInfo, PanicMessage};

use crate::{CreusotVerifier, CreusotWitness};
use amenable_core::{Evidence, Provenance, Witness};

use amenable_std::{RustStdProvenance, RustStdStandard};

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
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    "creusot",
                    || <RustStdStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                )
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
    std::vec::Splice<'static, std::vec::IntoIter<i32>>,
    // `RawWaker` and `RawWakerVTable` stay trusted in Creusot for the same
    // reason they do in Kani: constructing or exercising the concrete
    // vtable semantics requires `unsafe fn` entries, which this proof
    // stack does not admit.
    RawWaker,
    RawWakerVTable
);
