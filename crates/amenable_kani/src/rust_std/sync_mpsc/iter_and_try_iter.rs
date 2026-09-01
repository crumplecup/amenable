use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::receiver_and_into_iter::KaniChannelYieldThenStopToken;
use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<std::sync::mpsc::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_iter_yields_sent_values_then_stops".to_owned(),
            VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::Iter<'static, i32>> as KaniWitness>::proof().to_string(),
    )
}

/// Lawful token minted once `RustStdStandard<Iter<'static, i32>>`'s
/// yield-then-stop claim has been established from a `KaniChannel<i32>`
/// that has itself demonstrated the sent value yielded then the channel
/// stopping.
pub struct RustStdIterToken(());

impl ProofToken for RustStdIterToken {
    type Proposition = RustStdStandard<std::sync::mpsc::Iter<'static, i32>>;
}

impl Establish<KaniChannelYieldThenStopToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::Iter<'static, i32>>
{
    type Token = RustStdIterToken;

    fn establish(_credential: KaniChannelYieldThenStopToken) -> Self::Token {
        RustStdIterToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.iter()` borrows the `Receiver` instead of consuming it,
        /// with the same yield-then-stop behavior as `IntoIter`.
        /// Same `KaniChannel` model migration and rationale as
        /// `IntoIter`'s proof above -- `Iter::next()` is the same
        /// `recv().ok()` shape. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<Iter<'static, i32>>` from the channel instance
        /// that actually demonstrated the yield-then-stop shape, rather
        /// than asserted independently of it.
        #[kani::proof]
        fn verify_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::unbounded();
            let demonstration = channel.demonstrate_yield_then_stop(value);

            let _token =
                RustStdStandard::<std::sync::mpsc::Iter<'static, i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_iter_does_not_block_on_an_empty_open_channel".to_owned(),
            VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChannel<i32>` instance actually returned `None`
/// immediately from `.try_recv()` on an empty, still-open channel, minted
/// only by [`KaniChannel::demonstrate_non_blocking_empty`].
pub struct KaniChannelNonBlockingEmptyToken(());

impl ProofToken for KaniChannelNonBlockingEmptyToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Assert `.try_recv()` returns `None`-equivalent immediately on an
    /// empty, open channel rather than blocking. Consumes `self` for the
    /// same reason [`KaniChannel::demonstrate_delivery`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_non_blocking_empty(mut self) -> KaniChannelNonBlockingEmptyToken {
        assert_eq!(
            self.try_recv().ok(),
            None,
            "try_iter returns None immediately rather than blocking"
        );
        KaniChannelNonBlockingEmptyToken(())
    }
}

/// Lawful token minted once `RustStdStandard<TryIter<'static, i32>>`'s
/// non-blocking claim has been established from a `KaniChannel<i32>` that
/// has itself demonstrated returning `None` immediately rather than
/// blocking.
pub struct RustStdTryIterToken(());

impl ProofToken for RustStdTryIterToken {
    type Proposition = RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>;
}

impl Establish<KaniChannelNonBlockingEmptyToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>
{
    type Token = RustStdTryIterToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniChannelNonBlockingEmptyToken) -> Self::Token {
        RustStdTryIterToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC, {
        /// Unlike `Iter`, `.try_iter()` never blocks: on an empty
        /// channel whose `Sender` is still alive (where `Iter` would
        /// wait for a value), it returns `None` immediately.
        /// This proof uses the Amenable-owned channel model: `TryIter`'s
        /// `next()` is `self.rx.try_recv().ok()` under the hood, so
        /// `KaniChannel::try_recv()` models the same non-blocking shape.
        /// The claim is established through `Establish<KaniChannel<i32>,
        /// KaniVerifier> for RustStdStandard<TryIter<'static, i32>>` from
        /// the channel instance that actually demonstrated the
        /// non-blocking behavior, rather than asserted independently of
        /// it.
        #[kani::proof]
        fn verify_try_iter_does_not_block_on_an_empty_open_channel() {
            let channel = crate::KaniChannel::<i32>::unbounded();
            let demonstration = channel.demonstrate_non_blocking_empty();

            let _token =
                RustStdStandard::<std::sync::mpsc::TryIter<'static, i32>>::establish(demonstration);
        }
    }
}
