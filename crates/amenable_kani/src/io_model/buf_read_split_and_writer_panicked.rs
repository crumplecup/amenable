use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Root assumption behind the bounded delimiter-splitting observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniBufReadSplitWindow;

impl Provenance for KaniBufReadSplitWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded delimiter-splitting observation over `[first, delimiter, second, delimiter, third]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_builder::Builder)]
pub struct KaniBufReadSplitObservation {
    /// The first segment byte.
    first: u8,
    /// The repeated delimiter byte.
    delimiter: u8,
    /// The second segment byte.
    second: u8,
    /// The third segment byte.
    third: u8,
}

impl KaniBufReadSplitObservation {
    /// Model `BufRead::split(delimiter)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn segments(&self) -> ([u8; 1], [u8; 1], [u8; 1]) {
        ([self.first], [self.second], [self.third])
    }
}

impl Evidence for KaniBufReadSplitObservation {
    type Basis = KaniBufReadSplitWindow;
    type Audit = [u8; 5];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniBufReadSplitWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded panic-recovery observation for `WriterPanicked`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniWriterPanickedObservation {
    buffered: [u8; 2],
}

impl KaniWriterPanickedObservation {
    /// Model whether the inner writer panicked.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn panicked(&self) -> bool {
        true
    }

    /// Recover the bytes that remained buffered after the panic.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn recovered_buffer(&self) -> [u8; 2] {
        self.buffered
    }
}

impl Evidence for KaniWriterPanickedObservation {
    type Basis = KaniWriterPanickedWindow;
    type Audit = [u8; 2];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniWriterPanickedWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        self.buffered
    }
}
