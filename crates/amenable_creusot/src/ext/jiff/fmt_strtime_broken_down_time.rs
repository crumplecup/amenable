//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::strtime::BrokenDownTime>` — the real proof
//! content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::fmt_strtime_broken_down_time` sibling instead
//! (see this crate's own root doc comment for why); this file only
//! wires the
//! `VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::strtime::BrokenDownTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_strtime_broken_down_time_numeric_setters_round_trip".to_owned(),
            VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::strtime::BrokenDownTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::strtime::BrokenDownTime>",
        "creusot",
        || <ExtStandard<jiff::fmt::strtime::BrokenDownTime> as CreusotWitness>::proof()
            .to_string(),
    )
}
