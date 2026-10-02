#![cfg(creusot)]
//! `jiff::fmt::temporal::TimeZoneAnnotationName`'s trusted logic axiom
//! and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was two separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.

use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
fn tzan_str_value<'a, 'n>(_t: &'a jiff::fmt::temporal::TimeZoneAnnotationName<'n>) -> &'a str {
    dead
}

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
