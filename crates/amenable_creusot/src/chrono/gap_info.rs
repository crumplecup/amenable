//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono_tz::GapInfo>`.
//!
//! The real proof content (`extern_spec!`/`harness!`) lives in the
//! unconditional `crate::ext_chrono::gap_info` sibling. This file only
//! wires its claim constant into the witness.
//!
//! Genuinely proven, not trusted-by-citation: `Tz::
//! offset_from_local_datetime`'s and `GapInfo::new`'s real `None`/`Some`
//! agreement is confirmed by Creusot's own prover against two
//! independently-stated `extern_spec!` axioms grounded in chrono-tz's
//! real source (see `ext_chrono::gap_info::logic`'s own doc comment).
//! `GapInfo::new`'s bracketing claim rests on a separate, direct axiom
//! with no real-execution counterpart, for the documented reason (a
//! confirmed Creusot/WHY3 keyword-escaping bug, not a modeling choice).

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_GAP_INFO_MATCHES_OFFSET_FROM_LOCAL_DATETIME_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono_tz::GapInfo> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_gap_info_matches_offset_from_local_datetime".to_owned(),
            VERIFY_GAP_INFO_MATCHES_OFFSET_FROM_LOCAL_DATETIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<chrono_tz::GapInfo>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::GapInfo>",
        "creusot",
        || <ExtStandard<chrono_tz::GapInfo> as CreusotWitness>::proof().to_string(),
    )
}
