//! Real Creusot proof content for `jiff::tz::Dst`'s `From<bool>`/
//! `is_dst`/`is_std` round trip — the same claim
//! `amenable_kani::ext::jiff::tz_dst`'s own doc comment checks by
//! symbolic execution.
//!
//! `Dst` is a real, plain (NOT `#[non_exhaustive]`) two-variant enum
//! — matches exhaustively on `self`/`result` directly (the same
//! `fmt_friendly_fractional_unit.rs`-established technique for a
//! foreign enum), needing no opaque accessor and no wildcard arm.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::Dst {
        #[check(ghost)]
        #[ensures(result == match self {
            jiff::tz::Dst::Yes => true,
            jiff::tz::Dst::No => false,
        })]
        fn is_dst(self) -> bool;

        #[check(ghost)]
        #[ensures(result == match self {
            jiff::tz::Dst::No => true,
            jiff::tz::Dst::Yes => false,
        })]
        fn is_std(self) -> bool;
    }

    impl core::convert::From<bool> for jiff::tz::Dst {
        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::Dst::Yes => is_dst == true,
            jiff::tz::Dst::No => is_dst == false,
        })]
        fn from(is_dst: bool) -> jiff::tz::Dst;
    }
}

amenable_derive::harness! { creusot, TZ_DST_FROM_BOOL_ROUND_TRIPS_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::tz::Dst>` postcondition —
    /// real, callable Pearlite content, not just descriptive text
    /// alongside it.
    #[logic(open)]
    fn tz_dst_from_bool_round_trips_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

amenable_derive::harness! { creusot, VERIFY_TZ_DST_FROM_BOOL_ROUND_TRIPS_SRC, {
    /// `Dst::from(is_dst).is_dst()` always equals `is_dst`, and
    /// `.is_std()` is always its exact complement — a real Creusot
    /// postcondition resting on the `extern_spec!` above.
    #[requires(true)]
    #[ensures(tz_dst_from_bool_round_trips_holds(result))]
    fn verify_tz_dst_from_bool_round_trips(is_dst: bool) -> bool {
        let dst = jiff::tz::Dst::from(is_dst);
        dst.is_dst() == is_dst && dst.is_std() == !is_dst
    }
}}
