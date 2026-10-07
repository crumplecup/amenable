//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::NaiveDateTime>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_chrono::naive_date_time` sibling. This file only wires its claim
//! constant into the witness.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_NAIVE_DATE_TIME_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono::NaiveDateTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_date_time_round_trips".to_owned(),
            VERIFY_NAIVE_DATE_TIME_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::NaiveDateTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveDateTime>",
        "creusot",
        || <ExtStandard<chrono::NaiveDateTime> as CreusotWitness>::proof().to_string(),
    )
}
