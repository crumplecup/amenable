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
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
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
    }
}

/// Bounded one-delimiter split observation for `[before, delimiter, after]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniSplitObservation<T> {
    before: T,
    delimiter: T,
    after: T,
}

impl<T: Copy> KaniSplitObservation<T> {
    /// Construct the bounded `[before, delimiter, after]` witness.
    #[must_use]
    pub fn new(before: T, delimiter: T, after: T) -> Self {
        Self {
            before,
            delimiter,
            after,
        }
    }

    /// Model `split(predicate)` over the bounded witness.
    #[must_use]
    pub fn split(&self) -> ([T; 1], [T; 1]) {
        ([self.before], [self.after])
    }

    /// Model `split_inclusive(predicate)` over the bounded witness.
    #[must_use]
    pub fn split_inclusive(&self) -> ([T; 2], [T; 1]) {
        ([self.before, self.delimiter], [self.after])
    }

    /// Model `rsplit(predicate)` over the bounded witness.
    #[must_use]
    pub fn rsplit(&self) -> ([T; 1], [T; 1]) {
        ([self.after], [self.before])
    }

    /// Overwrite the first non-delimiter element, matching `split_mut`'s first piece.
    pub fn set_before(&mut self, value: T) {
        self.before = value;
    }

    /// Overwrite the last non-delimiter element, matching `rsplit_mut`'s first piece.
    pub fn set_after(&mut self, value: T) {
        self.after = value;
    }

    /// Recover the modeled underlying data after any write-through updates.
    #[must_use]
    pub fn data(&self) -> [T; 3] {
        [self.before, self.delimiter, self.after]
    }
}

impl<T: Copy> Evidence for KaniSplitObservation<T> {
    type Basis = KaniSplitWindow;
    type Audit = [T; 3];

    fn basis() -> Self::Basis {
        KaniSplitWindow
    }

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
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
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
    }
}

/// Bounded two-delimiter split observation for `[first, d1, middle, d2, last]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniSplitNObservation<T> {
    first: T,
    first_delimiter: T,
    middle: T,
    second_delimiter: T,
    last: T,
}

impl<T: Copy> KaniSplitNObservation<T> {
    /// Construct the bounded two-delimiter witness.
    #[must_use]
    pub fn new(first: T, first_delimiter: T, middle: T, second_delimiter: T, last: T) -> Self {
        Self {
            first,
            first_delimiter,
            middle,
            second_delimiter,
            last,
        }
    }

    /// Model `splitn(2, predicate)` over the bounded witness.
    #[must_use]
    pub fn splitn_two(&self) -> ([T; 1], [T; 3]) {
        (
            [self.first],
            [self.middle, self.second_delimiter, self.last],
        )
    }

    /// Model `rsplitn(2, predicate)` over the bounded witness.
    #[must_use]
    pub fn rsplitn_two(&self) -> ([T; 1], [T; 3]) {
        ([self.last], [self.first, self.first_delimiter, self.middle])
    }
}

impl<T: Copy> Evidence for KaniSplitNObservation<T> {
    type Basis = KaniSplitNWindow;
    type Audit = [T; 5];

    fn basis() -> Self::Basis {
        KaniSplitNWindow
    }

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
