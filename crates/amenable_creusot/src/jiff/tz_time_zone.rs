//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::tz::TimeZone>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::tz_time_zone` sibling instead (see this crate's
//! own root doc comment for why); this file only wires the
//! `VERIFY_TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_SRC` constant it
//! emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::tz::TimeZone> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_time_zone_unknown_and_fixed_round_trip".to_owned(),
            VERIFY_TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::tz::TimeZone>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::TimeZone>",
        "creusot",
        || <ExtStandard<jiff::tz::TimeZone> as CreusotWitness>::proof().to_string(),
    )
}
