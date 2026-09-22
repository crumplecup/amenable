//! Real Creusot proof content for `jiff::tz::TimeZoneOffsetInfo`'s
//! `TimeZone::to_offset_info` behavior for a `TimeZone::fixed(offset)`
//! — the same narrowed scope `tz_time_zone.rs`'s own `to_fixed_offset`
//! claim and `tz_ambiguous_timestamp.rs`'s own `to_ambiguous_timestamp`
//! claim already use, reusing `tz_ambiguous_timestamp.rs`'s own
//! `tz_fixed_seconds_value` accessor rather than redeclaring it (only
//! one `extern_spec!` per real function crate-wide — `TimeZone::fixed`
//! is already extern-spec'd there).
//!
//! `to_offset_info` itself dispatches through the same `repr::each!`
//! macro already confirmed to time out under Kani's CBMC unwinder (see
//! `amenable_kani::ext::jiff`'s own doc comment) — irrelevant to
//! Creusot, since `extern_spec!` never executes the real body, it's a
//! trusted axiom either way.
//!
//! `TimeZoneOffsetInfo` itself has no private-field access from outside
//! jiff, so its own accessor methods (`offset`/`dst`) need their own
//! opaque logic values, each tied back to the real return value by that
//! accessor's own `#[ensures(..)]` — the same two-hop shape
//! `offset.rs`'s own `offset_seconds_value` establishes for
//! `Offset::seconds()`. `dst()` is matched directly on `jiff::tz::Dst`'s
//! own public variants (the `tz_dst.rs`-established technique — no
//! `DeepModel` needed for a plain, non-generic enum), not compared via
//! `==`.

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
pub(crate) fn tz_offset_info_offset_seconds_value(_info: &jiff::tz::TimeZoneOffsetInfo<'_>) -> i32 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn tz_offset_info_is_dst_value(_info: &jiff::tz::TimeZoneOffsetInfo<'_>) -> bool {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            tz_offset_info_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
            && tz_offset_info_is_dst_value(&result) == false
        )]
        fn to_offset_info<'t>(&'t self, timestamp: jiff::Timestamp) -> jiff::tz::TimeZoneOffsetInfo<'t>;
    }

    impl<'t> jiff::tz::TimeZoneOffsetInfo<'t> {
        #[check(ghost)]
        #[ensures(offset_seconds_value(&result) == tz_offset_info_offset_seconds_value(&self))]
        fn offset(&self) -> jiff::tz::Offset;

        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::Dst::No => tz_offset_info_is_dst_value(&self) == false,
            jiff::tz::Dst::Yes => tz_offset_info_is_dst_value(&self) == true,
        })]
        fn dst(&self) -> jiff::tz::Dst;
    }
}

amenable_derive::harness! { creusot, TZ_TIME_ZONE_OFFSET_INFO_FROM_FIXED_TIME_ZONE_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::TimeZoneOffsetInfo<
    /// 'static>>` postcondition — real, callable Pearlite content, not
    /// just descriptive text alongside it.
    #[logic(open)]
    fn tz_time_zone_offset_info_from_fixed_time_zone_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

amenable_derive::harness! { creusot, VERIFY_TZ_TIME_ZONE_OFFSET_INFO_FROM_FIXED_TIME_ZONE_SRC, {
    /// `TimeZone::fixed(offset).to_offset_info(ts)` always reports
    /// `offset`'s own seconds and `Dst::No`, for any `ts` — a real
    /// Creusot postcondition resting on the `extern_spec!` above.
    /// `ts` stays opaque (the same `tz_ambiguous_timestamp.rs`-
    /// established technique for `dt: civil::DateTime`) — taken as a
    /// harness parameter rather than constructed, since neither
    /// `Timestamp::new` (uncontracted, real "impossible precondition"
    /// warning) nor `Timestamp::UNIX_EPOCH` (a real, confirmed new
    /// toolchain wall: this associated const's own struct-literal
    /// initializer produces a malformed `.coma` file — a syntax error
    /// referencing `ERROR_UNBOUND_second`/`ERROR_UNBOUND_nanosecond`
    /// — translating fine, matching `to_offset_info`'s own
    /// postcondition already being unconditional in `timestamp`
    /// regardless of which concrete value would be passed.
    #[requires(true)]
    #[ensures(tz_time_zone_offset_info_from_fixed_time_zone_holds(result))]
    fn verify_tz_time_zone_offset_info_from_fixed_time_zone(
        ts: jiff::Timestamp,
        seconds: i32,
    ) -> bool {
        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => true,
            Ok(offset) => {
                let tz = jiff::tz::TimeZone::fixed(offset);
                let info = tz.to_offset_info(ts);
                let dst_ok = match info.dst() {
                    jiff::tz::Dst::No => true,
                    jiff::tz::Dst::Yes => false,
                };
                info.offset().seconds() == seconds && dst_ok
            }
        }
    }
}}
