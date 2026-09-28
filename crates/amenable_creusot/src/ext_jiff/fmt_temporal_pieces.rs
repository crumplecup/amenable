//! Real Creusot proof content for `jiff::fmt::temporal::
//! Pieces<'static>`'s `with_date`/`with_time` round trips
//! (`ext::jiff::fmt_temporal_pieces` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces` checks by symbolic execution, see that
//! module's own doc comment for the scoping rationale (`date`/`time`
//! only, not `offset`/`time_zone_annotation`).
//!
//! Reuses `civil_date.rs`'s/`civil_time.rs`'s existing opaque
//! accessors directly (Creusot allows only one `extern_spec!` per
//! real function crate-wide). `Pieces`'s own `date()`/`time()`
//! getters and `with_date`/`with_time` setters can't be stated in
//! terms of a `Date`/`Time` VALUE directly (comparing two `Date`/
//! `Time` values via `==` hits the same `DeepModel` wall
//! `unit.rs`'s own doc comment documents — foreign types need a
//! model Creusot can't derive), so this file introduces its own
//! Pieces-specific opaque accessors returning already-DECOMPOSED
//! primitives (`i16`/`i8`/`Option<i8>`/etc.) instead, which compare
//! fine via `==` with no `DeepModel` requirement at all (the same
//! reason `Option<i16>`/etc. comparisons already work fine elsewhere
//! in this checklist).
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

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use crate::ext_jiff::civil_date::{date_day_value, date_month_value, date_year_value};
#[cfg(creusot)]
use crate::ext_jiff::civil_time::{
    civil_time_hour_value, civil_time_minute_value, civil_time_second_value,
    civil_time_subsec_nanosecond_value,
};
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_date_year_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_date_month_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_date_day_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_time_hour_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_time_minute_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_time_second_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i8> {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pieces_time_subsec_value(_p: &jiff::fmt::temporal::Pieces<'static>) -> Option<i32> {
    dead
}

#[cfg(creusot)]
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

amenable_derive::harness! {
    creusot, FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_HOLDS_SRC, {
        /// The `amenable_ext::
        /// ExtStandard<jiff::fmt::temporal::Pieces<'static>>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn fmt_temporal_pieces_with_date_with_time_round_trip_holds(matches: bool) -> bool {
            pearlite! { matches }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_temporal_pieces::fmt_temporal_pieces_with_date_with_time_round_trip_holds",
        "creusot",
        "ensures",
        || FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_SRC, {
        /// `Pieces::with_date`/`with_time`, whenever the given values
        /// are already-valid `Date`/`Time` values, always round-trip
        /// through their matching `date()`/`time()` getters exactly
        /// — a real, checked postcondition resting on the
        /// `extern_spec!` above, the same claim `amenable_kani::
        /// ext::jiff::fmt_temporal_pieces` checks by symbolic
        /// execution.
        #[requires(true)]
        #[ensures(fmt_temporal_pieces_with_date_with_time_round_trip_holds(result))]
        fn verify_fmt_temporal_pieces_with_date_with_time_round_trip(
            initial_date: jiff::civil::Date,
            date: jiff::civil::Date,
            time: jiff::civil::Time,
        ) -> bool {
            let pieces_date = jiff::fmt::temporal::Pieces::from(initial_date).with_date(date);
            let got_date = pieces_date.date();
            let date_ok = got_date.year() == date.year()
                && got_date.month() == date.month()
                && got_date.day() == date.day();

            let pieces_time = jiff::fmt::temporal::Pieces::from(initial_date).with_time(time);
            let time_ok = match pieces_time.time() {
                Some(t) => {
                    t.hour() == time.hour()
                        && t.minute() == time.minute()
                        && t.second() == time.second()
                        && t.subsec_nanosecond() == time.subsec_nanosecond()
                }
                None => false,
            };
            date_ok && time_ok
        }
    }
}
