//! `KaniWitness` impls for `std::sync::mpsc`.
//!
//! Every harness runs single-threaded: sending, then immediately
//! receiving (or checking a disconnect/timeout/full condition) without
//! ever blocking on another thread. This is enough to check each carrier's
//! own contract — the channel's transport behavior, not cross-thread
//! scheduling.

use std::sync::mpsc::{RecvError, SendError, SyncSender, TrySendError};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<std::sync::mpsc::Sender<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sender_delivers_to_the_paired_receiver".to_owned(),
            claim: VERIFY_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Sender<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Sender<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::Sender<i32>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually delivered a sent
/// value unchanged, minted only by [`KaniChannel::demonstrate_delivery`] —
/// the sole lawful credential for [`Establish`] impls that claim delivery
/// fidelity for a `std::sync::mpsc` sender.
pub struct KaniChannelDeliveryToken(());

impl ProofToken for KaniChannelDeliveryToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Send `value`, then receive it back, asserting the channel preserved
    /// it unchanged. Consumes `self`: the only way to obtain the token is
    /// to actually run the send/recv pair being claimed, not to assert it
    /// independently of a real channel instance.
    ///
    /// Asserts on the whole `Result`, not `.unwrap()`/`.expect()`: a
    /// `#[kani::proof]` harness can't propagate a `Result` any further --
    /// confirmed empirically (`probe_result_err_harness_behavior`, this
    /// session) that Kani reports `VERIFICATION:- SUCCESSFUL` for a
    /// harness that unconditionally returns `Err`, so a propagated
    /// failure would silently pass instead of failing the proof. An
    /// explicit `assert_eq!` is a real, CBMC-checked assertion either
    /// way.
    #[track_caller]
    pub fn demonstrate_delivery(mut self, value: i32) -> KaniChannelDeliveryToken {
        assert_eq!(
            self.send(value),
            Ok(()),
            "a fresh channel's send must succeed"
        );
        assert_eq!(self.recv(), Ok(value), "the sent value is receivable");
        KaniChannelDeliveryToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Sender<i32>>`'s delivery claim
/// has been established from a `KaniChannel<i32>` that has itself
/// demonstrated the sent value is receivable.
pub struct RustStdSenderToken(());

impl ProofToken for RustStdSenderToken {
    type Proposition = RustStdStandard<std::sync::mpsc::Sender<i32>>;
}

impl Establish<KaniChannelDeliveryToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::Sender<i32>>
{
    type Token = RustStdSenderToken;

    fn establish(_credential: KaniChannelDeliveryToken) -> Self::Token {
        RustStdSenderToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC, {
        /// A value sent on an unbounded channel is receivable on the
        /// paired `Receiver`.
        /// This proof uses the Amenable-owned channel model
        /// (`mpsc_model.rs`): the direct `std::sync::mpsc::channel` path
        /// times out even for a single send immediately followed by a recv
        /// with no blocking involved -- a pure in-memory implementation
        /// cost of the real flavor-switching, atomics-backed queue, not an
        /// unwinding-bound or foreign-boundary issue (see
        /// `gallery::replace_recommendations`). The claim is established
        /// through `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<Sender<i32>>` from the channel instance that
        /// actually demonstrated the delivery, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_sender_delivers_to_the_paired_receiver() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::unbounded();
            let demonstration = channel.demonstrate_delivery(value);

            let _token =
                RustStdStandard::<std::sync::mpsc::Sender<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SyncSender<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sync_sender_delivers_to_the_paired_receiver".to_owned(),
            claim: VERIFY_SYNC_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SyncSender<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SyncSender<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<SyncSender<i32>> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<SyncSender<i32>>`'s delivery
/// claim has been established from a `KaniChannel<i32>` that has itself
/// demonstrated the sent value is receivable.
pub struct RustStdSyncSenderToken(());

impl ProofToken for RustStdSyncSenderToken {
    type Proposition = RustStdStandard<SyncSender<i32>>;
}

impl Establish<KaniChannelDeliveryToken, KaniVerifier> for RustStdStandard<SyncSender<i32>> {
    type Token = RustStdSyncSenderToken;

    fn establish(_credential: KaniChannelDeliveryToken) -> Self::Token {
        RustStdSyncSenderToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SYNC_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC, {
        /// Same delivery contract as `Sender`, for a bounded channel
        /// with spare capacity.
        /// Same `KaniChannel` model migration as `Sender`'s proof above.
        /// The claim is established through `Establish<KaniChannel<i32>,
        /// KaniVerifier> for RustStdStandard<SyncSender<i32>>` from the
        /// channel instance that actually demonstrated the delivery,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_sync_sender_delivers_to_the_paired_receiver() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::bounded(1);
            let demonstration = channel.demonstrate_delivery(value);

            let _token = RustStdStandard::<SyncSender<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::mpsc::Receiver<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_receiver_fails_once_every_sender_is_dropped".to_owned(),
            claim: VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Receiver<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::Receiver<i32>> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_into_iter_yields_sent_values_then_stops".to_owned(),
            claim: VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
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

impl KaniWitness for RustStdStandard<std::sync::mpsc::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_yields_sent_values_then_stops".to_owned(),
            claim: VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::Iter<'static, i32>> as KaniWitness>::proof().to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_iter_does_not_block_on_an_empty_open_channel".to_owned(),
            claim: VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> as KaniWitness>::proof().to_string(),
    }
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

impl KaniWitness for RustStdStandard<RecvError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_recv_error_on_an_empty_disconnected_channel".to_owned(),
            claim: VERIFY_RECV_ERROR_ON_AN_EMPTY_DISCONNECTED_CHANNEL_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RecvError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RecvError>",
        verifier: "kani",
        describe: || <RustStdStandard<RecvError> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually failed `.recv()`
/// with exactly `Disconnected`, minted only by
/// [`KaniChannel::demonstrate_recv_disconnected`].
pub struct KaniChannelRecvDisconnectedToken(());

impl ProofToken for KaniChannelRecvDisconnectedToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Drop the sender, then assert `.recv()` fails with exactly
    /// `Disconnected`. Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_recv_disconnected(mut self) -> KaniChannelRecvDisconnectedToken {
        self.drop_sender();
        assert_eq!(self.recv(), Err(crate::KaniRecvError::Disconnected));
        KaniChannelRecvDisconnectedToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RecvError>`'s
/// empty-and-disconnected claim has been established from a
/// `KaniChannel<i32>` that has itself demonstrated `.recv()` failing with
/// exactly this error.
pub struct RustStdRecvErrorToken(());

impl ProofToken for RustStdRecvErrorToken {
    type Proposition = RustStdStandard<RecvError>;
}

impl Establish<KaniChannelRecvDisconnectedToken, KaniVerifier> for RustStdStandard<RecvError> {
    type Token = RustStdRecvErrorToken;

    fn establish(_credential: KaniChannelRecvDisconnectedToken) -> Self::Token {
        RustStdRecvErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RECV_ERROR_ON_AN_EMPTY_DISCONNECTED_CHANNEL_SRC, {
        /// `.recv()` fails with exactly this error when the channel is
        /// empty and every `Sender` has been dropped.
        /// Same `KaniChannel` model migration as the other `recv`-family
        /// proofs above. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<RecvError>` from the channel instance that
        /// actually demonstrated the error, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_recv_error_on_an_empty_disconnected_channel() {
            let channel = crate::KaniChannel::<i32>::unbounded();
            let demonstration = channel.demonstrate_recv_disconnected();

            let _token = RustStdStandard::<RecvError>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::mpsc::RecvTimeoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_recv_timeout_error_distinguishes_timeout_from_disconnected".to_owned(),
            claim: VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::RecvTimeoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::RecvTimeoutError>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::RecvTimeoutError> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually distinguished a
/// zero-duration timeout from a disconnect, minted only by
/// [`KaniChannel::demonstrate_recv_timeout_distinguishes_disconnect`].
pub struct KaniChannelRecvTimeoutToken(());

impl ProofToken for KaniChannelRecvTimeoutToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Assert a zero-duration timed receive times out while the channel is
    /// open and empty, then, after dropping the sender, assert it instead
    /// fails `Disconnected`. Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_recv_timeout_distinguishes_disconnect(
        mut self,
    ) -> KaniChannelRecvTimeoutToken {
        assert_eq!(
            self.recv_timeout_zero(),
            Err(crate::KaniRecvTimeoutError::Timeout),
            "an open, empty channel times out"
        );

        self.drop_sender();
        assert_eq!(
            self.recv_timeout_zero(),
            Err(crate::KaniRecvTimeoutError::Disconnected),
            "a disconnected channel fails immediately instead"
        );

        KaniChannelRecvTimeoutToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RecvTimeoutError>`'s
/// timeout-vs-disconnected classification claim has been established from a
/// `KaniChannel<i32>` that has itself demonstrated the corresponding
/// zero-duration timed-receive outcomes.
pub struct RustStdRecvTimeoutErrorToken(());

impl ProofToken for RustStdRecvTimeoutErrorToken {
    type Proposition = RustStdStandard<std::sync::mpsc::RecvTimeoutError>;
}

impl Establish<KaniChannelRecvTimeoutToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::RecvTimeoutError>
{
    type Token = RustStdRecvTimeoutErrorToken;

    fn establish(_credential: KaniChannelRecvTimeoutToken) -> Self::Token {
        RustStdRecvTimeoutErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC, {
        /// `.recv_timeout()`'s two failure modes are distinct: a
        /// channel that's open but empty times out; a disconnected
        /// channel fails immediately as `Disconnected` instead.
        /// This proof uses the Amenable-owned channel model's
        /// zero-duration timed-receive observation: the direct
        /// `std::sync::mpsc::Receiver::recv_timeout` path reaches the same
        /// gallery-documented `clock_gettime` boundary as timed waits on
        /// `Condvar`. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<RecvTimeoutError>` from the channel instance that
        /// actually demonstrated the timeout/disconnected distinction,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_recv_timeout_error_distinguishes_timeout_from_disconnected() {
            let channel = crate::KaniChannel::<i32>::unbounded();
            let demonstration = channel.demonstrate_recv_timeout_distinguishes_disconnect();

            let _token =
                RustStdStandard::<std::sync::mpsc::RecvTimeoutError>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SendError<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_send_error_recovers_the_unsent_value".to_owned(),
            claim: VERIFY_SEND_ERROR_RECOVERS_THE_UNSENT_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SendError<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SendError<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<SendError<i32>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually recovered an unsent
/// value from a `.send()` failure, minted only by
/// [`KaniChannel::demonstrate_send_error_recovers_value`].
pub struct KaniChannelSendErrorToken(());

impl ProofToken for KaniChannelSendErrorToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Drop the receiver, then send `value` and assert the resulting
    /// error recovers it unchanged. Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[track_caller]
    #[must_use]
    pub fn demonstrate_send_error_recovers_value(
        mut self,
        value: i32,
    ) -> KaniChannelSendErrorToken {
        self.drop_receiver();
        assert_eq!(
            self.send(value),
            Err(crate::KaniSendError::Disconnected(value)),
            "the unsent value is recoverable from the error"
        );
        KaniChannelSendErrorToken(())
    }
}

/// Lawful token minted once `RustStdStandard<SendError<i32>>`'s
/// value-recovery claim has been established from a `KaniChannel<i32>`
/// that has itself demonstrated the unsent value recoverable from the
/// error.
pub struct RustStdSendErrorToken(());

impl ProofToken for RustStdSendErrorToken {
    type Proposition = RustStdStandard<SendError<i32>>;
}

impl Establish<KaniChannelSendErrorToken, KaniVerifier> for RustStdStandard<SendError<i32>> {
    type Token = RustStdSendErrorToken;

    fn establish(_credential: KaniChannelSendErrorToken) -> Self::Token {
        RustStdSendErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SEND_ERROR_RECOVERS_THE_UNSENT_VALUE_SRC, {
        /// `.send()` fails once the `Receiver` is dropped, and the
        /// error doesn't discard the value: it's recoverable via the
        /// error's own field.
        /// Same `KaniChannel` model migration as the other `send`-family
        /// proofs; the direct `std::sync::mpsc` path was confirmed to
        /// time out even for this single send with no recv involved at
        /// all. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<SendError<i32>>` from the channel instance that
        /// actually demonstrated the recovery, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_send_error_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::unbounded();
            let demonstration = channel.demonstrate_send_error_recovers_value(value);

            let _token = RustStdStandard::<SendError<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<TrySendError<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_send_error_full_recovers_the_unsent_value".to_owned(),
            claim: VERIFY_TRY_SEND_ERROR_FULL_RECOVERS_THE_UNSENT_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TrySendError<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TrySendError<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<TrySendError<i32>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually recovered an unsent
/// value from a `.try_send()` `Full` failure, minted only by
/// [`KaniChannel::demonstrate_try_send_full_recovers_value`].
pub struct KaniChannelTrySendFullToken(());

impl ProofToken for KaniChannelTrySendFullToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Try to send `value` on a full (zero-capacity rendezvous) channel,
    /// and assert the resulting `Full` error recovers it unchanged.
    /// Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_try_send_full_recovers_value(
        mut self,
        value: i32,
    ) -> KaniChannelTrySendFullToken {
        match self.try_send(value) {
            Err(crate::KaniSendError::Full(v)) => {
                assert_eq!(v, value, "the unsent value is recoverable from Full");
            }
            other => panic!("expected Full, got {other:?}"),
        }
        KaniChannelTrySendFullToken(())
    }
}

/// Lawful token minted once `RustStdStandard<TrySendError<i32>>`'s
/// full-capacity recovery claim has been established from a
/// `KaniChannel<i32>` that has itself demonstrated the unsent value
/// recoverable from `Full`.
pub struct RustStdTrySendErrorToken(());

impl ProofToken for RustStdTrySendErrorToken {
    type Proposition = RustStdStandard<TrySendError<i32>>;
}

impl Establish<KaniChannelTrySendFullToken, KaniVerifier> for RustStdStandard<TrySendError<i32>> {
    type Token = RustStdTrySendErrorToken;

    fn establish(_credential: KaniChannelTrySendFullToken) -> Self::Token {
        RustStdTrySendErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_SEND_ERROR_FULL_RECOVERS_THE_UNSENT_VALUE_SRC, {
        /// A zero-capacity (rendezvous) channel's `.try_send()` fails
        /// `Full` immediately, since no receiver is ready to rendezvous
        /// with — and the error still recovers the unsent value.
        /// Same `KaniChannel` model migration as the other `send`-family
        /// proofs above: a `bounded(0)` channel models the rendezvous
        /// capacity directly. The claim is established through
        /// `Establish<KaniChannel<i32>, KaniVerifier> for
        /// RustStdStandard<TrySendError<i32>>` from the channel instance
        /// that actually demonstrated the recovery, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_try_send_error_full_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let channel = crate::KaniChannel::bounded(0);
            let demonstration = channel.demonstrate_try_send_full_recovers_value(value);

            let _token = RustStdStandard::<TrySendError<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::mpsc::TryRecvError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_recv_error_distinguishes_empty_from_disconnected".to_owned(),
            claim: VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::TryRecvError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryRecvError>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::mpsc::TryRecvError> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniChannel<i32>` instance actually distinguished
/// `.try_recv()`'s `Empty` and `Disconnected` failure modes, minted only
/// by [`KaniChannel::demonstrate_try_recv_distinguishes_disconnect`].
pub struct KaniChannelTryRecvToken(());

impl ProofToken for KaniChannelTryRecvToken {
    type Proposition = KaniChannel<i32>;
}

impl KaniChannel<i32> {
    /// Assert `.try_recv()` fails `Empty` on an open, empty channel, then,
    /// after dropping the sender, assert it instead fails `Disconnected`.
    /// Consumes `self` for the same reason
    /// [`KaniChannel::demonstrate_delivery`] does.
    #[must_use]
    pub fn demonstrate_try_recv_distinguishes_disconnect(mut self) -> KaniChannelTryRecvToken {
        assert_eq!(self.try_recv(), Err(crate::KaniRecvError::Empty));

        self.drop_sender();
        assert_eq!(self.try_recv(), Err(crate::KaniRecvError::Disconnected));

        KaniChannelTryRecvToken(())
    }
}

/// Lawful token minted once `RustStdStandard<TryRecvError>`'s
/// empty-vs-disconnected distinction claim has been established from a
/// `KaniChannel<i32>` that has itself demonstrated both failure modes.
pub struct RustStdTryRecvErrorToken(());

impl ProofToken for RustStdTryRecvErrorToken {
    type Proposition = RustStdStandard<std::sync::mpsc::TryRecvError>;
}

impl Establish<KaniChannelTryRecvToken, KaniVerifier>
    for RustStdStandard<std::sync::mpsc::TryRecvError>
{
    type Token = RustStdTryRecvErrorToken;

    fn establish(_credential: KaniChannelTryRecvToken) -> Self::Token {
        RustStdTryRecvErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC, {
        /// `.try_recv()`'s two failure modes are distinct, same as
        /// `RecvTimeoutError`'s: an open, empty channel is `Empty`; a
        /// disconnected one is `Disconnected` instead.
        /// Same `KaniChannel` model migration as `TryIter`'s proof above.
        /// The claim is established through `Establish<KaniChannel<i32>,
        /// KaniVerifier> for RustStdStandard<TryRecvError>` from the
        /// channel instance that actually demonstrated both failure modes,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_try_recv_error_distinguishes_empty_from_disconnected() {
            let channel = crate::KaniChannel::<i32>::unbounded();
            let demonstration = channel.demonstrate_try_recv_distinguishes_disconnect();

            let _token =
                RustStdStandard::<std::sync::mpsc::TryRecvError>::establish(demonstration);
        }
    }
}
