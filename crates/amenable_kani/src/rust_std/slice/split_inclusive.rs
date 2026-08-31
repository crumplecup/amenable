use std::slice::SplitInclusiveMut;

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
#[cfg(kani)]
use crate::FallibleOperationReportsSuccess;
#[cfg(kani)]
use crate::IndexRecoversTheStoredElement;
#[cfg(kani)]
use crate::IteratorYieldsAReferenceToTheStoredValue;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
#[cfg(kani)]
use crate::SplitOperandsAreDistinctFromThePattern;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniSplitObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_inclusive_keeps_the_match_at_the_end_of_each_piece".to_owned(),
            VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
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
        CheckedProof::new(
            "verify_split_inclusive_mut_keeps_the_match_at_the_end_of_each_piece".to_owned(),
            VERIFY_SPLIT_INCLUSIVE_MUT_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split_inclusive_lengths();

            let _token = RustStdStandard::<
                SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}
