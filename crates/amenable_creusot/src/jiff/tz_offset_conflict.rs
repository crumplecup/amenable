//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::tz::OffsetConflict>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::tz_offset_conflict` sibling instead (see this
//! crate's own root doc comment for why); this file only wires the
//! `VERIFY_TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::tz::OffsetConflict> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_offset_conflict_always_offset_and_always_time_zone".to_owned(),
            VERIFY_TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::tz::OffsetConflict>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::OffsetConflict>",
        "creusot",
        || <ExtStandard<jiff::tz::OffsetConflict> as CreusotWitness>::proof().to_string(),
    )
}
