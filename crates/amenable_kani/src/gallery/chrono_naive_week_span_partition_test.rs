//! Gallery investigation: does the year-partitioning technique that resolved
//! `chrono_day_count_staging`'s timeout transfer to the real, still-flaky
//! `week_spans_seven_days` claim in the production `NaiveWeek` witness?
//!
//! This is a different computation from the day-count spec equality: it uses chrono's
//! own `date.week(start).checked_first_day()`/`checked_last_day()` and `NaiveDate`
//! subtraction directly, not an independently derived formula. The partition size that
//! worked there (~5243 years) is not guaranteed to transfer; this is the empirical test
//! before committing to a production rewrite.
//!
//! One partition only, at the same size and base year used throughout this
//! investigation (`BASE_YEAR = 2001`, `PARTITION_SIZE = 5243`), month, day, and the
//! week-start day all left fully symbolic.

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

#[cfg(kani)]
const BASE_YEAR: i32 = 2001;

#[cfg(kani)]
const PARTITION_SIZE: i32 = 5243;

amenable_derive::gallery_harness! {
    kani, REAL_SPAN_CLAIM_ONE_PARTITION_SRC, {
        /// The real `week_spans_seven_days` claim, restricted to one year partition:
        /// when both checked ends exist, they span exactly six days.
        #[kani::proof]
        fn real_span_claim_one_partition() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            kani::assume(year >= BASE_YEAR && year < BASE_YEAR + PARTITION_SIZE);
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let week = date.week(start);
                if let (Some(first), Some(last)) = (week.checked_first_day(), week.checked_last_day()) {
                    assert!((last - first).num_days() == 6);
                }
            }
        }
    }
}

amenable_derive::gallery_harness! {
    kani, REAL_SPAN_CLAIM_BOUNDARY_PARTITION_SRC, {
        /// The same real claim, restricted to the partition touching chrono's minimum
        /// supported year.
        #[kani::proof]
        fn real_span_claim_boundary_partition() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            kani::assume(year >= -262_143 && year < -262_143 + PARTITION_SIZE);
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let week = date.week(start);
                if let (Some(first), Some(last)) = (week.checked_first_day(), week.checked_last_day()) {
                    assert!((last - first).num_days() == 6);
                }
            }
        }
    }
}
