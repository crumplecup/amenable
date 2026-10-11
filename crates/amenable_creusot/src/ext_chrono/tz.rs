//! Real Creusot proof content for `chrono_tz::Tz`'s `name()`/`FromStr`
//! round trip (`chrono::tz` holds the `CreusotWitness` bridge that
//! references the `_SRC` constant this file's `harness!` call emits).
//!
//! Each half (`name()`, `FromStr`) gets its own independent `extern_spec!`
//! axiom (`logic.rs`), not the whole round trip stated as one fact --
//! the wrapper below is what actually checks those two axioms are
//! consistent with each other, the same shape `ext_chrono::utc`'s own
//! `extern_spec!` + wrapper uses.

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, requires};

amenable_derive::harness! {
    creusot, VERIFY_TZ_NAME_AND_FROM_STR_ROUND_TRIP_SRC, {
        /// `Tz::from_str(tz.name())` always recovers `tz` -- the same
        /// claim `amenable_kani::chrono::tz`'s ten binned harnesses check
        /// by symbolic execution, restated here resting on the two
        /// `extern_spec!` axioms above. Compares names, not `Tz` values
        /// directly: `chrono_tz::Tz` has no `DeepModel` impl (a foreign
        /// type, can't add one), but every `Tz` variant has a distinct
        /// name by construction, so name equality and value equality
        /// coincide -- the same projection-to-a-comparable-primitive
        /// trick `ext_chrono::fixed_offset`'s own wrapper uses (comparing
        /// `local_minus_utc()` results, never `FixedOffset` values
        /// directly).
        #[requires(true)]
        #[ensures(result)]
        fn verify_tz_name_and_from_str_round_trip(tz: chrono_tz::Tz) -> bool {
            use std::str::FromStr;
            match chrono_tz::Tz::from_str(tz.name()) {
                Ok(t) => t.name() == tz.name(),
                Err(_) => false,
            }
        }
    }
}
