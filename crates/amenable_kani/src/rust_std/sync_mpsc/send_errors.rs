use std::sync::mpsc::{SendError, TrySendError};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<SendError<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_send_error_recovers_the_unsent_value".to_owned(),
            VERIFY_SEND_ERROR_RECOVERS_THE_UNSENT_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SendError<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SendError<i32>>",
        "kani",
        || <RustStdStandard<SendError<i32>> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_send_error_full_recovers_the_unsent_value".to_owned(),
            VERIFY_TRY_SEND_ERROR_FULL_RECOVERS_THE_UNSENT_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TrySendError<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TrySendError<i32>>",
        "kani",
        || <RustStdStandard<TrySendError<i32>> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn demonstrate_try_send_full_recovers_value(
        mut self,
        value: i32,
    ) -> KaniChannelTrySendFullToken {
        assert_eq!(
            self.try_send(value),
            Err(crate::KaniSendError::Full(value)),
            "the unsent value is recoverable from Full"
        );
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_recv_error_distinguishes_empty_from_disconnected".to_owned(),
            VERIFY_TRY_RECV_ERROR_DISTINGUISHES_EMPTY_FROM_DISCONNECTED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::TryRecvError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryRecvError>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::TryRecvError> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
