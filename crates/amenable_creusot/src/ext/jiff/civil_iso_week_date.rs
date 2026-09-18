//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::ISOWeekDate>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional `crate::
//! ext_jiff::civil_iso_week_date` sibling instead (see this crate's
//! own root doc comment for why); this file only wires the
//! `VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::ISOWeekDate> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_iso_week_date_new_year_week_weekday_round_trips".to_owned(),
            VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::ISOWeekDate>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::ISOWeekDate>",
        "creusot",
        || <ExtStandard<jiff::civil::ISOWeekDate> as CreusotWitness>::proof().to_string(),
    )
}
