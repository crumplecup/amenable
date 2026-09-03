//! `KaniWitness` impls and harnesses for the unlimited reverse slice
//! splitters `RSplit` and `RSplitMut`, proved through the Amenable-owned
//! `KaniSplitObservation`.

use std::slice::RSplitMut;

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::SplitOperandsAreDistinctFromThePattern;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniSplitObservation, KaniVerifier};

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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
