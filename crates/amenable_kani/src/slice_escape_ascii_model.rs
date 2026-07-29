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
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
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
    }
}

/// Audit payload for the bounded `escape_ascii` observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniEscapeAsciiAudit {
    source: [u8; 2],
    escaped: [u8; 3],
}

impl KaniEscapeAsciiAudit {
    /// Recover the modeled source bytes.
    #[must_use]
    pub fn source(&self) -> [u8; 2] {
        self.source
    }

    /// Recover the modeled escaped bytes.
    #[must_use]
    pub fn escaped(&self) -> [u8; 3] {
        self.escaped
    }
}

/// Bounded `escape_ascii` observation for `[printable, b'\n']`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniEscapeAsciiObservation {
    printable: u8,
}

impl KaniEscapeAsciiObservation {
    /// Construct the bounded `[printable, b'\n']` witness.
    #[must_use]
    pub fn new(printable: u8) -> Self {
        Self { printable }
    }

    /// Recover the modeled source bytes.
    #[must_use]
    pub fn source(&self) -> [u8; 2] {
        [self.printable, b'\n']
    }

    /// Recover the modeled escaped bytes.
    #[must_use]
    pub fn escaped(&self) -> [u8; 3] {
        [self.printable, b'\\', b'n']
    }
}

impl Evidence for KaniEscapeAsciiObservation {
    type Basis = KaniEscapeAsciiWindow;
    type Audit = KaniEscapeAsciiAudit;

    fn basis() -> Self::Basis {
        KaniEscapeAsciiWindow
    }

    fn audit(&self) -> Self::Audit {
        KaniEscapeAsciiAudit {
            source: self.source(),
            escaped: self.escaped(),
        }
    }
}
