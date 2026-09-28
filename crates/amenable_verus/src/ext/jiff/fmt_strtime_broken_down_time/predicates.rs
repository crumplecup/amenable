//! Range and post-state predicates for `BrokenDownTimeModel`'s 12
//! numeric setters — split out of `mod.rs` to keep that file under the
//! modularity line limit (`cordial`'s own "grow a subtree" suggestion
//! for a file whose extractable helpers form a named layer).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use super::BrokenDownTimeModel;

verus! {

/// `new()`'s result matches its own ghost value, named so the
/// exec-to-spec link is a citable fact.
pub open spec fn broken_down_time_new_matches_new_spec(result: BrokenDownTimeModel) -> bool {
    result == BrokenDownTimeModel::new_spec()
}

/// The `-9999..=9999` range shared by `set_year`/`set_iso_week_year`.
pub open spec fn broken_down_time_year_in_range(year: i16, result: bool) -> bool {
    result == (-9999 <= year && year <= 9999)
}

/// The `1..=12` range for `set_month`.
pub open spec fn broken_down_time_month_in_range(month: i8, result: bool) -> bool {
    result == (1 <= month && month <= 12)
}

/// The `1..=31` range for `set_day`.
pub open spec fn broken_down_time_day_in_range(day: i8, result: bool) -> bool {
    result == (1 <= day && day <= 31)
}

/// The `1..=366` range for `set_day_of_year`.
pub open spec fn broken_down_time_day_of_year_in_range(day_of_year: i16, result: bool) -> bool {
    result == (1 <= day_of_year && day_of_year <= 366)
}

/// The `1..=53` range for `set_iso_week`.
pub open spec fn broken_down_time_iso_week_in_range(week_number: i8, result: bool) -> bool {
    result == (1 <= week_number && week_number <= 53)
}

/// The `0..=53` range shared by `set_sunday_based_week`/
/// `set_monday_based_week`.
pub open spec fn broken_down_time_week_number_in_range(week_number: i8, result: bool) -> bool {
    result == (0 <= week_number && week_number <= 53)
}

/// The `0..=23` range for `set_hour`.
pub open spec fn broken_down_time_hour_in_range(hour: i8, result: bool) -> bool {
    result == (0 <= hour && hour <= 23)
}

/// The `0..=59` range shared by `set_minute`/`set_second`.
pub open spec fn broken_down_time_minute_or_second_in_range(value: i8, result: bool) -> bool {
    result == (0 <= value && value <= 59)
}

/// The `0..=999_999_999` range for `set_subsec_nanosecond`.
pub open spec fn broken_down_time_subsec_nanosecond_in_range(
    subsec_nanosecond: i32,
    result: bool,
) -> bool {
    result == (0 <= subsec_nanosecond && subsec_nanosecond <= 999999999)
}

/// `set_year`'s post-state: `year` set on success, preserved on failure.
pub open spec fn broken_down_time_year_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    year: i16,
    result: bool,
) -> bool {
    (result ==> after.year == Some(year)) && (!result ==> after.year == before.year)
}

/// `set_month`'s post-state: `month` set on success, preserved on failure.
pub open spec fn broken_down_time_month_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    month: i8,
    result: bool,
) -> bool {
    (result ==> after.month == Some(month)) && (!result ==> after.month == before.month)
}

/// `set_day`'s post-state: `day` set on success, preserved on failure.
pub open spec fn broken_down_time_day_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    day: i8,
    result: bool,
) -> bool {
    (result ==> after.day == Some(day)) && (!result ==> after.day == before.day)
}

/// `set_day_of_year`'s post-state: `day_of_year` set on success,
/// preserved on failure.
pub open spec fn broken_down_time_day_of_year_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    day_of_year: i16,
    result: bool,
) -> bool {
    (result ==> after.day_of_year == Some(day_of_year))
        && (!result ==> after.day_of_year == before.day_of_year)
}

/// `set_iso_week_year`'s post-state: `iso_week_year` set on success,
/// preserved on failure.
pub open spec fn broken_down_time_iso_week_year_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    year: i16,
    result: bool,
) -> bool {
    (result ==> after.iso_week_year == Some(year))
        && (!result ==> after.iso_week_year == before.iso_week_year)
}

/// `set_iso_week`'s post-state: `iso_week` set on success, preserved on
/// failure.
pub open spec fn broken_down_time_iso_week_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    week_number: i8,
    result: bool,
) -> bool {
    (result ==> after.iso_week == Some(week_number))
        && (!result ==> after.iso_week == before.iso_week)
}

/// `set_sunday_based_week`'s post-state: `week_sun` set on success,
/// preserved on failure.
pub open spec fn broken_down_time_week_sun_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    week_number: i8,
    result: bool,
) -> bool {
    (result ==> after.week_sun == Some(week_number))
        && (!result ==> after.week_sun == before.week_sun)
}

/// `set_monday_based_week`'s post-state: `week_mon` set on success,
/// preserved on failure.
pub open spec fn broken_down_time_week_mon_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    week_number: i8,
    result: bool,
) -> bool {
    (result ==> after.week_mon == Some(week_number))
        && (!result ==> after.week_mon == before.week_mon)
}

/// `set_hour`'s post-state: `hour` set on success, preserved on
/// failure.
pub open spec fn broken_down_time_hour_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    hour: i8,
    result: bool,
) -> bool {
    (result ==> after.hour == Some(hour)) && (!result ==> after.hour == before.hour)
}

/// `set_minute`'s post-state: `minute` set on success, preserved on
/// failure.
pub open spec fn broken_down_time_minute_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    minute: i8,
    result: bool,
) -> bool {
    (result ==> after.minute == Some(minute)) && (!result ==> after.minute == before.minute)
}

/// `set_second`'s post-state: `second` set on success, preserved on
/// failure.
pub open spec fn broken_down_time_second_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    second: i8,
    result: bool,
) -> bool {
    (result ==> after.second == Some(second)) && (!result ==> after.second == before.second)
}

/// `set_subsec_nanosecond`'s post-state: `subsec_nanosecond` set on
/// success, preserved on failure.
pub open spec fn broken_down_time_subsec_nanosecond_set_or_preserved(
    before: BrokenDownTimeModel,
    after: BrokenDownTimeModel,
    subsec_nanosecond: i32,
    result: bool,
) -> bool {
    (result ==> after.subsec_nanosecond == Some(subsec_nanosecond))
        && (!result ==> after.subsec_nanosecond == before.subsec_nanosecond)
}

} // verus!
