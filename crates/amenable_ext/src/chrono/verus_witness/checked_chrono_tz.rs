//! Checked `chrono_tz::Tz`/`chrono_tz::TzOffset` -- real, hand-verified
//! Verus accommodation models, gated separately from `checked_civil`
//! since they need the `chrono-tz` feature specifically, not just
//! `chrono`.

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

impl_verus_witness_checked_ext!(
    chrono_tz::TzOffset,
    "verify_tz_offset_consistency_model",
    "../../../../amenable_verus/src/chrono/tz_offset.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono_tz::TzOffset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono_tz::TzOffset),
        ">"
    ),
    "tz_offset_consistency_holds"
);
