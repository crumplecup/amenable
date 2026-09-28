//! Verus specs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). One file per contract,
//! each holding exactly the `verus! { ... }` spec function(s) its real
//! invariant needs — matching `rust_std`'s one-claim-per-carrier shape,
//! so `amenable_time::verus_witness` can `include_str!` a single file as
//! that contract's whole `claim`.

mod calendar_day_within_month_bounds_carrier;
mod calendar_month_carrier;
mod calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_carrier;
mod centennial_year_divisible_by_one_hundred_carrier;
mod century_ordinal_in_range_zero_to_ninety_nine_carrier;
mod common_year_has_three_hundred_sixty_five_calendar_days_carrier;
mod decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_carrier;
mod gregorian_leap_year_carrier;
mod hour_in_range_zero_to_twenty_four_carrier;
mod interval_duration_is_non_negative_carrier;
mod interval_start_precedes_end_carrier;
mod leap_day_occurs_only_in_leap_year_carrier;
mod leap_year_has_three_hundred_sixty_six_calendar_days_carrier;
mod minute_in_range_zero_to_fifty_nine_carrier;
mod month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_carrier;
mod ordinal_day_in_range_one_to_three_hundred_sixty_six_carrier;
mod second_in_range_zero_to_sixty_carrier;
mod utc_offset_hour_in_range_zero_to_twenty_three_carrier;
mod utc_offset_minute_in_range_zero_to_fifty_nine_carrier;
mod utc_timeline_ordering_applies_to_fixed_instants_carrier;
mod week_number_in_range_one_to_fifty_three_carrier;
mod weekday_in_range_one_to_seven_carrier;
mod year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_carrier;

pub use calendar_day_within_month_bounds_carrier::verify_calendar_day_within_month_bounds;
pub use calendar_month_carrier::verify_calendar_month_in_range;
pub use calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_carrier::verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine;
pub use centennial_year_divisible_by_one_hundred_carrier::verify_centennial_year_divisible_by_one_hundred;
pub use century_ordinal_in_range_zero_to_ninety_nine_carrier::verify_century_ordinal_in_range_zero_to_ninety_nine;
pub use common_year_has_three_hundred_sixty_five_calendar_days_carrier::verify_common_year_has_three_hundred_sixty_five_calendar_days;
pub use decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_carrier::verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine;
pub use gregorian_leap_year_carrier::verify_gregorian_leap_year;
pub use hour_in_range_zero_to_twenty_four_carrier::verify_hour_in_range_zero_to_twenty_four;
pub use interval_duration_is_non_negative_carrier::verify_interval_duration_is_non_negative;
pub use interval_start_precedes_end_carrier::verify_interval_start_precedes_end;
pub use leap_day_occurs_only_in_leap_year_carrier::verify_leap_day_occurs_only_in_leap_year;
pub use leap_year_has_three_hundred_sixty_six_calendar_days_carrier::verify_leap_year_has_three_hundred_sixty_six_calendar_days;
pub use minute_in_range_zero_to_fifty_nine_carrier::verify_minute_in_range_zero_to_fifty_nine;
pub use month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_carrier::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days;
pub use ordinal_day_in_range_one_to_three_hundred_sixty_six_carrier::verify_ordinal_day_in_range_one_to_three_hundred_sixty_six;
pub use second_in_range_zero_to_sixty_carrier::verify_second_in_range_zero_to_sixty;
pub use utc_offset_hour_in_range_zero_to_twenty_three_carrier::verify_utc_offset_hour_in_range_zero_to_twenty_three;
pub use utc_offset_minute_in_range_zero_to_fifty_nine_carrier::verify_utc_offset_minute_in_range_zero_to_fifty_nine;
pub use utc_timeline_ordering_applies_to_fixed_instants_carrier::verify_utc_timeline_ordering_applies_to_fixed_instants;
pub use week_number_in_range_one_to_fifty_three_carrier::verify_week_number_in_range_one_to_fifty_three;
pub use weekday_in_range_one_to_seven_carrier::verify_weekday_in_range_one_to_seven;
pub use year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_carrier::verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days;

