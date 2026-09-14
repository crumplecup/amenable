//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<jiff::
//! SignedDuration>` — the real proof content (`extern_spec!`/
//! `harness!`) lives in the unconditional `crate::ext_jiff::
//! signed_duration` sibling instead (see this crate's own root doc
//! comment for why); this file only wires the
//! `VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC`
//! constant it emits into a `CreusotWitness` impl.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::SignedDuration> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_signed_duration_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::SignedDuration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::SignedDuration>",
        "creusot",
        || <ExtStandard<jiff::SignedDuration> as CreusotWitness>::proof().to_string(),
    )
}
