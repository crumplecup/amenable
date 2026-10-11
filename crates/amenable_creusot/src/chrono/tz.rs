//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono_tz::Tz>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the
//! unconditional `crate::ext_chrono::tz` sibling. This file only wires
//! its claim constant into the witness.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_TZ_NAME_AND_FROM_STR_ROUND_TRIP_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono_tz::Tz> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_name_and_from_str_round_trip".to_owned(),
            VERIFY_TZ_NAME_AND_FROM_STR_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono_tz::Tz>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::Tz>",
        "creusot",
        || <ExtStandard<chrono_tz::Tz> as CreusotWitness>::proof().to_string(),
    )
}
