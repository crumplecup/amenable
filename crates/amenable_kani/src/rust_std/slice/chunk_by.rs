use std::slice::{ChunkBy, ChunkByMut};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{KaniChunkByObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunk_by_groups_adjacent_elements_matching_the_predicate".to_owned(),
            VERIFY_CHUNK_BY_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>",
        "kani",
        || <RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chunk_by_mut_groups_adjacent_elements_matching_the_predicate".to_owned(),
            VERIFY_CHUNK_BY_MUT_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>",
        "kani",
        || <RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

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
                assert!(
                    RustStdStandard::<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>::ensures((
                        first.len(),
                        2
                    )),
                    "matching adjacent elements are grouped together"
                );
            } else {
                assert!(
                    RustStdStandard::<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>::ensures((
                        first.len(),
                        1
                    )),
                    "a non-matching pair starts a new chunk"
                );
            }
        }
    }
}
