//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono_tz::ParseError>`.
//!
//! The real proof content (`harness!`) lives in the unconditional
//! `crate::ext_chrono::parse_error` sibling. This file only wires its
//! claim constant into the witness.
//!
//! Genuinely proven, not trusted-by-citation: see `ext_chrono::
//! parse_error`'s own doc comment for why a direct, mechanical proof
//! over the real `str` type isn't possible (`creusot-std`'s own
//! `str::view()` has no grounding for any literal), and how an
//! accommodation model -- the same move `ext_chrono::tz_offset`'s own
//! Verus model uses for `base`/`dst`/`fix` -- restates the same real
//! fact over plain integers instead, genuinely discharged by Creusot's
//! own prover, no `#[trusted]` anywhere.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono_tz::ParseError> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_zone_name_model_excludes_empty_query".to_owned(),
            VERIFY_ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono_tz::ParseError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::ParseError>",
        "creusot",
        || <ExtStandard<chrono_tz::ParseError> as CreusotWitness>::proof().to_string(),
    )
}
