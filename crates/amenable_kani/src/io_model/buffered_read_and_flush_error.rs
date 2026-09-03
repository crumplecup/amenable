use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Root assumption behind the bounded buffered-read observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniBufferedReadWindow;

impl Provenance for KaniBufferedReadWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded buffered-read observation over a two-byte payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniBufferedReadObservation {
    bytes: [u8; 2],
}

impl KaniBufferedReadObservation {
    /// Model reading the underlying source to completion.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn read_to_end(&self) -> [u8; 2] {
        self.bytes
    }
}

impl Evidence for KaniBufferedReadObservation {
    type Basis = KaniBufferedReadWindow;
    type Audit = [u8; 2];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniBufferedReadWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        self.bytes
    }
}

/// Root assumption behind the bounded flush-failure recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniFlushErrorWindow;

impl Provenance for KaniFlushErrorWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
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
        })
    }
}

/// Bounded `IntoInnerError`-style recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniFlushErrorObservation {
    buffered: [u8; 2],
}

impl KaniFlushErrorObservation {
    /// Model whether the flush failed.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn flush_failed(&self) -> bool {
        true
    }

    /// Recover the bytes that remained buffered in the writer.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn recovered_buffer(&self) -> [u8; 2] {
        self.buffered
    }
}

impl Evidence for KaniFlushErrorObservation {
    type Basis = KaniFlushErrorWindow;
    type Audit = [u8; 2];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniFlushErrorWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        self.buffered
    }
}
