//! Gallery cases for the staged `chrono::NaiveDate` day count under Kani.
//!
//! The goal was a staged `NaiveWeek` proof. Stage 1 is a proof of the `NaiveDate` day
//! count over a shared integer spec (Howard Hinnant's civil-days algorithm, offset to
//! the common era), and each direction proven once and stubbed, so the round trip
//! follows from the spec's own identity. The probes, each run through the project
//! verifier with a three-minute limit:
//!
//! Round trip, direct: `from_num_days_from_ce_opt(num_days_from_ce(d)) == Some(d)` over
//! every valid date. Times out. Each direction alone passes (below), so the relation
//! between the two `NaiveDate` values is the cost, not either conversion.
//!
//! - inverse conversion alone, symbolic `i32` input, no assertion: passes
//! - forward conversion alone, symbolic valid `(year, month, day)`, no assertion: passes
//! - integer spec alone, symbolic `i64` inputs over the valid domain, no chrono: passes
//! - forward contract, equality of chrono's count with the spec, plus validity: times out
//! - forward contract, equality only, validity clause dropped: times out
//!
//! Inverse direction, the one-way claim that chrono's date from a day count has the
//! spec's civil fields:
//!
//! - inverse spec alone, symbolic `i64` day count over chrono's range, no chrono: passes
//! - inverse wrapper alone, symbolic `i32` day count, no assertion: passes
//! - inverse contract, the spec's civil fields equal chrono's fields: times out
//!
//! The inverse direction has the same shape as the forward one, so the relation is the
//! cost there too.
//!
//! Case split by month class, forward direction. `month_class` maps every `(year, month)`
//! to exactly one class, so the classes are disjoint and exhaustive by construction. Each
//! class harness assumes its class and asserts the forward claim, over every year and day:
//!
//! - other months (outside `1..=12`), chrono builds no date: passes
//! - February of leap years: passes
//! - February of common years: times out
//! - 30-day months: times out
//! - 31-day months: times out
//!
//! The split narrows the state space per run, but the three larger classes still do not
//! finish. Splitting by month alone is not enough for those classes. The next split is by
//! year residue within the 400-year cycle, which is where the divisions by 4, 100, and 400
//! enter.
//!
//! Three-minute limit, tested. The 31-day-month class, which timed out at three minutes,
//! also times out at fifteen minutes. The limit is not what fails it.
//!
//! Year split within the 31-day-month class, by the Gregorian leap rule:
//!
//! - century leap years (divisible by 400): passes
//! - century common years (divisible by 100, not 400): passes
//! - leap years not divisible by 100: times out
//! - common years (not divisible by 4): times out
//!
//! The two classes that pass are the ones where the spec's residues are fixed by the year
//! class. The two that time out span most years, so the symbolic residues within the
//! 400-year cycle remain. The year split narrows the cases but does not yet isolate the
//! arithmetic that is expensive.
//!
//! Accommodation model attempt: mirror chrono's own formula shape instead of an
//! independently derived one. chrono's real `num_days_from_ce` (`chrono-0.4.45/src/
//! naive/date/mod.rs`) computes `year_part = ((year*1461)>>2) - year/100 +
//! (year/100>>2)`, then adds `ordinal`. Both pieces are reachable through the public
//! `Datelike` trait (`num_days_from_ce`, `ordinal`), so `year_part = num_days_from_ce -
//! ordinal` without touching chrono internals. The model copies that exact shape.
//!
//! - chrono's combined year part alone (`num_days_from_ce() - ordinal()`), symbolic
//!   valid date, no assertion: passes
//! - the model alone, unbounded symbolic `i32` year, no assertion: **fails**, not a
//!   timeout. `year * 1461` overflows `i32` for `year` outside chrono's supported
//!   range (confirmed: `chrono::day_numbers::model_year_part.assertion.1`, a multiply
//!   overflow check, at the `(year * 1461) >> 2` line). The model needs the same range
//!   bound every other spec here needs; it is not sound for an arbitrary `i32`.
//! - the model's year part equated to chrono's real year part, over every valid date:
//!   times out, even though both sides are shape-identical and each passes alone.
//!
//! So mirroring chrono's formula shape does not avoid the wall. The likely reason:
//! chrono's real `year_part` is not computed directly from the caller's `year` — it is
//! recovered from `NaiveDate`'s packed internal representation (`self.year()` decodes
//! the `yof` bits that `from_ymd_opt` encoded). The solver must also relate that
//! encode/decode round trip to the caller's original `year`, on top of the arithmetic
//! equality, even though the round trip is itself cheap when checked alone (confirmed
//! earlier: `NaiveDate::from_ymd_opt`'s own round trip proof passes). Checked together
//! in one proof obligation, the two cheap facts do not stay cheap.
//!
//! **Do not retry "mirror chrono's shape" expecting it to trivially pass equality
//! against chrono's real computation.** It removes the two-formula mismatch but not the
//! packed-representation round trip, which is the deeper cost.
//!
//! Also tested: the same equality as a bare `#[kani::proof]` assertion, with no
//! `kani::ensures`/`proof_for_contract` machinery at all. Still times out. The contract
//! framework is not adding the cost; the equality itself is expensive regardless of how
//! it is checked. **Do not retry "drop the contract and assert directly" expecting the
//! contract machinery to have been the problem.**
//!
//! The pattern matches the `NaiveWeek` probes: each piece is cheap alone, and the
//! relation between chrono's `NaiveDate` state and an arithmetic formula is what
//! CBMC cannot finish within the limit. Stubbing needs a relation to be provable first,
//! so the staging moves the cost down a level but does not remove it.
//!
//! What has not been tried, and is the next experiment: a spec whose arithmetic is
//! table-driven (cumulative days per month, a 400-year cycle constant) so the relation
//! avoids symbolic division by 4, 100, and 400; and a Kani-only accommodation model of
//! the day count, with chrono's conformance stated as the premise, in the style of the
//! B-tree and UTF-8 plans.
//!
//! This case is a false trail for the direct equality. It records the measured
//! boundary, not a verdict on chrono.

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use chrono::{Datelike, NaiveDate};

    /// The day count from the common era of 1970-01-01. Re-exported one level up:
    /// several sibling gallery files reuse it rather than duplicating it.
    pub(in crate::gallery) const CE_OFFSET_FROM_EPOCH: i64 = 719_163;

    /// The abandoned spec: days since 1970-01-01, derived independently of chrono's
    /// own formula shape (Howard Hinnant's civil-days algorithm). Re-exported one
    /// level up for the same reason.
    pub(in crate::gallery) fn civil_days_spec(year: i64, month: i64, day: i64) -> i64 {
        let y = if month <= 2 { year - 1 } else { year };
        let era = y.div_euclid(400);
        let yoe = y.rem_euclid(400);
        let mp = (month + 9) % 12;
        let doy = (153 * mp + 2) / 5 + day - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// chrono's real day count from the common era, for a valid date. Re-exported one
    /// level up for the same reason.
    pub(in crate::gallery) fn forward_day_count(year: i32, month: u32, day: u32) -> Option<i32> {
        NaiveDate::from_ymd_opt(year, month, day).map(|date| date.num_days_from_ce())
    }

    /// The accommodation model attempt: chrono's own year-part formula, copied in
    /// shape. Unsound outside chrono's supported year range (see
    /// `model_overflows_outside_supported_range` below): `year * 1461` overflows
    /// `i32` for `year` far from that range.
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

    /// chrono's real year part: its total day count minus its ordinal, both public.
    fn chrono_year_part(date: NaiveDate) -> i32 {
        date.num_days_from_ce() - i32::try_from(date.ordinal()).unwrap_or(0)
    }

    /// The forward wrapper: chrono's year part for a valid date.
    pub(super) fn forward_year_part(year: i32, month: u32, day: u32) -> Option<i32> {
        NaiveDate::from_ymd_opt(year, month, day).map(chrono_year_part)
    }
}

