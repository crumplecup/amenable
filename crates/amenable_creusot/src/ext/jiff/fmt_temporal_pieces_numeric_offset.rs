//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>` — the real
//! proof content (`extern_spec!`/`harness!`) lives in the
//! unconditional `crate::ext_jiff::fmt_temporal_pieces_numeric_offset`
//! sibling instead (see this crate's own root doc comment for why);
//! this file only wires the
//! `VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC`
//! constant it emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{
    CreusotWitness, VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC,
};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::temporal::PiecesNumericOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero".to_owned(),
            VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>",
        "creusot",
        || <ExtStandard<jiff::fmt::temporal::PiecesNumericOffset> as CreusotWitness>::proof()
            .to_string(),
    )
}
