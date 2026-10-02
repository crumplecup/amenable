#![cfg(creusot)]
//! `jiff::civil::Era`'s classification axiom and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was four separately `#[cfg(creusot)]`-gated items in the parent
//! file (three `use`, one `fn`) down to a single macro-import `use`
//! there, cordial's CFG-SCATTER finding. `era_discriminant` stays
//! fully `pub` (not `pub(crate)`): the parent's `#[logic(open)]`
//! harness-generated functions call it, and Creusot's proof-
//! transparency check requires a called item to be at least as
//! visible as its caller — confirmed empirically (two distinct
//! compiler errors, resolved only once this was plain `pub`; see the
//! parent file's own doc comment for the full story).

use crate::ext_jiff::shared_trusted_accessors::date_year_value;
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub fn era_discriminant(_e: &jiff::civil::Era) -> i8 {
    dead
}

// This crate's own axiom for "which Era variant this is" — 0 for
// BCE, 1 for CE. Doesn't need to match jiff's real discriminant
// values (private either way); only needs to be internally
// consistent within this file's own extern_spec.
extern_spec! {
    impl jiff::civil::Date {
        #[check(ghost)]
        #[ensures(if date_year_value(&self) >= 1i16 {
            result.0 == date_year_value(&self) && era_discriminant(&result.1) == 1i8
        } else {
            result.0 == -date_year_value(&self) + 1i16 && era_discriminant(&result.1) == 0i8
        })]
        fn era_year(self) -> (i16, jiff::civil::Era);
    }
}
