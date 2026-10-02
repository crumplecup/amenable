//! Real Creusot proof content for `jiff::tz::AmbiguousTimestamp`'s
//! `TimeZone::to_ambiguous_timestamp` behavior for a `TimeZone::
//! fixed(offset)` — the same claim `amenable_kani::ext::jiff::mod`'s
//! own doc comment documents as Kani-uncheckable (a real `TimeZone`
//! `Repr`-dispatch CBMC wall, unrelated to Drop glue).
//!
//! `AmbiguousTimestamp`'s own constructor is `pub(crate)`, only
//! reachable via `TimeZone::to_ambiguous_timestamp(dt)`. Deliberately
//! NARROWER than Kani's own (now-abandoned) attempt at the full
//! claim: doesn't check `.datetime()` round-trips `dt` at all —
//! `civil::DateTime` is itself trusted/opaque in this crate (no
//! decomposition accessors exist for it, by deliberate Phase 1
//! design), so `dt` is passed through this proof entirely as an
//! opaque black-box value, never inspected. Checks only `.offset()`/
//! `.is_ambiguous()`, decomposed via fresh opaque accessors (`amb_ts_
//! offset_is_unambiguous_value`/`amb_ts_offset_seconds_value`) tied
//! to a `TimeZone::fixed`-specific opaque accessor
//! (`tz_fixed_seconds_value`, `pub(crate)` at this module's own top
//! level for future reuse — `TimeZone` itself is a later, not-yet-
//! assessed checklist entry, matching `civil_date.rs`'s own
//! established cross-file-reuse convention).
//!
//! Matches directly on `AmbiguousOffset`'s own public variants (the
//! `fmt_friendly_fractional_unit.rs`-established technique) rather
//! than comparing whole `AmbiguousOffset` values via `==`.
//!
//! `fixed`'s own `extern_spec!` lives here (the first file to need
//! it), so `tz_time_zone.rs`'s own `unknown`/`is_unknown` claim
//! (`fixed`-constructed values are never unknown) extends THIS
//! ensures clause with an extra conjunct rather than redeclaring
//! `fixed` itself — only one `extern_spec!` per real function
//! crate-wide.

pub(crate) mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! { creusot, TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::AmbiguousTimestamp>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::tz_ambiguous_timestamp::tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_holds",
        "creusot",
        "ensures",
        || TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC, {
    /// `TimeZone::fixed(offset).to_ambiguous_timestamp(dt)` always
    /// reports `AmbiguousOffset::Unambiguous { offset }` through
    /// `.offset()`, and `.is_ambiguous()` is always `false` — the
    /// same claim `amenable_kani`'s own (now Kani-uncheckable)
    /// attempt documents, restated as a real Creusot postcondition
    /// resting on the `extern_spec!` above, at the narrower scope
    /// this module's own doc comment documents (`dt` stays opaque).
    #[requires(true)]
    #[ensures(tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_holds(result))]
    fn verify_tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous(
        dt: jiff::civil::DateTime,
        seconds: i32,
    ) -> bool {
        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => true,
            Ok(offset) => {
                let tz = jiff::tz::TimeZone::fixed(offset);
                let ts = tz.to_ambiguous_timestamp(dt);

                let offset_ok = match ts.offset() {
                    jiff::tz::AmbiguousOffset::Unambiguous { offset: o } => {
                        o.seconds() == seconds
                    }
                    _ => false,
                };
                offset_ok && !ts.is_ambiguous()
            }
        }
    }
}}
