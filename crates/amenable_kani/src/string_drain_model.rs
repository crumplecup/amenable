//! Kani-only accommodation model for `String::drain(..)`.
//!
//! The direct `String::drain(..)` path times out under Kani even when the
//! source string is forced down to a single ASCII character; the gallery
//! preserves that false trail. This model captures the bounded law the
//! production proof actually claims instead: draining the whole string yields
//! its exact UTF-8 content and leaves the source empty.

use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;

use crate::KaniUtf8Buffer;

/// Observable result of draining an entire bounded UTF-8 string.
///
/// The assumption this observation stands in for -- that draining the whole
/// string yields its exact UTF-8 content in order and leaves the source empty,
/// and nothing else about the direct `String::drain(..)` iterator machinery --
/// is named explicitly as a `Standard` rather than left as prose: the direct
/// std path times out under Kani even for a single ASCII character, so this
/// bounded observation is what the `Drain<'static>` proof actually rests on.
#[derive(Debug, Clone, PartialEq, Eq, Standard)]
#[standard(basis = "Self")]
pub struct KaniStringDrainObservation {
    yielded: KaniUtf8Buffer<2>,
}

impl Metadata for KaniStringDrainObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "draining a whole string yields its exact UTF-8 content in order and leaves the source empty, standing in for the direct String::drain iterator path",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct String::drain path times out under Kani even for a single ASCII character -- see gallery::string_drain",
            ),
            OwnedEntry::new("yielded_bytes", format!("{:?}", self.yielded.as_bytes())),
        ]
    }
}

impl Provenance for KaniStringDrainObservation {}

impl KaniStringDrainObservation {
    /// Model draining the entire source string.
    #[must_use]
    pub fn new(source: KaniUtf8Buffer<2>) -> Self {
        Self { yielded: source }
    }

    /// Borrow the drained bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn yielded_bytes(&self) -> &[u8] {
        self.yielded.as_bytes()
    }

    /// Report the drained UTF-8 byte length.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn yielded_len(&self) -> usize {
        self.yielded.len()
    }

    /// Report the source length after draining.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn source_len_after_drain(&self) -> usize {
        0
    }

    /// Report that the drained source string is empty afterward.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn source_is_empty(&self) -> bool {
        true
    }
}

impl Default for KaniStringDrainObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self::new(KaniUtf8Buffer::<2>::new([0, 0], 0).expect("empty bytes are valid UTF-8"))
    }
}
