use super::CheckedProof;

use std::time::Duration;

use crate::{
    CreusotVerifier, CreusotWitness, DURATION_NEW_HEADROOM_HOLDS_SRC,
    DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC,
    VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
};
use amenable_core::{Ensures, Evidence, Requires, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_duration_new_normalizes_nanos_and_carries_into_secs".to_string(),
            VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "creusot",
        || <RustStdStandard<Duration> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::DURATION_NEW_HEADROOM_HOLDS_SRC` /
/// `DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic]` fns the real site calls, not a hand-retyped copy of
/// their expressions.
impl Requires<CreusotVerifier> for RustStdStandard<Duration> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        DURATION_NEW_HEADROOM_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<Duration> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "creusot",
        "requires",
        || <RustStdStandard<Duration> as Requires<CreusotVerifier>>::requires(()),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "creusot",
        "ensures",
        || <RustStdStandard<Duration> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
