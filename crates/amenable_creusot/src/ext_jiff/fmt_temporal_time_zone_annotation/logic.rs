#![cfg(creusot)]
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>`'s trusted logic
//! axioms and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was five separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.

use crate::ext_jiff::offset::logic::offset_seconds_value;
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
fn tza_is_critical_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> bool {
    dead
}

#[trusted]
#[logic(opaque)]
fn tza_kind_is_named_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> bool {
    dead
}

#[trusted]
#[logic(opaque)]
fn tza_kind_offset_seconds_value(_t: &jiff::fmt::temporal::TimeZoneAnnotation<'static>) -> i32 {
    dead
}

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
