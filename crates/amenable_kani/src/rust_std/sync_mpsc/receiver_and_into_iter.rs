use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<std::sync::mpsc::Receiver<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_receiver_fails_once_every_sender_is_dropped".to_owned(),
            VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Receiver<i32>>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::Receiver<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChannel<i32>` instance actually failed `.recv()`
/// once every sender was dropped, minted only by
/// [`KaniChannel::demonstrate_disconnected_recv_fails`].
pub struct KaniChannelDisconnectedRecvToken(());

impl ProofToken for KaniChannelDisconnectedRecvToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Drop the sender, then assert `.recv()` fails on the now-empty,
    /// disconnected channel. Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_disconnected_recv_fails(mut self) -> KaniChannelDisconnectedRecvToken {
        self.drop_sender();
        assert!(
            self.recv().is_err(),
            "recv fails once the channel is empty and disconnected"
        );
        KaniChannelDisconnectedRecvToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Receiver<i32>>`'s
/// disconnect-on-drop claim has been established from a `KaniChannel<i32>`
/// that has itself demonstrated `.recv()` failing once every sender is
/// dropped.
pub struct RustStdReceiverToken(());

impl ProofToken for RustStdReceiverToken {
    type Proposition = RustStdStandard<std::sync::mpsc::Receiver<i32>>;
}

impl Establish<KaniChannelDisconnectedRecvToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::Receiver<i32>>
{
    type Token = RustStdReceiverToken;

    fn establish(_credential: KaniChannelDisconnectedRecvToken) -> Self::Token {
        RustStdReceiverToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC, {
        /// `.recv()` fails once the channel is both empty and every
        /// `Sender` has been dropped — it never blocks forever on a
        /// channel that can no longer receive anything.
        /// Same `KaniChannel` model migration as `Sender`'s proof above.
        /// The claim is established through `Establish<KaniChannel<i32>,
        /// KaniVerifier> for RustStdStandard<Receiver<i32>>` from the
        /// channel instance that actually demonstrated the disconnect,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_receiver_fails_once_every_sender_is_dropped() {
            let channel = crate::KaniChannel::<i32>::unbounded();
            let demonstration = channel.demonstrate_disconnected_recv_fails();

            let _token =
                RustStdStandard::<std::sync::mpsc::Receiver<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::mpsc::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_into_iter_yields_sent_values_then_stops".to_owned(),
            VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniChannel<i32>` instance actually yielded a sent
/// value and then stopped once disconnected and drained, minted only by
/// [`KaniChannel::demonstrate_yield_then_stop`].
pub struct KaniChannelYieldThenStopToken(());

impl ProofToken for KaniChannelYieldThenStopToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Send `value`, drop the sender, then assert `.recv()` yields the
    /// value once and `None`-equivalent thereafter. Consumes `self` for
    /// the same reason [`KaniChannel::demonstrate_delivery`] does.
    ///
    /// Asserts on the whole `Result`, not `.unwrap()`, the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn demonstrate_yield_then_stop(mut self, value: i32) -> KaniChannelYieldThenStopToken {
        assert_eq!(
            self.send(value),
            Ok(()),
            "a fresh channel's send must succeed"
        );
        self.drop_sender();
        assert_eq!(self.recv().ok(), Some(value));
        assert_eq!(self.recv().ok(), None);
        KaniChannelYieldThenStopToken(())
    }
}

/// Lawful token minted once `RustStdStandard<IntoIter<i32>>`'s
/// yield-then-stop claim has been established from a `KaniChannel<i32>`
/// that has itself demonstrated the sent value yielded then the channel
/// stopping.
pub struct RustStdIntoIterToken(());

impl ProofToken for RustStdIntoIterToken {
    type Proposition = RustStdStandard<std::sync::mpsc::IntoIter<i32>>;
}

impl Establish<KaniChannelYieldThenStopToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::IntoIter<i32>>
{
    type Token = RustStdIntoIterToken;

    fn establish(_credential: KaniChannelYieldThenStopToken) -> Self::Token {
        RustStdIntoIterToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.into_iter()` consumes the `Receiver`, yielding sent
        /// values and stopping once the channel is disconnected and
        /// drained.
        /// This proof uses the Amenable-owned channel model: `IntoIter`'s
        /// `next()` is `self.rx.recv().ok()` under the hood, so a
        /// `KaniChannel::recv()` call directly models the same
        /// yield-then-stop shape, conditional on the real `IntoIter`
        /// refining this law. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<IntoIter<i32>>` from the channel instance that
        /// actually demonstrated the yield-then-stop shape, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_into_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::unbounded();
            let demonstration = channel.demonstrate_yield_then_stop(value);

            let _token =
                RustStdStandard::<std::sync::mpsc::IntoIter<i32>>::establish(demonstration);
        }
    }
}
