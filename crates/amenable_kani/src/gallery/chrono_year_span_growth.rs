//! Gallery investigation: how Kani's verification time grows with the number of
//! distinct years a harness must consider, for the forward day-count equality that
//! `chrono_day_count_staging` already shows times out in general.
//!
//! Month and day are fixed (`1`, `1`), so the only free dimension is the year, and the
//! year is restricted to a span of `BASE_YEAR .. BASE_YEAR + N` for `N` in `1, 2, 4, 8,
//! 16`. `BASE_YEAR` (2001) is chosen to avoid the century and 400-year boundaries
//! (2000, 2100), so the spans measure pure domain size, not a residue-boundary effect
//! like the one that made the century classes in `chrono_day_count_staging` cheap.
//!
//! Timed with a plain `cargo kani --harness <name>` (no contract machinery, so no
//! unstable flags beyond the harness timeout), one at a time, wall-clock time from the
//! shell. Results recorded as they were measured, not estimated.
//!
//! | Span (years) | Result | Verification time |
//! | --- | --- | --- |
//! | 1 | passed | 0.62s |
//! | 2 | passed | 0.65s |
//! | 4 | passed | 0.61s |
//! | 8 | passed | 0.64s |
//! | 16 | passed | 0.61s |
//!
//! Flat. No growth at all across a 16x increase in the year domain, with month and day
//! fixed. This means year-domain size alone, in this range, is not the cost driver.
//! Compare to `chrono_day_count_staging`'s common-year and leap-ordinary classes, which
//! span roughly 100-300 years *and* leave month and day fully symbolic, and which do
//! time out.
//!
//! Continued doubling with month and day still fixed: 32 (0.65s), 64 (0.73s), 128
//! (0.70s, crossing the 2100 century boundary), 256 (0.83s), 512 (0.89s), 1024
//! (1.54s). A gentle, roughly linear creep, not exponential, confirming year-domain
//! size alone is cheap across three orders of magnitude.
//!
//! Decisive follow-up: year fixed to the single literal `BASE_YEAR`, month and day
//! left fully symbolic over their entire domains (no `assume` narrowing either one).
//! **Passes, in 1.33s.** Compare to `chrono_month_span_growth`'s and
//! `chrono_day_span_growth`'s mirror tests — month fixed (year, day free) and day
//! fixed (year, month free) — both of which time out with CBMC's own internal
//! timeout, at 3:11 each.
//!
//! This settles which dimension drives the cost: **year's unbounded range is the
//! specific cause, not symbolic-dimension interaction in general.** Fixing year, even
//! to one concrete value, collapses the cost regardless of what month and day do.
//! Fixing month or day instead, while leaving year's full range free, does not help at
//! all — the explosion survives inside a single month or a single day. A partition
//! strategy has to bound year specifically; a 12-way split by month or a 31-way split
//! by day, on their own, would not resolve the original timeout.

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use chrono::{Datelike, NaiveDate};

    /// chrono's real year part: its total day count minus its ordinal, both public.
    pub(super) fn chrono_year_part(date: NaiveDate) -> i32 {
        date.num_days_from_ce() - i32::try_from(date.ordinal()).unwrap_or(0)
    }

    /// The model: chrono's own year-part formula, copied in shape.
    pub(super) fn model_year_part(year: i32) -> i32 {
        let mut year = year - 1;
        let mut ndays = 0;
        if year < 0 {
            let excess = 1 + (-year) / 400;
            year += excess * 400;
            ndays -= excess * 146_097;
        }
        let div_100 = year / 100;
        ndays + ((year * 1461) >> 2) - div_100 + (div_100 >> 2)
    }

    /// The base year for every span below: inside chrono's supported range, and far
    /// from any century or 400-year boundary.
    pub(super) const BASE_YEAR: i32 = 2001;
}

#[cfg(kani)]
use chrono::NaiveDate;
#[cfg(kani)]
use kani_only::{BASE_YEAR, chrono_year_part, model_year_part};

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_1_SRC, {
        /// Year span of 1: a single concrete year, month and day fixed.
        #[kani::proof]
        fn year_span_1() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 1);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_2_SRC, {
        /// Year span of 2.
        #[kani::proof]
        fn year_span_2() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 2);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_4_SRC, {
        /// Year span of 4.
        #[kani::proof]
        fn year_span_4() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 4);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_8_SRC, {
        /// Year span of 8.
        #[kani::proof]
        fn year_span_8() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 8);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_16_SRC, {
        /// Year span of 16.
        #[kani::proof]
        fn year_span_16() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 16);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_32_SRC, {
        /// Year span of 32.
        #[kani::proof]
        fn year_span_32() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 32);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_64_SRC, {
        /// Year span of 64.
        #[kani::proof]
        fn year_span_64() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 64);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_128_SRC, {
        /// Year span of 128.
        #[kani::proof]
        fn year_span_128() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 128);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_256_SRC, {
        /// Year span of 256.
        #[kani::proof]
        fn year_span_256() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 256);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_512_SRC, {
        /// Year span of 512.
        #[kani::proof]
        fn year_span_512() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 512);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_1024_SRC, {
        /// Year span of 1024.
        #[kani::proof]
        fn year_span_1024() {
            let year: i32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 1024);
            if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
                assert!(chrono_year_part(date) == model_year_part(year));
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_year_span_growth::year_fixed_year_month_day_free".to_owned(),
            "gallery::chrono_year_span_growth::year_fixed_year_month_day_free".to_owned(),
            "amenable_kani".to_owned(),
            "fixing year alone (one literal) lets month and day vary over their entire free domains and still verify quickly, unlike fixing month or day alone".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, YEAR_FIXED_YEAR_MONTH_DAY_FREE_SRC, {
        /// Year fixed to the single literal `BASE_YEAR`. Month and day are fully
        /// symbolic over their entire domains, with no `assume` narrowing either one.
        /// The mirror of the month-fixed and day-fixed partition tests: does fixing
        /// year specifically, rather than month or day, let the other two vary freely
        /// without exploding?
        #[kani::proof]
        fn year_fixed_year_month_day_free() {
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            if let Some(count) = super::chrono_day_count_staging::forward_day_count(BASE_YEAR, month, day) {
                assert!(
                    i64::from(count)
                        == super::chrono_day_count_staging::civil_days_spec(
                            i64::from(BASE_YEAR),
                            i64::from(month),
                            i64::from(day)
                        ) + super::chrono_day_count_staging::CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}
