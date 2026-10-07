//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::Utc>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_chrono::utc` sibling. This file only wires its claim constant into
//! the witness.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono::Utc> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_utc_local_offset_is_always_single_and_fixed_offset_is_zero".to_owned(),
            VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::Utc>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::Utc>",
        "creusot",
        || <ExtStandard<chrono::Utc> as CreusotWitness>::proof().to_string(),
    )
}
