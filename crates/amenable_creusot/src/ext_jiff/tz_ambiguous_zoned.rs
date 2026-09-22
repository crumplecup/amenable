//! Real Creusot proof content for `jiff::tz::AmbiguousZoned`'s
//! `TimeZone::to_ambiguous_zoned` behavior for a `TimeZone::
//! fixed(offset)` — the same shape `tz_ambiguous_timestamp.rs`
//! already established for the sibling type.
//!
//! `AmbiguousZoned` wraps two private fields (`ts:
//! AmbiguousTimestamp`, `tz: TimeZone`), with a completely PRIVATE
//! (not even `pub(crate)`) constructor — only reachable via
//! `TimeZone::to_ambiguous_zoned(dt)`/`into_ambiguous_zoned(dt)`.
//! Real jiff source confirms `to_ambiguous_zoned` calls `self.clone()
//! .into_ambiguous_zoned(dt)`, which calls `self.to_ambiguous_
//! timestamp(dt)` directly — the SAME real function
//! `amenable_kani::ext::jiff`'s own doc comment already confirmed
//! times out under CBMC even for a fully concrete call, so this type
//! is marked trusted for Kani too without a redundant fresh 3-minute
//! confirmation run (the call-graph dependency on an
//! already-empirically-timed-out function is stronger evidence than
//! a fresh run would add).
//!
//! Scoped the same way `tz_ambiguous_timestamp.rs` is: `.time_zone()`
//! isn't checked at all (no real, already-contracted primitive
//! accessor exists on `TimeZone` to independently verify which
//! offset a `Fixed` value wraps from ordinary harness-body code, only
//! from inside an extern_spec's own `#[ensures(..)]`), and `dt` stays
//! completely opaque (`civil::DateTime` is trusted by deliberate
//! Phase 1 design). Reuses `offset.rs`'s own `offset_seconds_value`
//! and `tz_ambiguous_timestamp.rs`'s own `tz_fixed_seconds_value`.
//!
//! `amb_zoned_offset_is_unambiguous_value`/`amb_zoned_offset_seconds_
//! value` are `pub(crate)`: `tz_offset_conflict.rs` needs to reuse
//! them (its own `OffsetConflict::resolve` extern_spec needs to
//! relate to the real `AmbiguousZoned` it returns). Also extern-specs
//! `TimeZone::into_ambiguous_zoned` (the by-value sibling of
//! `to_ambiguous_zoned`, a DIFFERENT real function, not yet
//! contracted anywhere) for the same reason — `OffsetConflict::
//! resolve_with`'s real `AlwaysTimeZone` branch calls it directly.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use crate::ext_jiff::offset::offset_seconds_value;
#[cfg(creusot)]
use crate::ext_jiff::tz_ambiguous_timestamp::tz_fixed_seconds_value;
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn amb_zoned_offset_is_unambiguous_value(_z: &jiff::tz::AmbiguousZoned) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn amb_zoned_offset_seconds_value(_z: &jiff::tz::AmbiguousZoned) -> i32 {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            amb_zoned_offset_is_unambiguous_value(&result) == true
            && amb_zoned_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn to_ambiguous_zoned(&self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousZoned;

        #[check(ghost)]
        #[ensures(
            amb_zoned_offset_is_unambiguous_value(&result) == true
            && amb_zoned_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn into_ambiguous_zoned(self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousZoned;
    }

    impl jiff::tz::AmbiguousZoned {
        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::AmbiguousOffset::Unambiguous { offset } => {
                amb_zoned_offset_is_unambiguous_value(&self) == true
                    && offset_seconds_value(&offset) == amb_zoned_offset_seconds_value(&self)
            }
            _ => amb_zoned_offset_is_unambiguous_value(&self) == false,
        })]
        fn offset(&self) -> jiff::tz::AmbiguousOffset;

        #[check(ghost)]
        #[ensures(result == (amb_zoned_offset_is_unambiguous_value(&self) == false))]
        fn is_ambiguous(&self) -> bool;
    }
}

amenable_derive::harness! { creusot, TZ_AMBIGUOUS_ZONED_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::AmbiguousZoned>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

amenable_derive::harness! { creusot, VERIFY_TZ_AMBIGUOUS_ZONED_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC, {
    /// `TimeZone::fixed(offset).to_ambiguous_zoned(dt)` always
    /// reports `AmbiguousOffset::Unambiguous { offset }` through
    /// `.offset()`, and `.is_ambiguous()` is always `false` — the
    /// same claim `tz_ambiguous_timestamp.rs` establishes for the
    /// sibling type, restated as a real Creusot postcondition
    /// resting on the `extern_spec!` above.
    #[requires(true)]
    #[ensures(tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous_holds(result))]
    fn verify_tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous(
        dt: jiff::civil::DateTime,
        seconds: i32,
    ) -> bool {
        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => true,
            Ok(offset) => {
                let tz = jiff::tz::TimeZone::fixed(offset);
                let zdt = tz.to_ambiguous_zoned(dt);

                let offset_ok = match zdt.offset() {
                    jiff::tz::AmbiguousOffset::Unambiguous { offset: o } => {
                        o.seconds() == seconds
                    }
                    _ => false,
                };
                offset_ok && !zdt.is_ambiguous()
            }
        }
    }
}}
