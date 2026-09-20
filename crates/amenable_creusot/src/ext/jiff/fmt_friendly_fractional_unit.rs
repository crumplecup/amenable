//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::friendly::FractionalUnit>` — the real proof
//! content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::fmt_friendly_fractional_unit` sibling instead
//! (see this crate's own root doc comment for why); this file only
//! wires the
//! `VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::friendly::FractionalUnit> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_friendly_fractional_unit_from_matches_documented_mapping".to_owned(),
            VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::friendly::FractionalUnit>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::friendly::FractionalUnit>",
        "creusot",
        || <ExtStandard<jiff::fmt::friendly::FractionalUnit> as CreusotWitness>::proof()
            .to_string(),
    )
}
