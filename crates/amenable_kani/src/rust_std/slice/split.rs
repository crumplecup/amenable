use std::slice::SplitMut;

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::split_n_and_rsplit::VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniSplitObservation, KaniVerifier};

/// A `(before, pattern, after)` triple known to satisfy the precondition
/// every split-family harness assumes about its symbolic inputs: the two
/// elements surrounding a match are themselves distinct from the split
/// pattern, so the split has an unambiguous delimiter to find.
///
/// Independently hand-written as `kani::assume(before != pattern && after
/// != pattern)` at 9 real sites split between `rust_std::slice` (6 sites,
/// where the pattern is the fixed literal `0`) and `rust_std::str` (3
/// sites, where the pattern is itself a symbolic `kani::any()` byte) --
/// the identical precondition regardless of whether the pattern is fixed
/// or symbolic. Generic over the element type rather than one
/// registration per split family, the same reasoning (and the same
/// reason it needs a hand-written `Witness`/`Requires` impl instead of
/// the `bridge_kani_witness!`/`kani_requires!` macros) as
/// `IteratorYieldsNoneWhenExhausted`.
pub struct SplitOperandsAreDistinctFromThePattern<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for SplitOperandsAreDistinctFromThePattern<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for SplitOperandsAreDistinctFromThePattern<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for SplitOperandsAreDistinctFromThePattern<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_yields_subslices_between_matches".to_owned(),
            VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for SplitOperandsAreDistinctFromThePattern<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Requires<crate::KaniVerifier>
    for SplitOperandsAreDistinctFromThePattern<T>
{
    type Input = (T, T, T);
    type Bound = bool;

    fn requires((before, pattern, after): (T, T, T)) -> bool {
        before != pattern && after != pattern
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::SplitOperandsAreDistinctFromThePattern",
        "kani",
        "requires",
        || stringify!(before != pattern && after != pattern),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::SplitOperandsAreDistinctFromThePattern",
        "kani",
        || <SplitOperandsAreDistinctFromThePattern<i32> as KaniWitness>::proof().to_string(),
    )
}

/// The three-operand sibling of [`SplitOperandsAreDistinctFromThePattern`]:
/// an `(a, b, c, pattern)` quadruple known to satisfy the precondition
/// every `*n`-capped or match-counting split-family harness assumes about
/// its three symbolic pieces -- each is itself distinct from the split
/// pattern.
///
/// Independently hand-written as `kani::assume(a != pattern && b !=
/// pattern && c != pattern)` at 9 real sites, same slice/str split (fixed
/// vs. symbolic pattern) as its two-operand sibling. A separate type
/// rather than a re-run of the two-operand one since Rust generics have
/// no variadic tuple, and merging the two into one `Vec`-shaped `Input`
/// would trade a real fixed-arity check for a weaker runtime-length one.
pub struct ThreeSplitOperandsAreDistinctFromThePattern<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ThreeSplitOperandsAreDistinctFromThePattern<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ThreeSplitOperandsAreDistinctFromThePattern<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ThreeSplitOperandsAreDistinctFromThePattern<T> {
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

impl<T> amenable_core::Witness<crate::KaniVerifier>
    for ThreeSplitOperandsAreDistinctFromThePattern<T>
{
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Requires<crate::KaniVerifier>
    for ThreeSplitOperandsAreDistinctFromThePattern<T>
{
    type Input = (T, T, T, T);
    type Bound = bool;

    fn requires((a, b, c, pattern): (T, T, T, T)) -> bool {
        a != pattern && b != pattern && c != pattern
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ThreeSplitOperandsAreDistinctFromThePattern",
        "kani",
        "requires",
        || stringify!(a != pattern && b != pattern && c != pattern),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ThreeSplitOperandsAreDistinctFromThePattern",
        "kani",
        || <ThreeSplitOperandsAreDistinctFromThePattern<i32> as KaniWitness>::proof()
            .to_string(),
    )
}

impl KaniWitness for RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_yields_subslices_between_matches".to_owned(),
            VERIFY_SPLIT_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_mut_yields_writable_subslices_between_matches".to_owned(),
            VERIFY_SPLIT_MUT_YIELDS_WRITABLE_SUBSLICES_BETWEEN_MATCHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

        self = self.with_before(updated);

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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((a, 0, b)));
            let observation = crate::KaniSplitObservation::new(a, 0, b);
            let demonstration = observation.demonstrate_split_mut_write_through(a, b, updated);

            let _token = RustStdStandard::<SplitMut<'static, i32, fn(&i32) -> bool>>::establish(
                demonstration,
            );
        }
    }
}
