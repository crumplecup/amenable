#![cfg(creusot)]
//! `jiff::tz::TimeZoneDatabase`'s trusted logic axiom and
//! `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was two separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.

use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
fn tzdb_is_definitively_empty_value(_db: &jiff::tz::TimeZoneDatabase) -> bool {
    dead
}

extern_spec! {
    impl jiff::tz::TimeZoneDatabase {
        #[check(ghost)]
        #[ensures(tzdb_is_definitively_empty_value(&result) == true)]
        fn none() -> jiff::tz::TimeZoneDatabase;

        #[check(ghost)]
        #[ensures(result == tzdb_is_definitively_empty_value(&self))]
        fn is_definitively_empty(&self) -> bool;
    }
}