// Every spec fn (`pub open spec fn`) across this directory's carriers is
// invisible under plain rustc (`verus! {}` erases spec content there),
// so their re-exports all share one real on/off switch. One
// `#[cfg(verus_keep_ghost)]` module instead of one gate per re-export
// (`cordial`'s own recommended fix for scattered identical `#[cfg]`
// predicates) — the harness fns above stay outside it since `verus!
// {}` preserves ordinary exec-fn declarations under plain rustc too.
#[cfg(verus_keep_ghost)]
mod ghost {
    pub use super::calendar_day_within_month_bounds_carrier::{
        calendar_day_feb29_result_implies_leap_year, calendar_day_within_bounds_result_implies_day_in_range,
        calendar_day_within_bounds_result_matches, calendar_day_within_month_bounds_holds,
        month_in_range_one_to_twelve_for_requires,
    };
    pub use super::calendar_month_carrier::{calendar_month_is_enumerated, calendar_month_range_result_matches};
    pub use super::calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_carrier::{
        calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds,
        calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_result_matches,
    };
    pub use super::centennial_year_divisible_by_one_hundred_carrier::{
        centennial_year_divisible_by_one_hundred_holds, centennial_year_result_matches,
    };
    pub use super::century_ordinal_in_range_zero_to_ninety_nine_carrier::{
        century_ordinal_in_range_zero_to_ninety_nine_holds,
        century_ordinal_in_range_zero_to_ninety_nine_result_matches,
    };
    pub use super::common_year_has_three_hundred_sixty_five_calendar_days_carrier::{
        common_year_day_count_matches_gregorian_rule, common_year_has_365_days_result_matches,
        common_year_has_three_hundred_sixty_five_calendar_days_holds,
    };
    pub use super::decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_carrier::{
        decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds,
        decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_result_matches,
    };
    pub use super::gregorian_leap_year_carrier::{
        gregorian_leap_year_holds, gregorian_leap_year_implies_divisible_by_four,
        gregorian_leap_year_result_matches,
    };
    pub use super::hour_in_range_zero_to_twenty_four_carrier::{
        hour_in_range_zero_to_twenty_four_holds, hour_in_range_zero_to_twenty_four_result_matches,
    };
    pub use super::interval_duration_is_non_negative_carrier::{
        interval_duration_is_non_negative_holds, interval_duration_is_non_negative_matches_start_le_end_form,
        interval_duration_is_non_negative_result_matches, interval_zero_span_iff_endpoints_equal,
    };
    pub use super::interval_start_precedes_end_carrier::{
        interval_start_precedes_end_holds, interval_start_precedes_end_matches_negated_form,
        interval_start_precedes_end_matches_nonneg_span_form, interval_start_precedes_end_result_matches,
    };
    pub use super::leap_day_occurs_only_in_leap_year_carrier::{
        leap_day_occurs_only_in_leap_year_holds, leap_day_result_matches,
        leap_day_validity_matches_gregorian_rule,
    };
    pub use super::leap_year_has_three_hundred_sixty_six_calendar_days_carrier::{
        days_in_year, days_in_year_is_365_or_366, leap_year_day_count_matches_gregorian_rule,
        leap_year_has_366_days_result_matches, leap_year_has_three_hundred_sixty_six_calendar_days_holds,
        year_is_non_negative,
    };
    pub use super::minute_in_range_zero_to_fifty_nine_carrier::{
        minute_in_range_zero_to_fifty_nine_holds, minute_in_range_zero_to_fifty_nine_result_matches,
    };
    pub use super::month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_carrier::{
        days_in_month, month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds,
        month_duration_result_matches, month_durations_sum_to_year_duration, valid_calendar_day,
    };
    pub use super::ordinal_day_in_range_one_to_three_hundred_sixty_six_carrier::{
        ordinal_day_in_range_one_to_three_hundred_sixty_six_at_least_1,
        ordinal_day_in_range_one_to_three_hundred_sixty_six_below_367,
        ordinal_day_in_range_one_to_three_hundred_sixty_six_holds,
        ordinal_day_in_range_one_to_three_hundred_sixty_six_result_matches,
    };
    pub use super::second_in_range_zero_to_sixty_carrier::{
        second_in_range_zero_to_sixty_holds, second_in_range_zero_to_sixty_result_matches,
    };
    pub use super::utc_offset_hour_in_range_zero_to_twenty_three_carrier::{
        utc_offset_hour_in_range_zero_to_twenty_three_holds,
        utc_offset_hour_in_range_zero_to_twenty_three_result_matches,
    };
    pub use super::utc_offset_minute_in_range_zero_to_fifty_nine_carrier::{
        utc_offset_minute_in_range_zero_to_fifty_nine_holds,
        utc_offset_minute_in_range_zero_to_fifty_nine_result_matches,
    };
    pub use super::utc_timeline_ordering_applies_to_fixed_instants_carrier::{
        utc_timeline_ordering_applies_to_fixed_instants_holds, utc_timeline_ordering_is_antisymmetric,
        utc_timeline_ordering_is_total, utc_timeline_ordering_is_transitive,
        utc_timeline_ordering_result_matches,
    };
    pub use super::week_number_in_range_one_to_fifty_three_carrier::{
        week_number_in_range_one_to_fifty_three_at_least_1, week_number_in_range_one_to_fifty_three_below_54,
        week_number_in_range_one_to_fifty_three_holds, week_number_in_range_one_to_fifty_three_result_matches,
    };
    pub use super::weekday_in_range_one_to_seven_carrier::{
        weekday_in_range_one_to_seven_holds, weekday_in_range_one_to_seven_result_matches,
    };
    pub use super::year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_carrier::{
        year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds,
        year_duration_result_matches,
    };
}

