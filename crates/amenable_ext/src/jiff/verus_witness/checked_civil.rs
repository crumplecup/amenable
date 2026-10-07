//! Checked `jiff::civil::*` types -- real, hand-verified Verus
//! accommodation models.

use crate::ExtStandard;
use crate::ext_verus_witness::{ExtCheckedProof, impl_verus_witness_checked_ext};
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    jiff::civil::Date,
    "verify_civil_date_new_year_month_day_round_trips_model",
    "../../../../amenable_verus/src/jiff/civil_date.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::Date>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::Date),
        ">"
    ),
    "civil_date_new_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Era,
    "verify_civil_era_year_classifies_bce_and_ce_correctly_model",
    "../../../../amenable_verus/src/jiff/civil_era.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::Era>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::Era),
        ">"
    ),
    "civil_era_year_in_range"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::Era>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::Era),
        ">"
    ),
    "civil_era_year_classifies_bce_and_ce_correctly"
);

impl_verus_witness_checked_ext!(
    jiff::civil::ISOWeekDate,
    "verify_civil_iso_week_date_new_year_week_weekday_round_trips_model",
    "../../../../amenable_verus/src/jiff/civil_iso_week_date.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::ISOWeekDate>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::ISOWeekDate),
        ">"
    ),
    "civil_iso_week_date_new_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Time,
    "verify_civil_time_new_hour_minute_second_subsec_round_trips_model",
    "../../../../amenable_verus/src/jiff/civil_time.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::Time>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::Time),
        ">"
    ),
    "civil_time_new_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::WeekdaysReverse,
    "verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor",
    "../../../../amenable_verus/src/jiff/civil_weekdays_reverse.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::WeekdaysReverse>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::WeekdaysReverse),
        ">"
    ),
    "civil_weekdays_reverse_start_offset_in_range"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::WeekdaysReverse>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::WeekdaysReverse),
        ">"
    ),
    "civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::WeekdaysForward,
    "verify_civil_weekdays_forward_next_yields_start_then_its_successor",
    "../../../../amenable_verus/src/jiff/civil_weekdays_forward.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::WeekdaysForward>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::WeekdaysForward),
        ">"
    ),
    "civil_weekdays_forward_start_offset_in_range"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::WeekdaysForward>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::WeekdaysForward),
        ">"
    ),
    "civil_weekdays_forward_next_yields_start_then_its_successor_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Weekday,
    "verify_civil_weekday_monday_one_offset_round_trips_model",
    "../../../../amenable_verus/src/jiff/civil_weekday.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::Weekday>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::Weekday),
        ">"
    ),
    "civil_weekday_monday_one_offset_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::TimeSeries,
    "verify_civil_time_series_next_yields_start_then_advances_by_period",
    "../../../../amenable_verus/src/jiff/civil_time_series.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::TimeSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::TimeSeries),
        ">"
    ),
    [
        "civil_time_series_start_nanos_in_range",
        "civil_time_series_period_nanos_in_range"
    ]
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::TimeSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::TimeSeries),
        ">"
    ),
    "civil_time_series_next_yields_start_then_advances_by_period_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::DateSeries,
    "verify_date_series_next_yields_start_then_advances_by_period",
    "../../../../amenable_verus/src/jiff/date_series.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::DateSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::DateSeries),
        ">"
    ),
    [
        "date_series_start_days_in_range",
        "date_series_period_days_in_range"
    ]
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::DateSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::DateSeries),
        ">"
    ),
    "date_series_next_yields_start_then_advances_by_period_holds"
);

impl_verus_witness_checked_ext!(
    jiff::civil::DateTimeSeries,
    "verify_date_time_series_next_yields_start_then_advances_by_period",
    "../../../../amenable_verus/src/jiff/date_time_series.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::civil::DateTimeSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::DateTimeSeries),
        ">"
    ),
    [
        "date_time_series_start_days_in_range",
        "date_time_series_period_days_in_range"
    ]
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::civil::DateTimeSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::civil::DateTimeSeries),
        ">"
    ),
    "date_time_series_next_yields_start_then_advances_by_period_holds"
);
