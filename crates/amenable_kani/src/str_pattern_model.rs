//! Kani-only accommodation model for `core::str`'s `char`-`Pattern` search
//! family: `RSplit`, `RSplitN`, `SplitTerminator`, `RSplitTerminator`,
//! `Matches`, `RMatches`, `MatchIndices`, `RMatchIndices`.
//!
//! The direct path for all 8 remains preserved in the proof gallery as a
//! timeout boundary (`gallery::replace_recommendations`'s
//! `str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`
//! and `str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`).
//! This module captures the bounded observable laws the production proofs
//! actually claim, mirroring `slice_split_model`'s Window/Observation
//! shape for `core::str`:
//!
//! - a one-occurrence `[before, pattern, after]` witness for `rsplit`
//! - a two-occurrence `[a, pattern, b, pattern, c]` witness (capped at two
//!   pieces) for `rsplitn`
//! - a two-occurrence, nothing-after-the-last-match `[a, pattern, b,
//!   pattern]` witness for `split_terminator`/`rsplit_terminator`
//! - a two-occurrence `[f0, pattern, f1, pattern, f2]` witness for
//!   `matches`/`rmatches`/`match_indices`/`rmatch_indices`
//!
//! Every field is a single symbolic ASCII `char` (never a multi-character
//! `&str`): the position/order laws these 8 types are missing a proof for
//! don't depend on multi-character content, only on where the pattern
//! falls relative to its neighbors. Production proofs using this model are
//! therefore conditional, exactly as `slice_split_model`'s are: if the
//! real `core::str` path refines these modeled laws on the same bounded
//! layouts, then the modeled Kani proof carries the intended Rust-facing
//! claim.

use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// The assumption `KaniStrRSplitWindow` stands in for: a one-occurrence
/// `rsplit` reduces to a fixed `[before, pattern, after]` window reversed
/// to `[after, before]`, and nothing else about the real `CharSearcher`
/// backward search `str::RSplit` performs internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniStrRSplitWindow;

impl Provenance for KaniStrRSplitWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a one-occurrence rsplit reduces to a fixed [before, pattern, after] window, reversed to [after, before], standing in for str::RSplit's internal CharSearcher::next_match_back search",
            ),
            MetadataEntry::new(
                "rationale",
                "CharSearcher::next_match_back bottoms out in memchr::memrchr, whose internal scan loop CBMC can't bound even for a single .next() call on a five-byte fixed str -- see gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call",
            ),
        ]
        .into_iter()
    }
}

/// Bounded one-occurrence `rsplit` observation for `[before, pattern, after]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniStrRSplitObservation {
    before: char,
    pattern: char,
    after: char,
}

impl KaniStrRSplitObservation {
    /// Construct the bounded `[before, pattern, after]` witness.
    #[must_use]
    pub fn new(before: char, pattern: char, after: char) -> Self {
        Self {
            before,
            pattern,
            after,
        }
    }

    /// Model `rsplit(pattern)` over the bounded witness: the piece after
    /// the match comes first.
    #[must_use]
    pub fn rsplit(&self) -> [char; 2] {
        [self.after, self.before]
    }

    /// Recover the modeled underlying data.
    #[must_use]
    pub fn data(&self) -> [char; 3] {
        [self.before, self.pattern, self.after]
    }
}

impl Evidence for KaniStrRSplitObservation {
    type Basis = KaniStrRSplitWindow;
    type Audit = [char; 3];

    fn basis() -> Self::Basis {
        KaniStrRSplitWindow
    }

    fn audit(&self) -> Self::Audit {
        self.data()
    }
}

/// The assumption `KaniStrRSplitNWindow` stands in for: a two-occurrence
/// `rsplitn(2, pattern)` reduces to a fixed `[a, pattern, b, pattern, c]`
/// window capped at two pieces (`c`, then `a`+`pattern`+`b` uncut), and
/// nothing else about the real, symbolic-length reverse search
/// `str::RSplitN` performs internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniStrRSplitNWindow;

impl Provenance for KaniStrRSplitNWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a two-occurrence rsplitn(2, pattern) reduces to a fixed [a, pattern, b, pattern, c] window capped at two pieces, standing in for str::RSplitN's internal reverse CharSearcher search",
            ),
            MetadataEntry::new(
                "rationale",
                "the same memchr::memrchr scan-loop cost documented for KaniStrRSplitWindow applies here too -- see gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call",
            ),
        ]
        .into_iter()
    }
}

/// Bounded two-occurrence `rsplitn` observation for `[a, pattern, b,
/// pattern, c]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniStrRSplitNObservation {
    a: char,
    pattern: char,
    b: char,
    c: char,
}

impl KaniStrRSplitNObservation {
    /// Construct the bounded two-occurrence witness. Both occurrences of
    /// the pattern share the same `pattern` value, matching a real
    /// `char`-`Pattern` search on a str that contains it exactly twice.
    #[must_use]
    pub fn new(a: char, pattern: char, b: char, c: char) -> Self {
        Self { a, pattern, b, c }
    }

    /// Model `rsplitn(2, pattern)` over the bounded witness: the piece
    /// after the last match comes first, then everything before it, uncut.
    #[must_use]
    pub fn rsplitn_two(&self) -> (char, [char; 3]) {
        (self.c, [self.a, self.pattern, self.b])
    }
}

impl Evidence for KaniStrRSplitNObservation {
    type Basis = KaniStrRSplitNWindow;
    type Audit = [char; 4];

    fn basis() -> Self::Basis {
        KaniStrRSplitNWindow
    }

    fn audit(&self) -> Self::Audit {
        [self.a, self.pattern, self.b, self.c]
    }
}

