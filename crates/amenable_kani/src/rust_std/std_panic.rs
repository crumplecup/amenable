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

/// Lawful token minted once `RustStdStandard<PanicHookInfo<'static>>`'s
/// panic-payload reporting claim has been established from a
/// `KaniPanicHookObservation` that has itself demonstrated exact message
/// capture.
pub struct RustStdPanicHookInfoToken(());

impl ProofToken for RustStdPanicHookInfoToken {
    type Proposition = RustStdStandard<PanicHookInfo<'static>>;
}

impl Establish<KaniPanicHookObservation, KaniVerifier> for RustStdStandard<PanicHookInfo<'static>> {
    type Token = RustStdPanicHookInfoToken;

    fn establish(_credential: &KaniPanicHookObservation) -> Self::Token {
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
            assert_eq!(observation.captured_message(), "captured panic message");

            let _token = RustStdStandard::<PanicHookInfo<'static>>::establish(&observation);
        }
    }
}
