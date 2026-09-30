//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>`
//! — the real proof content (`extern_spec!`/`harness!`) lives in the
//! unconditional `crate::ext_jiff::
//! fmt_temporal_time_zone_annotation_name` sibling instead (see this
//! crate's own root doc comment for why); this file only wires the
//! `VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips".to_owned(),
            VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>",
        "creusot",
        || <ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>> as CreusotWitness>::proof()
            .to_string(),
    )
}
