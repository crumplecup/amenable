use std::slice::{RSplitMut, RSplitNMut, SplitNMut};

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniSplitNObservation, KaniSplitObservation, KaniVerifier};
#[cfg(kani)]
use crate::{SplitOperandsAreDistinctFromThePattern, ThreeSplitOperandsAreDistinctFromThePattern};

impl KaniWitness for RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

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

impl KaniWitness for RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_yields_subslices_from_the_back".to_owned(),
            VERIFY_RSPLIT_YIELDS_SUBSLICES_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
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
        CheckedProof::new(
            "verify_rsplit_mut_yields_writable_subslices_from_the_back".to_owned(),
            VERIFY_RSPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

        self = self.with_after(updated);

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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
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
