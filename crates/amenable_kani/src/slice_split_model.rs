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
