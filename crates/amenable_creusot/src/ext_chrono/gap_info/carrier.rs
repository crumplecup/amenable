#![cfg(creusot)]
//! A private-field newtype around `chrono_tz::GapInfo`, used only so
//! this type can appear in Creusot-translated code at all.
//!
//! `chrono_tz::GapInfo`'s own fields are public (`pub begin`, `pub
//! end`), and real `cargo creusot` output confirmed (twice, including
//! after removing all literal field projection -- see
//! `amenable_std::creusot_gallery`'s own
//! `public_field_foreign_struct_forces_full_record_translation` case)
//! that Creusot always models a type's full WHY3 record whenever one of
//! its public-field values appears *anywhere* Creusot must translate a
//! signature -- not only when a field is actually projected. Since one
//! of those two fields is named `begin`, a WHY3 reserved word Creusot's
//! own keyword-escaping table (`why3/src/name.rs`'s `RESERVED_STR`)
//! omits (it escapes `end` to `end'` but has no entry for `begin` at
//! all), that record always fails to translate. `TzOffset` never hits
//! this because its fields are private to `chrono_tz`, so Creusot can
//! never resolve its layout at all, regardless of where a `TzOffset`
//! value appears, and stays fully opaque unconditionally.
//!
//! This carrier recreates that same opacity for `GapInfo`: its own
//! field is private, and -- critically, per the gallery finding above --
//! `chrono_tz::GapInfo` must never appear in the *signature* of anything
//! Creusot translates, not just avoid being destructured. So:
//! - `from_local` is the only place that ever calls the real
//!   `chrono_tz::GapInfo::new`; its own signature returns
//!   `Option<GapInfoHandle>`, never mentioning `chrono_tz::GapInfo`.
//! - `begin_naive`/`end` are the only places that ever read `self.0`'s
//!   real fields; their own signatures take `&GapInfoHandle` (opaque)
//!   and return plain chrono types, never `&chrono_tz::GapInfo`.
//! - All three are `#[trusted]`, the same mechanism this gallery's own
//!   `NonZero::new` case already relies on to keep Creusot from fully
//!   translating a body it can't otherwise handle (there, an
//!   uncontracted call; here, a type whose record it can't emit).
//! - The opaque logic accessors these three ground themselves against
//!   (`gap_info_handle_begin_naive_value`/`gap_info_handle_end_value`)
//!   take `&GapInfoHandle`, never `&chrono_tz::GapInfo` -- the second
//!   gallery attempt's whole failure was an accessor taking the real
//!   type as its parameter, even with a `dead` body.
//!
//! Each `#[ensures]` below names a `#[logic(open)]` predicate defined in
//! the sibling `contracts` module (`from_local_gap_detection_agrees`,
//! `from_local_brackets_local`, `begin_naive_matches_handle`,
//! `end_matches_handle`), each with its own `ContractRecord`
//! registration -- never a raw inline expression, which cordial's own
//! `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001` check (correctly) flags.

use crate::ext_chrono::gap_info::contracts::{
    begin_naive_matches_handle, end_matches_handle, from_local_brackets_local,
    from_local_gap_detection_agrees,
};
use creusot_std::macros::{ensures, logic, trusted};

/// Opaque carrier for a real `chrono_tz::GapInfo` value. Its one field
/// is deliberately private -- see this module's own doc comment.
pub(crate) struct GapInfoHandle(chrono_tz::GapInfo);

#[trusted]
#[logic(opaque)]
pub fn gap_info_handle_begin_naive_value(_h: &GapInfoHandle) -> Option<chrono::NaiveDateTime> {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn gap_info_handle_end_value(_h: &GapInfoHandle) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    dead
}

impl GapInfoHandle {
    /// The real, uncontracted `chrono_tz::GapInfo::new` call lives here,
    /// and only here -- `#[trusted]` so Creusot never tries to translate
    /// this body (which would otherwise need `chrono_tz::GapInfo` as a
    /// local type) and never needs to represent it in this function's
    /// own signature either, since that signature never names it.
    #[trusted]
    #[ensures(from_local_gap_detection_agrees(result, tz, local))]
    #[ensures(from_local_brackets_local(result, local))]
    pub(crate) fn from_local(local: &chrono::NaiveDateTime, tz: &chrono_tz::Tz) -> Option<Self> {
        chrono_tz::GapInfo::new(local, tz).map(Self)
    }

    /// The real `begin` field's own `NaiveDateTime`, when present.
    #[trusted]
    #[ensures(begin_naive_matches_handle(result, self))]
    pub(crate) fn begin_naive(&self) -> Option<chrono::NaiveDateTime> {
        self.0.begin.map(|(dt, _)| dt)
    }

    /// The real `end` field's own `DateTime<Tz>`, when present.
    #[trusted]
    #[ensures(end_matches_handle(result, self))]
    pub(crate) fn end(&self) -> Option<chrono::DateTime<chrono_tz::Tz>> {
        self.0.end
    }
}
