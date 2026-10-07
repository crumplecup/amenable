//! Verus accommodation model for `chrono::NaiveDate::iso_week`/
//! `NaiveDate::from_isoywd_opt`'s round trip: for a date, its ISO week's
//! year and week number, together with the date's own weekday, rebuild
//! the same date, and the week number lies in `1..=53`.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — see `fixed_offset.rs`'s own doc comment for the established
//! response this mirrors. This proof is conditional on the model being
//! faithful — which `amenable_kani::chrono::civil_iso_week`'s own
//! harness, checked directly against chrono's real
//! `iso_week`/`from_isoywd_opt`, independently confirms.
//!
//! Like `naive_week.rs`, a date is represented abstractly by its signed
//! day-count, `day_count: int` (position on the calendar's own number
//! line), with no Gregorian or leap-year arithmetic needed. Unlike
//! `naive_week.rs`, this does need to reach for chrono's real ISO
//! week-date algorithm (reading `NaiveDate::from_isoywd_opt`'s real
//! source first, not guessed): week 1 of an ISO year starts on a
//! specific Monday, `m1`, and each ISO year has either 52 or 53 whole
//! weeks before the next one's week 1 starts.
//!
//! Rather than re-deriving chrono's own rule for exactly which Monday
//! starts week 1 of a given year (`YearFlags`/`isoweek_delta`'s real
//! leap-week bookkeeping — real, but orthogonal to this claim, and
//! independently confirmed by the Kani witness above), this model takes
//! that Monday, `m1`, and that year's week count, `weeks_in_year`, as
//! given inputs, constrained only by the two real properties chrono's
//! own algorithm guarantees for whichever ISO year a date's week falls
//! in: `weeks_in_year` is `52` or `53`, and the date's own week-Monday
//! falls somewhere in `m1 .. m1 + 7 * weeks_in_year`. Given only that,
//! the round trip and the week-number bound both follow by exact
//! division arithmetic — the real reason chrono's own claim holds,
//! independently of which specific Monday starts which specific year
//! (a real but separate fact, not needed here).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// This date's weekday, as an index `0..=6` (Monday`..=`Sunday, in
/// whichever consistent numbering `day_count`'s own zero point implies
/// — the round trip holds for any such numbering, so this model does
/// not calibrate it against chrono's real epoch).
pub open spec fn iso_week_weekday_of(day_count: int) -> int {
    day_count % 7
}

/// The Monday starting this date's own week.
pub open spec fn iso_week_monday_of(day_count: int) -> int {
    day_count - iso_week_weekday_of(day_count)
}

/// The claim: when `m1` (the Monday starting ISO week 1 of this date's
/// ISO year) and `weeks_in_year` (that year's real week count, `52` or
/// `53`) are related to `day_count` the way chrono's real algorithm
/// guarantees, the week number computed from them lies in `1..=53`,
/// and reconstructing from `(iso_year, week_number, weekday)` yields
/// back the same `day_count`.
pub open spec fn iso_week_round_trip_holds(
    day_count: int,
    m1: int,
    weeks_in_year: int,
    week_number: int,
    rebuilt_day_count: int,
) -> bool {
    week_number >= 1 && week_number <= 53
        && rebuilt_day_count == day_count
}

/// Euclidean weekday index, proven equal to the specification -- Rust's
/// native `%` on `i64` truncates toward zero (a negative remainder for
/// a negative `day_count`), unlike Verus's own `int % 7`, which is
/// always non-negative for a positive divisor; this corrects for that
/// difference the same way `div_euclid`/`rem_euclid` would.
fn iso_week_weekday_exec(day_count: i64) -> (result: i64)
    ensures
        result as int == iso_week_weekday_of(day_count as int),
{
    let r = day_count % 7;
    if r < 0 { r + 7 } else { r }
}

/// A model of `(date.iso_week().year(), date.iso_week().week(),
/// date.weekday())`, then `NaiveDate::from_isoywd_opt` applied to that
/// triple: given `m1` and `weeks_in_year` for this date's real ISO
/// year (constrained the way chrono's own algorithm guarantees), the
/// computed week number and the rebuilt day-count satisfy the round
/// trip — the same claim `amenable_kani::chrono::civil_iso_week::
/// verify_iso_week_of_a_date_rebuilds_that_date` checks against
/// chrono's real API.
pub fn verify_iso_week_round_trips_model(
    day_count: i64,
    m1: i64,
    _weeks_in_year: i64,
    iso_year: i64,
) -> (result: (i64, i64, i64))
    requires
        _weeks_in_year == 52 || _weeks_in_year == 53,
        m1 <= iso_week_monday_of(day_count as int),
        iso_week_monday_of(day_count as int) < m1 + 7 * _weeks_in_year,
        (iso_week_monday_of(day_count as int) - m1) % 7 == 0,
        day_count >= -100_000_000 && day_count <= 100_000_000,
        m1 >= -100_000_000 && m1 <= 100_000_000,
    ensures
        iso_week_round_trip_holds(
            day_count as int,
            m1 as int,
            _weeks_in_year as int,
            result.1 as int,
            result.2 as int,
        ),
        result.0 == iso_year,
{
    let weekday = iso_week_weekday_exec(day_count);
    let monday = day_count - weekday;
    let week_number = (monday - m1) / 7 + 1;
    let rebuilt = m1 + (week_number - 1) * 7 + weekday;
    (iso_year, week_number, rebuilt)
}

} // verus!
