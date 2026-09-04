//! `KaniWitness` impls and harnesses for the reverse count-limited slice
//! splitters `RSplitN` and `RSplitNMut`, proved through the Amenable-owned
//! `KaniSplitNObservation`.

use std::slice::RSplitNMut;

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ThreeSplitOperandsAreDistinctFromThePattern;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniSplitNObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_n_caps_the_number_of_pieces_from_the_back".to_owned(),
            VERIFY_RSPLIT_N_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((a, b, c, 0)));
            let observation = crate::KaniSplitNObservationBuilder::default().first(a).first_delimiter(0).middle(b).second_delimiter(0).last(c).build().expect("all fields set");
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_n_mut_caps_the_number_of_pieces_from_the_back".to_owned(),
            VERIFY_RSPLIT_N_MUT_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((a, b, c, 0)));
            let observation = crate::KaniSplitNObservationBuilder::default().first(a).first_delimiter(0).middle(b).second_delimiter(0).last(c).build().expect("all fields set");
            let demonstration = observation.demonstrate_rsplitn_two(a, b, c);

            let _token = RustStdStandard::<
                RSplitNMut<'static, i32, fn(&i32) -> bool>,
            >::establish(demonstration);
        }
    }
}