/// The assumption `KaniStrSplitTerminatorWindow` stands in for: a
/// two-occurrence `split_terminator`/`rsplit_terminator` reduces to a
/// fixed `[a, pattern, b, pattern]` window with nothing after the final
/// match (so the trailing empty piece a plain `split` would yield is
/// structurally absent, not merely dropped), and nothing else about the
/// real `str::SplitTerminator`/`RSplitTerminator` internals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniStrSplitTerminatorWindow;

impl Provenance for KaniStrSplitTerminatorWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a two-occurrence split_terminator/rsplit_terminator reduces to a fixed [a, pattern, b, pattern] window with nothing after the final match, standing in for str::SplitTerminator's/RSplitTerminator's internal search",
            ),
            MetadataEntry::new(
                "rationale",
                "the forward direction reaches an unexplained real-crate-only timeout (passes in an isolated probe crate, still times out inside amenable_kani) and the reverse direction hits the same memchr::memrchr scan-loop cost as KaniStrRSplitWindow -- see gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate and str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call",
            ),
        ]
        .into_iter()
    }
}

/// Bounded two-occurrence, nothing-trailing `split_terminator` observation
/// for `[a, pattern, b, pattern]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniStrSplitTerminatorObservation {
    a: char,
    pattern: char,
    b: char,
}

impl KaniStrSplitTerminatorObservation {
    /// Construct the bounded `[a, pattern, b, pattern]` witness (nothing
    /// after the second, terminal match).
    #[must_use]
    pub fn new(a: char, pattern: char, b: char) -> Self {
        Self { a, pattern, b }
    }

    /// Model `split_terminator(pattern)` over the bounded witness: no
    /// trailing empty piece after the terminal match.
    #[must_use]
    pub fn split_terminator(&self) -> [char; 2] {
        [self.a, self.b]
    }

    /// Model `rsplit_terminator(pattern)` over the bounded witness: same
    /// suppression, traversed from the back.
    #[must_use]
    pub fn rsplit_terminator(&self) -> [char; 2] {
        [self.b, self.a]
    }
}

impl Evidence for KaniStrSplitTerminatorObservation {
    type Basis = KaniStrSplitTerminatorWindow;
    type Audit = [char; 3];

    fn basis() -> Self::Basis {
        KaniStrSplitTerminatorWindow
    }

    fn audit(&self) -> Self::Audit {
        [self.a, self.pattern, self.b]
    }
}

/// The assumption `KaniStrMatchWindow` stands in for: a two-occurrence
/// `matches`/`rmatches`/`match_indices`/`rmatch_indices` reduces to a
/// fixed `[f0, pattern, f1, pattern, f2]` window (matches at byte offsets
/// 1 and 3, since every field is exactly one ASCII byte), and nothing
/// else about the real, symbolic-length `MatchIndicesInternal` search
/// `str::Matches`/`RMatches`/`MatchIndices`/`RMatchIndices` perform
/// internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniStrMatchWindow;

impl Provenance for KaniStrMatchWindow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "a two-occurrence matches/rmatches/match_indices/rmatch_indices reduces to a fixed [f0, pattern, f1, pattern, f2] window with matches at byte offsets 1 and 3, standing in for str::MatchIndicesInternal's internal search",
            ),
            MetadataEntry::new(
                "rationale",
                "the forward direction (Matches/MatchIndices) reaches an unexplained real-crate-only timeout and the reverse direction (RMatches/RMatchIndices) hits the same memchr::memrchr scan-loop cost as KaniStrRSplitWindow -- see gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate and str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call",
            ),
        ]
        .into_iter()
    }
}

/// Bounded two-occurrence `matches`/`match_indices` observation for
/// `[f0, pattern, f1, pattern, f2]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniStrMatchObservation {
    f0: char,
    pattern: char,
    f1: char,
    f2: char,
}

impl KaniStrMatchObservation {
    /// Construct the bounded `[f0, pattern, f1, pattern, f2]` witness.
    #[must_use]
    pub fn new(f0: char, pattern: char, f1: char, f2: char) -> Self {
        Self {
            f0,
            pattern,
            f1,
            f2,
        }
    }

    /// Model `matches(pattern)` over the bounded witness.
    #[must_use]
    pub fn matches(&self) -> [char; 2] {
        [self.pattern, self.pattern]
    }

    /// Model `rmatches(pattern)` over the bounded witness: same content,
    /// content alone can't distinguish traversal order for a `char`
    /// pattern (see `amenable_kani::rust_std::str`'s module doc).
    #[must_use]
    pub fn rmatches(&self) -> [char; 2] {
        [self.pattern, self.pattern]
    }

    /// Model `match_indices(pattern)` over the bounded witness: byte
    /// offsets 1 and 3, forward order.
    #[must_use]
    pub fn match_indices(&self) -> [(usize, char); 2] {
        [(1, self.pattern), (3, self.pattern)]
    }

    /// Model `rmatch_indices(pattern)` over the bounded witness: the same
    /// pairs, reverse order -- this is the one place order is directly
    /// assertable, since the byte offset differs per occurrence even
    /// though the matched char doesn't.
    #[must_use]
    pub fn rmatch_indices(&self) -> [(usize, char); 2] {
        [(3, self.pattern), (1, self.pattern)]
    }
}

impl Evidence for KaniStrMatchObservation {
    type Basis = KaniStrMatchWindow;
    type Audit = [char; 4];

    fn basis() -> Self::Basis {
        KaniStrMatchWindow
    }

    fn audit(&self) -> Self::Audit {
        [self.f0, self.pattern, self.f1, self.f2]
    }
}
