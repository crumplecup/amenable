//! Gallery investigation: how Kani's verification time grows with the number of
//! distinct days a harness must consider, completing the per-axis series alongside
//! `chrono_year_span_growth` and `chrono_month_span_growth`.
//!
//! Year and month are fixed (`BASE_YEAR`, `1`, January, which has 31 days), so the only
//! free dimension is the day, restricted to `1 .. 1 + N` for `N` in `1, 2, 4, 8, 16, 31`
//! (`31` is the real domain size for January).
//!
//! Same claim as the month axis, for the same reason: the full day-count equality
//! (`forward_day_count` against `civil_days_spec`, reused from `chrono_day_count_staging`),
//! since the day-of-year term depends on day.
//!
//! | Span (days) | Result | Verification time |
//! | --- | --- | --- |
//! | 1 | passed | 0.83s |
//! | 2 | passed | 0.85s |
//!
//! (The 4/8/16/31 points were not finished: the investigation moved to the
//! partition-by-day question below before completing this axis. The two points
//! measured are flat, consistent with the year and month axes.)
//!
//! Follow-up, testing partition-by-day directly, the mirror of
//! `chrono_month_span_growth`'s partition-by-month test: day fixed to the single
//! literal `1`, year and month left fully symbolic over their entire domains. **Times
//! out**, with CBMC's own internal timeout, confirmed at 3:11 wall time — almost
//! exactly the same as the month-fixed case (also 3:11).
//!
//! So a 31-way split by day would not resolve the original timeout either, for the same
//! reason the 12-way month split does not: the explosion survives with year's full
//! range free, regardless of which of month or day is the one left free alongside it.
//! Fixing either single dimension while leaving year fully free is not enough; year's
//! full range combined with any other free dimension is.

#[cfg(kani)]
use super::chrono_day_count_staging::{CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count};

/// The fixed year for every span below: the same base the other two axes use.
#[cfg(kani)]
const BASE_YEAR: i32 = 2001;

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_1_SRC, {
        /// Day span of 1: a single concrete day, year and month fixed.
        #[kani::proof]
        fn day_span_1() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 1);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_2_SRC, {
        /// Day span of 2.
        #[kani::proof]
        fn day_span_2() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 2);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_4_SRC, {
        /// Day span of 4.
        #[kani::proof]
        fn day_span_4() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 4);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_8_SRC, {
        /// Day span of 8.
        #[kani::proof]
        fn day_span_8() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 8);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_16_SRC, {
        /// Day span of 16.
        #[kani::proof]
        fn day_span_16() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 16);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, DAY_SPAN_31_SRC, {
        /// Day span of 31: the full day domain for January, year and month still fixed.
        #[kani::proof]
        fn day_span_31() {
            let day: u32 = kani::any();
            kani::assume(day >= 1 && day < 1 + 31);
            if let Some(count) = forward_day_count(BASE_YEAR, 1, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(BASE_YEAR), 1, i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_day_span_growth::day_fixed_1_year_month_free".to_owned(),
            "gallery::chrono_day_span_growth::day_fixed_1_year_month_free".to_owned(),
            "amenable_kani".to_owned(),
            "fixing day alone does not help: year and month left fully free, with day pinned to one literal, still times out on CBMC's own internal timeout".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DAY_FIXED_1_YEAR_MONTH_FREE_SRC, {
        /// Day fixed to the single literal 1. Year and month are fully symbolic over
        /// their entire domains, with no `assume` narrowing either one. Tests the
        /// partition-by-day strategy directly: does one concrete day, with everything
        /// else free, verify at all?
        #[kani::proof]
        fn day_fixed_1_year_month_free() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            if let Some(count) = forward_day_count(year, month, 1) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), 1)
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}
