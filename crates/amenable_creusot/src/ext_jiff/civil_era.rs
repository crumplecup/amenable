//! Real Creusot proof content for `jiff::civil::Era`'s classification
//! property (`ext::jiff::civil_era` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim
//! `amenable_kani::ext::jiff::civil_era`'s own doc comment checks by
//! symbolic execution.
//!
//! `jiff::civil::Era` is uncontracted everywhere — not `creusot-std`,
//! not `elicitation`. `Era` has no `Ord` (unlike `jiff::Unit`, which
//! hit a real `DeepModel` wall extern-speccing `Ord::cmp` — see
//! `unit.rs`'s own doc comment), but a bare equality comparison
//! (`==`, i.e. `PartialEq::eq`) is still an ordinary method call that
//! can't appear inside an `#[ensures(..)]`/`#[logic]` clause either —
//! so this file avoids calling it at all, using the same opaque-
//! accessor pattern as `offset.rs`/`civil_date.rs`: an
//! `era_discriminant(&Era) -> i8` axiom stands in for "which variant
//! this is," and the postcondition is stated purely in terms of that
//! discriminant, never `Era::eq` itself.
//!
//! `Date::era_year`'s own postcondition reuses `civil_date.rs`'s
//! `date_year_value` accessor (imported directly, not redeclared) —
//! `Date::new`'s `extern_spec!` already lives there, and Creusot
//! allows only one `extern_spec!` per real function crate-wide (see
//! `civil_date.rs`'s own doc comment for the confirmed compiler
//! error this file's first draft hit).
//!
//! `era_discriminant` is a plain `pub fn` at this module's own top
//! level, not `pub(crate)` and not nested in a private `mirror`
//! module — a real, MORE STRICT visibility requirement than the
//! cross-file-reuse fix above (`date_year_value` only needed
//! `pub(crate)`, confirmed working in the `extern_spec!`'s own
//! `#[ensures(..)]` below). The stricter requirement is specific to
//! `#[logic(open)]` helper functions (the ones `harness!` generates
//! from a `#[logic(open)] fn ... { pearlite! { .. } }` block):
//! confirmed via two real, DISTINCT compiler errors from two
//! successive attempts — first "Cannot make ... transparent ... as
//! it would call a less-visible item" with `era_discriminant` as
//! `pub(super)` inside `mirror`, THEN THE SAME ERROR AGAIN with it
//! promoted to `pub(crate)` (not `mirror`-nested) — only a full `pub`
//! resolved it. `harness!`'s generated `#[logic(open)]` function is
//! evidently itself fully `pub`, and Creusot's proof-transparency
//! check requires anything an open/transparent function calls to be
//! at least as visible as the function itself — `pub(crate)` isn't
//! enough once the caller is `pub`, even though the SAME `pub(crate)`
//! accessor is fine when called from an ordinary `extern_spec!`
//! `#[ensures(..)]` clause instead (as `date_year_value` and
//! `span_get_years_value`/etc. already are, confirmed by both working
//! in this exact file and in `span_fieldwise.rs`). This refines this
//! crate's own `reference_creusot_toolchain_findings` memory's
//! existing proof-transparency finding, which only documents the
//! `extern_spec!`-on-public-trait-method case.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, requires};
}
#[cfg(creusot)]
use super::civil_date::date_year_value;
#[cfg(creusot)]
use creusot_std::macros::{logic, trusted};
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, requires};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn era_discriminant(_e: &jiff::civil::Era) -> i8 {
    dead
}

// This crate's own axiom for "which Era variant this is" — 0 for
// BCE, 1 for CE. Doesn't need to match jiff's real discriminant
// values (private either way); only needs to be internally
// consistent within this file's own extern_spec.
#[cfg(creusot)]
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

amenable_derive::harness! {
    creusot, CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::Era>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn civil_era_year_classifies_bce_and_ce_correctly(
            year: i16,
            observed: (i16, jiff::civil::Era),
        ) -> bool {
            pearlite! {
                if year >= 1i16 {
                    observed.0 == year && era_discriminant(&observed.1) == 1i8
                } else {
                    observed.0 == -year + 1i16 && era_discriminant(&observed.1) == 0i8
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_era::civil_era_year_classifies_bce_and_ce_correctly",
        "creusot",
        "ensures",
        || CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, CIVIL_ERA_YEAR_IN_DATE_NEW_RANGE_HOLDS_SRC, {
        /// `jiff::civil::Date::new`'s own documented valid year range
        /// (`-9999..=9999`) — named so this file's `#[requires(..)]`
        /// points at a real predicate instead of restating the bound
        /// inline.
        #[logic(open)]
        fn civil_era_year_in_date_new_range(year: i16) -> bool {
            pearlite! { year > -9999i16 - 1i16 && year < 9999i16 + 1i16 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_era::civil_era_year_in_date_new_range",
        "creusot",
        "requires",
        || CIVIL_ERA_YEAR_IN_DATE_NEW_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC, {
        /// `Date::new(year, 1, 1).era_year()` classifies `year >= 1`
        /// as CE and `year <= 0` as BCE — the same claim
        /// `amenable_kani::ext::jiff::civil_era::
        /// verify_civil_era_year_classifies_bce_and_ce_correctly`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition resting on the `extern_spec!` above.
        #[requires(civil_era_year_in_date_new_range(year))]
        #[ensures(civil_era_year_classifies_bce_and_ce_correctly(year, result))]
        fn verify_civil_era_year_classifies_bce_and_ce_correctly(
            year: i16,
        ) -> (i16, jiff::civil::Era) {
            jiff::civil::Date::new(year, 1, 1)
                .expect("year is already checked to be in Date::new's valid range, month/day fixed at 1")
                .era_year()
        }
    }
}
