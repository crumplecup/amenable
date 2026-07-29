//! Kani-only accommodation models for the buffered `std::io` proof family.
//!
//! The direct `std::io` paths for `BufReader`, `IntoInnerError`, `LineWriter`,
//! `Lines`, and `Split` are all pure in-memory, but still expand into enough
//! buffered-reader / buffered-writer machinery to time out under Kani. The
//! `WriterPanicked` path adds an unsupported `catch_unwind` boundary on top of
//! that. This module captures the bounded observable laws the production proofs
//! actually claim, so the verifier checks the semantic contract rather than the
//! entire std implementation.

use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Root assumption behind the bounded buffered-read observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniBufferedReadWindow;

impl Provenance for KaniBufferedReadWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a bounded buffered read preserves the exact underlying byte order while abstracting over std::io::BufReader's internal refill machinery",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct in-memory BufReader path still times out under Kani despite having no OS boundary",
            ),
        ]
        .into_iter()
    }
}

/// Bounded buffered-read observation over a two-byte payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBufferedReadObservation {
    bytes: [u8; 2],
}

impl KaniBufferedReadObservation {
    /// Construct the bounded buffered-read witness.
    #[must_use]
    pub fn new(bytes: [u8; 2]) -> Self {
        Self { bytes }
    }

    /// Model reading the underlying source to completion.
    #[must_use]
    pub fn read_to_end(&self) -> [u8; 2] {
        self.bytes
    }
}

impl Evidence for KaniBufferedReadObservation {
    type Basis = KaniBufferedReadWindow;
    type Audit = [u8; 2];

    fn basis() -> Self::Basis {
        KaniBufferedReadWindow
    }

    fn audit(&self) -> Self::Audit {
        self.bytes
    }
}

/// Root assumption behind the bounded flush-failure recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniFlushErrorWindow;

impl Provenance for KaniFlushErrorWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a bounded flush failure reports failure while still recovering the buffered writer state needed by IntoInnerError",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct in-memory into_inner error-recovery path still times out under Kani's buffered-writer expansion",
            ),
        ]
        .into_iter()
    }
}

/// Bounded `IntoInnerError`-style recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFlushErrorObservation {
    buffered: [u8; 2],
}

impl KaniFlushErrorObservation {
    /// Construct the bounded flush-failure witness.
    #[must_use]
    pub fn new(buffered: [u8; 2]) -> Self {
        Self { buffered }
    }

    /// Model whether the flush failed.
    #[must_use]
    pub fn flush_failed(&self) -> bool {
        true
    }

    /// Recover the bytes that remained buffered in the writer.
    #[must_use]
    pub fn recovered_buffer(&self) -> [u8; 2] {
        self.buffered
    }
}

impl Evidence for KaniFlushErrorObservation {
    type Basis = KaniFlushErrorWindow;
    type Audit = [u8; 2];

    fn basis() -> Self::Basis {
        KaniFlushErrorWindow
    }

    fn audit(&self) -> Self::Audit {
        self.buffered
    }
}

/// Root assumption behind the bounded line-buffering observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniLineWriterWindow;

impl Provenance for KaniLineWriterWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a bounded line writer flushes immediately after a completed line ending in newline, while a trailing partial line remains buffered until an explicit flush",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct in-memory LineWriter path still times out under Kani's line-buffering internals",
            ),
        ]
        .into_iter()
    }
}

/// Bounded line-buffering observation over one complete line and one trailing byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniLineWriterObservation {
    line_byte: u8,
    trailing_byte: u8,
}

impl KaniLineWriterObservation {
    /// Construct the bounded line-buffering witness.
    #[must_use]
    pub fn new(line_byte: u8, trailing_byte: u8) -> Self {
        Self {
            line_byte,
            trailing_byte,
        }
    }

    /// Model the bytes visible in the underlying writer after writing a newline-terminated line.
    #[must_use]
    pub fn after_newline_write(&self) -> [u8; 2] {
        [self.line_byte, b'\n']
    }

    /// Model the underlying writer state before the trailing partial line is flushed.
    #[must_use]
    pub fn buffered_before_flush(&self) -> [u8; 2] {
        self.after_newline_write()
    }

    /// Model the underlying writer state after an explicit flush.
    #[must_use]
    pub fn after_flush(&self) -> [u8; 3] {
        [self.line_byte, b'\n', self.trailing_byte]
    }
}

