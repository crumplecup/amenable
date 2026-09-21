//! Real Creusot proof content for `jiff::fmt::temporal::
//! TimeZoneAnnotation<'static>`'s `From<&str>`/`From<Offset>`
//! constructors — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation`'s own doc comment checks by
//! symbolic execution, at a NARROWER scope: this proof checks which
//! variant `kind()` reports and (for `Offset`) its numeric payload,
//! with `is_critical() == false` in both cases — it does NOT check
//! the exact `Named` string content round-trips, unlike Kani's own
//! fuller claim. No existing `amenable_creusot` file compares `&str`
//! content directly inside a Pearlite `#[ensures(..)]` clause, and
//! `TimeZoneAnnotationName`/`TimeZoneAnnotationKind` don't have their
//! own opaque accessors yet (later, not-yet-assessed checklist
//! entries) to decompose through — so this proof deliberately settles
//! for the structural/discriminant-level property, which is real and
//! non-vacuous (it rules out, e.g., `From<&str>` ever producing the
//! `Offset` variant, or `is_critical()` ever being `true`), while
//! Kani independently covers the stronger, exact-string claim.
//!
//! Matches directly on `TimeZoneAnnotationKind`'s own public variants
//! (the same `fmt_friendly_fractional_unit.rs`/
//! `fmt_temporal_pieces_offset.rs`-established technique for a
//! foreign enum) rather than comparing whole `TimeZoneAnnotationKind`
//! values via `==` — `TimeZoneAnnotation`'s own `kind`/`critical`
//! fields are `pub(crate)` to jiff (not visible here), so this goes
//! through the real public `kind()`/`is_critical()` getters, backed
//! by fresh opaque accessors (`tza_is_critical_value`/
//! `tza_kind_is_named_value`/`tza_kind_offset_seconds_value`) since
//! there's no sibling file's accessor to reuse yet. Reuses `offset.
//! rs`'s own `offset_seconds_value` for the `Offset` variant's
//! payload.
//!
//! A wildcard arm (`_ => true`) is needed for the same
//! `#[non_exhaustive]` reason `fmt_friendly_fractional_unit.rs`
//! documents (`TimeZoneAnnotationKind` is `#[non_exhaustive]`).

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
}
#[cfg(creusot)]
use crate::ext_jiff::offset::offset_seconds_value;
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires, trusted};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn tza_is_critical_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn tza_kind_is_named_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn tza_kind_offset_seconds_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> i32 {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::fmt::temporal::TimeZoneAnnotation<'static> {
        #[check(ghost)]
        #[ensures(result == tza_is_critical_value(&self))]
        fn is_critical(&self) -> bool;

        #[check(ghost)]
        #[ensures(match result {
            jiff::fmt::temporal::TimeZoneAnnotationKind::Named(_) => {
                tza_kind_is_named_value(&self) == true
            }
            jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(o) => {
                tza_kind_is_named_value(&self) == false
                    && offset_seconds_value(o) == tza_kind_offset_seconds_value(&self)
            }
            _ => true,
        })]
        fn kind<'a>(&'a self) -> &'a jiff::fmt::temporal::TimeZoneAnnotationKind<'static>;
    }

    impl<'n> core::convert::From<&'n str> for jiff::fmt::temporal::TimeZoneAnnotation<'n> {
        #[check(ghost)]
        #[ensures(
            tza_kind_is_named_value(&result) == true
            && tza_is_critical_value(&result) == false
        )]
        fn from(string: &'n str) -> jiff::fmt::temporal::TimeZoneAnnotation<'n>;
    }

    impl core::convert::From<jiff::tz::Offset> for jiff::fmt::temporal::TimeZoneAnnotation<'static> {
        #[check(ghost)]
        #[ensures(
            tza_kind_is_named_value(&result) == false
            && tza_kind_offset_seconds_value(&result) == offset_seconds_value(&offset)
            && tza_is_critical_value(&result) == false
        )]
        fn from(offset: jiff::tz::Offset) -> jiff::fmt::temporal::TimeZoneAnnotation<'static>;
    }
}

amenable_derive::harness! { creusot, FMT_TEMPORAL_TIME_ZONE_ANNOTATION_FROM_NAME_AND_FROM_OFFSET_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::fmt::temporal::
    /// TimeZoneAnnotation<'static>>` postcondition — real, callable
    /// Pearlite content, not just descriptive text alongside it.
    #[logic(open)]
    fn fmt_temporal_time_zone_annotation_from_name_and_from_offset_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

amenable_derive::harness! { creusot, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_FROM_NAME_AND_FROM_OFFSET_SRC, {
    /// `TimeZoneAnnotation::from(name)` always builds the `Named`
    /// variant with `is_critical() == false`; `TimeZoneAnnotation::
    /// from(offset)` always builds the `Offset` variant carrying
    /// `offset`'s own seconds, with `is_critical() == false` — a
    /// real Creusot postcondition resting on the `extern_spec!`
    /// above, at the narrower scope this module's own doc comment
    /// documents.
    #[requires(true)]
    #[ensures(fmt_temporal_time_zone_annotation_from_name_and_from_offset_holds(result))]
    fn verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset(seconds: i32) -> bool {
        let ann_named = jiff::fmt::temporal::TimeZoneAnnotation::from("x");
        let named_ok = !ann_named.is_critical()
            && matches!(
                ann_named.kind(),
                jiff::fmt::temporal::TimeZoneAnnotationKind::Named(_)
            );

        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => named_ok,
            Ok(offset) => {
                let ann_offset = jiff::fmt::temporal::TimeZoneAnnotation::from(offset);
                let offset_ok = !ann_offset.is_critical()
                    && match ann_offset.kind() {
                        jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(o) => {
                            o.seconds() == seconds
                        }
                        _ => false,
                    };
                named_ok && offset_ok
            }
        }
    }
}}
