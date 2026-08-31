//! Findings that borrowed-slice iterator families (`chunks`, reverse chunks,
//! predicate-driven `chunk_by`/`split`) have no Creusot iterator contracts to
//! lean on.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::borrowed_slice_chunk_iterators_lack_creusot_contracts".to_owned(),
            "borrowed slice chunk/window iterators still need trusted boundaries because creusot-std lacks the direct contracts".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<Chunks<'static, i32>> and the
// related ChunksExact/ChunksMut/ChunksExactMut/Windows carriers):
#[ensures(match result {
    (first_chunk, second_chunk, exhausted) =>
        first_chunk == Some((a, b))
            && second_chunk == Some(c)
            && exhausted,
})]
fn verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk(
    a: i32,
    b: i32,
    c: i32,
) -> (Option<(i32, i32)>, Option<i32>, bool) {
    let data = [a, b, c];
    let mut chunks = data.chunks(2);
    let first_chunk = match chunks.next() {
        Some(chunk) => match chunk {
            [first, second] => Some((*first, *second)),
            _ => None,
        },
        None => None,
    };
    let second_chunk = match chunks.next() {
        Some(chunk) => match chunk {
            [only] => Some(*only),
            _ => None,
        },
        None => None,
    };
    let exhausted = match chunks.next() {
        Some(_) => false,
        None => true,
    };
    (first_chunk, second_chunk, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `chunks` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::Chunks<'_, i32>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The direct route fails before the law can be discharged: `creusot-std`
// does not currently provide the `IteratorSpec`/`next` contract coverage
// this borrowed carrier family needs. While reducing the repro, matching
// yielded slices with patterns like `[first, second]` also triggered a
// separate creusot-rustc ICE:
//   error: internal compiler error: Unsupported projection
//   ConstantIndex { offset: 0, min_length: 2, from_end: false }
// So this is not a one-harness typo or a bad postcondition shape; the
// family is blocked on real translator/library gaps.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<Chunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for ChunksExact, ChunksMut,
// ChunksExactMut, and Windows until creusot-std grows the missing
// contracts and creusot-rustc stops ICE-ing on the slice-pattern route.
"#,
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::borrowed_slice_reverse_chunk_iterators_lack_creusot_contracts".to_owned(),
            "borrowed reverse chunk iterators still need trusted boundaries because creusot-std lacks the direct contracts".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<RChunks<'static, i32>> and the
// related RChunksExact/RChunksExactMut/RChunksMut carriers):
#[ensures(match result {
    (first_len, first_second_last, second_len, second_first, exhausted) =>
        first_len == 2usize
            && first_second_last == Some((b, c))
            && second_len == 1usize
            && second_first == Some(a)
            && exhausted,
})]
fn verify_rchunks_groups_from_the_back(
    a: i32,
    b: i32,
    c: i32,
) -> (usize, Option<(i32, i32)>, usize, Option<i32>, bool) {
    let data = [a, b, c];
    let mut it = data.rchunks(2);
    let (first_len, first_second_last) = match it.next() {
        Some(chunk) => {
            let pair = if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            };
            (chunk.len(), pair)
        }
        None => (0usize, None),
    };
    let (second_len, second_first) = match it.next() {
        Some(chunk) => {
            let first = if chunk.len() == 1 {
                Some(chunk[0])
            } else {
                None
            };
            (chunk.len(), first)
        }
        None => (0usize, None),
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (
        first_len,
        first_second_last,
        second_len,
        second_first,
        exhausted,
    )
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `rchunks` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::RChunks<'_, i32>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same pattern repeated for `RChunksExact`, `RChunksExactMut`, and
// `RChunksMut`, with companion contractless-external warnings on
// `rchunks_exact`, `rchunks_exact_mut`, `rchunks_mut`, `remainder`, and
// `into_remainder`. This is a real `creusot-std` contract gap for the
// reverse borrowed chunk family, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<RChunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for RChunksExact, RChunksExactMut, and
// RChunksMut until creusot-std grows the missing contracts.
"#,
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::slice_predicate_iterators_lack_creusot_iterator_contracts".to_owned(),
            "slice predicate carriers still need trusted boundaries because creusot-std lacks both method contracts and IteratorSpec for them".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (representative probes run while adding direct Creusot
// coverage for the remaining predicate-driven slice carriers in
// amenable_std::rust_std):
#[ensures(match result {
    (first_len, second_len, exhausted) =>
        first_len >= 1usize && second_len <= 1usize && exhausted,
})]
fn verify_slice_chunk_by_groups_adjacent_equality(a: i32, b: i32) -> (usize, usize, bool) {
    let data = [a, b];
    let mut it = data.chunk_by(|left, right| *left == *right);
    let first_len = match it.next() {
        Some(chunk) => chunk.len(),
        None => 0usize,
    };
    let second_len = match it.next() {
        Some(chunk) => chunk.len(),
        None => 0usize,
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first_len, second_len, exhausted)
}

#[ensures(match result {
    (first_len, second_len, exhausted) =>
        first_len == 1usize && second_len == 1usize && exhausted,
})]
fn verify_slice_split_separates_on_zero(a: i32, b: i32) -> (usize, usize, bool) {
    let data = [a, 0, b];
    let mut it = data.split(|value| *value == 0);
    let first_len = match it.next() {
        Some(piece) => piece.len(),
        None => 0usize,
    };
    let second_len = match it.next() {
        Some(piece) => piece.len(),
        None => 0usize,
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first_len, second_len, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `chunk_by` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `ChunkBy<'_, i32, {closure@...}>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and likewise:
//   warning: calling external function `split` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `Split<'_, i32, {closure@...}>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same boundary applies to the rest of the predicate-driven slice
// family: `ChunkByMut`, `RSplit`, `RSplitMut`, `RSplitN`, `RSplitNMut`,
// `SplitInclusive`, `SplitInclusiveMut`, `SplitMut`, `SplitN`, and
// `SplitNMut`. This is a real `creusot-std` contract gap for these
// carriers, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable laws
// through direct proofs or accommodation models as appropriate.
impl CreusotWitness for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for ChunkByMut, RSplit, RSplitMut,
// RSplitN, RSplitNMut, Split, SplitInclusive, SplitInclusiveMut, SplitMut,
// SplitN, and SplitNMut until creusot-std grows the missing contracts.
"#,
        ),
    )
}
