//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::civil::Era>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional `crate::
//! ext_jiff::civil_era` sibling instead (see this crate's own root
//! doc comment for why); this file only wires the
//! `VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::civil::Era> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_era_year_classifies_bce_and_ce_correctly".to_owned(),
            VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::civil::Era>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Era>",
        "creusot",
        || <ExtStandard<jiff::civil::Era> as CreusotWitness>::proof().to_string(),
    )
}
