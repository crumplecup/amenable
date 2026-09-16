//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::DateSeries>` — the real proof content
//! (`harness!`, an accommodation model) lives in the unconditional
//! `crate::ext_jiff::date_series` sibling instead (see this crate's
//! own root doc comment for why); this file only wires the
//! `VERIFY_DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::DateSeries> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_date_series_next_yields_start_then_advances_by_period".to_owned(),
            VERIFY_DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::DateSeries>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::DateSeries>",
        "creusot",
        || <ExtStandard<jiff::civil::DateSeries> as CreusotWitness>::proof().to_string(),
    )
}
