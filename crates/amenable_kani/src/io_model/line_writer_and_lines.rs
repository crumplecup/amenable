use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Root assumption behind the bounded line-buffering observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniLineWriterWindow;

impl Provenance for KaniLineWriterWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded line-buffering observation over one complete line and one trailing byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniLineWriterObservation {
    line_byte: u8,
    trailing_byte: u8,
}

impl KaniLineWriterObservation {
    /// Model the bytes visible in the underlying writer after writing a newline-terminated line.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn after_newline_write(&self) -> [u8; 2] {
        [self.line_byte, b'\n']
    }

    /// Model the underlying writer state before the trailing partial line is flushed.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn buffered_before_flush(&self) -> [u8; 2] {
        self.after_newline_write()
    }

    /// Model the underlying writer state after an explicit flush.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded line-splitting observation over three one-byte ASCII lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniLinesObservation {
    first: u8,
    second: u8,
    third: u8,
}

impl KaniLinesObservation {
    /// Model `BufRead::lines()` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
