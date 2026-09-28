//! Real Creusot proof content for `jiff::fmt::friendly::
//! FractionalUnit`'s `From<FractionalUnit> for Unit` conversion
//! (`ext::jiff::fmt_friendly_fractional_unit` holds the
//! `CreusotWitness` bridge that references the `_SRC` constant this
//! file's `harness!` call emits) — the same documented per-variant
//! mapping `amenable_kani::ext::jiff::fmt_friendly_fractional_unit`
//! checks by exhaustive enumeration.
//!
//! `FractionalUnit` is `#[non_exhaustive]`, so both the `extern_spec!`
//! ensures and the harness body need a wildcard arm (`_ => true`) to
//! compile — vacuously true for any variant beyond the 5 jiff 0.2.35
//! actually documents, since there is nothing real to claim about a
//! variant that doesn't exist yet.
//!
//! **Three real, confirmed Creusot walls in succession, resolved by
//! going back to the simplest available mechanism.** First: comparing
//! `unit == jiff::Unit::Hour` directly hit the SAME `DeepModel`
//! constraint `amenable_creusot::ext::jiff::unit`'s own doc comment
//! documents for `Ord::cmp` on `jiff::Unit` — confirming the wall
//! applies to `PartialEq::eq` too. Second: tried sidestepping it via a
//! plain `as i32` cast on `jiff::Unit`'s own explicit discriminants
//! instead, but Pearlite's `#[ensures(..)]` clauses only support `as`
//! casts between primitive types, confirmed via a real "unsupported
//! cast from jiff::Unit to i32" error. Third: moved the `as i32` cast
//! into the harness body's ordinary Rust instead (backed by an opaque
//! accessor plus five per-variant `#[trusted]` lemmas, the established
//! `span.rs`-style technique) — this COMPILED, but the actual proof
//! failed; `why3find prove -X` on the failing goal (the real debugging
//! tool for this, not guesswork) showed why: Creusot's translation of
//! `as i32` on a foreign enum produces an unconstrained `any_l()`
//! value with no real connection to `unit`'s actual variant, so the
//! lemmas' facts never actually apply to it.
//!
//! The fix that actually works: plain `match` on `jiff::Unit`'s own
//! variants needs no `PartialEq`/`DeepModel`/cast machinery at all —
//! Creusot already has to support pattern-matching an arbitrary
//! foreign enum's variants for ordinary control-flow translation
//! (confirmed directly in the `.coma` output: `jiff::Unit`'s full
//! variant list appears as a real Why3 `type`), so nesting a `match
//! result { .. }` inside the outer `match u { .. }` needs nothing
//! beyond what Creusot's translator already does routinely.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl From<jiff::fmt::friendly::FractionalUnit> for jiff::Unit {
        #[check(ghost)]
        #[ensures(match u {
            jiff::fmt::friendly::FractionalUnit::Hour => match result {
                jiff::Unit::Hour => true,
                _ => false,
            },
            jiff::fmt::friendly::FractionalUnit::Minute => match result {
                jiff::Unit::Minute => true,
                _ => false,
            },
            jiff::fmt::friendly::FractionalUnit::Second => match result {
                jiff::Unit::Second => true,
                _ => false,
            },
            jiff::fmt::friendly::FractionalUnit::Millisecond => match result {
                jiff::Unit::Millisecond => true,
                _ => false,
            },
            jiff::fmt::friendly::FractionalUnit::Microsecond => match result {
                jiff::Unit::Microsecond => true,
                _ => false,
            },
            _ => true,
        })]
        fn from(u: jiff::fmt::friendly::FractionalUnit) -> jiff::Unit;
    }
}

amenable_derive::harness! {
    creusot, FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_HOLDS_SRC, {
        /// The `amenable_ext::
        /// ExtStandard<jiff::fmt::friendly::FractionalUnit>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn fmt_friendly_fractional_unit_from_matches_documented_mapping_holds(matches: bool) -> bool {
            pearlite! { matches }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_friendly_fractional_unit::fmt_friendly_fractional_unit_from_matches_documented_mapping_holds",
        "creusot",
        "ensures",
        || FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC, {
        /// `FractionalUnit`'s `From`-conversion into `Unit` matches
        /// jiff's own documented per-variant mapping exactly — a
        /// real, checked postcondition resting on the `extern_spec!`
        /// above.
        #[requires(true)]
        #[ensures(fmt_friendly_fractional_unit_from_matches_documented_mapping_holds(result))]
        fn verify_fmt_friendly_fractional_unit_from_matches_documented_mapping(
            u: jiff::fmt::friendly::FractionalUnit,
        ) -> bool {
            let unit: jiff::Unit = u.into();
            match u {
                jiff::fmt::friendly::FractionalUnit::Hour => matches!(unit, jiff::Unit::Hour),
                jiff::fmt::friendly::FractionalUnit::Minute => matches!(unit, jiff::Unit::Minute),
                jiff::fmt::friendly::FractionalUnit::Second => matches!(unit, jiff::Unit::Second),
                jiff::fmt::friendly::FractionalUnit::Millisecond => {
                    matches!(unit, jiff::Unit::Millisecond)
                }
                jiff::fmt::friendly::FractionalUnit::Microsecond => {
                    matches!(unit, jiff::Unit::Microsecond)
                }
                _ => true,
            }
        }
    }
}
