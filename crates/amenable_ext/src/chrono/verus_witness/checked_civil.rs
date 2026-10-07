//! Checked `chrono::FixedOffset` -- a real, hand-verified Verus
//! accommodation model.

use crate::ExtStandard;
use crate::ext_verus_witness::{ExtCheckedProof, impl_verus_witness_checked_ext};
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    chrono::FixedOffset,
    "verify_fixed_offset_east_and_west_round_trips_model",
    "../../../../amenable_verus/src/chrono/fixed_offset.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono::FixedOffset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono::FixedOffset),
        ">"
    ),
    "fixed_offset_east_and_west_model_round_trip_holds"
);
