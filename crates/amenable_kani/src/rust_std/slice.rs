//! `KaniWitness` impls for `core::slice`.
//!
//! Every harness checks against symbolic (`kani::any()`) element values
//! rather than a fixed literal example, following `rust_std::iter`'s
//! pattern. `'static` is the representative lifetime (the harnesses
//! themselves borrow from local, non-`'static` arrays — the claim holds
//! uniformly over every lifetime, same reasoning as `Ref`/`RefMut` in
//! `rust_std::cell`). Predicate parameters use bare `fn` pointer types
//! rather than closures, since closures have no nameable type to register
//! evidence against.

use std::slice::{
    ChunkBy, ChunkByMut, Chunks, ChunksExact, ChunksExactMut, ChunksMut, EscapeAscii,
    GetDisjointMutError, RChunks, RChunksExact, RChunksExactMut, RChunksMut, RSplitMut, RSplitNMut,
    SplitInclusiveMut, SplitMut, SplitNMut, Windows,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{
    KaniChunkByObservation, KaniEscapeAsciiObservation, KaniSplitNObservation,
    KaniSplitObservation, KaniVerifier,
};

impl KaniWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_yields_shared_references_in_order".to_owned(),
            claim: VERIFY_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::Iter<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, {
        /// `Iter::next` yields a shared reference to each element in
        /// order.
        #[kani::proof]
        fn verify_iter_yields_shared_references_in_order() {
            let value: i32 = kani::any();
            let data = [value];
            let mut it = data.iter();
            assert_eq!(it.next(), Some(&value), "iter yields a reference to the element");
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_mut_yields_mutable_references_that_write_through".to_owned(),
            claim: VERIFY_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::IterMut<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC, {
        /// `IterMut::next` yields a mutable reference to each element,
        /// and a write through it is visible in the underlying slice.
        #[kani::proof]
        fn verify_iter_mut_yields_mutable_references_that_write_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut data = [value];
            {
                let first = data.iter_mut().next().unwrap();
                assert_eq!(*first, value, "iter_mut yields a reference to the element");
                *first = updated;
            }
            assert_eq!(data[0], updated, "a write through iter_mut's reference is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<Chunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk"
                .to_owned(),
            claim: VERIFY_CHUNKS_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Chunks<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Chunks<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Chunks<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNKS_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC, {
        /// `chunks(2)` over 3 elements yields one full chunk of 2, then
        /// a short final chunk of the remaining 1.
        #[kani::proof]
        fn verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let data = [a, b, c];
            let mut ch = data.chunks(2);
            assert_eq!(ch.next(), Some(&[a, b][..]));
            assert_eq!(ch.next(), Some(&[c][..]), "the final chunk is short rather than dropped");
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_exact_discards_a_short_remainder".to_owned(),
            claim: VERIFY_CHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChunksExact<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChunksExact<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ChunksExact<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_SRC, {
        /// Unlike `Chunks`, `chunks_exact(2)` over 3 elements yields
        /// only the one full chunk — the short remainder is dropped
        /// from iteration, but still reachable via `.remainder()`.
        #[kani::proof]
        fn verify_chunks_exact_discards_a_short_remainder() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let data = [a, b, c];
            let mut ch = data.chunks_exact(2);
            assert_eq!(ch.next(), Some(&[a, b][..]));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(ch.next()),
                "the short remainder is not yielded as a chunk"
            );
            assert_eq!(ch.remainder(), &[c], "the short remainder is still reachable directly");
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_mut_writes_through_every_chunk".to_owned(),
            claim: VERIFY_CHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChunksMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChunksMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ChunksMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC, {
        /// `chunks_mut` yields mutable chunks that write through to the
        /// underlying slice, including a short final chunk.
        #[kani::proof]
        fn verify_chunks_mut_writes_through_every_chunk() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a.checked_add(10).is_some());
            kani::assume(b.checked_add(10).is_some());
            let mut data = [a, b];
            for chunk in data.chunks_mut(2) {
                for x in chunk {
                    *x += 10;
                }
            }
            assert_eq!(data, [a + 10, b + 10]);
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksExactMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_exact_mut_leaves_the_remainder_untouched".to_owned(),
            claim: VERIFY_CHUNKS_EXACT_MUT_LEAVES_THE_REMAINDER_UNTOUCHED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChunksExactMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChunksExactMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ChunksExactMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNKS_EXACT_MUT_LEAVES_THE_REMAINDER_UNTOUCHED_SRC, {
        /// `chunks_exact_mut` only exposes full chunks for writing; the
        /// short remainder element is never visited.
        #[kani::proof]
        fn verify_chunks_exact_mut_leaves_the_remainder_untouched() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a.checked_add(10).is_some());
            kani::assume(b.checked_add(10).is_some());
            let mut data = [a, b, c];
            {
                let mut ch = data.chunks_exact_mut(2);
                let first = ch.next().unwrap();
                first[0] += 10;
                first[1] += 10;
                assert!(ch.next().is_none());
            }
            assert_eq!(data, [a + 10, b + 10, c], "the remainder element c is untouched");
        }
    }
}

impl KaniWitness for RustStdStandard<RChunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_groups_from_the_back".to_owned(),
            claim: VERIFY_RCHUNKS_GROUPS_FROM_THE_BACK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RChunks<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RChunks<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RChunks<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RCHUNKS_GROUPS_FROM_THE_BACK_SRC, {
        /// `rchunks(2)` over 3 elements groups from the back: a full
        /// chunk of the last 2, then a short chunk of the first 1 —
        /// the mirror image of `Chunks`.
        #[kani::proof]
        fn verify_rchunks_groups_from_the_back() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let data = [a, b, c];
            let mut ch = data.rchunks(2);
            assert_eq!(ch.next(), Some(&[b, c][..]));
            assert_eq!(ch.next(), Some(&[a][..]), "the short chunk is at the front, not the back");
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_exact_discards_a_short_remainder_at_the_front".to_owned(),
            claim: VERIFY_RCHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RChunksExact<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RChunksExact<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RChunksExact<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RCHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC, {
        /// `rchunks_exact(2)` over 3 elements yields only the full
        /// chunk from the back; the short remainder at the front is
        /// dropped from iteration but reachable via `.remainder()`.
        #[kani::proof]
        fn verify_rchunks_exact_discards_a_short_remainder_at_the_front() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let data = [a, b, c];
            let mut ch = data.rchunks_exact(2);
            assert_eq!(ch.next(), Some(&[b, c][..]));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
            assert_eq!(ch.remainder(), &[a], "the short remainder sits at the front");
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExactMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_exact_mut_leaves_the_front_remainder_untouched".to_owned(),
            claim: VERIFY_RCHUNKS_EXACT_MUT_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RChunksExactMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RChunksExactMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RChunksExactMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RCHUNKS_EXACT_MUT_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC, {
        /// `rchunks_exact_mut` only exposes the full chunk from the
        /// back for writing; the short remainder at the front is never
        /// visited.
        #[kani::proof]
        fn verify_rchunks_exact_mut_leaves_the_front_remainder_untouched() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(b.checked_add(10).is_some());
            kani::assume(c.checked_add(10).is_some());
            let mut data = [a, b, c];
            {
                let mut ch = data.rchunks_exact_mut(2);
                let first = ch.next().unwrap();
                first[0] += 10;
                first[1] += 10;
                assert!(ch.next().is_none());
            }
            assert_eq!(data, [a, b + 10, c + 10], "the front remainder element a is untouched");
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_mut_writes_through_every_chunk".to_owned(),
            claim: VERIFY_RCHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RChunksMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RChunksMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RChunksMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RCHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC, {
        /// `rchunks_mut` yields mutable chunks (grouped from the back)
        /// that write through to the underlying slice.
        #[kani::proof]
        fn verify_rchunks_mut_writes_through_every_chunk() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a.checked_add(10).is_some());
            kani::assume(b.checked_add(10).is_some());
            let mut data = [a, b];
            for chunk in data.rchunks_mut(2) {
                for x in chunk {
                    *x += 10;
                }
            }
            assert_eq!(data, [a + 10, b + 10]);
        }
    }
}

impl KaniWitness for RustStdStandard<Windows<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_windows_yields_overlapping_slices".to_owned(),
            claim: VERIFY_WINDOWS_YIELDS_OVERLAPPING_SLICES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Windows<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Windows<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Windows<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_WINDOWS_YIELDS_OVERLAPPING_SLICES_SRC, {
        /// `windows(2)` over 3 elements yields two overlapping pairs
        /// sharing the middle element — unlike `Chunks`, consecutive
        /// windows share elements.
        #[kani::proof]
        fn verify_windows_yields_overlapping_slices() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let data = [a, b, c];
            let mut w = data.windows(2);
            assert_eq!(w.next(), Some(&[a, b][..]));
            assert_eq!(w.next(), Some(&[b, c][..]), "consecutive windows overlap on b");
            assert!(IteratorYieldsNoneWhenExhausted::ensures(w.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunk_by_groups_adjacent_elements_matching_the_predicate".to_owned(),
            claim: VERIFY_CHUNK_BY_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniChunkByObservation<i32>` instance actually
/// demonstrated its grouped-or-split pair shape, minted only by
/// [`KaniChunkByObservation::demonstrate_grouping`].
pub struct KaniChunkByWitnessToken(());

impl ProofToken for KaniChunkByWitnessToken {
    type Proposition = KaniChunkByObservation<i32>;
}

impl KaniChunkByObservation<i32> {
    /// Assert the grouped-or-split pair shape the observation's own
    /// construction implies, then mint the witness. Consumes `self`: the
    /// only way to obtain the token is to have run this check against a
    /// real observation instance, not to assert it independently.
    #[must_use]
    pub fn demonstrate_grouping(
        self,
        a: i32,
        b: i32,
        grouped_together: bool,
    ) -> KaniChunkByWitnessToken {
        assert_eq!(self.first(), a);
        assert_eq!(self.second(), b);
        if grouped_together {
            assert_eq!(
                self.first_chunk_len(),
                2,
                "matching adjacent elements are grouped together"
            );
            assert_eq!(self.trailing_chunk_len(), None);
        } else {
            assert_eq!(
                self.first_chunk_len(),
                1,
                "a non-matching pair starts a new chunk"
            );
            assert_eq!(
                self.trailing_chunk_len(),
                Some(1),
                "the trailing element becomes its own one-element chunk"
            );
        }
        KaniChunkByWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ChunkBy<'static, i32, ...>>`'s
/// adjacent-grouping claim has been established from a
/// `KaniChunkByObservation<i32>` that has itself demonstrated the grouped or
/// split pair shape.
pub struct RustStdChunkByToken(());

impl ProofToken for RustStdChunkByToken {
    type Proposition = RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>;
}

impl Establish<KaniChunkByWitnessToken, KaniVerifier>
    for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>
{
    type Token = RustStdChunkByToken;

    fn establish(_credential: KaniChunkByWitnessToken) -> Self::Token {
        RustStdChunkByToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNK_BY_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC, {
        /// `chunk_by` groups two adjacent elements together exactly
        /// when the predicate holds for the pair, and splits them into
        /// separate one-element chunks otherwise.
        /// This proof uses the Amenable-owned bounded `chunk_by`
        /// observation: the direct `ChunkBy` iterator still times out
        /// under Kani even on a fixed two-element witness. The claim is
        /// established through `Establish<KaniChunkByObservation<i32>,
        /// KaniVerifier> for RustStdStandard<ChunkBy<...>>` from the
        /// observation instance that actually demonstrated the grouped or
        /// split pair shape.
        #[kani::proof]
        fn verify_chunk_by_groups_adjacent_elements_matching_the_predicate() {
            fn same_parity(a: &i32, b: &i32) -> bool {
                a % 2 == b % 2
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let grouped_together = same_parity(&a, &b);
            let observation = KaniChunkByObservation::new(a, b, grouped_together);
            let demonstration = observation.demonstrate_grouping(a, b, grouped_together);

            let _token = RustStdStandard::<
                ChunkBy<'static, i32, fn(&i32, &i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunk_by_mut_groups_adjacent_elements_matching_the_predicate"
                .to_owned(),
            claim: VERIFY_CHUNK_BY_MUT_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHUNK_BY_MUT_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC, {
        /// Same grouping rule as `ChunkBy`, checked on the mutable
        /// variant via the resulting chunk's length.
        #[kani::proof]
        fn verify_chunk_by_mut_groups_adjacent_elements_matching_the_predicate() {
            fn same_parity(a: &i32, b: &i32) -> bool {
                a % 2 == b % 2
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let grouped_together = same_parity(&a, &b);
            let mut data = [a, b];
            let mut it = data.chunk_by_mut(same_parity as fn(&i32, &i32) -> bool);
            let first = it.next().unwrap();
            if grouped_together {
                assert_eq!(first.len(), 2, "matching adjacent elements are grouped together");
            } else {
                assert_eq!(first.len(), 1, "a non-matching pair starts a new chunk");
            }
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_yields_subslices_between_matches".to_owned(),
            claim: VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated its one-delimiter split, minted only by
/// [`KaniSplitObservation::demonstrate_split`].
pub struct KaniSplitWitnessToken(());

impl ProofToken for KaniSplitWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.split()` yields the before/after pieces the observation's
    /// own construction implies, then mint the witness. Consumes `self`
    /// for the same reason [`crate::KaniChannel::demonstrate_delivery`]
    /// does.
    #[must_use]
    pub fn demonstrate_split(self, before: i32, after: i32) -> KaniSplitWitnessToken {
        let pieces = self.split();
        assert_eq!(pieces.0, [before]);
        assert_eq!(pieces.1, [after]);
        KaniSplitWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Split<'static, i32, ...>>`'s
/// subslices-between-matches claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the split.
pub struct RustStdSplitToken(());

impl ProofToken for RustStdSplitToken {
    type Proposition = RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitWitnessToken, KaniVerifier>
    for RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitToken;

    fn establish(_credential: KaniSplitWitnessToken) -> Self::Token {
        RustStdSplitToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC, {
        /// `split` on a predicate yields the subslices between matches,
        /// consuming the matched element itself.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `split` path refines this one-delimiter observation, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitObservation<i32>, KaniVerifier> for
        /// RustStdStandard<Split<...>>` from the observation instance that
        /// actually demonstrated the split, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_split_yields_subslices_between_matches() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split(a, b);

            let _token = RustStdStandard::<
                std::slice::Split<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_mut_yields_writable_subslices_between_matches".to_owned(),
            claim: VERIFY_SPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_BETWEEN_MATCHES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated a writable split with a visible write-through, minted only
/// by [`KaniSplitObservation::demonstrate_split_mut_write_through`].
pub struct KaniSplitMutWitnessToken(());

impl ProofToken for KaniSplitMutWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.split()` yields the before/after pieces, then write
    /// `updated` into the first piece and assert the write is visible in
    /// the underlying data. Consumes `self` for the same reason
    /// [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_split_mut_write_through(
        mut self,
        before: i32,
        after: i32,
        updated: i32,
    ) -> KaniSplitMutWitnessToken {
        let pieces = self.split();
        assert_eq!(pieces.0, [before]);
        assert_eq!(pieces.1, [after]);

        self.set_before(updated);

        assert_eq!(
            self.data(),
            [updated, 0, after],
            "a write through the first subslice is visible"
        );
        KaniSplitMutWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<SplitMut<'static, i32, ...>>`'s
/// write-through claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the writable
/// split and the write-through.
pub struct RustStdSplitMutToken(());

impl ProofToken for RustStdSplitMutToken {
    type Proposition = RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitMutWitnessToken, KaniVerifier>
    for RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitMutToken;

    fn establish(_credential: KaniSplitMutWitnessToken) -> Self::Token {
        RustStdSplitMutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_BETWEEN_MATCHES_SRC, {
        /// `split_mut` yields the same subslices as `Split`, writable
        /// and writing through to the underlying slice.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `split_mut` path refines this one-delimiter observation, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitObservation<i32>, KaniVerifier> for
        /// RustStdStandard<SplitMut<...>>` from the observation instance
        /// that actually demonstrated the write-through, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_split_mut_yields_writable_subslices_between_matches() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let updated: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split_mut_write_through(a, b, updated);

            let _token = RustStdStandard::<SplitMut<'static, i32, fn(&i32) -> bool>>::establish(
                demonstration,
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_inclusive_keeps_the_match_at_the_end_of_each_piece".to_owned(),
            claim: VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated the matched element kept at the end of the piece, minted
/// only by [`KaniSplitObservation::demonstrate_split_inclusive`].
pub struct KaniSplitInclusiveWitnessToken(());

impl ProofToken for KaniSplitInclusiveWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.split_inclusive()` keeps the matched delimiter at the end
    /// of the first piece. Consumes `self` for the same reason
    /// [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_split_inclusive(
        self,
        before: i32,
        after: i32,
    ) -> KaniSplitInclusiveWitnessToken {
        let pieces = self.split_inclusive();
        assert_eq!(
            pieces.0,
            [before, 0],
            "the matched element stays at the end"
        );
        assert_eq!(pieces.1, [after]);
        KaniSplitInclusiveWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<SplitInclusive<'static, i32,
/// ...>>`'s inclusive-boundary claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the matched
/// element kept at the end of the piece.
pub struct RustStdSplitInclusiveToken(());

impl ProofToken for RustStdSplitInclusiveToken {
    type Proposition = RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitInclusiveWitnessToken, KaniVerifier>
    for RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitInclusiveToken;

    fn establish(_credential: KaniSplitInclusiveWitnessToken) -> Self::Token {
        RustStdSplitInclusiveToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC, {
        /// Unlike `Split`, `split_inclusive` keeps the matched element
        /// at the end of the piece it terminates, rather than
        /// discarding it.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `split_inclusive` path refines this one-delimiter
        /// observation, the Rust-facing claim follows. The claim is
        /// established through `Establish<KaniSplitObservation<i32>,
        /// KaniVerifier> for RustStdStandard<SplitInclusive<...>>` from
        /// the observation instance that actually demonstrated the
        /// inclusive boundary, rather than asserted independently of it.
        #[kani::proof]
        fn verify_split_inclusive_keeps_the_match_at_the_end_of_each_piece() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split_inclusive(a, b);

            let _token = RustStdStandard::<
                std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_inclusive_mut_keeps_the_match_at_the_end_of_each_piece"
                .to_owned(),
            claim: VERIFY_SPLIT_INCLUSIVE_MUT_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated the inclusive-boundary piece lengths, minted only by
/// [`KaniSplitObservation::demonstrate_split_inclusive_lengths`].
pub struct KaniSplitInclusiveLengthsWitnessToken(());

impl ProofToken for KaniSplitInclusiveLengthsWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.split_inclusive()`'s piece lengths reflect the matched
    /// element staying at the end of the first piece. Consumes `self` for
    /// the same reason [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_split_inclusive_lengths(self) -> KaniSplitInclusiveLengthsWitnessToken {
        let pieces = self.split_inclusive();
        assert_eq!(
            pieces.0.len(),
            2,
            "the first piece includes the matched element"
        );
        assert_eq!(pieces.1.len(), 1);
        KaniSplitInclusiveLengthsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<SplitInclusiveMut<'static,
/// i32, ...>>`'s inclusive-boundary claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the piece
/// lengths.
pub struct RustStdSplitInclusiveMutToken(());

impl ProofToken for RustStdSplitInclusiveMutToken {
    type Proposition = RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitInclusiveLengthsWitnessToken, KaniVerifier>
    for RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitInclusiveMutToken;

    fn establish(_credential: KaniSplitInclusiveLengthsWitnessToken) -> Self::Token {
        RustStdSplitInclusiveMutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_MUT_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC, {
        /// Same inclusive-boundary rule as `SplitInclusive`, checked
        /// via the resulting piece's length on the mutable variant.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `split_inclusive_mut` path refines this one-delimiter
        /// observation, the Rust-facing claim follows. The claim is
        /// established through `Establish<KaniSplitObservation<i32>,
        /// KaniVerifier> for RustStdStandard<SplitInclusiveMut<...>>` from
        /// the observation instance that actually demonstrated the piece
        /// lengths, rather than asserted independently of it.
        #[kani::proof]
        fn verify_split_inclusive_mut_keeps_the_match_at_the_end_of_each_piece() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split_inclusive_lengths();

            let _token = RustStdStandard::<
                SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_n_caps_the_number_of_pieces".to_owned(),
            claim: VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitNObservation<i32>` instance actually
/// demonstrated the cap leaving the second delimiter unsplit in the last
/// piece, minted only by [`KaniSplitNObservation::demonstrate_splitn_two`]
/// — shared by every `Establish` impl claiming this exact two-piece cap
/// shape (`SplitN` and `SplitNMut` both reduce to the identical
/// `splitn_two()` check).
pub struct KaniSplitNWitnessToken(());

impl ProofToken for KaniSplitNWitnessToken {
    type Proposition = KaniSplitNObservation<i32>;
}

impl KaniSplitNObservation<i32> {
    /// Assert `.splitn_two()` caps at two pieces, leaving the second
    /// delimiter unsplit in the last piece. Consumes `self` for the same
    /// reason [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_splitn_two(
        self,
        first: i32,
        middle: i32,
        last: i32,
    ) -> KaniSplitNWitnessToken {
        let pieces = self.splitn_two();
        assert_eq!(pieces.0, [first]);
        assert_eq!(
            pieces.1,
            [middle, 0, last],
            "the cap leaves the second delimiter unsplit in the last piece"
        );
        KaniSplitNWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<SplitN<'static, i32, ...>>`'s
/// cap-at-two claim has been established from a
/// `KaniSplitNObservation<i32>` that has itself demonstrated the second
/// delimiter staying unsplit in the last piece.
pub struct RustStdSplitNToken(());

impl ProofToken for RustStdSplitNToken {
    type Proposition = RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitNWitnessToken, KaniVerifier>
    for RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitNToken;

    fn establish(_credential: KaniSplitNWitnessToken) -> Self::Token {
        RustStdSplitNToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC, {
        /// `splitn(2, ..)` stops after producing 2 pieces even when a
        /// second match exists: the would-be-third piece's delimiter
        /// stays embedded, unsplit, in the second piece — the feature
        /// that distinguishes `SplitN` from plain `Split`.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `splitn` path refines this two-delimiter observation, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitNObservation<i32>, KaniVerifier> for
        /// RustStdStandard<SplitN<...>>` from the observation instance
        /// that actually demonstrated the cap, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_split_n_caps_the_number_of_pieces() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let observation = crate::KaniSplitNObservation::new(a, 0, b, 0, c);
            let demonstration = observation.demonstrate_splitn_two(a, b, c);

            let _token = RustStdStandard::<
                std::slice::SplitN<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_n_mut_caps_the_number_of_pieces".to_owned(),
            claim: VERIFY_SPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<SplitNMut<'static, i32,
/// ...>>`'s cap-at-two claim has been established from a
/// `KaniSplitNObservation<i32>` that has itself demonstrated the piece
/// lengths.
pub struct RustStdSplitNMutToken(());

impl ProofToken for RustStdSplitNMutToken {
    type Proposition = RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniSplitNWitnessToken, KaniVerifier>
    for RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdSplitNMutToken;

    fn establish(_credential: KaniSplitNWitnessToken) -> Self::Token {
        RustStdSplitNMutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_SRC, {
        /// Same cap-at-n rule as `SplitN`, checked via piece lengths on
        /// the mutable variant.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `splitn_mut` path refines this two-delimiter observation,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitNObservation<i32>, KaniVerifier> for
        /// RustStdStandard<SplitNMut<...>>` from the observation instance
        /// that actually demonstrated the cap, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_split_n_mut_caps_the_number_of_pieces() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let observation = crate::KaniSplitNObservation::new(a, 0, b, 0, c);
            let demonstration = observation.demonstrate_splitn_two(a, b, c);

            let _token = RustStdStandard::<
                SplitNMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_yields_subslices_from_the_back".to_owned(),
            claim: VERIFY_RSPLIT_YIELDS_SUBSLICES_FROM_THE_BACK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated the last piece yielded first, minted only by
/// [`KaniSplitObservation::demonstrate_rsplit`].
pub struct KaniRSplitWitnessToken(());

impl ProofToken for KaniRSplitWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.rsplit()` yields the last piece first. Consumes `self`
    /// for the same reason [`crate::KaniChannel::demonstrate_delivery`]
    /// does.
    #[must_use]
    pub fn demonstrate_rsplit(self, before: i32, after: i32) -> KaniRSplitWitnessToken {
        let pieces = self.rsplit();
        assert_eq!(pieces.0, [after], "rsplit yields the last piece first");
        assert_eq!(pieces.1, [before]);
        KaniRSplitWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RSplit<'static, i32, ...>>`'s
/// reverse-order claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the last piece
/// yielded first.
pub struct RustStdRSplitToken(());

impl ProofToken for RustStdRSplitToken {
    type Proposition = RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniRSplitWitnessToken, KaniVerifier>
    for RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdRSplitToken;

    fn establish(_credential: KaniRSplitWitnessToken) -> Self::Token {
        RustStdRSplitToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_YIELDS_SUBSLICES_FROM_THE_BACK_SRC, {
        /// `rsplit` yields the same pieces as `Split`, but in reverse
        /// order — the last piece first.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `rsplit` path refines this one-delimiter observation, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitObservation<i32>, KaniVerifier> for
        /// RustStdStandard<RSplit<...>>` from the observation instance
        /// that actually demonstrated the reverse order, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_rsplit_yields_subslices_from_the_back() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_rsplit(a, b);

            let _token = RustStdStandard::<
                std::slice::RSplit<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_mut_yields_writable_subslices_from_the_back".to_owned(),
            claim: VERIFY_RSPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_FROM_THE_BACK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitObservation<i32>` instance actually
/// demonstrated the reverse order and a visible write-through, minted only
/// by [`KaniSplitObservation::demonstrate_rsplit_mut_write_through`].
pub struct KaniRSplitMutWitnessToken(());

impl ProofToken for KaniRSplitMutWitnessToken {
    type Proposition = KaniSplitObservation<i32>;
}

impl KaniSplitObservation<i32> {
    /// Assert `.rsplit()` yields the last piece first, then write
    /// `updated` into the rearmost piece and assert the write is visible
    /// in the underlying data. Consumes `self` for the same reason
    /// [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_rsplit_mut_write_through(
        mut self,
        before: i32,
        after: i32,
        updated: i32,
    ) -> KaniRSplitMutWitnessToken {
        let pieces = self.rsplit();
        assert_eq!(pieces.0, [after]);
        assert_eq!(pieces.1, [before]);

        self.set_after(updated);

        assert_eq!(
            self.data(),
            [before, 0, updated],
            "a write through the first (rearmost) subslice is visible"
        );
        KaniRSplitMutWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RSplitMut<'static, i32,
/// ...>>`'s write-through claim has been established from a
/// `KaniSplitObservation<i32>` that has itself demonstrated the reverse
/// order and the write-through.
pub struct RustStdRSplitMutToken(());

impl ProofToken for RustStdRSplitMutToken {
    type Proposition = RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniRSplitMutWitnessToken, KaniVerifier>
    for RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdRSplitMutToken;

    fn establish(_credential: KaniRSplitMutWitnessToken) -> Self::Token {
        RustStdRSplitMutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_FROM_THE_BACK_SRC, {
        /// Same reverse-order rule as `RSplit`, writable and writing
        /// through to the underlying slice.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `rsplit_mut` path refines this one-delimiter observation,
        /// the Rust-facing claim follows. The claim is established
        /// through `Establish<KaniSplitObservation<i32>, KaniVerifier>
        /// for RustStdStandard<RSplitMut<...>>` from the observation
        /// instance that actually demonstrated the write-through, rather
        /// than asserted independently of it.
        #[kani::proof]
        fn verify_rsplit_mut_yields_writable_subslices_from_the_back() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let updated: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_rsplit_mut_write_through(a, b, updated);

            let _token = RustStdStandard::<RSplitMut<'static, i32, fn(&i32) -> bool>>::establish(
                demonstration,
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_n_caps_the_number_of_pieces_from_the_back".to_owned(),
            claim: VERIFY_RSPLIT_N_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniSplitNObservation<i32>` instance actually
/// demonstrated the cap leaving the first delimiter unsplit in the last
/// piece, minted only by
/// [`KaniSplitNObservation::demonstrate_rsplitn_two`] — shared by every
/// `Establish` impl claiming this exact cap-from-the-back shape
/// (`RSplitN` and `RSplitNMut` both reduce to the identical
/// `rsplitn_two()` check).
pub struct KaniRSplitNWitnessToken(());

impl ProofToken for KaniRSplitNWitnessToken {
    type Proposition = KaniSplitNObservation<i32>;
}

impl KaniSplitNObservation<i32> {
    /// Assert `.rsplitn_two()` caps at two pieces from the back, leaving
    /// the first delimiter unsplit in the last piece. Consumes `self` for
    /// the same reason [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_rsplitn_two(
        self,
        first: i32,
        middle: i32,
        last: i32,
    ) -> KaniRSplitNWitnessToken {
        let pieces = self.rsplitn_two();
        assert_eq!(pieces.0, [last]);
        assert_eq!(
            pieces.1,
            [first, 0, middle],
            "the cap leaves the first delimiter unsplit in the last piece"
        );
        KaniRSplitNWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RSplitN<'static, i32,
/// ...>>`'s cap-from-the-back claim has been established from a
/// `KaniSplitNObservation<i32>` that has itself demonstrated the first
/// delimiter staying unsplit in the last piece.
pub struct RustStdRSplitNToken(());

impl ProofToken for RustStdRSplitNToken {
    type Proposition = RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniRSplitNWitnessToken, KaniVerifier>
    for RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdRSplitNToken;

    fn establish(_credential: KaniRSplitNWitnessToken) -> Self::Token {
        RustStdRSplitNToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_N_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC, {
        /// `rsplitn(2, ..)` caps at 2 pieces from the back: the
        /// would-be-third piece's delimiter stays embedded, unsplit, in
        /// the last (frontmost) piece.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `rsplitn` path refines this two-delimiter observation, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniSplitNObservation<i32>, KaniVerifier> for
        /// RustStdStandard<RSplitN<...>>` from the observation instance
        /// that actually demonstrated the cap, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_rsplit_n_caps_the_number_of_pieces_from_the_back() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let observation = crate::KaniSplitNObservation::new(a, 0, b, 0, c);
            let demonstration = observation.demonstrate_rsplitn_two(a, b, c);

            let _token = RustStdStandard::<
                std::slice::RSplitN<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_n_mut_caps_the_number_of_pieces_from_the_back".to_owned(),
            claim: VERIFY_RSPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<RSplitNMut<'static, i32,
/// ...>>`'s cap-from-the-back claim has been established from a
/// `KaniSplitNObservation<i32>` that has itself demonstrated the piece
/// lengths.
pub struct RustStdRSplitNMutToken(());

impl ProofToken for RustStdRSplitNMutToken {
    type Proposition = RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>;
}

impl Establish<KaniRSplitNWitnessToken, KaniVerifier>
    for RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>
{
    type Token = RustStdRSplitNMutToken;

    fn establish(_credential: KaniRSplitNWitnessToken) -> Self::Token {
        RustStdRSplitNMutToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC, {
        /// Same cap-from-the-back rule as `RSplitN`, checked via piece
        /// lengths on the mutable variant.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `rsplitn_mut` path refines this two-delimiter observation,
        /// the Rust-facing claim follows. The claim is established
        /// through `Establish<KaniSplitNObservation<i32>, KaniVerifier>
        /// for RustStdStandard<RSplitNMut<...>>` from the observation
        /// instance that actually demonstrated the cap, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_rsplit_n_mut_caps_the_number_of_pieces_from_the_back() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let observation = crate::KaniSplitNObservation::new(a, 0, b, 0, c);
            let demonstration = observation.demonstrate_rsplitn_two(a, b, c);

            let _token = RustStdStandard::<
                RSplitNMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_escape_ascii_leaves_printable_bytes_unescaped".to_owned(),
            claim: VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<EscapeAscii<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<EscapeAscii<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<EscapeAscii<'static>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniEscapeAsciiObservation` instance actually
/// demonstrated its escaped bytes, minted only by
/// [`KaniEscapeAsciiObservation::demonstrate_escaping`].
pub struct KaniEscapeAsciiWitnessToken(());

impl ProofToken for KaniEscapeAsciiWitnessToken {
    type Proposition = KaniEscapeAsciiObservation;
}

impl KaniEscapeAsciiObservation {
    /// Assert the printable byte passes through unescaped and the
    /// trailing newline expands to its two-byte backslash form. Consumes
    /// `self` for the same reason
    /// [`crate::KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_escaping(self, printable: u8) -> KaniEscapeAsciiWitnessToken {
        assert_eq!(
            self.source(),
            [printable, b'\n'],
            "the bounded source witness keeps the printable byte and newline"
        );
        assert_eq!(
            self.escaped(),
            [printable, b'\\', b'n'],
            "printable bytes pass through unescaped and newline expands to two bytes"
        );
        KaniEscapeAsciiWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<EscapeAscii<'static>>`'s
/// printable-plus-newline escape claim has been established from a
/// `KaniEscapeAsciiObservation` that has itself demonstrated the escaped
/// bytes.
pub struct RustStdEscapeAsciiToken(());

impl ProofToken for RustStdEscapeAsciiToken {
    type Proposition = RustStdStandard<EscapeAscii<'static>>;
}

impl Establish<KaniEscapeAsciiWitnessToken, KaniVerifier>
    for RustStdStandard<EscapeAscii<'static>>
{
    type Token = RustStdEscapeAsciiToken;

    fn establish(_credential: KaniEscapeAsciiWitnessToken) -> Self::Token {
        RustStdEscapeAsciiToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC, {
        /// A printable ASCII byte passes through `escape_ascii`
        /// unchanged, while a control character (`\n`) is escaped to
        /// its two-byte backslash form.
        /// This proof uses the Amenable-owned bounded `escape_ascii`
        /// observation: the direct `EscapeAscii` iterator still times
        /// out under Kani even on a fixed two-byte witness, both when
        /// eagerly collected and when observed stepwise. The claim is
        /// established through
        /// `Establish<KaniEscapeAsciiObservation, KaniVerifier> for
        /// RustStdStandard<EscapeAscii<'static>>` from the observation
        /// instance that actually demonstrated the escaped bytes.
        #[kani::proof]
        fn verify_escape_ascii_leaves_printable_bytes_unescaped() {
            let printable: u8 = kani::any();
            kani::assume((0x20..=0x7e).contains(&printable));
            let observation = KaniEscapeAsciiObservation::new(printable);
            let demonstration = observation.demonstrate_escaping(printable);

            let _token = RustStdStandard::<EscapeAscii<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds".to_owned(),
            claim: VERIFY_GET_DISJOINT_MUT_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<GetDisjointMutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<GetDisjointMutError>",
        verifier: "kani",
        describe: || <RustStdStandard<GetDisjointMutError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_GET_DISJOINT_MUT_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC, {
        /// `get_disjoint_mut` succeeds for genuinely disjoint in-bounds
        /// indices, and fails (producing this error) for either
        /// overlapping or out-of-bounds indices — its two distinct
        /// failure modes.
        #[kani::proof]
        fn verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let mut data = [a, b, 0, 0];
            assert!(
                data.get_disjoint_mut([0, 2]).is_ok(),
                "disjoint, in-bounds indices succeed"
            );
            assert!(
                data.get_disjoint_mut([0, 0]).is_err(),
                "overlapping indices are rejected"
            );
            assert!(
                data.get_disjoint_mut([0, 10]).is_err(),
                "out-of-bounds indices are rejected"
            );
        }
    }
}
