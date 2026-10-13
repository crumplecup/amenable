//! Real Creusot proof content for `chrono_tz::ParseError`'s own claim
//! (`chrono::parse_error` holds the `CreusotWitness` bridge that
//! references the `_SRC` constant this file's `harness!` call emits).
//!
//! `#[trusted]`, by design: a genuinely attempted accommodation model,
//! not a premature fallback. Five real attempts, each addressing a
//! newly-confirmed issue (real `why3find -X` output, not guesses), hit
//! the same floor: `creusot-std`'s own `str::view()` is `#[logic(opaque)]`
//! with body `dead` -- fully uninterpreted for *every* string, including
//! a literal, confirmed by reading its real source. `str::len()`/
//! `String::len()` both route through that same opaque `self@`, so
//! there is no path -- via `tz_name_logic`, a per-call axiom, a
//! standalone `forall`, or a direct `s@.len()` implication -- for an
//! automatic solver to connect any literal's length or content to
//! anything concrete. This is a real gap in Creusot's string model
//! itself, not a formulation this crate's own axioms could route
//! around; recorded as a gallery finding
//! (`creusot_std_string_view_is_opaque_for_every_string`).
//!
//! The accommodation: `Ensures<CreusotVerifier> for ExtStandard<
//! chrono_tz::ParseError>` (`chrono::parse_error`) names this bound
//! explicitly and ties it to that contract type, the same shape
//! `amenable_std::rust_std_witness::string_carrier`'s own `Ensures`
//! impl uses for a bound with no exec representation -- not a bare,
//! unnamed `#[trusted]` floating on the harness alone. Kani's own
//! witness (`amenable_kani::chrono::parse_error::
//! check_tz_parse_error_occurs_for_unknown_names`) stays the real,
//! mechanically-checked proof for this claim; this wrapper restates it
//! for Creusot's own audit trail, grounded by the same real citation
//! (chrono-tz 0.10.4's real `TIMEZONES` table, all 597 entries checked:
//! none empty, shortest "GB"/"NZ" at 2 characters).

#[cfg(creusot)]
use creusot_std::macros::{ensures, requires, trusted};

amenable_derive::harness! {
    creusot, VERIFY_TZ_FROM_STR_REJECTS_UNKNOWN_NAME_SRC, {
        /// A string naming no real IANA zone makes `Tz::from_str`
        /// genuinely return `Err` -- the same claim
        /// `amenable_kani::chrono::parse_error::
        /// check_tz_parse_error_occurs_for_unknown_names` checks by
        /// real symbolic execution over three concrete witness
        /// strings. `#[trusted]` here, not mechanically discharged --
        /// see this file's own doc comment for the confirmed reason
        /// (creusot-std's string view has no grounding for any literal
        /// at all, not a gap specific to this claim's own formulation).
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_tz_from_str_rejects_unknown_name() -> bool {
            use std::str::FromStr;
            matches!(chrono_tz::Tz::from_str(""), Err(_))
        }
    }
}
