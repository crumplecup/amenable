//! Checked `chrono::FixedOffset`/`chrono::NaiveDate` -- real, hand-
//! verified Verus accommodation models.

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

impl_verus_witness_checked_ext!(
    chrono::NaiveDate,
    "verify_naive_date_from_ymd_round_trips_model",
    "../../../../amenable_verus/src/chrono/naive_date.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono::NaiveDate>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono::NaiveDate),
        ">"
    ),
    "naive_date_from_ymd_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    chrono::NaiveTime,
    "verify_naive_time_from_hms_nano_round_trips_model",
    "../../../../amenable_verus/src/chrono/naive_time.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono::NaiveTime>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono::NaiveTime),
        ">"
    ),
    "naive_time_from_hms_nano_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    chrono::NaiveDateTime,
    "verify_naive_date_time_new_round_trips_model",
    "../../../../amenable_verus/src/chrono/naive_date_time.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono::NaiveDateTime>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono::NaiveDateTime),
        ">"
    ),
    "naive_date_time_new_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    chrono::NaiveWeek,
    "verify_naive_week_span_model",
    "../../../../amenable_verus/src/chrono/naive_week.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<chrono::NaiveWeek>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(chrono::NaiveWeek),
        ">"
    ),
    "naive_week_span_holds"
);
