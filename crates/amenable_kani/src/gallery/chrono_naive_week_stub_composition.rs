//! Gallery case: composing the `NaiveWeek` claim with Kani contracts and
//! `stub_verified`, the way the GAAP ledger and Exchange plans compose proofs.
//!
//! Two wrapper functions carried a contract each: `week_first_day` (the first day of
//! the week, on or before the date) and `week_last_day` (the last day, on or after the
//! date). Both proved with `proof_for_contract`. A third, `week_span_holds`, composed
//! them by calling both and checking the six-day span, with `stub_verified` on both
//! calls so its own proof would not re-run their bodies.
//!
//! Compiling the stubs needed a type `kani::Arbitrary`, and `Option<NaiveDate>` is not
//! one (chrono does not implement it, and the orphan rule blocks us from doing so). A
//! local newtype, `KaniDate`, with a hand-written `Arbitrary` generating exactly the
//! valid dates in chrono's supported range, resolved that.
//!
//! With the newtype in place, the composition compiled:
//!
//! - `week_first_day` contract: passes
//! - `week_last_day` contract: passes
//! - `week_span_holds`, composed with both stubbed: does not verify. The first measured
//!   run reported a quick counterexample (failed in a few seconds). After this case
//!   moved into the gallery, alongside the day-count cases, a fresh measured run
//!   reported a timeout at three minutes instead, same code, same contracts. Which one
//!   reproduces is not pinned down; both runs are recorded rather than picking one.
//!
//! Either way, the result is real and it is ours, not chrono's: the first two contracts
//! only state that each end is on the correct side of the date — they say nothing about
//! where exactly it is. Stubbing replaces each call with *any* value satisfying its
//! contract, so the composed proof sees two independently arbitrary dates, each merely
//! on or before, or on or after, the real date. Nothing ties them six days apart.
//! `stub_verified` composition needs the contract to carry the exact relation the
//! composing proof depends on; a one-sided inequality is not enough, whether Kani
//! reports that as a found counterexample or as a timeout while searching for one.
//!
//! The follow-up attempt restated both contracts with the exact relation (`first ==
//! date - back days`, `last == date + (6-back) days`, both as an equality against
//! chrono's checked day arithmetic). That resolved the under-specification, but hit the
//! separate, deeper wall the `chrono_day_count_staging` gallery case documents: an
//! exact-value equality between two independently derived dates times out, regardless
//! of the contract's shape.
//!
//! **Do not retry a one-sided (inequality-only) contract expecting `stub_verified`
//! composition to work for an exact downstream relation.** The contract has to state
//! the relation the composing proof actually needs.
//!
//! This case is a false trail for the composition as first written, not a verdict on
//! the general technique: the two independently verified contracts (first and last day,
//! one-sided) remain sound; only their use as a basis for the span relation was wrong.

#[cfg(kani)]
use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Map any `u8` onto the seven weekdays.
#[cfg(kani)]
fn weekday_of(index: u8) -> Weekday {
    match index % 7 {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

/// Days a date steps back from its own weekday to reach `start` (`0..=6`).
#[cfg(kani)]
fn days_back_to(date: NaiveDate, start: Weekday) -> u32 {
    (date.weekday().num_days_from_monday() + 7 - start.num_days_from_monday()) % 7
}

/// A `NaiveDate` newtype `kani::Arbitrary` can generate: the one piece this attempt did
/// resolve, since `Option<NaiveDate>` itself is not `Arbitrary` and chrono cannot be
/// made to implement it from outside the crate.
#[cfg(kani)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KaniDate(NaiveDate);

#[cfg(kani)]
impl kani::Arbitrary for KaniDate {
    fn any() -> Self {
        let year: i32 = kani::any();
        let month: u32 = kani::any();
        let day: u32 = kani::any();
        let in_range = (NaiveDate::MIN.year()..=NaiveDate::MAX.year()).contains(&year);
        kani::assume(
            in_range
                && (1..=12).contains(&month)
                && day >= 1
                && day
                    <= match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
                        2 => 28,
                        _ => 0,
                    },
        );
        KaniDate(
            NaiveDate::from_ymd_opt(year, month, day)
                .expect("assumed a valid Gregorian date, so from_ymd_opt is Some"),
        )
    }
}

/// The first day of the week, one-sided contract: on or before the date. Says nothing
/// about exactly which day it is.
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Option<KaniDate>| match *result {
        Some(first) => first.0 <= date,
        None => true,
    })
)]
#[cfg(kani)]
fn week_first_day(date: NaiveDate, start: Weekday) -> Option<KaniDate> {
    date.week(start).checked_first_day().map(KaniDate)
}

/// The last day of the week, one-sided contract: on or after the date.
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Option<KaniDate>| match *result {
        Some(last) => date <= last.0,
        None => true,
    })
)]
#[cfg(kani)]
fn week_last_day(date: NaiveDate, start: Weekday) -> Option<KaniDate> {
    date.week(start).checked_last_day().map(KaniDate)
}

/// The composed claim: when both ends exist, they span six days. Fails under stubbing,
/// because the one-sided contracts above do not pin the ends six days apart.
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(kani, kani::ensures(|result: &bool| *result))]
#[cfg(kani)]
fn week_span_holds(date: NaiveDate, start: Weekday) -> bool {
    match (week_first_day(date, start), week_last_day(date, start)) {
        (Some(first), Some(last)) => (last.0 - first.0).num_days() == 6,
        _ => true,
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_stub_composition::week_first_day_one_sided_contract_passes".to_owned(),
            "gallery::chrono_naive_week_stub_composition::check_week_first_day".to_owned(),
            "amenable_kani".to_owned(),
            "the one-sided week_first_day contract (on or before the date) passes, and stub_verified requires its own proof_for_contract harness to exist".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_WEEK_FIRST_DAY_SRC, {
        /// `week_first_day`'s one-sided contract, required by `stub_verified` above even
        /// though this case is about the composed span, not this contract alone.
        #[kani::proof_for_contract(week_first_day)]
        fn check_week_first_day() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let _ = week_first_day(date, start);
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_stub_composition::week_last_day_one_sided_contract_passes".to_owned(),
            "gallery::chrono_naive_week_stub_composition::check_week_last_day".to_owned(),
            "amenable_kani".to_owned(),
            "the one-sided week_last_day contract (on or after the date) passes, and stub_verified requires its own proof_for_contract harness to exist".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_WEEK_LAST_DAY_SRC, {
        /// `week_last_day`'s one-sided contract, required by `stub_verified` above.
        #[kani::proof_for_contract(week_last_day)]
        fn check_week_last_day() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let _ = week_last_day(date, start);
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_stub_composition::span_fails_under_one_sided_stubs".to_owned(),
            "gallery::chrono_naive_week_stub_composition::span_fails_under_one_sided_stubs".to_owned(),
            "amenable_kani".to_owned(),
            "stub_verified composition of the week span does not verify: the one-sided end contracts do not pin the ends six days apart".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SPAN_FAILS_UNDER_ONE_SIDED_STUBS_SRC, {
        /// `week_span_holds`, composed with both ends stubbed under their one-sided
        /// contracts. Confirmed to fail, not time out: the stubs can return any dates
        /// satisfying "on or before" / "on or after", not specifically the real ends.
        #[kani::proof_for_contract(week_span_holds)]
        #[kani::stub_verified(week_first_day)]
        #[kani::stub_verified(week_last_day)]
        fn span_fails_under_one_sided_stubs() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let _ = week_span_holds(date, start);
            }
        }
    }
}
