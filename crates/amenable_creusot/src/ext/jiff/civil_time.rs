//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::Time>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional `crate::
//! ext_jiff::civil_time` sibling instead (see this crate's own root
//! doc comment for why); this file only wires the
//! `VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::Time> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_time_new_hour_minute_second_subsec_round_trips".to_owned(),
            VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::Time>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Time>",
        "creusot",
        || <ExtStandard<jiff::civil::Time> as CreusotWitness>::proof().to_string(),
    )
}
