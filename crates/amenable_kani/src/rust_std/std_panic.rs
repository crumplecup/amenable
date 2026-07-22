//! `KaniWitness` impls for `std::panic`.

use std::panic::PanicHookInfo;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

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

amenable_derive::harness! {
    kani, VERIFY_PANIC_HOOK_INFO_REPORTS_THE_PANICS_OWN_MESSAGE_SRC, {
        /// A custom panic hook installed via `set_hook()` observes the
        /// in-progress panic's own payload, exactly as it was passed to
        /// `panic!()`.
        #[kani::proof]
        fn verify_panic_hook_info_reports_the_panics_own_message() {
            let captured: std::sync::Arc<std::sync::Mutex<Option<String>>> =
                std::sync::Arc::new(std::sync::Mutex::new(None));
            let captured_clone = captured.clone();

            std::panic::set_hook(Box::new(move |info| {
                let message = info.payload().downcast_ref::<&str>().map(|s| s.to_string());
                *captured_clone.lock().unwrap() = message;
            }));

            let result = std::panic::catch_unwind(|| {
                panic!("captured panic message");
            });
            assert!(result.is_err());

            let _ = std::panic::take_hook();
            assert_eq!(
                captured.lock().unwrap().clone(),
                Some("captured panic message".to_string())
            );
        }
    }
}
