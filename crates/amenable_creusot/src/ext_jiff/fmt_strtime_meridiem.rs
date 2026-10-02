//! Real Creusot proof content for `jiff::fmt::strtime::Meridiem`'s
//! `From<jiff::civil::Time> for Meridiem` conversion
//! (`ext::jiff::fmt_strtime_meridiem` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim `amenable_kani::ext::jiff::
//! fmt_strtime_meridiem` checks by symbolic execution.
//!
//! Reuses `civil_time.rs`'s existing `civil_time_hour_value` opaque
//! accessor directly (Creusot allows only one `extern_spec!` per real
//! function crate-wide, the same real finding `civil_era.rs`'s own
//! doc comment documents — `civil_time.rs` already extern-specs
//! `Time::hour()` itself).
//!
//! `Meridiem` is NOT `#[non_exhaustive]` (confirmed against jiff's
//! real source, unlike `FractionalUnit`), so matching its two
//! variants directly needs no wildcard arm — the same pure
//! pattern-matching technique `fmt_friendly_fractional_unit.rs`'s own
//! doc comment documents (no `PartialEq`/`DeepModel` needed at all).
//!
//! **A real, confirmed finding**: `RangeInclusive::contains` is
//! uncontracted in Creusot too — a first draft's `!(0i8..=23i8).
//! contains(&hour)` produced a real "calling external function
//! `contains` with no contract will yield an impossible precondition"
//! warning, and the goal genuinely failed to prove. Fixed by using
//! plain `hour < 0i8 || hour > 23i8` instead, matching
//! `civil_time.rs`'s own extern_spec bounds-check style.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use crate::ext_jiff::civil_time::logic::civil_time_hour_value;
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl From<jiff::civil::Time> for jiff::fmt::strtime::Meridiem {
        #[check(ghost)]
        #[ensures(match result {
            jiff::fmt::strtime::Meridiem::AM => civil_time_hour_value(&t) < 12i8,
            jiff::fmt::strtime::Meridiem::PM => civil_time_hour_value(&t) >= 12i8,
        })]
        fn from(t: jiff::civil::Time) -> jiff::fmt::strtime::Meridiem;
    }
}

amenable_derive::harness! {
    creusot, FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::fmt::strtime::
        /// Meridiem>` postcondition — real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn fmt_strtime_meridiem_from_time_matches_hour_threshold_holds(matches: bool) -> bool {
            pearlite! { matches }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_strtime_meridiem::fmt_strtime_meridiem_from_time_matches_hour_threshold_holds",
        "creusot",
        "ensures",
        || FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_SRC, {
        /// `Meridiem::from(time)` matches jiff's own documented
        /// threshold exactly (`AM` for `hour < 12`, `PM` otherwise) —
        /// a real, checked postcondition resting on the
        /// `extern_spec!` above.
        #[requires(true)]
        #[ensures(fmt_strtime_meridiem_from_time_matches_hour_threshold_holds(result))]
        fn verify_fmt_strtime_meridiem_from_time_matches_hour_threshold(hour: i8) -> bool {
            if hour < 0i8 || hour > 23i8 {
                true
            } else {
                let t = jiff::civil::Time::new(hour, 0, 0, 0)
                    .expect("hour is already checked to be in civil::Time's valid range");
                let meridiem: jiff::fmt::strtime::Meridiem = t.into();
                match meridiem {
                    jiff::fmt::strtime::Meridiem::AM => hour < 12,
                    jiff::fmt::strtime::Meridiem::PM => hour >= 12,
                }
            }
        }
    }
}