#[cfg(verus_keep_ghost)]
pub use ghost::{
    calendar_day_feb29_result_implies_leap_year,
    calendar_day_within_bounds_result_implies_day_in_range,
    calendar_day_within_bounds_result_matches, calendar_day_within_month_bounds_holds,
    calendar_month_is_enumerated, calendar_month_range_result_matches,
    calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds,
    calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_result_matches,
    centennial_year_divisible_by_one_hundred_holds, centennial_year_result_matches,
    century_ordinal_in_range_zero_to_ninety_nine_holds,
    century_ordinal_in_range_zero_to_ninety_nine_result_matches,
    common_year_day_count_matches_gregorian_rule, common_year_has_365_days_result_matches,
    common_year_has_three_hundred_sixty_five_calendar_days_holds, days_in_month, days_in_year,
    days_in_year_is_365_or_366, decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds,
    decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_result_matches,
    gregorian_leap_year_holds, gregorian_leap_year_implies_divisible_by_four,
    gregorian_leap_year_result_matches, hour_in_range_zero_to_twenty_four_holds,
    hour_in_range_zero_to_twenty_four_result_matches, interval_duration_is_non_negative_holds,
    interval_duration_is_non_negative_matches_start_le_end_form,
    interval_duration_is_non_negative_result_matches, interval_start_precedes_end_holds,
    interval_start_precedes_end_matches_negated_form,
    interval_start_precedes_end_matches_nonneg_span_form,
    interval_start_precedes_end_result_matches, interval_zero_span_iff_endpoints_equal,
    leap_day_occurs_only_in_leap_year_holds, leap_day_result_matches,
    leap_day_validity_matches_gregorian_rule, leap_year_day_count_matches_gregorian_rule,
    leap_year_has_366_days_result_matches,
    leap_year_has_three_hundred_sixty_six_calendar_days_holds,
    minute_in_range_zero_to_fifty_nine_holds, minute_in_range_zero_to_fifty_nine_result_matches,
    month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds,
    month_duration_result_matches, month_durations_sum_to_year_duration,
    month_in_range_one_to_twelve_for_requires,
    ordinal_day_in_range_one_to_three_hundred_sixty_six_at_least_1,
    ordinal_day_in_range_one_to_three_hundred_sixty_six_below_367,
    ordinal_day_in_range_one_to_three_hundred_sixty_six_holds,
    ordinal_day_in_range_one_to_three_hundred_sixty_six_result_matches,
    second_in_range_zero_to_sixty_holds, second_in_range_zero_to_sixty_result_matches,
    utc_offset_hour_in_range_zero_to_twenty_three_holds,
    utc_offset_hour_in_range_zero_to_twenty_three_result_matches,
    utc_offset_minute_in_range_zero_to_fifty_nine_holds,
    utc_offset_minute_in_range_zero_to_fifty_nine_result_matches,
    utc_timeline_ordering_applies_to_fixed_instants_holds, utc_timeline_ordering_is_antisymmetric,
    utc_timeline_ordering_is_total, utc_timeline_ordering_is_transitive,
    utc_timeline_ordering_result_matches, valid_calendar_day,
    week_number_in_range_one_to_fifty_three_at_least_1,
    week_number_in_range_one_to_fifty_three_below_54,
    week_number_in_range_one_to_fifty_three_holds,
    week_number_in_range_one_to_fifty_three_result_matches, weekday_in_range_one_to_seven_holds,
    weekday_in_range_one_to_seven_result_matches,
    year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds,
    year_duration_result_matches, year_is_non_negative,
};