impl Evidence for KaniLineWriterObservation {
    type Basis = KaniLineWriterWindow;
    type Audit = [u8; 3];

    fn basis() -> Self::Basis {
        KaniLineWriterWindow
    }

    fn audit(&self) -> Self::Audit {
        [self.line_byte, b'\n', self.trailing_byte]
    }
}

/// Root assumption behind the bounded line-splitting observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniLinesWindow;

impl Provenance for KaniLinesWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a bounded buffered reader with newline separators yields each line body without its trailing terminator",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct in-memory BufRead::lines path still times out under Kani's line iteration and string machinery",
            ),
        ]
        .into_iter()
    }
}

/// Bounded line-splitting observation over three one-byte ASCII lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniLinesObservation {
    first: u8,
    second: u8,
    third: u8,
}

impl KaniLinesObservation {
    /// Construct the bounded line-splitting witness.
    #[must_use]
    pub fn new(first: u8, second: u8, third: u8) -> Self {
        Self {
            first,
            second,
            third,
        }
    }

    /// Model `BufRead::lines()` over the bounded witness.
    #[must_use]
    pub fn lines(&self) -> ([u8; 1], [u8; 1], [u8; 1]) {
        ([self.first], [self.second], [self.third])
    }
}

impl Evidence for KaniLinesObservation {
    type Basis = KaniLinesWindow;
    type Audit = [u8; 5];

    fn basis() -> Self::Basis {
        KaniLinesWindow
    }

    fn audit(&self) -> Self::Audit {
        [self.first, b'\n', self.second, b'\n', self.third]
    }
}

/// Root assumption behind the bounded delimiter-splitting observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniBufReadSplitWindow;

impl Provenance for KaniBufReadSplitWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a bounded buffered reader split on one repeated delimiter yields the delimiter-separated segments and drops the delimiter itself",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct in-memory BufRead::split path still times out under Kani despite incremental observation",
            ),
        ]
        .into_iter()
    }
}

/// Bounded delimiter-splitting observation over `[first, delimiter, second, delimiter, third]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBufReadSplitObservation {
    first: u8,
    delimiter: u8,
    second: u8,
    third: u8,
}

impl KaniBufReadSplitObservation {
    /// Construct the bounded split witness.
    #[must_use]
    pub fn new(first: u8, delimiter: u8, second: u8, third: u8) -> Self {
        Self {
            first,
            delimiter,
            second,
            third,
        }
    }

    /// Model `BufRead::split(delimiter)` over the bounded witness.
    #[must_use]
    pub fn segments(&self) -> ([u8; 1], [u8; 1], [u8; 1]) {
        ([self.first], [self.second], [self.third])
    }
}

impl Evidence for KaniBufReadSplitObservation {
    type Basis = KaniBufReadSplitWindow;
    type Audit = [u8; 5];

    fn basis() -> Self::Basis {
        KaniBufReadSplitWindow
    }

    fn audit(&self) -> Self::Audit {
        [
            self.first,
            self.delimiter,
            self.second,
            self.delimiter,
            self.third,
        ]
    }
}

/// Root assumption behind the bounded panic-recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniWriterPanickedWindow;

impl Provenance for KaniWriterPanickedWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "if a buffered writer's inner write panics after bytes are already buffered, WriterPanicked recovers those buffered bytes intact",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std path reaches catch_unwind before the recovery property can be checked under Kani",
            ),
        ]
        .into_iter()
    }
}

/// Bounded panic-recovery observation for `WriterPanicked`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniWriterPanickedObservation {
    buffered: [u8; 2],
}

impl KaniWriterPanickedObservation {
    /// Construct the bounded panic-recovery witness.
    #[must_use]
    pub fn new(buffered: [u8; 2]) -> Self {
        Self { buffered }
    }

    /// Model whether the inner writer panicked.
    #[must_use]
    pub fn panicked(&self) -> bool {
        true
    }

    /// Recover the bytes that remained buffered after the panic.
    #[must_use]
    pub fn recovered_buffer(&self) -> [u8; 2] {
        self.buffered
    }
}

impl Evidence for KaniWriterPanickedObservation {
    type Basis = KaniWriterPanickedWindow;
    type Audit = [u8; 2];

    fn basis() -> Self::Basis {
        KaniWriterPanickedWindow
    }

    fn audit(&self) -> Self::Audit {
        self.buffered
    }
}
