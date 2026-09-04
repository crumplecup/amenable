use std::sync::mpsc::RecvError;

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<RecvError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_recv_error_on_an_empty_disconnected_channel".to_owned(),
            VERIFY_RECV_ERROR_ON_AN_EMPTY_DISCONNECTED_CHANNEL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RecvError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RecvError>",
        "kani",
        || <RustStdStandard<RecvError> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_recv_timeout_error_distinguishes_timeout_from_disconnected".to_owned(),
            VERIFY_RECV_TIMEOUT_ERROR_DISTINGUISHES_TIMEOUT_FROM_DISCONNECTED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::RecvTimeoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::RecvTimeoutError>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::RecvTimeoutError> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
