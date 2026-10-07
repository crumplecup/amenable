//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::NaiveWeek>`.
//!
//! The proof content lives in the unconditional `crate::ext_chrono::naive_week`
//! sibling. This file wires its claim constant into the witness.
//!
//! The witness is conditional. The proof holds for the model, over every
//! input. It holds for chrono's real `checked_first_day`/`checked_last_day`
//! only if that pair refines the model's own back/forward relationship over
//! chrono's supported day-count range. That refinement premise is stated in
//! `REFINEMENT_PREMISE`, and it is not discharged by Creusot.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_NAIVE_WEEK_SPAN_MODEL_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

/// The refinement premise the proof rests on. Stated, not discharged by Creusot.
pub const REFINEMENT_PREMISE: &str = "chrono::NaiveWeek's real checked_first_day/ \
    checked_last_day refine verify_naive_week_span_model's back/forward relationship: \
    representing the week's reference date by its real day-count, checked_first_day \
    steps back by the number of days (0..=6) separating the date from the week's start, \
    and checked_last_day steps forward by the complementary number of days (6 - back), \
    returning None exactly when that step falls outside chrono's supported day-count \
    range -- confirmed by amenable_kani::chrono::week_span_partitions's own 100 \
    year-partitioned harnesses, checked directly against chrono's real API.";

impl CreusotWitness for ExtStandard<chrono::NaiveWeek> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_week_span_model".to_owned(),
            format!(
                "{VERIFY_NAIVE_WEEK_SPAN_MODEL_SRC}\n// refinement premise (trusted): {REFINEMENT_PREMISE}"
            ),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::NaiveWeek>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveWeek>",
        "creusot",
        || <ExtStandard<chrono::NaiveWeek> as CreusotWitness>::proof().to_string(),
    )
}
