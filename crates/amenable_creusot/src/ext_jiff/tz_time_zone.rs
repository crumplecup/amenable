//! Real Creusot proof content for `jiff::tz::TimeZone`'s own
//! `unknown`/`is_unknown`/`to_fixed_offset` behavior — the type
//! underlying every `Repr`-dispatch CBMC wall already confirmed this
//! session for `AmbiguousTimestamp`/`AmbiguousZoned`/`OffsetConflict`
//! (see `amenable_kani::ext::jiff`'s own doc comment); trusted for
//! Kani for the identical reason.
//!
//! `TimeZone::fixed`/`to_ambiguous_timestamp`/`to_ambiguous_zoned`/
//! `into_ambiguous_zoned` are ALREADY extern-spec'd in
//! `tz_ambiguous_timestamp.rs`/`tz_ambiguous_zoned.rs` (only one
//! `extern_spec!` per real function crate-wide) — this file adds
//! `unknown`/`is_unknown`/`to_fixed_offset`, the first real
//! TimeZone-only claims not already covered by a sibling file's own
//! `AmbiguousTimestamp`/`AmbiguousZoned`/`OffsetConflict` proof.
//! `tz_is_unknown_value` is `pub(crate)` so `tz_ambiguous_timestamp.
//! rs`'s own `fixed` extern_spec can extend its ensures with an extra
//! conjunct (`fixed`-constructed values are never unknown) instead of
//! redeclaring `fixed` itself.
//!
//! `to_fixed_offset`'s own real body ALSO dispatches through
//! `repr::each!` (confirmed by reading jiff's real source directly),
//! but that's irrelevant to Creusot — `extern_spec!` never executes
//! the real body, it's a trusted axiom either way. Scoped the same
//! way every sibling `TimeZone`-touching file is: only the
//! `TimeZone::fixed`-constructed case is exercised, reusing
//! `tz_ambiguous_timestamp.rs`'s own `tz_fixed_seconds_value`.

pub(crate) mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! { creusot, TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::TimeZone>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_time_zone_unknown_and_fixed_round_trip_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::tz_time_zone::tz_time_zone_unknown_and_fixed_round_trip_holds",
        "creusot",
        "ensures",
        || TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_SRC, {
    /// `TimeZone::unknown().is_unknown()` is always `true`;
    /// `TimeZone::fixed(offset)` is never unknown, and its own
    /// `.to_fixed_offset()` always round-trips `offset`'s own
    /// seconds — a real Creusot postcondition resting on the
    /// `extern_spec!` above.
    #[requires(true)]
    #[ensures(tz_time_zone_unknown_and_fixed_round_trip_holds(result))]
    fn verify_tz_time_zone_unknown_and_fixed_round_trip(seconds: i32) -> bool {
        let unknown_ok = jiff::tz::TimeZone::unknown().is_unknown();

        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => unknown_ok,
            Ok(offset) => {
                let tz = jiff::tz::TimeZone::fixed(offset);
                let not_unknown_ok = !tz.is_unknown();
                let fixed_offset_ok = match tz.to_fixed_offset() {
                    Ok(o) => o.seconds() == seconds,
                    Err(_) => false,
                };
                unknown_ok && not_unknown_ok && fixed_offset_ok
            }
        }
    }
}}
