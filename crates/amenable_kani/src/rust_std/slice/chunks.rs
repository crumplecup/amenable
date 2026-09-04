use std::slice::{
    Chunks, ChunksExact, ChunksExactMut, ChunksMut, RChunks, RChunksExact, RChunksExactMut,
    RChunksMut, Windows,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::bridge_kani_witness;

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};

    pub(super) use crate::AccessorRecoversTheExpectedValue;
    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::IteratorYieldsAReferenceToTheStoredValue;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    AccessorRecoversTheExpectedValue, CollectedSequenceMatchesExpected, Ensures,
    IteratorYieldsAReferenceToTheStoredValue, IteratorYieldsNoneWhenExhausted, Requires,
};

impl KaniWitness for RustStdStandard<Chunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk".to_owned(),
            VERIFY_CHUNKS_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Chunks<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Chunks<'static, i32>>",
        "kani",
        || <RustStdStandard<Chunks<'static, i32>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[a, b][..]))));
            assert!(
                IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[c][..]))),
                "the final chunk is short rather than dropped"
            );
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunks_exact_discards_a_short_remainder".to_owned(),
            VERIFY_CHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChunksExact<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChunksExact<'static, i32>>",
        "kani",
        || <RustStdStandard<ChunksExact<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[a, b][..]))));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(ch.next()),
                "the short remainder is not yielded as a chunk"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((ch.remainder(), &[c][..])),
                "the short remainder is still reachable directly"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunks_mut_writes_through_every_chunk".to_owned(),
            VERIFY_CHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChunksMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChunksMut<'static, i32>>",
        "kani",
        || <RustStdStandard<ChunksMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_CHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC, {
        /// `chunks_mut` yields mutable chunks that write through to the
        /// underlying slice, including a short final chunk.
        #[kani::proof]
        fn verify_chunks_mut_writes_through_every_chunk() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(RustStdStandard::<i32>::requires((a, 10)));
            kani::assume(RustStdStandard::<i32>::requires((b, 10)));
            let mut data = [a, b];
            for chunk in data.chunks_mut(2) {
                for x in chunk {
                    *x += 10;
                }
            }
            assert!(CollectedSequenceMatchesExpected::ensures((data, [a + 10, b + 10])));
        }
    }
}

impl KaniWitness for RustStdStandard<ChunksExactMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunks_exact_mut_leaves_the_remainder_untouched".to_owned(),
            VERIFY_CHUNKS_EXACT_MUT_LEAVES_THE_REMAINDER_UNTOUCHED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChunksExactMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChunksExactMut<'static, i32>>",
        "kani",
        || <RustStdStandard<ChunksExactMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(RustStdStandard::<i32>::requires((a, 10)));
            kani::assume(RustStdStandard::<i32>::requires((b, 10)));
            let mut data = [a, b, c];
            {
                let mut ch = data.chunks_exact_mut(2);
                let first = ch.next().unwrap();
                first[0] += 10;
                first[1] += 10;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
            }
            assert!(
                CollectedSequenceMatchesExpected::ensures((data, [a + 10, b + 10, c])),
                "the remainder element c is untouched"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RChunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rchunks_groups_from_the_back".to_owned(),
            VERIFY_RCHUNKS_GROUPS_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RChunks<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RChunks<'static, i32>>",
        "kani",
        || <RustStdStandard<RChunks<'static, i32>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[b, c][..]))));
            assert!(
                IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[a][..]))),
                "the short chunk is at the front, not the back"
            );
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExact<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rchunks_exact_discards_a_short_remainder_at_the_front".to_owned(),
            VERIFY_RCHUNKS_EXACT_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RChunksExact<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RChunksExact<'static, i32>>",
        "kani",
        || <RustStdStandard<RChunksExact<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((ch.next(), Some(&[b, c][..]))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((ch.remainder(), &[a][..])),
                "the short remainder sits at the front"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksExactMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rchunks_exact_mut_leaves_the_front_remainder_untouched".to_owned(),
            VERIFY_RCHUNKS_EXACT_MUT_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RChunksExactMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RChunksExactMut<'static, i32>>",
        "kani",
        || <RustStdStandard<RChunksExactMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(RustStdStandard::<i32>::requires((b, 10)));
            kani::assume(RustStdStandard::<i32>::requires((c, 10)));
            let mut data = [a, b, c];
            {
                let mut ch = data.rchunks_exact_mut(2);
                let first = ch.next().unwrap();
                first[0] += 10;
                first[1] += 10;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(ch.next()));
            }
            assert!(
                CollectedSequenceMatchesExpected::ensures((data, [a, b + 10, c + 10])),
                "the front remainder element a is untouched"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RChunksMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rchunks_mut_writes_through_every_chunk".to_owned(),
            VERIFY_RCHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RChunksMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RChunksMut<'static, i32>>",
        "kani",
        || <RustStdStandard<RChunksMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RCHUNKS_MUT_WRITES_THROUGH_EVERY_CHUNK_SRC, {
        /// `rchunks_mut` yields mutable chunks (grouped from the back)
        /// that write through to the underlying slice.
        #[kani::proof]
        fn verify_rchunks_mut_writes_through_every_chunk() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(RustStdStandard::<i32>::requires((a, 10)));
            kani::assume(RustStdStandard::<i32>::requires((b, 10)));
            let mut data = [a, b];
            for chunk in data.rchunks_mut(2) {
                for x in chunk {
                    *x += 10;
                }
            }
            assert!(CollectedSequenceMatchesExpected::ensures((data, [a + 10, b + 10])));
        }
    }
}

impl KaniWitness for RustStdStandard<Windows<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_windows_yields_overlapping_slices".to_owned(),
            VERIFY_WINDOWS_YIELDS_OVERLAPPING_SLICES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Windows<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Windows<'static, i32>>",
        "kani",
        || <RustStdStandard<Windows<'static, i32>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((w.next(), Some(&[a, b][..]))));
            assert!(
                IteratorYieldsAReferenceToTheStoredValue::ensures((w.next(), Some(&[b, c][..]))),
                "consecutive windows overlap on b"
            );
            assert!(IteratorYieldsNoneWhenExhausted::ensures(w.next()));
        }
    }
}
