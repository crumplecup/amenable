//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::SpanFieldwise>` — the real proof content
//! (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::span_fieldwise` sibling instead (see this crate's
//! own root doc comment for why); this file only wires the
//! `VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::SpanFieldwise> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_span_fieldwise_negation_negates_every_unit_getter".to_owned(),
            VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::SpanFieldwise>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::SpanFieldwise>",
        "creusot",
        || <ExtStandard<jiff::SpanFieldwise> as CreusotWitness>::proof().to_string(),
    )
}
