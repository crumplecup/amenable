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
    BufReader, BufWriter, Bytes, IntoInnerError, LineWriter, Lines, PipeReader, PipeWriter, Split,
    Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, Write, WriterPanicked,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_lifetime0,
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

impl_rust_std_type!(
    WriterPanicked,
    "std",
    "std::io",
    "https://doc.rust-lang.org/std/io/struct.WriterPanicked.html",
    "The WriterPanicked carrier reports that a LineWriter's inner writer panicked, and returns the buffered data that couldn't be written."
);
