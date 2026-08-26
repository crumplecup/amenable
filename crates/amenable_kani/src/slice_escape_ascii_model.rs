//! Kani-only accommodation model for `std::slice::EscapeAscii`.
//!
//! The direct `EscapeAscii` iterator path times out under Kani even on the
//! fixed two-byte witness `[printable, b'\n']`, whether observed through eager
//! `collect::<Vec<u8>>()` or stepwise `next()` calls. This module keeps the
//! smaller escaped-byte law the production proof actually claims.

use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// The assumption `KaniEscapeAsciiWindow` stands in for: a fixed two-byte
/// source `[printable, b'\n']` escapes to `[printable, b'\\', b'n']`
/// without executing the real `std::slice::EscapeAscii` iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniEscapeAsciiWindow;

impl Provenance for KaniEscapeAsciiWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a fixed two-byte source [printable, newline] escapes to [printable, backslash, n]",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::slice::EscapeAscii iterator times out under Kani even on that fixed witness, both when eagerly collected and when observed with next()",
            ),
        ]
        .into_iter()
        })
    }
}

/// Audit payload for the bounded `escape_ascii` observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters)]
pub struct KaniEscapeAsciiAudit {
    /// The modeled source bytes.
    #[getter(copy)]
    source: [u8; 2],
    /// The modeled escaped bytes.
    #[getter(copy)]
    escaped: [u8; 3],
}

/// Bounded `escape_ascii` observation for `[printable, b'\n']`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct KaniEscapeAsciiObservation {
    printable: u8,
}

impl KaniEscapeAsciiObservation {
    /// Recover the modeled source bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn source(&self) -> [u8; 2] {
        [self.printable, b'\n']
    }

    /// Recover the modeled escaped bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn escaped(&self) -> [u8; 3] {
        [self.printable, b'\\', b'n']
    }
}

impl Evidence for KaniEscapeAsciiObservation {
    type Basis = KaniEscapeAsciiWindow;
    type Audit = KaniEscapeAsciiAudit;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniEscapeAsciiWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        KaniEscapeAsciiAudit {
            source: self.source(),
            escaped: self.escaped(),
        }
    }
}
