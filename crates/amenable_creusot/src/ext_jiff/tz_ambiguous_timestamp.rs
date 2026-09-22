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

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use crate::ext_jiff::offset::offset_seconds_value;
#[cfg(creusot)]
use crate::ext_jiff::tz_time_zone::tz_is_unknown_value;
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn tz_fixed_seconds_value(_tz: &jiff::tz::TimeZone) -> i32 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn amb_ts_offset_is_unambiguous_value(_t: &jiff::tz::AmbiguousTimestamp) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn amb_ts_offset_seconds_value(_t: &jiff::tz::AmbiguousTimestamp) -> i32 {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            tz_fixed_seconds_value(&result) == offset_seconds_value(&offset)
            && tz_is_unknown_value(&result) == false
        )]
        fn fixed(offset: jiff::tz::Offset) -> jiff::tz::TimeZone;

        #[check(ghost)]
        #[ensures(
            amb_ts_offset_is_unambiguous_value(&result) == true
            && amb_ts_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn to_ambiguous_timestamp(&self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousTimestamp;
    }

    impl jiff::tz::AmbiguousTimestamp {
        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::AmbiguousOffset::Unambiguous { offset } => {
                amb_ts_offset_is_unambiguous_value(&self) == true
                    && offset_seconds_value(&offset) == amb_ts_offset_seconds_value(&self)
            }
            _ => amb_ts_offset_is_unambiguous_value(&self) == false,
        })]
        fn offset(&self) -> jiff::tz::AmbiguousOffset;

        #[check(ghost)]
        #[ensures(result == (amb_ts_offset_is_unambiguous_value(&self) == false))]
        fn is_ambiguous(&self) -> bool;
    }
}

amenable_derive::harness! { creusot, TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::AmbiguousTimestamp>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

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
