//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::FixedOffset>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_chrono::fixed_offset` sibling. This file only wires its claim
//! constant into the witness.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono::FixedOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fixed_offset_east_and_west_round_trip".to_owned(),
            VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::FixedOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::FixedOffset>",
        "creusot",
        || <ExtStandard<chrono::FixedOffset> as CreusotWitness>::proof().to_string(),
    )
}
