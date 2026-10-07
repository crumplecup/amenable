//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::IsoWeek>`.
//!
//! The proof content lives in the unconditional `crate::ext_chrono::iso_week`
//! sibling. This file wires its claim constant into the witness.
//!
//! The witness is conditional. The proof holds for the model, over every
//! input satisfying its requires. It holds for chrono's real
//! `iso_week`/`from_isoywd_opt` only if that pair refines the model's own
//! `m1`/`weeks_in_year` relationship for whichever ISO year a date's week
//! falls in. That refinement premise is stated in `REFINEMENT_PREMISE`, and
//! it is not discharged by Creusot.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_ISO_WEEK_ROUND_TRIPS_MODEL_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

/// The refinement premise the proof rests on. Stated, not discharged by Creusot.
pub const REFINEMENT_PREMISE: &str = "chrono's real NaiveDate::iso_week/from_isoywd_opt \
    refine verify_iso_week_round_trips_model's m1/weeks_in_year relationship: for any \
    date, there is a real Monday m1 (the start of week 1 of that date's real ISO year) \
    and a real week count (52 or 53, that year's real number of ISO weeks) such that \
    the date's own week-Monday falls in m1 .. m1 + 7 * weeks_in_year, on the same 7-day \
    grid as m1 -- confirmed by amenable_kani::chrono::civil_iso_week's own harness, \
    checked directly against chrono's real API.";

impl CreusotWitness for ExtStandard<chrono::IsoWeek> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_iso_week_round_trips_model".to_owned(),
            format!(
                "{VERIFY_ISO_WEEK_ROUND_TRIPS_MODEL_SRC}\n// refinement premise (trusted): {REFINEMENT_PREMISE}"
            ),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::IsoWeek>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::IsoWeek>",
        "creusot",
        || <ExtStandard<chrono::IsoWeek> as CreusotWitness>::proof().to_string(),
    )
}
