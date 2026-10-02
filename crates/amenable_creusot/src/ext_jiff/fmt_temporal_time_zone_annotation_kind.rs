//! Real Creusot proof content for `jiff::fmt::temporal::
//! TimeZoneAnnotationKind<'static>`'s `From<&str>`/`From<Offset>`
//! constructors — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_kind`'s own doc comment checks
//! by symbolic execution.
//!
//! Unlike `TimeZoneAnnotation<'static>` (whose `kind`/`critical`
//! fields are `pub(crate)` to jiff, needing opaque accessors behind
//! its public getters), `TimeZoneAnnotationKind`'s own variants are
//! themselves the public API — `result` in each constructor's own
//! `extern_spec!` ensures IS the value being constructed, so this
//! matches directly on it (the same `fmt_friendly_fractional_unit.rs`/
//! `fmt_temporal_pieces_offset.rs`-established technique for a
//! foreign enum), needing NO opaque accessor at all for the
//! discriminant. Reuses `offset.rs`'s own `offset_seconds_value` for
//! the `Offset` variant's payload.
//!
//! A wildcard arm (`_ => false`/`_ => true`) is needed for the same
//! `#[non_exhaustive]` reason `fmt_friendly_fractional_unit.rs`
//! documents.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use crate::ext_jiff::shared_trusted_accessors::offset_seconds_value;
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl<'n> core::convert::From<&'n str> for jiff::fmt::temporal::TimeZoneAnnotationKind<'n> {
        #[check(ghost)]
        #[ensures(match result {
            jiff::fmt::temporal::TimeZoneAnnotationKind::Named(_) => true,
            _ => false,
        })]
        fn from(string: &'n str) -> jiff::fmt::temporal::TimeZoneAnnotationKind<'n>;
    }

    impl core::convert::From<jiff::tz::Offset> for jiff::fmt::temporal::TimeZoneAnnotationKind<'static> {
        #[check(ghost)]
        #[ensures(match result {
            jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(o) => {
                offset_seconds_value(&o) == offset_seconds_value(&offset)
            }
            _ => false,
        })]
        fn from(offset: jiff::tz::Offset) -> jiff::fmt::temporal::TimeZoneAnnotationKind<'static>;
    }
}

amenable_derive::harness! { creusot, FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::fmt::temporal::
    /// TimeZoneAnnotationKind<'static>>` postcondition — real,
    /// callable Pearlite content, not just descriptive text alongside
    /// it.
    #[logic(open)]
    fn fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_temporal_time_zone_annotation_kind::fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_holds",
        "creusot",
        "ensures",
        || FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_SRC, {
    /// `TimeZoneAnnotationKind::from(name)` always builds the
    /// `Named` variant; `TimeZoneAnnotationKind::from(offset)` always
    /// builds the `Offset` variant carrying `offset`'s own seconds —
    /// a real Creusot postcondition resting on the `extern_spec!`
    /// above.
    #[requires(true)]
    #[ensures(fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_holds(result))]
    fn verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset(
        seconds: i32,
    ) -> bool {
        let kind_named = jiff::fmt::temporal::TimeZoneAnnotationKind::from("x");
        let named_ok = matches!(
            kind_named,
            jiff::fmt::temporal::TimeZoneAnnotationKind::Named(_)
        );

        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => named_ok,
            Ok(offset) => {
                let kind_offset = jiff::fmt::temporal::TimeZoneAnnotationKind::from(offset);
                let offset_ok = match kind_offset {
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
