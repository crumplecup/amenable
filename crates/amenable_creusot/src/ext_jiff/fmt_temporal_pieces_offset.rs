//! Real Creusot proof content for `jiff::fmt::temporal::
//! PiecesOffset`'s `Zulu`/`From<Offset>` mapping into
//! `to_numeric_offset` — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_offset`'s own doc comment checks by symbolic
//! execution.
//!
//! `PiecesOffset` is a real, `#[non_exhaustive]` two-variant enum
//! (`Zulu`, `Numeric(PiecesNumericOffset)`). Matches directly on its
//! own variants inside the `extern_spec!`'s `#[ensures(..)]` clause —
//! the same `fmt_friendly_fractional_unit.rs`-established technique
//! for a foreign enum: plain structural pattern matching needs no
//! `PartialEq`/`DeepModel`/cast machinery at all, unlike comparing two
//! variants via `==`. Reuses `fmt_temporal_pieces_numeric_offset.rs`'s
//! own `pno_offset_seconds_value` opaque accessor (hoisted to
//! `pub(crate)` for this purpose, per that module's own doc comment)
//! and `offset.rs`'s `offset_seconds_value`, rather than redeclaring
//! disconnected copies.
//!
//! A wildcard arm (`_ => true`) is needed in the ensures match for
//! the same `#[non_exhaustive]` reason `fmt_friendly_fractional_unit.
//! rs` documents — vacuously true for any variant beyond the two jiff
//! 0.2.35 actually documents.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use crate::ext_jiff::shared_trusted_accessors::{offset_seconds_value, pno_offset_seconds_value};
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl jiff::fmt::temporal::PiecesOffset {
        #[check(ghost)]
        #[ensures(match self {
            jiff::fmt::temporal::PiecesOffset::Zulu => offset_seconds_value(&result) == 0i32,
            jiff::fmt::temporal::PiecesOffset::Numeric(noffset) => {
                offset_seconds_value(&result) == pno_offset_seconds_value(noffset)
            }
            _ => true,
        })]
        fn to_numeric_offset(&self) -> jiff::tz::Offset;
    }

    impl core::convert::From<jiff::tz::Offset> for jiff::fmt::temporal::PiecesOffset {
        #[check(ghost)]
        #[ensures(match result {
            jiff::fmt::temporal::PiecesOffset::Numeric(noffset) => {
                pno_offset_seconds_value(&noffset) == offset_seconds_value(&offset)
            }
            _ => false,
        })]
        fn from(offset: jiff::tz::Offset) -> jiff::fmt::temporal::PiecesOffset;
    }
}

amenable_derive::harness! { creusot, FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_HOLDS_SRC, {
    /// The `amenable_ext::ExtStandard<jiff::fmt::temporal::
    /// PiecesOffset>` postcondition — real, callable Pearlite content,
    /// not just descriptive text alongside it.
    #[logic(open)]
    fn fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_holds(matches: bool) -> bool {
        pearlite! { matches }
    }
}}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_temporal_pieces_offset::fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_holds",
        "creusot",
        "ensures",
        || FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! { creusot, VERIFY_FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_SRC, {
    /// `PiecesOffset::Zulu.to_numeric_offset()` is always
    /// `Offset::UTC`, and `PiecesOffset::from(offset)
    /// .to_numeric_offset()` always round-trips `offset`'s own
    /// seconds back — the same claim `amenable_kani::ext::jiff::
    /// fmt_temporal_pieces_offset::
    /// verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip`
    /// checks by symbolic execution, restated as a real Creusot
    /// postcondition resting on the `extern_spec!` above.
    #[requires(true)]
    #[ensures(fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_holds(result))]
    fn verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip(seconds: i32) -> bool {
        let zulu_ok = jiff::fmt::temporal::PiecesOffset::Zulu.to_numeric_offset().seconds() == 0i32;

        match jiff::tz::Offset::from_seconds(seconds) {
            Err(_) => zulu_ok,
            Ok(offset) => {
                let from_ok = jiff::fmt::temporal::PiecesOffset::from(offset)
                    .to_numeric_offset()
                    .seconds()
                    == seconds;
                zulu_ok && from_ok
            }
        }
    }
}}
