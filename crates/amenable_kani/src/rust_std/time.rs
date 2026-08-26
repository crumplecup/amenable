//! `KaniWitness` impls for `core::time`.

use std::time::{Duration, TryFromFloatSecsError};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_duration_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "kani",
        || <RustStdStandard<Duration> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `Duration::new` does not require `nanos < 1_000_000_000` — it
        /// normalizes: any whole-second carry in `nanos` is added to
        /// `secs`, and `subsec_nanos()` reports the remainder. This states
        /// that full behavior (confirmed empirically: `Duration::new(5,
        /// 2_500_000_000)` yields `secs=7, nanos=500_000_000`) rather than
        /// assuming the carry never happens, which is the narrower claim
        /// a `nanos < 1_000_000_000` precondition would produce.
        #[kani::proof]
        fn verify_duration_new_normalizes_nanos_and_carries_into_secs() {
            let secs: u64 = kani::any();
            let nanos: u32 = kani::any();
            let carry = (nanos / 1_000_000_000) as u64;
            kani::assume(RustStdStandard::<u64>::requires((secs, carry)));

            let d = Duration::new(secs, nanos);
            assert_eq!(
                d.as_secs(),
                secs + carry,
                "Duration::new carries a nanos overflow into secs"
            );
            assert_eq!(
                d.subsec_nanos(),
                nanos % 1_000_000_000,
                "Duration::new normalizes nanos to below one second"
            );
        }
    }
}

impl_kani_witness_trusted!(TryFromFloatSecsError);
