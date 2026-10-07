//! Gallery investigation: does Kani's `proof_for_contract`/`stub_verified` mechanism
//! require multiple `proof_for_contract` harnesses to collectively cover a function's
//! full domain, or does it trust the declared `#[kani::ensures]` once any one harness
//! exists? This is the mechanism question behind "generate a stub per slice, let a
//! single contracted function stand for the whole range."
//!
//! `week_span_holds` carries one contract, unconditional over its whole domain
//! (`#[kani::requires(true)]`, `#[kani::ensures(|result: &bool| *result)]`). Two
//! `proof_for_contract` harnesses each discharge it over a small, deliberately
//! non-adjacent slice (years 2001..2003 and 3001..3003), leaving most of the domain,
//! including everything between them, unchecked by either.

#[cfg(kani)]
use chrono::{Datelike, NaiveDate, Weekday};

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

/// One contract, unconditional over its whole domain. Only years 2001..2003 and
/// 3001..3003 are ever actually checked, by the two harnesses below.
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(kani, kani::ensures(|result: &bool| *result))]
#[cfg(kani)]
fn week_span_holds(date: NaiveDate, start: Weekday) -> bool {
    match (
        date.week(start).checked_first_day(),
        date.week(start).checked_last_day(),
    ) {
        (Some(first), Some(last)) => (last - first).num_days() == 6,
        _ => true,
    }
}

amenable_derive::gallery_harness! {
    kani, DISCHARGE_SLICE_A_SRC, {
        /// Discharges the contract for years 2001..2003 only.
        #[kani::proof_for_contract(week_span_holds)]
        fn discharge_slice_a() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            kani::assume(year >= 2001 && year < 2003);
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let _ = week_span_holds(date, start);
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DISCHARGE_SLICE_B_SRC, {
        /// Discharges the contract for years 3001..3003 only — a disjoint, distant slice.
        #[kani::proof_for_contract(week_span_holds)]
        fn discharge_slice_b() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            kani::assume(year >= 3001 && year < 3003);
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let _ = week_span_holds(date, start);
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, STUB_VERIFIED_OUTSIDE_EITHER_SLICE_SRC, {
        /// A consumer that stubs `week_span_holds` and calls it for a year inside
        /// neither discharged slice (year 5000, far from both). If Kani requires
        /// coverage to be exhaustive before trusting the contract, this should fail or
        /// be flagged; if Kani only checks that the attribute exists, this will pass
        /// regardless of the gap.
        #[kani::proof_for_contract(stub_consumer)]
        #[kani::stub_verified(week_span_holds)]
        fn stub_verified_outside_either_slice() {
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = NaiveDate::from_ymd_opt(5000, month, day) {
                let _ = stub_consumer(date, start);
            }
        }
    }
}

/// A trivial consumer with its own contract, so the stub-verified harness above has a
/// `proof_for_contract` target. Its contract just restates `week_span_holds`'s.
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(kani, kani::ensures(|result: &bool| *result))]
#[cfg(kani)]
fn stub_consumer(date: NaiveDate, start: Weekday) -> bool {
    week_span_holds(date, start)
}
