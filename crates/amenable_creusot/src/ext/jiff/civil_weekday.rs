//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::Weekday>` — the real proof content
//! (`extern_spec!` reuse + `harness!`) lives in the unconditional
//! `crate::ext_jiff::civil_weekday` sibling instead (see this crate's
//! own root doc comment for why); this file only wires the
//! `VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC` constant
//! it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::Weekday> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_weekday_monday_one_offset_round_trips".to_owned(),
            VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::Weekday>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Weekday>",
        "creusot",
        || <ExtStandard<jiff::civil::Weekday> as CreusotWitness>::proof().to_string(),
    )
}
