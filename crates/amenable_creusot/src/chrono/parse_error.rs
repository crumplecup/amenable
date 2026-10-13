//! `CreusotWitness` bridge for `amenable_ext::ExtStandard<chrono_tz::ParseError>`.
//!
//! The real proof content (`harness!`) lives in the unconditional
//! `crate::ext_chrono::parse_error` sibling. This file wires its claim
//! constant into the witness, and -- since that claim is `#[trusted]`,
//! not mechanically discharged -- also names the bound explicitly via
//! `Ensures<CreusotVerifier>`, the same shape
//! `amenable_std::rust_std_witness::string_carrier`'s own `Ensures`
//! impl uses for a bound with no exec representation. See `ext_chrono::
//! parse_error`'s own doc comment for the confirmed reason this rests
//! on an accommodation model rather than a mechanically-checked proof:
//! `creusot-std`'s string view has no grounding for any literal at all,
//! a real gap in Creusot's string model, not this claim's own
//! formulation.

use crate::ext_bridge::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotVerifier, CreusotWitness, VERIFY_TZ_FROM_STR_REJECTS_UNKNOWN_NAME_SRC};
use amenable_core::{Ensures, Evidence};
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<chrono_tz::ParseError> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_from_str_rejects_unknown_name".to_owned(),
            VERIFY_TZ_FROM_STR_REJECTS_UNKNOWN_NAME_SRC.to_owned(),
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

/// Returns `amenable_creusot::VERIFY_TZ_FROM_STR_REJECTS_UNKNOWN_NAME_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[trusted]` wrapper, not a hand-retyped copy of its claim. Named
/// here, not left as a bare `#[trusted]` floating on the harness alone:
/// the accommodation model's whole point is that this specific bound
/// (creusot-std's string view can't ground any literal) is audited and
/// tied to the evidence type it backs, not silently assumed.
impl Ensures<CreusotVerifier> for ExtStandard<chrono_tz::ParseError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VERIFY_TZ_FROM_STR_REJECTS_UNKNOWN_NAME_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::ParseError>",
        "creusot",
        "ensures",
        || <ExtStandard<chrono_tz::ParseError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
