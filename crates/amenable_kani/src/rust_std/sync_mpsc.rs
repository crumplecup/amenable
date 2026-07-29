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

/// Lawful token minted once `RustStdStandard<Sender<i32>>`'s delivery claim
/// has been established from a `KaniChannel<i32>` that has itself
/// demonstrated the sent value is receivable.
pub struct RustStdSenderToken(());

impl ProofToken for RustStdSenderToken {
    type Proposition = RustStdStandard<std::sync::mpsc::Sender<i32>>;
}

impl Establish<KaniChannel<i32>, KaniVerifier> for RustStdStandard<std::sync::mpsc::Sender<i32>> {
    type Token = RustStdSenderToken;

    fn establish(_credential: &KaniChannel<i32>) -> Self::Token {
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
            let mut channel = crate::KaniChannel::unbounded();
            channel.send(value).unwrap();
            assert_eq!(channel.recv(), Ok(value), "the sent value is receivable");

            let _token =
                RustStdStandard::<std::sync::mpsc::Sender<i32>>::establish(&channel);
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

amenable_derive::harness! {
    kani, VERIFY_SYNC_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC, {
        /// Same delivery contract as `Sender`, for a bounded channel
        /// with spare capacity.
        /// Same `KaniChannel` model migration as `Sender`'s proof above.
        #[kani::proof]
        fn verify_sync_sender_delivers_to_the_paired_receiver() {
            let value: i32 = kani::any();
            let mut channel = crate::KaniChannel::bounded(1);
            channel.send(value).unwrap();
            assert_eq!(channel.recv(), Ok(value));
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

amenable_derive::harness! {
    kani, VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC, {
        /// `.recv()` fails once the channel is both empty and every
        /// `Sender` has been dropped — it never blocks forever on a
        /// channel that can no longer receive anything.
        /// Same `KaniChannel` model migration as `Sender`'s proof above.
        #[kani::proof]
        fn verify_receiver_fails_once_every_sender_is_dropped() {
            let mut channel = crate::KaniChannel::<i32>::unbounded();
            channel.drop_sender();
            assert!(
                channel.recv().is_err(),
                "recv fails once the channel is empty and disconnected"
            );
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

amenable_derive::harness! {
    kani, VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.into_iter()` consumes the `Receiver`, yielding sent
        /// values and stopping once the channel is disconnected and
        /// drained.
        /// This proof uses the Amenable-owned channel model: `IntoIter`'s
        /// `next()` is `self.rx.recv().ok()` under the hood, so a
        /// `KaniChannel::recv()` call directly models the same
        /// yield-then-stop shape, conditional on the real `IntoIter`
        /// refining this law.
        #[kani::proof]
        fn verify_into_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let mut channel = crate::KaniChannel::unbounded();
            channel.send(value).unwrap();
            channel.drop_sender();
            assert_eq!(channel.recv().ok(), Some(value));
            assert_eq!(channel.recv().ok(), None);
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

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.iter()` borrows the `Receiver` instead of consuming it,
        /// with the same yield-then-stop behavior as `IntoIter`.
        /// Same `KaniChannel` model migration and rationale as
        /// `IntoIter`'s proof above -- `Iter::next()` is the same
        /// `recv().ok()` shape.
        #[kani::proof]
        fn verify_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let mut channel = crate::KaniChannel::unbounded();
            channel.send(value).unwrap();
            channel.drop_sender();
            assert_eq!(channel.recv().ok(), Some(value));
            assert_eq!(channel.recv().ok(), None);
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

amenable_derive::harness! {
    kani, VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC, {
        /// Unlike `Iter`, `.try_iter()` never blocks: on an empty
        /// channel whose `Sender` is still alive (where `Iter` would
        /// wait for a value), it returns `None` immediately.
        /// This proof uses the Amenable-owned channel model: `TryIter`'s
        /// `next()` is `self.rx.try_recv().ok()` under the hood, so
        /// `KaniChannel::try_recv()` models the same non-blocking shape.
        #[kani::proof]
        fn verify_try_iter_does_not_block_on_an_empty_open_channel() {
            let mut channel = crate::KaniChannel::<i32>::unbounded();
            assert_eq!(
                channel.try_recv().ok(),
                None,
                "try_iter returns None immediately rather than blocking"
            );
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

amenable_derive::harness! {
    kani, VERIFY_RECV_ERROR_ON_AN_EMPTY_DISCONNECTED_CHANNEL_SRC, {
        /// `.recv()` fails with exactly this error when the channel is
        /// empty and every `Sender` has been dropped.
        /// Same `KaniChannel` model migration as the other `recv`-family
        /// proofs above.
        #[kani::proof]
        fn verify_recv_error_on_an_empty_disconnected_channel() {
            let mut channel = crate::KaniChannel::<i32>::unbounded();
            channel.drop_sender();
            assert_eq!(channel.recv().unwrap_err(), crate::KaniRecvError::Disconnected);
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

amenable_derive::harness! {
    kani, VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC, {
        /// `.recv_timeout()`'s two failure modes are distinct: a
        /// channel that's open but empty times out; a disconnected
        /// channel fails immediately as `Disconnected` instead.
        #[kani::proof]
        fn verify_recv_timeout_error_distinguishes_timeout_from_disconnected() {
            use std::sync::mpsc::RecvTimeoutError;

            let (tx, rx) = std::sync::mpsc::channel::<i32>();
            assert_eq!(
                rx.recv_timeout(std::time::Duration::from_millis(0)),
                Err(RecvTimeoutError::Timeout),
                "an open, empty channel times out"
            );

            drop(tx);
            assert_eq!(
                rx.recv_timeout(std::time::Duration::from_millis(0)),
                Err(RecvTimeoutError::Disconnected),
                "a disconnected channel fails immediately instead"
            );
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

amenable_derive::harness! {
    kani, VERIFY_SEND_ERROR_RECOVERS_THE_UNSENT_VALUE_SRC, {
        /// `.send()` fails once the `Receiver` is dropped, and the
        /// error doesn't discard the value: it's recoverable via the
        /// error's own field.
        /// Same `KaniChannel` model migration as the other `send`-family
        /// proofs; the direct `std::sync::mpsc` path was confirmed to
        /// time out even for this single send with no recv involved at
        /// all.
        #[kani::proof]
        fn verify_send_error_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let mut channel = crate::KaniChannel::unbounded();
            channel.drop_receiver();
            let err = channel.send(value).unwrap_err();
            assert_eq!(
                err,
                crate::KaniSendError::Disconnected(value),
                "the unsent value is recoverable from the error"
            );
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

amenable_derive::harness! {
    kani, VERIFY_TRY_SEND_ERROR_FULL_RECOVERS_THE_UNSENT_VALUE_SRC, {
        /// A zero-capacity (rendezvous) channel's `.try_send()` fails
        /// `Full` immediately, since no receiver is ready to rendezvous
        /// with — and the error still recovers the unsent value.
        /// Same `KaniChannel` model migration as the other `send`-family
        /// proofs above: a `bounded(0)` channel models the rendezvous
        /// capacity directly.
        #[kani::proof]
        fn verify_try_send_error_full_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let mut channel = crate::KaniChannel::bounded(0);
            match channel.try_send(value) {
                Err(crate::KaniSendError::Full(v)) => {
                    assert_eq!(v, value, "the unsent value is recoverable from Full")
                }
                other => panic!("expected Full, got {other:?}"),
            }
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

amenable_derive::harness! {
    kani, VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC, {
        /// `.try_recv()`'s two failure modes are distinct, same as
        /// `RecvTimeoutError`'s: an open, empty channel is `Empty`; a
        /// disconnected one is `Disconnected` instead.
        /// Same `KaniChannel` model migration as `TryIter`'s proof above.
        #[kani::proof]
        fn verify_try_recv_error_distinguishes_empty_from_disconnected() {
            let mut channel = crate::KaniChannel::<i32>::unbounded();
            assert_eq!(channel.try_recv().unwrap_err(), crate::KaniRecvError::Empty);

            channel.drop_sender();
            assert_eq!(
                channel.try_recv().unwrap_err(),
                crate::KaniRecvError::Disconnected
            );
        }
    }
}
