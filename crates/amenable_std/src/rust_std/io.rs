//! `RustStdType` registrations for `std::io`.
//!
//! `core::io` (a `no_std`-compatible mirror of a subset of this module,
//! e.g. `core::io::Cursor`/`ErrorKind`/`IoSlice`/`SeekFrom`) is deliberately
//! not covered — unstable (`core_io`, rust-lang/rust#154046). It isn't
//! caught by elicit_doc's item-level stability screening, since these
//! particular items don't carry their own `#[unstable]` attribute (only
//! the surrounding feature gate does) — a real, documented limitation of
//! that screening, not a bug in the exclusion itself.

use std::io::{
    BufReader, BufWriter, Bytes, Chain, Cursor, IntoInnerError, IoSlice, IoSliceMut, LineWriter,
    Lines, PipeReader, PipeWriter, SeekFrom, Split, Stderr, StderrLock, Stdin, StdinLock, Stdout,
    StdoutLock, Take, Write, WriterPanicked,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type_generic1!(
    BufReader,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.BufReader.html",
    "The BufReader carrier wraps a Read implementor, adding a buffer to reduce the number of small reads made against it."
);

impl<W: Write> crate::RustStdType for BufWriter<W> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("std", "std::io")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/io/struct.BufWriter.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The BufWriter carrier wraps a Write implementor, adding a buffer to reduce the number of small writes made against it."
    }
}

impl_rust_std_type_generic1!(
    Bytes,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Bytes.html",
    "The Bytes carrier lazily yields the bytes of an underlying reader one at a time."
);

impl_rust_std_type_generic1!(
    IntoInnerError,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.IntoInnerError.html",
    "The IntoInnerError carrier reports that flushing a buffered writer failed while recovering its inner writer, and returns both."
);

impl<W: Write> crate::RustStdType for LineWriter<W> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("std", "std::io")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/io/struct.LineWriter.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The LineWriter carrier wraps a Write implementor, flushing automatically whenever a newline is written."
    }
}

impl_rust_std_type_generic1!(
    Lines,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Lines.html",
    "The Lines carrier lazily yields the lines of an underlying buffered reader."
);

impl_rust_std_type_generic2!(
    Chain,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Chain.html",
    "The Chain carrier reads one source to exhaustion before reading the next."
);

impl_rust_std_type_generic1!(
    Cursor,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Cursor.html",
    "The Cursor carrier adds Read/Write/Seek over an in-memory byte buffer, tracked by an internal position."
);

impl_rust_std_type!(
    std::io::Empty,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Empty.html",
    "The Empty carrier is a reader that always reports end-of-file and a writer/seeker that ignores its input."
);

impl_rust_std_type!(
    std::io::Error,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Error.html",
    "The Error carrier reports an I/O failure, carrying a portable ErrorKind and an optional platform or custom payload."
);

impl_rust_std_type_lifetime0!(
    IoSlice,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.IoSlice.html",
    "The IoSlice carrier borrows a byte buffer for vectored I/O reads without owning or copying it."
);

impl_rust_std_type_lifetime0!(
    IoSliceMut,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.IoSliceMut.html",
    "The IoSliceMut carrier mutably borrows a byte buffer for vectored I/O writes without owning or copying it."
);

impl_rust_std_type!(
    PipeReader,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.PipeReader.html",
    "The PipeReader carrier is the reading half of an anonymous OS pipe."
);

impl_rust_std_type!(
    PipeWriter,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.PipeWriter.html",
    "The PipeWriter carrier is the writing half of an anonymous OS pipe."
);

impl_rust_std_type!(
    std::io::Repeat,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Repeat.html",
    "The Repeat carrier is a reader that endlessly repeats a single byte."
);

impl_rust_std_type!(
    SeekFrom,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/enum.SeekFrom.html",
    "The SeekFrom carrier specifies a seek offset relative to the start, end, or current position of a stream."
);

impl_rust_std_type!(
    std::io::Sink,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Sink.html",
    "The Sink carrier is a writer that discards every byte written to it, always reporting success."
);

impl_rust_std_type_generic1!(
    Split,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Split.html",
    "The Split carrier lazily yields the segments of an underlying buffered reader, split on a given byte."
);

impl_rust_std_type!(
    Stderr,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Stderr.html",
    "The Stderr carrier is a handle to the process's standard error stream."
);

impl_rust_std_type_lifetime0!(
    StderrLock,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.StderrLock.html",
    "The StderrLock carrier is a locked, exclusive handle to the process's standard error stream."
);

impl_rust_std_type!(
    Stdin,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Stdin.html",
    "The Stdin carrier is a handle to the process's standard input stream."
);

impl_rust_std_type_lifetime0!(
    StdinLock,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.StdinLock.html",
    "The StdinLock carrier is a locked, exclusive handle to the process's standard input stream."
);

impl_rust_std_type!(
    Stdout,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Stdout.html",
    "The Stdout carrier is a handle to the process's standard output stream."
);

impl_rust_std_type_lifetime0!(
    StdoutLock,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.StdoutLock.html",
    "The StdoutLock carrier is a locked, exclusive handle to the process's standard output stream."
);

impl_rust_std_type_generic1!(
    Take,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.Take.html",
    "The Take carrier caps a reader at a fixed number of remaining bytes."
);

impl_rust_std_type!(
    WriterPanicked,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.WriterPanicked.html",
    "The WriterPanicked carrier reports that a BufWriter's inner writer panicked, and returns the buffered data that couldn't be written."
);

// `&'static [u8]` (an in-memory Read+BufRead source) and `Vec<u8>` (an
// in-memory Write sink) are the representative reader/writer types this
// module's proof batch covers.
//
// Bytes/Chain/Error/Lines/Split/Take are written fully-qualified: all of
// these bare names are shared by other modules (e.g. `std::str::Bytes`,
// `std::str::Lines`, `std::slice::Split`, `std::iter::Chain`,
// `std::iter::Take`, `core::fmt::Error`) — only the qualified path
// disambiguates which one a given registration means for tooling reading
// the registry (e.g. `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
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
    SeekFrom,
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
);
