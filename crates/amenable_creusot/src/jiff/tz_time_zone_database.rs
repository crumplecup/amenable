//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::tz::TimeZoneDatabase>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::tz_time_zone_database` sibling instead (see this
//! crate's own root doc comment for why); this file only wires the
//! `VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::tz::TimeZoneDatabase> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_time_zone_database_none_is_definitively_empty".to_owned(),
            VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::tz::TimeZoneDatabase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::TimeZoneDatabase>",
        "creusot",
        || <ExtStandard<jiff::tz::TimeZoneDatabase> as CreusotWitness>::proof().to_string(),
    )
}
