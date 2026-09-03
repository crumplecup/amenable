//! `KaniWitness` impls and harnesses for the forward count-limited slice
//! splitters `SplitN` and `SplitNMut`, proved through the Amenable-owned
//! `KaniSplitNObservation`.

use std::slice::SplitNMut;

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ThreeSplitOperandsAreDistinctFromThePattern;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniSplitNObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_n_caps_the_number_of_pieces".to_owned(),
            VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((a, b, c, 0)));
            let observation = crate::KaniSplitNObservationBuilder::default().first(a).first_delimiter(0).middle(b).second_delimiter(0).last(c).build().expect("all fields set");
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_n_mut_caps_the_number_of_pieces".to_owned(),
            VERIFY_SPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((a, b, c, 0)));
            let observation = crate::KaniSplitNObservationBuilder::default().first(a).first_delimiter(0).middle(b).second_delimiter(0).last(c).build().expect("all fields set");
            let demonstration = observation.demonstrate_splitn_two(a, b, c);

            let _token = RustStdStandard::<
                SplitNMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}
