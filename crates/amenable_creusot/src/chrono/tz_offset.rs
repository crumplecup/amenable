//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono_tz::TzOffset>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the
//! unconditional `crate::ext_chrono::tz_offset` sibling. This file only
//! wires its claim constant into the witness.
//!
//! Genuinely proven, not trusted-by-citation: `fix() ==
//! base_utc_offset() + dst_offset()` is confirmed by Creusot's own
//! prover against three independently-stated `extern_spec!` axioms
//! grounded in chrono-tz's real source and its own public doctest
//! (see `ext_chrono::tz_offset::logic`'s own doc comment).

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_TZ_OFFSET_CONSISTENCY_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono_tz::TzOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_offset_consistency".to_owned(),
            VERIFY_TZ_OFFSET_CONSISTENCY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono_tz::TzOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::TzOffset>",
        "creusot",
        || <ExtStandard<chrono_tz::TzOffset> as CreusotWitness>::proof().to_string(),
    )
}
