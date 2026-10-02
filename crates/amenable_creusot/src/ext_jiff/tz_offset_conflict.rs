//! Real Creusot proof content for `jiff::tz::OffsetConflict::resolve`'s
//! `AlwaysOffset`/`AlwaysTimeZone` behavior — the same claim
//! `amenable_kani::ext::jiff`'s own doc comment documents as
//! Kani-uncheckable (real jiff source shows `resolve_with`'s
//! `AlwaysTimeZone` branch calls `TimeZone::into_ambiguous_zoned`
//! directly — the SAME real function already confirmed timing out
//! under CBMC).
//!
//! `OffsetConflict` is a real, `#[non_exhaustive]` FOUR-variant enum,
//! but only `AlwaysOffset`/`AlwaysTimeZone` are checked here:
//! `PreferOffset`/`Reject` delegate to PRIVATE helpers
//! (`resolve_via_prefer`/`resolve_via_reject`) with their own,
//! separately-nontrivial logic (an `is_equal` closure parameter, real
//! error conditions) — disproportionate to add here, matching this
//! checklist's own established "checked subset, not exhaustive
//! reproduction" discipline (see `offset.rs`'s own doc comment for
//! the precedent on that phrase). Real jiff source directly confirms
//! `AlwaysOffset`/`AlwaysTimeZone`'s own bodies:
//! `resolve_with` calls `AmbiguousTimestamp::new(dt, Unambiguous {
//! offset }).into_ambiguous_zoned(tz)` for `AlwaysOffset`, and
//! `tz.into_ambiguous_zoned(dt)` directly for `AlwaysTimeZone`.
//!
//! Reuses `tz_ambiguous_zoned.rs`'s own `amb_zoned_offset_is_
//! unambiguous_value`/`amb_zoned_offset_seconds_value` (hoisted
//! `pub(crate)` for this purpose) and `tz_ambiguous_timestamp.rs`'s
//! own `tz_fixed_seconds_value`, plus `offset.rs`'s own
//! `offset_seconds_value` — no fresh opaque accessor needed here at
//! all. A wildcard arm (`_ => true`) covers `PreferOffset`/`Reject`,
//! vacuously true since nothing is claimed about them.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use crate::ext_jiff::shared_trusted_accessors::{
    amb_zoned_offset_is_unambiguous_value, amb_zoned_offset_seconds_value, offset_seconds_value,
    tz_fixed_seconds_value,
};
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::OffsetConflict {
        #[check(ghost)]
        #[ensures(match self {
            jiff::tz::OffsetConflict::AlwaysOffset => match result {
                Ok(ref z) => {
                    amb_zoned_offset_is_unambiguous_value(z) == true
                        && amb_zoned_offset_seconds_value(z) == offset_seconds_value(&offset)
                }
                Err(_) => false,
            },
            jiff::tz::OffsetConflict::AlwaysTimeZone => match result {
                Ok(ref z) => {
                    amb_zoned_offset_is_unambiguous_value(z) == true
                        && amb_zoned_offset_seconds_value(z) == tz_fixed_seconds_value(&tz)
                }
                Err(_) => false,
            },
            _ => true,
        })]
        fn resolve(
            self,
            dt: jiff::civil::DateTime,
            offset: jiff::tz::Offset,
            tz: jiff::tz::TimeZone,
        ) -> Result<jiff::tz::AmbiguousZoned, jiff::Error>;
    }
}

amenable_derive::harness! { creusot, TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::OffsetConflict>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_offset_conflict_always_offset_and_always_time_zone_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::tz_offset_conflict::tz_offset_conflict_always_offset_and_always_time_zone_holds",
        "creusot",
        "ensures",
        || TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_SRC, {
    /// `OffsetConflict::AlwaysOffset.resolve(dt, offset_claimed,
    /// tz)` always uses `offset_claimed` (ignoring `tz`'s own
    /// offset), and `OffsetConflict::AlwaysTimeZone.resolve(dt,
    /// offset_claimed, tz)` always uses `tz`'s own offset (ignoring
    /// `offset_claimed`) — checked against a `TimeZone::fixed`
    /// wrapping a DIFFERENT offset than the one claimed, a genuine
    /// conflict scenario, fully controllable without real IANA tzdb
    /// data. A real Creusot postcondition resting on the
    /// `extern_spec!` above.
    #[requires(true)]
    #[ensures(tz_offset_conflict_always_offset_and_always_time_zone_holds(result))]
    fn verify_tz_offset_conflict_always_offset_and_always_time_zone(
        dt: jiff::civil::DateTime,
        claimed_seconds: i32,
        actual_seconds: i32,
    ) -> bool {
        match jiff::tz::Offset::from_seconds(claimed_seconds) {
            Err(_) => true,
            Ok(claimed_offset) => match jiff::tz::Offset::from_seconds(actual_seconds) {
                Err(_) => true,
                Ok(actual_offset) => {
                    let tz = jiff::tz::TimeZone::fixed(actual_offset);

                    let always_offset_ok = match jiff::tz::OffsetConflict::AlwaysOffset.resolve(
                        dt,
                        claimed_offset,
                        tz.clone(),
                    ) {
                        Ok(z) => match z.offset() {
                            jiff::tz::AmbiguousOffset::Unambiguous { offset: o } => {
                                o.seconds() == claimed_seconds
                            }
                            _ => false,
                        },
                        Err(_) => false,
                    };

                    let always_time_zone_ok = match jiff::tz::OffsetConflict::AlwaysTimeZone
                        .resolve(dt, claimed_offset, tz)
                    {
                        Ok(z) => match z.offset() {
                            jiff::tz::AmbiguousOffset::Unambiguous { offset: o } => {
                                o.seconds() == actual_seconds
                            }
                            _ => false,
                        },
                        Err(_) => false,
                    };

                    always_offset_ok && always_time_zone_ok
                }
            },
        }
    }
}}
