//! `KaniWitness` impls for `std::sync::mpsc`.
//!
//! Every harness runs single-threaded: sending, then immediately
//! receiving (or checking a disconnect/timeout/full condition) without
//! ever blocking on another thread. This is enough to check each carrier's
//! own contract — the channel's transport behavior, not cross-thread
//! scheduling.

use std::sync::mpsc::{
    IntoIter, Iter, Receiver, RecvError, RecvTimeoutError, SendError, Sender, SyncSender, TryIter,
    TryRecvError, TrySendError,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Sender<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sender_delivers_to_the_paired_receiver",
            claim: VERIFY_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Sender<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Sender<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Sender<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC, {
        /// A value sent on an unbounded channel is receivable on the
        /// paired `Receiver`.
        #[kani::proof]
        fn verify_sender_delivers_to_the_paired_receiver() {
            let value: i32 = kani::any();
            let (tx, rx) = std::sync::mpsc::channel();
            tx.send(value).unwrap();
            assert_eq!(rx.recv(), Ok(value), "the sent value is receivable");
        }
    }
}

impl KaniWitness for RustStdStandard<SyncSender<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sync_sender_delivers_to_the_paired_receiver",
            claim: VERIFY_SYNC_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC,
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
        #[kani::proof]
        fn verify_sync_sender_delivers_to_the_paired_receiver() {
            let value: i32 = kani::any();
            let (tx, rx) = std::sync::mpsc::sync_channel(1);
            tx.send(value).unwrap();
            assert_eq!(rx.recv(), Ok(value));
        }
    }
}

impl KaniWitness for RustStdStandard<Receiver<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_receiver_fails_once_every_sender_is_dropped",
            claim: VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Receiver<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Receiver<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RECEIVER_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC, {
        /// `.recv()` fails once the channel is both empty and every
        /// `Sender` has been dropped — it never blocks forever on a
        /// channel that can no longer receive anything.
        #[kani::proof]
        fn verify_receiver_fails_once_every_sender_is_dropped() {
            let (tx, rx) = std::sync::mpsc::channel::<i32>();
            drop(tx);
            assert!(
                rx.recv().is_err(),
                "recv fails once the channel is empty and disconnected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_into_iter_yields_sent_values_then_stops",
            claim: VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<IntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_INTO_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.into_iter()` consumes the `Receiver`, yielding sent
        /// values and stopping once the channel is disconnected and
        /// drained.
        #[kani::proof]
        fn verify_into_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let (tx, rx) = std::sync::mpsc::channel();
            tx.send(value).unwrap();
            drop(tx);
            let mut it = rx.into_iter();
            assert_eq!(it.next(), Some(value));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_yields_sent_values_then_stops",
            claim: VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Iter<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_SENT_VALUES_THEN_STOPS_SRC, {
        /// `.iter()` borrows the `Receiver` instead of consuming it,
        /// with the same yield-then-stop behavior as `IntoIter`.
        #[kani::proof]
        fn verify_iter_yields_sent_values_then_stops() {
            let value: i32 = kani::any();
            let (tx, rx) = std::sync::mpsc::channel();
            tx.send(value).unwrap();
            drop(tx);
            let mut it = rx.iter();
            assert_eq!(it.next(), Some(value));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<TryIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_iter_does_not_block_on_an_empty_open_channel",
            claim: VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryIter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<TryIter<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_ITER_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC, {
        /// Unlike `Iter`, `.try_iter()` never blocks: on an empty
        /// channel whose `Sender` is still alive (where `Iter` would
        /// wait for a value), it returns `None` immediately.
        #[kani::proof]
        fn verify_try_iter_does_not_block_on_an_empty_open_channel() {
            let (_tx, rx) = std::sync::mpsc::channel::<i32>();
            assert_eq!(
                rx.try_iter().next(),
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
            harness: "verify_recv_error_on_an_empty_disconnected_channel",
            claim: VERIFY_RECV_ERROR_ON_AN_EMPTY_DISCONNECTED_CHANNEL_SRC,
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
        #[kani::proof]
        fn verify_recv_error_on_an_empty_disconnected_channel() {
            let (tx, rx) = std::sync::mpsc::channel::<i32>();
            drop(tx);
            assert_eq!(rx.recv(), Err(RecvError));
        }
    }
}

impl KaniWitness for RustStdStandard<RecvTimeoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_recv_timeout_error_distinguishes_timeout_from_disconnected",
            claim: VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RecvTimeoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RecvTimeoutError>",
        verifier: "kani",
        describe: || <RustStdStandard<RecvTimeoutError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC, {
        /// `.recv_timeout()`'s two failure modes are distinct: a
        /// channel that's open but empty times out; a disconnected
        /// channel fails immediately as `Disconnected` instead.
        #[kani::proof]
        fn verify_recv_timeout_error_distinguishes_timeout_from_disconnected() {
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
            harness: "verify_send_error_recovers_the_unsent_value",
            claim: VERIFY_SEND_ERROR_RECOVERS_THE_UNSENT_VALUE_SRC,
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
        #[kani::proof]
        fn verify_send_error_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let (tx, rx) = std::sync::mpsc::channel();
            drop(rx);
            let err = tx.send(value).unwrap_err();
            assert_eq!(err.0, value, "the unsent value is recoverable from the error");
        }
    }
}

impl KaniWitness for RustStdStandard<TrySendError<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_send_error_full_recovers_the_unsent_value",
            claim: VERIFY_TRY_SEND_ERROR_FULL_RECOVERS_THE_UNSENT_VALUE_SRC,
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
        #[kani::proof]
        fn verify_try_send_error_full_recovers_the_unsent_value() {
            let value: i32 = kani::any();
            let (tx, _rx) = std::sync::mpsc::sync_channel(0);
            match tx.try_send(value) {
                Err(TrySendError::Full(v)) => {
                    assert_eq!(v, value, "the unsent value is recoverable from Full")
                }
                other => panic!("expected Full, got {other:?}"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<TryRecvError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_recv_error_distinguishes_empty_from_disconnected",
            claim: VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TryRecvError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryRecvError>",
        verifier: "kani",
        describe: || <RustStdStandard<TryRecvError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC, {
        /// `.try_recv()`'s two failure modes are distinct, same as
        /// `RecvTimeoutError`'s: an open, empty channel is `Empty`; a
        /// disconnected one is `Disconnected` instead.
        #[kani::proof]
        fn verify_try_recv_error_distinguishes_empty_from_disconnected() {
            let (tx, rx) = std::sync::mpsc::channel::<i32>();
            assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));

            drop(tx);
            assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
        }
    }
}
