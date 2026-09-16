//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::ZonedSeries>` — the real proof content
//! (`harness!`, an accommodation model scoped to `TimeZone::UTC`)
//! lives in the unconditional `crate::ext_jiff::zoned_series` sibling
//! instead (see this crate's own root doc comment for why); this file
//! only wires the
//! `VERIFY_ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::ZonedSeries> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_zoned_series_next_yields_start_then_advances_by_period_under_utc".to_owned(),
            VERIFY_ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::ZonedSeries>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::ZonedSeries>",
        "creusot",
        || <ExtStandard<jiff::ZonedSeries> as CreusotWitness>::proof().to_string(),
    )
}
