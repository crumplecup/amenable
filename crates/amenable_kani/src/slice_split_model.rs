//! Kani-only accommodation model for `std::slice`'s delimiter-search split family.
//!
//! The direct `std::slice::Split` / `SplitMut` / `SplitInclusive` /
//! `SplitInclusiveMut` / `SplitN` / `SplitNMut` / `RSplit` / `RSplitMut` /
//! `RSplitN` / `RSplitNMut` path remains preserved in the proof gallery as a
//! timeout boundary. This module captures the bounded observable laws the
//! current production proofs actually claim:
//!
//! - a one-delimiter `[before, delimiter, after]` witness for split, inclusive
//!   split, reverse split, and write-through mutation
//! - a two-delimiter `[first, delimiter, middle, delimiter, last]` witness for
//!   cap-at-two `splitn` / `rsplitn`
//!
//! Production proofs that use this model are therefore conditional:
//!
//! - if the real std split-family path refines these modeled laws on the same
//!   bounded layouts,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// The assumption `KaniSplitObservation` stands in for: a one-delimiter
/// split reduces to a fixed `[before, delimiter, after]` window, and
/// nothing else about the real, symbolic-length delimiter search
/// `std::slice::Split` performs internally. Named explicitly as a
/// `Standard` rather than left as prose: `Split::next()` times out under
/// Kani even on the identical fixed 3-element array and predicate a bare
/// `Iter::position` resolves immediately (see
/// `gallery::slice_split_position`), so this bounded window is what every
/// one-delimiter split proof in this crate actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniSplitWindow;

impl Provenance for KaniSplitWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a one-delimiter split reduces to a fixed [before, delimiter, after] window, standing in for std::slice::Split's internal symbolic-length delimiter search",
            ),
            MetadataEntry::new(
                "rationale",
                "Split::next() times out under Kani even on the identical fixed 3-element array and predicate a bare Iter::position resolves immediately -- see gallery::slice_split_position",
            ),
        ]
        .into_iter()
        })
    }
}

/// Bounded one-delimiter split observation for `[before, delimiter, after]`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_setters::Setters,
)]
#[setters(prefix = "with_")]
pub struct KaniSplitObservation<T> {
    /// The element before the delimiter.
    before: T,
    /// The delimiter element.
    delimiter: T,
    /// The element after the delimiter.
    after: T,
}

impl<T: Copy> KaniSplitObservation<T> {
    /// Model `split(predicate)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn split(&self) -> ([T; 1], [T; 1]) {
        ([self.before], [self.after])
    }

    /// Model `split_inclusive(predicate)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn split_inclusive(&self) -> ([T; 2], [T; 1]) {
        ([self.before, self.delimiter], [self.after])
    }

    /// Model `rsplit(predicate)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn rsplit(&self) -> ([T; 1], [T; 1]) {
        ([self.after], [self.before])
    }

    /// Recover the modeled underlying data after any write-through updates.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn data(&self) -> [T; 3] {
        [self.before, self.delimiter, self.after]
    }
}

impl<T: Copy> Evidence for KaniSplitObservation<T> {
    type Basis = KaniSplitWindow;
    type Audit = [T; 3];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniSplitWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        self.data()
    }
}

/// The assumption `KaniSplitNObservation` stands in for: a two-delimiter
/// `splitn`/`rsplitn` (capped at two pieces) reduces to a fixed `[first,
/// d1, middle, d2, last]` window, and nothing else about the real,
/// symbolic-length delimiter search `std::slice::SplitN` performs
/// internally. Named explicitly as a `Standard` for the same reason as
/// `KaniSplitWindow`: the same internal `Iterator::position` search cost
/// documented in `gallery::slice_split_position` applies here too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniSplitNWindow;

impl Provenance for KaniSplitNWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a two-delimiter splitn/rsplitn reduces to a fixed [first, d1, middle, d2, last] window capped at two pieces, standing in for std::slice::SplitN's internal symbolic-length delimiter search",
            ),
            MetadataEntry::new(
                "rationale",
                "the same internal Iterator::position search cost documented for the one-delimiter case applies here too -- see gallery::slice_split_position",
            ),
        ]
        .into_iter()
        })
    }
}

/// Bounded two-delimiter split observation for `[first, d1, middle, d2, last]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_builder::Builder)]
pub struct KaniSplitNObservation<T: Copy> {
    /// The leading element.
    first: T,
    /// The first delimiter.
    first_delimiter: T,
    /// The middle element.
    middle: T,
    /// The second delimiter.
    second_delimiter: T,
    /// The trailing element.
    last: T,
}

impl<T: Copy> KaniSplitNObservation<T> {
    /// Model `splitn(2, predicate)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn splitn_two(&self) -> ([T; 1], [T; 3]) {
        (
            [self.first],
            [self.middle, self.second_delimiter, self.last],
        )
    }

    /// Model `rsplitn(2, predicate)` over the bounded witness.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn rsplitn_two(&self) -> ([T; 1], [T; 3]) {
        ([self.last], [self.first, self.first_delimiter, self.middle])
    }
}

impl<T: Copy> Evidence for KaniSplitNObservation<T> {
    type Basis = KaniSplitNWindow;
    type Audit = [T; 5];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniSplitNWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        [
            self.first,
            self.first_delimiter,
            self.middle,
            self.second_delimiter,
            self.last,
        ]
    }
}

/// The assumption `KaniChunkByWindow` stands in for: a two-element
/// `chunk_by` witness reduces to deciding whether the adjacent pair stays
/// together or splits into two one-element chunks, without executing the
/// real std iterator path. The direct `ChunkBy` proof still times out under
/// Kani on a fixed two-element array even after stepwise observation, so the
/// bounded grouping shape is named explicitly here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniChunkByWindow;

impl Provenance for KaniChunkByWindow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a two-element chunk_by witness reduces to either one grouped pair or two one-element chunks depending only on whether the adjacent predicate holds",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::slice::ChunkBy path still times out under Kani even on a fixed two-element array with stepwise observation, so the production proof uses a bounded grouping observation instead",
            ),
        ]
        .into_iter()
        })
    }
}

/// Audit payload for a bounded `chunk_by` observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters)]
pub struct KaniChunkByAudit<T: Copy> {
    /// The modeled adjacent pair.
    #[getter(copy)]
    data: [T; 2],
    /// Whether the adjacent pair is modeled as one chunk.
    #[getter(copy)]
    grouped_together: bool,
}

/// Bounded two-element `chunk_by` observation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct KaniChunkByObservation<T: Copy> {
    /// The first element.
    #[getter(copy)]
    first: T,
    /// The second element.
    #[getter(copy)]
    second: T,
    /// Whether the adjacent pair is modeled as one chunk.
    #[getter(copy)]
    grouped_together: bool,
}

impl<T: Copy> KaniChunkByObservation<T> {
    /// Report the first chunk length under the modeled grouping rule.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn first_chunk_len(&self) -> usize {
        if self.grouped_together { 2 } else { 1 }
    }

    /// Report the trailing chunk length, if any.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn trailing_chunk_len(&self) -> Option<usize> {
        if self.grouped_together { None } else { Some(1) }
    }
}

impl<T: Copy> Evidence for KaniChunkByObservation<T> {
    type Basis = KaniChunkByWindow;
    type Audit = KaniChunkByAudit<T>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniChunkByWindow
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        KaniChunkByAudit {
            data: [self.first, self.second],
            grouped_together: self.grouped_together,
        }
    }
}
