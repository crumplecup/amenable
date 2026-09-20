//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::WeekdaysReverse>` — the real proof
//! content (`harness!`, an accommodation model) lives in the
//! unconditional `crate::ext_jiff::civil_weekdays_reverse` sibling
//! instead (see this crate's own root doc comment for why); this
//! file only wires the
//! `VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::WeekdaysReverse> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor".to_owned(),
            VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::WeekdaysReverse>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::WeekdaysReverse>",
        "creusot",
        || <ExtStandard<jiff::civil::WeekdaysReverse> as CreusotWitness>::proof().to_string(),
    )
}
