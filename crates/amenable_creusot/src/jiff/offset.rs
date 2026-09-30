//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<jiff::tz::
//! Offset>` — the real proof content (`extern_spec!`/`harness!`) lives
//! in the unconditional `crate::ext_jiff::offset` sibling instead (see
//! this crate's own root doc comment for why); this file only wires the
//! `VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC` constant it emits into a
//! `CreusotWitness` impl, mirroring every `rust_std_witness` file's
//! relationship to its own `rust_std` sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::tz::Offset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_offset_from_seconds_round_trips".to_owned(),
            VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::tz::Offset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::Offset>",
        "creusot",
        || <ExtStandard<jiff::tz::Offset> as CreusotWitness>::proof().to_string(),
    )
}
