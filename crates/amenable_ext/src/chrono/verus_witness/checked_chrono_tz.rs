//! Checked `chrono_tz::Tz` -- a real, hand-verified Verus accommodation
//! model, gated separately from `checked_civil` since it needs the
//! `chrono-tz` feature specifically, not just `chrono`.

use crate::ExtStandard;
use crate::ext_verus_witness::{ExtCheckedProof, impl_verus_witness_checked_ext};
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    chrono_tz::Tz,
    "verify_tz_name_round_trips_model",
    "../../../../amenable_verus/src/chrono/tz.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono_tz::Tz>,
    concat!("amenable_ext::ExtStandard<", stringify!(chrono_tz::Tz), ">"),
    "tz_name_round_trip_holds"
);
