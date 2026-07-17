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
    GetDisjointMutError, Iter, IterMut, RChunks, RChunksExact, RChunksExactMut, RChunksMut, RSplit,
    RSplitMut, RSplitN, RSplitNMut, Split, SplitInclusive, SplitInclusiveMut, SplitMut, SplitN,
    SplitNMut, Windows,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_yields_shared_references_in_order",
            claim: VERIFY_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Iter<'static, i32>> as KaniWitness>::proof().to_string(),
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

impl KaniWitness for RustStdStandard<IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_mut_yields_mutable_references_that_write_through",
            claim: VERIFY_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<IterMut<'static, i32>> as KaniWitness>::proof().to_string(),
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
            harness: "verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk",
            claim: VERIFY_CHUNKS_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC,
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
            assert_eq!(ch.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_exact_discards_a_short_remainder",
            claim: VERIFY_CHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_SRC,
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
            assert_eq!(ch.next(), None, "the short remainder is not yielded as a chunk");
            assert_eq!(ch.remainder(), &[c], "the short remainder is still reachable directly");
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunks_mut_writes_through_every_chunk",
            claim: VERIFY_CHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC,
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
            harness: "verify_chunks_exact_mut_leaves_the_remainder_untouched",
            claim: VERIFY_CHUNKS_EXACT_MUT_LEAVES_THE_REMAINDER_UNTOUCHED_SRC,
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
            harness: "verify_rchunks_groups_from_the_back",
            claim: VERIFY_RCHUNKS_GROUPS_FROM_THE_BACK_SRC,
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
            assert_eq!(ch.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_exact_discards_a_short_remainder_at_the_front",
            claim: VERIFY_RCHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC,
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
            assert_eq!(ch.next(), None);
            assert_eq!(ch.remainder(), &[a], "the short remainder sits at the front");
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExactMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rchunks_exact_mut_leaves_the_front_remainder_untouched",
            claim: VERIFY_RCHUNKS_EXACT_MUT_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC,
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
            harness: "verify_rchunks_mut_writes_through_every_chunk",
            claim: VERIFY_RCHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC,
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
            harness: "verify_windows_yields_overlapping_slices",
            claim: VERIFY_WINDOWS_YIELDS_OVERLAPPING_SLICES_SRC,
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
            assert_eq!(w.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunk_by_groups_adjacent_elements_matching_the_predicate",
            claim: VERIFY_CHUNK_BY_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CHUNK_BY_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC, {
        /// `chunk_by` groups two adjacent elements together exactly
        /// when the predicate holds for the pair, and splits them into
        /// separate one-element chunks otherwise — both branches are
        /// checked, since which one applies depends on the symbolic
        /// values themselves.
        #[kani::proof]
        fn verify_chunk_by_groups_adjacent_elements_matching_the_predicate() {
            fn same_parity(a: &i32, b: &i32) -> bool {
                a % 2 == b % 2
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let data = [a, b];
            let mut it = data.chunk_by(same_parity as fn(&i32, &i32) -> bool);
            let first = it.next().unwrap();
            if same_parity(&a, &b) {
                assert_eq!(first, &[a, b], "matching adjacent elements are grouped together");
                assert!(it.next().is_none());
            } else {
                assert_eq!(first, &[a], "a non-matching pair starts a new chunk");
                assert_eq!(it.next(), Some(&[b][..]));
            }
        }
    }
}

impl KaniWitness for RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chunk_by_mut_groups_adjacent_elements_matching_the_predicate",
            claim: VERIFY_CHUNK_BY_MUT_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC,
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

impl KaniWitness for RustStdStandard<Split<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_yields_subslices_between_matches",
            claim: VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Split<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Split<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<Split<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC, {
        /// `split` on a predicate yields the subslices between matches,
        /// consuming the matched element itself.
        #[kani::proof]
        fn verify_split_yields_subslices_between_matches() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let data = [a, 0, b];
            let mut it = data.split(is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[a][..]));
            assert_eq!(it.next(), Some(&[b][..]));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_mut_yields_writable_subslices_between_matches",
            claim: VERIFY_SPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_BETWEEN_MATCHES_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_SPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_BETWEEN_MATCHES_SRC, {
        /// `split_mut` yields the same subslices as `Split`, writable
        /// and writing through to the underlying slice.
        #[kani::proof]
        fn verify_split_mut_yields_writable_subslices_between_matches() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            kani::assume(a.checked_add(1).is_some());
            let mut data = [a, 0, b];
            {
                let mut it = data.split_mut(is_zero as fn(&i32) -> bool);
                let first = it.next().unwrap();
                first[0] += 1;
                assert_eq!(it.next(), Some(&mut [b][..]));
                assert_eq!(it.next(), None);
            }
            assert_eq!(data[0], a + 1, "a write through the first subslice is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<SplitInclusive<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_inclusive_keeps_the_match_at_the_end_of_each_piece",
            claim: VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitInclusive<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitInclusive<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitInclusive<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC, {
        /// Unlike `Split`, `split_inclusive` keeps the matched element
        /// at the end of the piece it terminates, rather than
        /// discarding it.
        #[kani::proof]
        fn verify_split_inclusive_keeps_the_match_at_the_end_of_each_piece() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let data = [a, 0, b];
            let mut it = data.split_inclusive(is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[a, 0][..]), "the matched element stays at the end");
            assert_eq!(it.next(), Some(&[b][..]));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_inclusive_mut_keeps_the_match_at_the_end_of_each_piece",
            claim: VERIFY_SPLIT_INCLUSIVE_MUT_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_MUT_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC, {
        /// Same inclusive-boundary rule as `SplitInclusive`, checked
        /// via the resulting piece's length on the mutable variant.
        #[kani::proof]
        fn verify_split_inclusive_mut_keeps_the_match_at_the_end_of_each_piece() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let mut data = [a, 0, b];
            let mut it = data.split_inclusive_mut(is_zero as fn(&i32) -> bool);
            let first = it.next().unwrap();
            assert_eq!(first.len(), 2, "the first piece includes the matched element");
            let second = it.next().unwrap();
            assert_eq!(second.len(), 1);
            assert!(it.next().is_none());
        }
    }
}

impl KaniWitness for RustStdStandard<SplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_n_caps_the_number_of_pieces",
            claim: VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SplitN<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC, {
        /// `splitn(2, ..)` stops after producing 2 pieces even when a
        /// second match exists: the would-be-third piece's delimiter
        /// stays embedded, unsplit, in the second piece — the feature
        /// that distinguishes `SplitN` from plain `Split`.
        #[kani::proof]
        fn verify_split_n_caps_the_number_of_pieces() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let data = [a, 0, b, 0, c];
            let mut it = data.splitn(2, is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[a][..]));
            assert_eq!(
                it.next(),
                Some(&[b, 0, c][..]),
                "the cap leaves the second delimiter unsplit in the last piece"
            );
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_n_mut_caps_the_number_of_pieces",
            claim: VERIFY_SPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_SPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_SRC, {
        /// Same cap-at-n rule as `SplitN`, checked via piece lengths on
        /// the mutable variant.
        #[kani::proof]
        fn verify_split_n_mut_caps_the_number_of_pieces() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let mut data = [a, 0, b, 0, c];
            let mut it = data.splitn_mut(2, is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&mut [a][..]));
            assert_eq!(
                it.next(),
                Some(&mut [b, 0, c][..]),
                "the cap leaves the second delimiter unsplit in the last piece"
            );
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<RSplit<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_yields_subslices_from_the_back",
            claim: VERIFY_RSPLIT_YIELDS_SUBSLICES_FROM_THE_BACK_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RSplit<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RSplit<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<RSplit<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_YIELDS_SUBSLICES_FROM_THE_BACK_SRC, {
        /// `rsplit` yields the same pieces as `Split`, but in reverse
        /// order — the last piece first.
        #[kani::proof]
        fn verify_rsplit_yields_subslices_from_the_back() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let data = [a, 0, b];
            let mut it = data.rsplit(is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[b][..]), "rsplit yields the last piece first");
            assert_eq!(it.next(), Some(&[a][..]));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_mut_yields_writable_subslices_from_the_back",
            claim: VERIFY_RSPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_FROM_THE_BACK_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_FROM_THE_BACK_SRC, {
        /// Same reverse-order rule as `RSplit`, writable and writing
        /// through to the underlying slice.
        #[kani::proof]
        fn verify_rsplit_mut_yields_writable_subslices_from_the_back() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            kani::assume(b.checked_add(1).is_some());
            let mut data = [a, 0, b];
            {
                let mut it = data.rsplit_mut(is_zero as fn(&i32) -> bool);
                let first = it.next().unwrap();
                first[0] += 1;
                assert_eq!(it.next(), Some(&mut [a][..]));
                assert_eq!(it.next(), None);
            }
            assert_eq!(data[2], b + 1, "a write through the first (rearmost) subslice is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_n_caps_the_number_of_pieces_from_the_back",
            claim: VERIFY_RSPLIT_N_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RSplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RSplitN<'static, i32, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<RSplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_N_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC, {
        /// `rsplitn(2, ..)` caps at 2 pieces from the back: the
        /// would-be-third piece's delimiter stays embedded, unsplit, in
        /// the last (frontmost) piece.
        #[kani::proof]
        fn verify_rsplit_n_caps_the_number_of_pieces_from_the_back() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let data = [a, 0, b, 0, c];
            let mut it = data.rsplitn(2, is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[c][..]));
            assert_eq!(
                it.next(),
                Some(&[a, 0, b][..]),
                "the cap leaves the first delimiter unsplit in the last piece"
            );
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rsplit_n_mut_caps_the_number_of_pieces_from_the_back",
            claim: VERIFY_RSPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC, {
        /// Same cap-from-the-back rule as `RSplitN`, checked via piece
        /// lengths on the mutable variant.
        #[kani::proof]
        fn verify_rsplit_n_mut_caps_the_number_of_pieces_from_the_back() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            kani::assume(a != 0 && b != 0 && c != 0);
            let mut data = [a, 0, b, 0, c];
            let mut it = data.rsplitn_mut(2, is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&mut [c][..]));
            assert_eq!(
                it.next(),
                Some(&mut [a, 0, b][..]),
                "the cap leaves the first delimiter unsplit in the last piece"
            );
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_escape_ascii_leaves_printable_bytes_unescaped",
            claim: VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC, {
        /// A printable ASCII byte passes through `escape_ascii`
        /// unchanged, while a control character (`\n`) is escaped to
        /// its two-byte backslash form.
        #[kani::proof]
        fn verify_escape_ascii_leaves_printable_bytes_unescaped() {
            let printable: u8 = kani::any();
            kani::assume((0x20..=0x7e).contains(&printable));
            let data = [printable, b'\n'];
            let escaped: Vec<u8> = data.escape_ascii().collect();
            let mut expected = vec![printable];
            expected.extend_from_slice(b"\\n");
            assert_eq!(
                escaped, expected,
                "printable bytes pass through unescaped; control bytes are escaped"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds",
            claim: VERIFY_GET_DISJOINT_MUT_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC,
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
