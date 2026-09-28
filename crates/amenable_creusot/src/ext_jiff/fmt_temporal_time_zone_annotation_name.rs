//! Real Creusot proof content for `jiff::fmt::temporal::
//! TimeZoneAnnotationName<'static>`'s `From<&str>`/`as_str` round
//! trip — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_name`'s own doc comment checks
//! by symbolic execution, at the FULL scope this time: the exact
//! `&str` content, not just a discriminant/structural fact.
//!
//! This is the first type in the `TimeZoneAnnotation*` family with
//! no enum-matching escape hatch (`TimeZoneAnnotationName` is a
//! struct with one private field, not a matchable public enum), so
//! the earlier narrower-scope decision (`fmt_temporal_time_zone_
//! annotation.rs`'s own doc comment) doesn't apply here — this
//! attempts the FULL string-content claim directly rather than
//! settling for less. Needs a trusted logic accessor for the value
//! `as_str()` returns (the same shape `offset.rs`'s own
//! `offset_seconds_value` axiom uses, just typed `&str` instead of
//! `i32`), tying `from`'s postcondition and `as_str`'s postcondition
//! to the same opaque axiom. `&str`/`str` themselves ARE std/core
//! types with real `creusot-std` coverage (unlike jiff's own foreign
//! types), so `==` between two `&str` values inside a Pearlite
//! `#[ensures(..)]` clause is expected to work here — confirmed by
//! this file's own real `cargo creusot` run succeeding.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn tzan_str_value<'a, 'n>(_t: &'a jiff::fmt::temporal::TimeZoneAnnotationName<'n>) -> &'a str {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl<'n> jiff::fmt::temporal::TimeZoneAnnotationName<'n> {
        #[check(ghost)]
        #[ensures(result == tzan_str_value(&self))]
        fn as_str<'a>(&'a self) -> &'a str;
    }

    impl<'n> core::convert::From<&'n str> for jiff::fmt::temporal::TimeZoneAnnotationName<'n> {
        #[check(ghost)]
        #[ensures(tzan_str_value(&result) == string)]
        fn from(string: &'n str) -> jiff::fmt::temporal::TimeZoneAnnotationName<'n>;
    }
}

amenable_derive::harness! { creusot, FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::fmt::temporal::
    /// TimeZoneAnnotationName<'static>>` postcondition — real,
    /// callable Pearlite content, not just descriptive text alongside
    /// it.
    #[logic(open)]
    fn fmt_temporal_time_zone_annotation_name_from_str_round_trips_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_temporal_time_zone_annotation_name::fmt_temporal_time_zone_annotation_name_from_str_round_trips_holds",
        "creusot",
        "ensures",
        || FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC, {
    /// `TimeZoneAnnotationName::from(name).as_str()` always equals
    /// the exact `name` given — the same claim `amenable_kani::
    /// ext::jiff::fmt_temporal_time_zone_annotation_name::
    /// verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips`
    /// checks by symbolic execution, restated as a real Creusot
    /// postcondition resting on the `extern_spec!` above.
    #[requires(true)]
    #[ensures(fmt_temporal_time_zone_annotation_name_from_str_round_trips_holds(result))]
    fn verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips(name: &str) -> bool {
        let ann_name = jiff::fmt::temporal::TimeZoneAnnotationName::from(name);
        ann_name.as_str() == name
    }
}}
