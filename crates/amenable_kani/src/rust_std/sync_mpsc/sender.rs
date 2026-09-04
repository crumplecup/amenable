use std::sync::mpsc::SyncSender;

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniChannel, KaniVerifier};

impl KaniWitness for RustStdStandard<std::sync::mpsc::Sender<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_sender_delivers_to_the_paired_receiver".to_owned(),
            VERIFY_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::mpsc::Sender<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Sender<i32>>",
        "kani",
        || <RustStdStandard<std::sync::mpsc::Sender<i32>> as KaniWitness>::proof().to_string(),
    )
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_sync_sender_delivers_to_the_paired_receiver".to_owned(),
            VERIFY_SYNC_SENDER_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SyncSender<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SyncSender<i32>>",
        "kani",
        || <RustStdStandard<SyncSender<i32>> as KaniWitness>::proof().to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
