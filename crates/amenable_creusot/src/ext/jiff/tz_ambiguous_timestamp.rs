//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::tz::AmbiguousTimestamp>` — the real proof
//! content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::tz_ambiguous_timestamp` sibling instead (see
//! this crate's own root doc comment for why); this file only wires
//! the
//! `VERIFY_TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::tz::AmbiguousTimestamp> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous".to_owned(),
            VERIFY_TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::tz::AmbiguousTimestamp>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::AmbiguousTimestamp>",
        "creusot",
        || <ExtStandard<jiff::tz::AmbiguousTimestamp> as CreusotWitness>::proof().to_string(),
    )
}
