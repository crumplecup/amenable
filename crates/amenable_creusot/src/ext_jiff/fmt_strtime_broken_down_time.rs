//! Real Creusot proof content for `jiff::fmt::strtime::
//! BrokenDownTime`'s 12 numeric setter/getter round trips
//! (`ext::jiff::fmt_strtime_broken_down_time` holds the
//! `CreusotWitness` bridge that references the `_SRC` constant this
//! file's `harness!` call emits) — the same claim
//! `amenable_kani::ext::jiff::fmt_strtime_broken_down_time`'s own
//! harness checks by symbolic execution, see that module's own doc
//! comment for the real bounds and the scoping rationale (exactly
//! these 12 fields, not the 5 reference-type ones).
//!
//! The opaque per-field accessors and the `extern_spec!` bridging them
//! to jiff's real setters/getters live in the `logic` submodule
//! (self-gated, see its own doc comment for the real `^self`
//! prophecy-operator finding this checklist's first `&mut self`
//! setter extern-spec needed).

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_HOLDS_SRC, {
        /// The `amenable_ext::
        /// ExtStandard<jiff::fmt::strtime::BrokenDownTime>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn fmt_strtime_broken_down_time_numeric_setters_round_trip_holds(matches: bool) -> bool {
            pearlite! { matches }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_strtime_broken_down_time::fmt_strtime_broken_down_time_numeric_setters_round_trip_holds",
        "creusot",
        "ensures",
        || FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC, {
        /// Each of `BrokenDownTime`'s 12 numeric setters, whenever it
        /// succeeds on an in-range value, has its matching getter
        /// return exactly that value back — a real, checked
        /// postcondition resting on the `extern_spec!` above,
        /// checked for all 12 fields independently on a single fresh
        /// instance per field, matching `amenable_kani::ext::jiff::
        /// fmt_strtime_broken_down_time`'s own symbolic-execution
        /// claim.
        #[requires(true)]
        #[ensures(fmt_strtime_broken_down_time_numeric_setters_round_trip_holds(result))]
        fn verify_fmt_strtime_broken_down_time_numeric_setters_round_trip(
            year: i16,
            month: i8,
            day: i8,
            day_of_year: i16,
            iso_week_year: i16,
            iso_week: i8,
            week_sun: i8,
            week_mon: i8,
            hour: i8,
            minute: i8,
            second: i8,
            subsec_nanosecond: i32,
        ) -> bool {
            let mut tm1 = jiff::fmt::strtime::BrokenDownTime::default();
            let year_ok =
                tm1.set_year(Some(year)).is_err() || tm1.year() == Some(year);

            let mut tm2 = jiff::fmt::strtime::BrokenDownTime::default();
            let month_ok =
                tm2.set_month(Some(month)).is_err() || tm2.month() == Some(month);

            let mut tm3 = jiff::fmt::strtime::BrokenDownTime::default();
            let day_ok = tm3.set_day(Some(day)).is_err() || tm3.day() == Some(day);

            let mut tm4 = jiff::fmt::strtime::BrokenDownTime::default();
            let day_of_year_ok = tm4.set_day_of_year(Some(day_of_year)).is_err()
                || tm4.day_of_year() == Some(day_of_year);

            let mut tm5 = jiff::fmt::strtime::BrokenDownTime::default();
            let iso_week_year_ok = tm5.set_iso_week_year(Some(iso_week_year)).is_err()
                || tm5.iso_week_year() == Some(iso_week_year);

            let mut tm6 = jiff::fmt::strtime::BrokenDownTime::default();
            let iso_week_ok = tm6.set_iso_week(Some(iso_week)).is_err()
                || tm6.iso_week() == Some(iso_week);

            let mut tm7 = jiff::fmt::strtime::BrokenDownTime::default();
            let week_sun_ok = tm7.set_sunday_based_week(Some(week_sun)).is_err()
                || tm7.sunday_based_week() == Some(week_sun);

            let mut tm8 = jiff::fmt::strtime::BrokenDownTime::default();
            let week_mon_ok = tm8.set_monday_based_week(Some(week_mon)).is_err()
                || tm8.monday_based_week() == Some(week_mon);

            let mut tm9 = jiff::fmt::strtime::BrokenDownTime::default();
            let hour_ok = tm9.set_hour(Some(hour)).is_err() || tm9.hour() == Some(hour);

            let mut tm10 = jiff::fmt::strtime::BrokenDownTime::default();
            let minute_ok =
                tm10.set_minute(Some(minute)).is_err() || tm10.minute() == Some(minute);

            let mut tm11 = jiff::fmt::strtime::BrokenDownTime::default();
            let second_ok =
                tm11.set_second(Some(second)).is_err() || tm11.second() == Some(second);

            let mut tm12 = jiff::fmt::strtime::BrokenDownTime::default();
            let subsec_nanosecond_ok = tm12.set_subsec_nanosecond(Some(subsec_nanosecond)).is_err()
                || tm12.subsec_nanosecond() == Some(subsec_nanosecond);

            year_ok
                && month_ok
                && day_ok
                && day_of_year_ok
                && iso_week_year_ok
                && iso_week_ok
                && week_sun_ok
                && week_mon_ok
                && hour_ok
                && minute_ok
                && second_ok
                && subsec_nanosecond_ok
        }
    }
}
