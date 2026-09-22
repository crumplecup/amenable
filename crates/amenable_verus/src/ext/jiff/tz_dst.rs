//! Verus accommodation model for `jiff::tz::Dst`'s `From<bool>`/
//! `is_dst`/`is_std` round trip.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! tz_dst`'s and `amenable_creusot::ext_jiff::tz_dst`'s own harnesses
//! for the identical claim, checked directly against jiff's real
//! API, independently confirm.
//!
//! Models `Dst` as a plain, field-less two-variant enum (matches
//! `fmt_strtime_meridiem.rs`'s own `MeridiemModel` shape — every
//! field-less enum model in this directory verifies clean, unlike a
//! data-carrying variant, see `fmt_temporal_pieces_offset.rs`'s own
//! doc comment for that real, confirmed wall).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::tz::Dst`'s two real variants.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DstModel {
    /// Models `jiff::tz::Dst::No`.
    No,
    /// Models `jiff::tz::Dst::Yes`.
    Yes,
}

impl DstModel {
    /// Models `Dst::from(is_dst)`.
    pub fn from_bool(is_dst: bool) -> (result: DstModel)
        ensures
            is_dst ==> result == DstModel::Yes,
            !is_dst ==> result == DstModel::No,
    {
        if is_dst { DstModel::Yes } else { DstModel::No }
    }

    /// Models `Dst::is_dst`.
    pub fn is_dst(&self) -> (result: bool)
        ensures
            result == (*self == DstModel::Yes),
    {
        matches!(self, DstModel::Yes)
    }

    /// Models `Dst::is_std`.
    pub fn is_std(&self) -> (result: bool)
        ensures
            result == (*self == DstModel::No),
    {
        matches!(self, DstModel::No)
    }
}

/// Exercises `from_bool`/`is_dst`/`is_std` — the same claim
/// `amenable_kani::ext::jiff::tz_dst` and `amenable_creusot::
/// ext_jiff::tz_dst` both check against jiff's real API.
pub fn verify_tz_dst_from_bool_round_trips_model(is_dst: bool) -> (result: bool)
    ensures
        result,
{
    let dst = DstModel::from_bool(is_dst);
    dst.is_dst() == is_dst && dst.is_std() == !is_dst
}

} // verus!
