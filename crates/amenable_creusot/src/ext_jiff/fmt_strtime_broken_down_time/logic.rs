#![cfg(creusot)]
//! `jiff::fmt::strtime::BrokenDownTime`'s opaque per-field accessors and
//! the `extern_spec!` bridge for its 12 numeric setter/getter pairs.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was thirteen separately `#[cfg(creusot)]`-gated items in the parent
//! file (twelve `fn`s, one `use`) down to a single macro-import `use`
//! there, cordial's CFG-SCATTER finding.
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

use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};

#[trusted]
#[logic(opaque)]
fn broken_down_time_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_month_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_day_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_day_of_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_iso_week_year_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i16> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_iso_week_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_week_sun_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_week_mon_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_hour_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_minute_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_second_value(_tm: &jiff::fmt::strtime::BrokenDownTime) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn broken_down_time_subsec_nanosecond_value(
    _tm: &jiff::fmt::strtime::BrokenDownTime,
) -> Option<i32> {
    dead
}

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
