//! Gallery investigation: year span growth with month and day left fully symbolic,
//! the combined test `chrono_year_span_growth`, `chrono_month_span_growth`, and
//! `chrono_day_span_growth` each held one axis free and the other two fixed, and each
//! showed flat timing. This test holds only the year restricted, and leaves month and
//! day free, to see whether the interaction between symbolic dimensions is the actual
//! cost driver, not any single axis alone.
//!
//! Year restricted to `BASE_YEAR .. BASE_YEAR + N`, month and day fully symbolic
//! (`kani::any()`, no `assume` beyond what `from_ymd_opt` itself requires to return
//! `Some`). Same full day-count equality as the month and day axes.
//!
//! | Span (years) | Result | Verification time |
//! | --- | --- | --- |
//! | 1 | passed | 2.04s |
//! | 2 | passed | 2.31s |
//! | 4 | passed | 3.68s |
//! | 8 | passed | 2.52s (confirmed on rerun, a reproducible dip, not noise) |
//! | 16 | passed | 3.70s |
//! | 32 | passed | 4.47s |
//! | 64 | passed | 5.39s |
//! | 128 | passed | 7.11s |
//! | 256 | passed | 8.60s |
//! | 512 | passed | 15.47s |
//! | 1024 | passed | 30.40s |
//! | 2048 | passed | 42.35s |
//! | 4096 | passed | 75.44s |
//! | 8192 | passed | 129.97s |
//! | 16384 | passed, but at 222.09s (measured with a 10-minute bound) | **crosses our usual 3-minute (180s) bound** |
//!
//! Unlike any single axis (all flat), this one grows steadily: a compounding, roughly
//! 1.1-2x increase per doubling, not a sudden wall. The practical crossing point against
//! our usual three-minute harness limit sits between spans 8192 and 16384 years — a tiny
//! fraction of chrono's full ~262,000-year range. This is consistent with (and explains)
//! why the fully unrestricted proof in `chrono_day_count_staging` never had a chance: the
//! full range is roughly 16-32x past where this combined test already crosses the
//! practical bound, and the growth trend gives no reason to expect it to level off.

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    pub(super) use super::super::chrono_day_count_staging::{
        CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count,
    };

    /// The same base year the other axes use.
    pub(super) const BASE_YEAR: i32 = 2001;
}

#[cfg(kani)]
use kani_only::{BASE_YEAR, CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count};

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_1_SRC, {
        /// Year span of 1, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_1() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 1);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_2_SRC, {
        /// Year span of 2, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_2() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 2);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_4_SRC, {
        /// Year span of 4, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_4() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 4);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_8_SRC, {
        /// Year span of 8, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_8() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 8);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_16_SRC, {
        /// Year span of 16, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_16() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 16);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_32_SRC, {
        /// Year span of 32, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_32() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 32);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_64_SRC, {
        /// Year span of 64, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_64() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 64);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_128_SRC, {
        /// Year span of 128, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_128() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 128);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_256_SRC, {
        /// Year span of 256, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_256() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 256);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_512_SRC, {
        /// Year span of 512, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_512() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 512);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_1024_SRC, {
        /// Year span of 1024, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_1024() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 1024);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_2048_SRC, {
        /// Year span of 2048, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_2048() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 2048);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_4096_SRC, {
        /// Year span of 4096, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_4096() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 4096);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_8192_SRC, {
        /// Year span of 8192, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_8192() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 8192);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, YEAR_SPAN_FREE_MD_16384_SRC, {
        /// Year span of 16384, month and day fully symbolic.
        #[kani::proof]
        fn year_span_free_md_16384() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + 16384);
            if let Some(count) = forward_day_count(year, month, day) {
                assert!(
                    i64::from(count)
                        == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                            + CE_OFFSET_FROM_EPOCH
                );
            }
        }
    }
}
