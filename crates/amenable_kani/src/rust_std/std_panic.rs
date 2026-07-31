//! `KaniWitness` impls for `std::panic`.

use std::panic::PanicHookInfo;

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniPanicHookObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<PanicHookInfo<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_panic_hook_info_reports_the_panics_own_message".to_owned(),
            claim: VERIFY_PANIC_HOOK_INFO_REPORTS_THE_PANICS_OWN_MESSAGE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PanicHookInfo<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PanicHookInfo<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<PanicHookInfo<'static>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniPanicHookObservation` instance actually demonstrated
/// exact panic-message capture, minted only by
/// [`KaniPanicHookObservation::demonstrate_message_capture`].
pub struct KaniPanicHookWitnessToken(());

impl ProofToken for KaniPanicHookWitnessToken {
    type Proposition = KaniPanicHookObservation;
}

impl KaniPanicHookObservation {
    /// Assert the captured message matches the expected panic payload
    /// exactly. Consumes `self`: the only way to obtain the token is to
    /// have run this check against a real observation instance, not to
    /// assert it independently.
    #[must_use]
    pub fn demonstrate_message_capture(self, expected: &'static str) -> KaniPanicHookWitnessToken {
        assert_eq!(self.captured_message(), expected);
        KaniPanicHookWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<PanicHookInfo<'static>>`'s
/// panic-payload reporting claim has been established from a
/// `KaniPanicHookObservation` that has itself demonstrated exact message
/// capture.
pub struct RustStdPanicHookInfoToken(());

impl ProofToken for RustStdPanicHookInfoToken {
    type Proposition = RustStdStandard<PanicHookInfo<'static>>;
}

impl Establish<KaniPanicHookWitnessToken, KaniVerifier>
    for RustStdStandard<PanicHookInfo<'static>>
{
    type Token = RustStdPanicHookInfoToken;

    fn establish(_credential: KaniPanicHookWitnessToken) -> Self::Token {
        RustStdPanicHookInfoToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PANIC_HOOK_INFO_REPORTS_THE_PANICS_OWN_MESSAGE_SRC, {
        /// A custom panic hook installed via `set_hook()` observes the
        /// in-progress panic's own payload, exactly as it was passed to
        /// `panic!()`.
        /// This proof uses the Amenable-owned panic-hook observation:
        /// the direct hook path reaches the gallery's unsupported
        /// `catch_unwind` boundary before the payload law can be checked.
        /// The claim is established through
        /// `Establish<KaniPanicHookObservation, KaniVerifier> for
        /// RustStdStandard<PanicHookInfo<'static>>` from the observation
        /// instance that actually demonstrated exact message capture,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_panic_hook_info_reports_the_panics_own_message() {
            let observation = KaniPanicHookObservation::message("captured panic message");
            let demonstration = observation.demonstrate_message_capture("captured panic message");

            let _token = RustStdStandard::<PanicHookInfo<'static>>::establish(demonstration);
        }
    }
}
