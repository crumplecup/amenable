//! Gallery investigation: how Kani's verification time grows with the number of
//! distinct months a harness must consider, mirroring `chrono_year_span_growth`'s year
//! axis but for month.
//!
//! Year and day are fixed (`BASE_YEAR`, `1`), so the only free dimension is the month,
//! restricted to `1 .. 1 + N` for `N` in `1, 2, 4, 8, 12` (`12` is the real domain size,
//! since a month cannot exceed `12`).
//!
//! The claim here is the full day-count equality (`forward_day_count` against
//! `civil_days_spec`, reused from `chrono_day_count_staging` rather than duplicated),
//! not the year-part-only model `chrono_year_span_growth` uses. The year-part model
//! does not depend on month at all — it is exactly the part of chrono's formula with
//! the day-of-year term subtracted out — so it would show "no growth" for the wrong
//! reason. The full equality's day-of-year term (`mp`, `doy` in `civil_days_spec`) does
//! depend on month, which is what this axis needs to actually exercise.
//!
//! Timed with a plain `cargo kani --harness <name>`, one at a time, wall-clock time
//! from the shell, with the verification time read from Kani's own report.
//!
//! | Span (months) | Result | Verification time |
//! | --- | --- | --- |
//! | 1 | passed | 0.91s |
//! | 2 | passed | 0.97s |
//! | 4 | passed | 1.00s |
//! | 8 | passed | 0.98s |
//! | 12 | passed | 1.01s |
//!
//! Flat across the full month domain, the same as the year axis. Month alone, with year
//! and day fixed, is not the cost driver either.
//!
//! Follow-up, testing partition-by-month directly: month fixed to the single literal
//! `1`, year and day left fully symbolic over their entire domains (no `assume`
//! narrowing either one). **Times out**, and not just against the harness-timeout
//! wrapper: CBMC itself reports its own internal timeout, confirmed at 3:11 wall time.
//!
//! This answers whether a 12-way split by month (one harness per concrete month) would
//! resolve the original timeout: it would not. The explosion survives inside a single
//! month, when year keeps its full ~262,000-year range and day is free. Since the year
//! axis alone was flat up to a span of 1024 (`chrono_year_span_growth`), the cost here
//! comes from the full year range combined with a free day, not from month.

#[cfg(kani)]
use super::chrono_day_count_staging::{CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count};

/// The fixed year for every span below: the same base `chrono_year_span_growth` uses.
#[cfg(kani)]
const BASE_YEAR: i32 = 2001;

amenable_derive::gallery_harness! {
    kani, MONTH_SPAN_1_SRC, {
        /// Month span of 1: a single concrete month, year and day fixed.
        #[kani::proof]
        fn month_span_1() {
            let month: u32 = kani::any();
            kani::assume(month >= 1 && month < 1 + 1);
            if let Some(count) = forward_day_count(BASE_YEAR, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, MONTH_SPAN_2_SRC, {
        /// Month span of 2.
        #[kani::proof]
        fn month_span_2() {
            let month: u32 = kani::any();
            kani::assume(month >= 1 && month < 1 + 2);
            if let Some(count) = forward_day_count(BASE_YEAR, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, MONTH_SPAN_4_SRC, {
        /// Month span of 4.
        #[kani::proof]
        fn month_span_4() {
            let month: u32 = kani::any();
            kani::assume(month >= 1 && month < 1 + 4);
            if let Some(count) = forward_day_count(BASE_YEAR, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, MONTH_SPAN_8_SRC, {
        /// Month span of 8.
        #[kani::proof]
        fn month_span_8() {
            let month: u32 = kani::any();
            kani::assume(month >= 1 && month < 1 + 8);
            if let Some(count) = forward_day_count(BASE_YEAR, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, MONTH_SPAN_12_SRC, {
        /// Month span of 12: the full month domain, year and day still fixed.
        #[kani::proof]
        fn month_span_12() {
            let month: u32 = kani::any();
            kani::assume(month >= 1 && month < 1 + 12);
            if let Some(count) = forward_day_count(BASE_YEAR, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_month_span_growth::month_fixed_1_year_day_free".to_owned(),
            "gallery::chrono_month_span_growth::month_fixed_1_year_day_free".to_owned(),
            "amenable_kani".to_owned(),
            "fixing month alone does not help: year and day left fully free, with month pinned to one literal, still times out on CBMC's own internal timeout".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, MONTH_FIXED_1_YEAR_DAY_FREE_SRC, {
        /// Month fixed to the single literal 1 (January). Year and day are fully
        /// symbolic over their entire domains, with no `assume` narrowing either one.
        /// Tests the partition-by-month strategy directly: does one concrete month,
        /// with everything else free, verify at all?
        #[kani::proof]
        fn month_fixed_1_year_day_free() {
            let year: i32 = kani::any();
            let day: u32 = kani::any();
            if let Some(count) = forward_day_count(year, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}
