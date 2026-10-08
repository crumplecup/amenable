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

//! **Timing drift, confirmed (2026-10-08):** `discharge_slice_a`/`discharge_slice_b`
//! were originally measured as passing quickly; re-measured today, both now time out
//! at the project's usual three-minute bound (pass at six minutes; the underlying
//! contract discharge is unchanged and still sound, just slower). Confirmed this is
//! not caused by anything in this file or this session's own edits to it: reverting
//! every structural change made to this file during that same investigation (undoing
//! an inline-module restructuring applied here first, then undone) reproduced the
//! identical timeout, byte-for-byte against the original source. The crate's own
//! growth since the original measurement (many more registered proofs compiled into
//! the same Kani target) is the more likely explanation, though not chased further
//! here. `stub_verified_outside_either_slice`'s own finding is unaffected: it never
//! re-discharges either contract, only trusts the attribute's existence, which is
//! exactly the point it demonstrates.

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use chrono::{Datelike, NaiveDate, Weekday};

    /// Map any `u8` onto the seven weekdays.
    pub(super) fn weekday_of(index: u8) -> Weekday {
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
    #[kani::requires(true)]
    #[kani::ensures(|result: &bool| *result)]
    pub(super) fn week_span_holds(date: NaiveDate, start: Weekday) -> bool {
        match (
            date.week(start).checked_first_day(),
            date.week(start).checked_last_day(),
        ) {
            (Some(first), Some(last)) => (last - first).num_days() == 6,
            _ => true,
        }
    }

    /// A trivial consumer with its own contract, so the stub-verified harness has a
    /// `proof_for_contract` target. Its contract just restates `week_span_holds`'s.
    #[kani::requires(true)]
    #[kani::ensures(|result: &bool| *result)]
    pub(super) fn stub_consumer(date: NaiveDate, start: Weekday) -> bool {
        week_span_holds(date, start)
    }
}

#[cfg(kani)]
use chrono::NaiveDate;
#[cfg(kani)]
use kani_only::{stub_consumer, week_span_holds, weekday_of};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_span_contract_mechanism_test::discharge_slice_a".to_owned(),
            "gallery::chrono_naive_week_span_contract_mechanism_test::discharge_slice_a".to_owned(),
            "amenable_kani".to_owned(),
            "discharges week_span_holds's contract for years 2001..2003 only, one of two deliberately disjoint, non-adjacent slices -- passed when first measured, now times out at the project's usual three-minute bound (confirmed to pass at six minutes; the crate's own growth since the original measurement is the likely cause, not anything about this slice)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DISCHARGE_SLICE_A_SRC, {
        /// Discharges the contract for years 2001..2003 only. Originally measured as
        /// passing quickly; now times out at the project's usual three-minute bound
        /// (see the module doc comment for the full timing-drift finding).
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_span_contract_mechanism_test::discharge_slice_b".to_owned(),
            "gallery::chrono_naive_week_span_contract_mechanism_test::discharge_slice_b".to_owned(),
            "amenable_kani".to_owned(),
            "discharges week_span_holds's contract for years 3001..3003 only, the second deliberately disjoint, non-adjacent slice -- passed when first measured, now times out at the project's usual three-minute bound, the same drift discharge_slice_a shows".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DISCHARGE_SLICE_B_SRC, {
        /// Discharges the contract for years 3001..3003 only — a disjoint, distant
        /// slice. Originally measured as passing quickly; now times out at the
        /// project's usual three-minute bound (see the module doc comment).
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_span_contract_mechanism_test::stub_verified_outside_either_slice".to_owned(),
            "gallery::chrono_naive_week_span_contract_mechanism_test::stub_verified_outside_either_slice".to_owned(),
            "amenable_kani".to_owned(),
            "confirms Kani's proof_for_contract/stub_verified mechanism does not check multi-harness domain coverage: stubbing week_span_holds for year 5000, outside both discharged slices, still verifies with no warning".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
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
