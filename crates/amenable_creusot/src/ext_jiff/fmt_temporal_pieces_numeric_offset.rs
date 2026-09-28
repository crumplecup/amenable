//! Real Creusot proof content for `jiff::fmt::temporal::
//! PiecesNumericOffset`'s `From<Offset>`/`with_negative_zero` round
//! trips — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_numeric_offset`'s own doc comment checks by
//! symbolic execution.
//!
//! `PiecesNumericOffset` wraps a real `jiff::tz::Offset` plus one
//! `is_negative` bit. Reuses `offset.rs`'s own `offset_seconds_value`
//! opaque accessor (now `pub(crate)` at that module's top level, per
//! its own doc comment) rather than redeclaring a disconnected copy —
//! that accessor is the one already tied, via `offset.rs`'s own
//! `extern_spec!`, to `Offset::from_seconds`/`Offset::seconds()`'s real
//! behavior, so reusing it keeps this proof's claims meaningfully
//! connected to jiff's actual `Offset` semantics rather than resting on
//! a fresh, unconnected axiom.
//!
//! Two new opaque accessors decompose `PiecesNumericOffset` itself
//! (the same `DeepModel`-avoidance shape `fmt_temporal_pieces.rs` uses
//! for `Pieces`): `pno_offset_seconds_value`/`pno_is_negative_value`.
//! `Offset::is_negative()` is extern-spec'd here for the first time in
//! this crate (not previously contracted by any other file) as
//! `result == (offset_seconds_value(&self) < 0i32)` — jiff's own real
//! body (`self.inner.is_negative()`) and doc examples (`offset(5)` →
//! `false`, `offset(0)` → `false`, `offset(-5)` → `true`) confirm this
//! is exactly what it means for a seconds-backed offset.
//!
//! `pno_offset_seconds_value` is `pub(crate)` (not module-private):
//! `fmt_temporal_pieces_offset.rs` needs to reuse it (its own
//! `PiecesOffset::to_numeric_offset` extern_spec needs to relate to
//! the real `PiecesNumericOffset` its `Numeric` variant wraps), the
//! same cross-file-reuse shape `offset.rs`'s own `offset_seconds_value`
//! now has.

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
pub(crate) fn pno_offset_seconds_value(_p: &jiff::fmt::temporal::PiecesNumericOffset) -> i32 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn pno_is_negative_value(_p: &jiff::fmt::temporal::PiecesNumericOffset) -> bool {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::tz::Offset {
        #[check(ghost)]
        #[ensures(result == (offset_seconds_value(&self) < 0i32))]
        fn is_negative(self) -> bool;
    }

    impl jiff::fmt::temporal::PiecesNumericOffset {
        #[check(ghost)]
        #[ensures(offset_seconds_value(&result) == pno_offset_seconds_value(&self))]
        fn offset(&self) -> jiff::tz::Offset;

        #[check(ghost)]
        #[ensures(result == pno_is_negative_value(&self))]
        fn is_negative(&self) -> bool;

        #[check(ghost)]
        #[ensures(
            pno_offset_seconds_value(&result) == pno_offset_seconds_value(&self)
            && pno_is_negative_value(&result) == true
        )]
        fn with_negative_zero(self) -> jiff::fmt::temporal::PiecesNumericOffset;
    }

    impl core::convert::From<jiff::tz::Offset> for jiff::fmt::temporal::PiecesNumericOffset {
        #[check(ghost)]
        #[ensures(
            pno_offset_seconds_value(&result) == offset_seconds_value(&offset)
            && pno_is_negative_value(&result) == (offset_seconds_value(&offset) < 0i32)
        )]
        fn from(offset: jiff::tz::Offset) -> jiff::fmt::temporal::PiecesNumericOffset;
    }
}

amenable_derive::harness! { creusot, FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::fmt::temporal::
    /// PiecesNumericOffset>` postcondition — real, callable Pearlite
    /// content, not just descriptive text alongside it.
    #[logic(open)]
    fn fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_temporal_pieces_numeric_offset::fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_holds",
        "creusot",
        "ensures",
        || FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC, {
    /// `PiecesNumericOffset::from(offset)` always round-trips
    /// `offset`'s own seconds through `.offset().seconds()`, and sets
    /// `.is_negative()` to exactly `offset.seconds() < 0`;
    /// `.with_negative_zero()` always preserves the wrapped `offset`
    /// while forcing `.is_negative()` to `true` — the same claim
    /// `amenable_kani::ext::jiff::
    /// fmt_temporal_pieces_numeric_offset::
    /// verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero`
    /// checks by symbolic execution, restated as a real Creusot
    /// postcondition resting on the `extern_spec!` above.
    #[requires(true)]
    #[ensures(fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_holds(result))]
    fn verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero(seconds: i32) -> bool {
        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => true,
            Ok(offset) => {
                let pno = jiff::fmt::temporal::PiecesNumericOffset::from(offset);
                let from_ok =
                    pno.offset().seconds() == seconds && pno.is_negative() == (seconds < 0i32);

                let pno_zeroed =
                    jiff::fmt::temporal::PiecesNumericOffset::from(offset).with_negative_zero();
                let with_negative_zero_ok =
                    pno_zeroed.offset().seconds() == seconds && pno_zeroed.is_negative();

                from_ok && with_negative_zero_ok
            }
        }
    }
}}