#[cfg(kani)]
pub(super) use kani_only::{CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count};
#[cfg(kani)]
use kani_only::{forward_year_part, model_year_part};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_day_count_staging::forward_equality_times_out".to_owned(),
            "gallery::chrono_day_count_staging::forward_equality_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "chrono's day count equals the integer spec only on a timeout, though each side passes alone".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, FORWARD_EQUALITY_TIMES_OUT_SRC, {
        /// The forward equality, combined: chrono's count from the common era equals the
        /// integer spec offset to the epoch, for every valid date. Confirmed to time out
        /// at three minutes, while the spec and the conversion each pass alone.
        #[kani::proof]
        fn forward_equality_times_out() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_day_count_staging::model_equality_times_out".to_owned(),
            "gallery::chrono_day_count_staging::model_equality_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "the model's year part, shape-identical to chrono's own formula, still times out when equated to chrono's real computed value".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, MODEL_EQUALITY_TIMES_OUT_SRC, {
        /// The model's year part equated to chrono's real year part, as a bare assertion
        /// (no contract machinery), for every valid date. Confirmed to time out at three
        /// minutes, even though the model mirrors chrono's own formula shape exactly and
        /// each side passes alone. Also confirmed: wrapping this in `kani::ensures`/
        /// `proof_for_contract` makes no difference — the contract framework was not the
        /// cost.
        #[kani::proof]
        fn model_equality_times_out() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            if let Some(part) = forward_year_part(year, month, day) {
                assert!(part == model_year_part(year));
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_day_count_staging::model_overflows_outside_supported_range".to_owned(),
            "gallery::chrono_day_count_staging::model_overflows_outside_supported_range".to_owned(),
            "amenable_kani".to_owned(),
            "the accommodation model's year*1461 multiplication overflows i32 for a year outside chrono's supported range".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, MODEL_OVERFLOWS_OUTSIDE_SUPPORTED_RANGE_SRC, {
        /// The model alone, over every `i32` year with no range bound. Confirmed to fail
        /// on a real overflow check (`year * 1461`), not a timeout: the model is only
        /// sound inside chrono's supported range, the same bound every spec in this file
        /// needs.
        #[kani::proof]
        fn model_overflows_outside_supported_range() {
            let year: i32 = kani::any();
            let _ = model_year_part(year);
        }
    }
}
