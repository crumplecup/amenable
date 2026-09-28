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
//! **Genuinely new territory for this checklist: the first real
//! `&mut self` setter extern-spec'd anywhere in `ext_jiff`.** Every
//! prior extern_spec in this family either built a fresh value
//! (`Date::new`) or left `self` untouched (`write_str`); `set_year`
//! etc. mutate `self` in place, so their postconditions need
//! Creusot's `^self` ("final") prophecy operator to state the
//! post-mutation value — the same operator `creusot-std`'s own
//! `Vec::push` extern_spec uses (`(^self)@ == self@.push_back(v))`),
//! just applied to our own opaque per-field accessor instead of a
//! built-in `View`, since `BrokenDownTime` has no `creusot-std`
//! coverage at all. `^self` yields `BrokenDownTime` BY VALUE, not
//! `&BrokenDownTime` — confirmed via a real "expected `&BrokenDownTime`,
//! found `BrokenDownTime`" compiler error on a first attempt calling
//! the accessor as `accessor(^self)`; the accessor (which takes
//! `&BrokenDownTime`, matching every other opaque accessor in this
//! checklist) needs `accessor(&^self)` instead.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_month_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_day_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_day_of_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_iso_week_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_iso_week_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_week_sun_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_week_mon_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_hour_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_minute_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_second_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn broken_down_time_subsec_nanosecond_value(
    _tm: &jiff::fmt::strtime::BrokenDownTime,
) -> Option<i32> {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::fmt::strtime::BrokenDownTime {
        #[check(ghost)]
        #[ensures(result == broken_down_time_year_value(self))]
        fn year(&self) -> Option<i16>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_year_value(&^self) == year,
            Err(_) => match year {
                Some(y) => y < -9999i16 || y > 9999i16,
                None => false,
            },
        })]
        fn set_year(&mut self, year: Option<i16>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_month_value(self))]
        fn month(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_month_value(&^self) == month,
            Err(_) => match month {
                Some(m) => m < 1i8 || m > 12i8,
                None => false,
            },
        })]
        fn set_month(&mut self, month: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_day_value(self))]
        fn day(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_day_value(&^self) == day,
            Err(_) => match day {
                Some(d) => d < 1i8 || d > 31i8,
                None => false,
            },
        })]
        fn set_day(&mut self, day: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_day_of_year_value(self))]
        fn day_of_year(&self) -> Option<i16>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_day_of_year_value(&^self) == day,
            Err(_) => match day {
                Some(d) => d < 1i16 || d > 366i16,
                None => false,
            },
        })]
        fn set_day_of_year(&mut self, day: Option<i16>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_iso_week_year_value(self))]
        fn iso_week_year(&self) -> Option<i16>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_iso_week_year_value(&^self) == year,
            Err(_) => match year {
                Some(y) => y < -9999i16 || y > 9999i16,
                None => false,
            },
        })]
        fn set_iso_week_year(&mut self, year: Option<i16>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_iso_week_value(self))]
        fn iso_week(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_iso_week_value(&^self) == week_number,
            Err(_) => match week_number {
                Some(w) => w < 1i8 || w > 53i8,
                None => false,
            },
        })]
        fn set_iso_week(&mut self, week_number: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_week_sun_value(self))]
        fn sunday_based_week(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_week_sun_value(&^self) == week_number,
            Err(_) => match week_number {
                Some(w) => w < 0i8 || w > 53i8,
                None => false,
            },
        })]
        fn set_sunday_based_week(&mut self, week_number: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_week_mon_value(self))]
        fn monday_based_week(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_week_mon_value(&^self) == week_number,
            Err(_) => match week_number {
                Some(w) => w < 0i8 || w > 53i8,
                None => false,
            },
        })]
        fn set_monday_based_week(&mut self, week_number: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_hour_value(self))]
        fn hour(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_hour_value(&^self) == hour,
            Err(_) => match hour {
                Some(h) => h < 0i8 || h > 23i8,
                None => false,
            },
        })]
        fn set_hour(&mut self, hour: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_minute_value(self))]
        fn minute(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_minute_value(&^self) == minute,
            Err(_) => match minute {
                Some(m) => m < 0i8 || m > 59i8,
                None => false,
            },
        })]
        fn set_minute(&mut self, minute: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_second_value(self))]
        fn second(&self) -> Option<i8>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_second_value(&^self) == second,
            Err(_) => match second {
                Some(s) => s < 0i8 || s > 59i8,
                None => false,
            },
        })]
        fn set_second(&mut self, second: Option<i8>) -> Result<(), jiff::Error>;

        #[check(ghost)]
        #[ensures(result == broken_down_time_subsec_nanosecond_value(self))]
        fn subsec_nanosecond(&self) -> Option<i32>;

        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => broken_down_time_subsec_nanosecond_value(&^self) == subsec_nanosecond,
            Err(_) => match subsec_nanosecond {
                Some(n) => n < 0i32 || n > 999_999_999i32,
                None => false,
            },
        })]
        fn set_subsec_nanosecond(
            &mut self,
            subsec_nanosecond: Option<i32>,
        ) -> Result<(), jiff::Error>;
    }
}

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
