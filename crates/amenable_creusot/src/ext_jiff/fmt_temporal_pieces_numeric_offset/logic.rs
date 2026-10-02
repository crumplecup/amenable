#![cfg(creusot)]
//! `jiff::fmt::temporal::PiecesNumericOffset`'s trusted logic axioms
//! and `extern_spec!` bridge, plus `jiff::tz::Offset::is_negative`'s
//! own first contract in this crate.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was four separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.
//!
//! `pno_offset_seconds_value` stays `pub(crate)`, declared directly
//! at this module's own top level: `fmt_temporal_pieces_offset.rs`
//! needs to reuse it (its own `PiecesOffset::to_numeric_offset`
//! extern_spec needs to relate to the real `PiecesNumericOffset` its
//! `Numeric` variant wraps).

use crate::ext_jiff::offset::logic::offset_seconds_value;
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn pno_offset_seconds_value(_p: &jiff::fmt::temporal::PiecesNumericOffset) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
fn pno_is_negative_value(_p: &jiff::fmt::temporal::PiecesNumericOffset) -> bool {
    dead
}

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
