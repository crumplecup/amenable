#![cfg(creusot)]
//! `jiff::fmt::temporal::Pieces<'static>`'s opaque per-field accessors
//! and the `extern_spec!` bridge for `date()`/`time()`/`with_date`/
//! `with_time`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was ten separately `#[cfg(creusot)]`-gated items in the parent file
//! down to zero there, cordial's CFG-SCATTER finding.
//!
//! Reuses `civil_date.rs`'s/`civil_time.rs`'s existing opaque
//! accessors directly (Creusot allows only one `extern_spec!` per real
//! function crate-wide). `Pieces`'s own `date()`/`time()` getters and
//! `with_date`/`with_time` setters can't be stated in terms of a
//! `Date`/`Time` VALUE directly (comparing two `Date`/`Time` values
//! via `==` hits the same `DeepModel` wall `unit.rs`'s own doc comment
//! documents — foreign types need a model Creusot can't derive), so
//! this file introduces its own Pieces-specific opaque accessors
//! returning already-DECOMPOSED primitives (`i16`/`i8`/`Option<i8>`/
//! etc.) instead, which compare fine via `==` with no `DeepModel`
//! requirement at all (the same reason `Option<i16>`/etc. comparisons
//! already work fine elsewhere in this checklist).
//!
//! **A real, confirmed finding**: a first harness chained
//! `.with_date(date).with_time(time)` on ONE `Pieces` value and
//! checked BOTH fields at the end — this compiled but the proof goal
//! genuinely failed. `why3find prove -X` (the real debugging tool,
//! not guesswork) showed why: neither `with_date`'s nor `with_time`'s
//! `extern_spec!` states a frame condition (that the OTHER field is
//! preserved, matching jiff's real `Pieces { date, ..self }`/`Pieces
//! { time: Some(time), ..self }` bodies), so Creusot has no way to
//! know `with_time` doesn't clobber the date `with_date` already set.
//! Fixed by testing each setter independently on its own fresh
//! `Pieces::from(initial_date)` instance instead — the same
//! established pattern `span.rs`'s own per-field harness already
//! uses, sidestepping the need for frame conditions entirely rather
//! than writing them.

use crate::ext_jiff::civil_date::logic::{date_day_value, date_month_value, date_year_value};
use crate::ext_jiff::civil_time::logic::{
    civil_time_hour_value, civil_time_minute_value, civil_time_second_value,
    civil_time_subsec_nanosecond_value,
};
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
fn pieces_date_year_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i16 {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_date_month_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_date_day_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_time_hour_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_time_minute_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_time_second_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[trusted]
#[logic(opaque)]
fn pieces_time_subsec_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i32> {
    dead
}

extern_spec! {
    impl jiff::fmt::temporal::Pieces<'static> {
        #[check(ghost)]
        #[ensures(
            date_year_value(&result) == pieces_date_year_value(&self)
            && date_month_value(&result) == pieces_date_month_value(&self)
            && date_day_value(&result) == pieces_date_day_value(&self)
        )]
        fn date(&self) -> jiff::civil::Date;

        #[check(ghost)]
        #[ensures(match result {
            Some(t) => pieces_time_hour_value(&self) == Some(civil_time_hour_value(&t))
                && pieces_time_minute_value(&self) == Some(civil_time_minute_value(&t))
                && pieces_time_second_value(&self) == Some(civil_time_second_value(&t))
                && pieces_time_subsec_value(&self) == Some(civil_time_subsec_nanosecond_value(&t)),
            None => pieces_time_hour_value(&self) == None,
        })]
        fn time(&self) -> Option<jiff::civil::Time>;

        #[check(ghost)]
        #[ensures(
            pieces_date_year_value(&result) == date_year_value(&date)
            && pieces_date_month_value(&result) == date_month_value(&date)
            && pieces_date_day_value(&result) == date_day_value(&date)
        )]
        fn with_date(self, date: jiff::civil::Date) -> jiff::fmt::temporal::Pieces<'static>;

        #[check(ghost)]
        #[ensures(
            pieces_time_hour_value(&result) == Some(civil_time_hour_value(&time))
            && pieces_time_minute_value(&result) == Some(civil_time_minute_value(&time))
            && pieces_time_second_value(&result) == Some(civil_time_second_value(&time))
            && pieces_time_subsec_value(&result) == Some(civil_time_subsec_nanosecond_value(&time))
        )]
        fn with_time(self, time: jiff::civil::Time) -> jiff::fmt::temporal::Pieces<'static>;
    }
}
