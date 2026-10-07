//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono::NaiveDate>`.
//!
//! The proof content lives in the unconditional `crate::ext_chrono::civil_naive_date`
//! sibling. This file wires its claim constant into the witness.
//!
//! The witness is conditional. The proof holds for chrono's `NaiveDate::from_ymd_opt`
//! only if chrono refines the Amenable-owned model over chrono's supported year range.
//! That refinement premise is stated in `REFINEMENT_PREMISE`, and it is not discharged
//! by Creusot.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_NAIVE_DATE_MODEL_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

/// The refinement premise the proof rests on. Stated, not discharged by Creusot.
pub const REFINEMENT_PREMISE: &str = "chrono::NaiveDate::from_ymd_opt refines \
    gregorian_from_ymd_model over chrono's supported year range -262143..=262142 \
    (chrono 0.4.45's MIN_YEAR and MAX_YEAR): it returns Some exactly when the triple \
    is a valid proleptic Gregorian date, and a Some date reports back the same \
    year, month, and day.";

impl CreusotWitness for ExtStandard<chrono::NaiveDate> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_naive_date_model_round_trips".to_owned(),
            format!(
                "{VERIFY_NAIVE_DATE_MODEL_ROUND_TRIPS_SRC}\n// refinement premise (trusted): {REFINEMENT_PREMISE}"
            ),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono::NaiveDate>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveDate>",
        "creusot",
        || <ExtStandard<chrono::NaiveDate> as CreusotWitness>::proof().to_string(),
    )
}
