//! Real Creusot proof content for `jiff::tz::TimeZoneDatabase`'s
//! `none`/`is_definitively_empty` behavior — the same claim
//! `amenable_kani::ext::jiff::tz_time_zone_database`'s own doc
//! comment checks by symbolic execution.
//!
//! `TimeZoneDatabase` has a substantial public surface, but only
//! `none()`/`is_definitively_empty()` are checked here — see
//! `amenable_kani::ext::jiff::tz_time_zone_database`'s own doc
//! comment for why `get()`/`bundled()`/etc. are out of scope
//! (real IANA tzdb data / `jiff::Error` construction).

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! { creusot, TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::TimeZoneDatabase>`
    /// postcondition — real, callable Pearlite content, not just
    /// descriptive text alongside it.
    #[logic(open)]
    fn tz_time_zone_database_none_is_definitively_empty_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::tz_time_zone_database::tz_time_zone_database_none_is_definitively_empty_holds",
        "creusot",
        "ensures",
        || TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC, {
    /// `TimeZoneDatabase::none().is_definitively_empty()` is always
    /// `true` — a real Creusot postcondition resting on the
    /// `extern_spec!` above.
    #[requires(true)]
    #[ensures(tz_time_zone_database_none_is_definitively_empty_holds(result))]
    fn verify_tz_time_zone_database_none_is_definitively_empty() -> bool {
        jiff::tz::TimeZoneDatabase::none().is_definitively_empty()
    }
}}
