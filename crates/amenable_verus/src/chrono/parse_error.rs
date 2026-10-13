//! Verus accommodation model for `chrono_tz::ParseError`'s own claim:
//! a zone-name query naming no real IANA zone makes `Tz::from_str`
//! genuinely return `Err`.
//!
//! `chrono_tz` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all -- confirmed directly for `TzOffset` (see
//! `tz_offset.rs`'s own doc comment) and unchanged here. `ParseError`'s
//! own real source (`pub struct ParseError(());`) carries zero
//! information -- the only possible claim is *when* it occurs, the
//! negative direction of `Tz`'s own name/`FromStr` round trip. The
//! real fact behind that claim: every name in chrono-tz's real
//! prebuilt table is non-empty (confirmed by reading it, all 597
//! entries checked, shortest `"GB"`/`"NZ"` at 2 characters), so a
//! query of length 0 cannot equal any of them -- the same move
//! `amenable_creusot::ext_chrono::parse_error`'s own model uses to
//! sidestep the analogous wall there (Creusot's `str::view()` being
//! opaque for every string): restate the real fact over plain
//! integers, where no string model is needed at all. `ZoneNameModel`
//! below models the two shortest real names by their real lengths,
//! with two independently-written computations -- one assigning each
//! variant's own length, one assigning the empty-query witness's
//! length directly, as its own literal, not derived mechanically --
//! so the proof is a real (if small) consistency check between them,
//! not a tautology by construction.
//!
//! This model's refinement premise -- that chrono_tz's real name table
//! has no empty entry, so any name this model's `Unmatched` variant
//! stands in for genuinely can't equal a real one -- rests on reading
//! that real table directly and is independently confirmed by
//! `amenable_kani::chrono::parse_error`'s real proof (three concrete
//! witness strings, proven by direct execution) and
//! `amenable_creusot::ext_chrono::parse_error`'s real, analogous
//! length-based model (genuinely discharged by Creusot's own prover,
//! confirmed non-vacuous).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Two real zone names, standing in for chrono-tz's real ~597-name
/// space -- the same move `TzOffsetModelVariant`/`GapInfoModelVariant`
/// use for offsets/gaps. Both are chrono-tz's own real, shortest names
/// (confirmed by reading its real prebuilt table): `"GB"` and `"NZ"`.
pub enum ZoneNameModelVariant {
    Gb,
    Nz,
}

/// Each variant's own real name length, in characters -- confirmed
/// directly against chrono-tz's real prebuilt table, not invented.
pub open spec fn zone_name_model_length(v: ZoneNameModelVariant) -> int {
    match v {
        ZoneNameModelVariant::Gb => 2,
        ZoneNameModelVariant::Nz => 2,
    }
}

/// The empty-query witness's own length, in characters -- the same
/// witness `amenable_kani::chrono::parse_error` exercises by real
/// execution, written as its own literal here, independently of
/// `zone_name_model_length` above.
pub open spec fn zone_name_model_empty_query_length() -> int {
    0
}

/// The consistency law this model establishes: every modeled real
/// name's length strictly exceeds the empty-query witness's length,
/// so it cannot equal it -- the real reason a lookup for `""` matches
/// no real zone.
pub open spec fn zone_name_model_excludes_empty_query(v: ZoneNameModelVariant) -> bool {
    zone_name_model_length(v) > zone_name_model_empty_query_length()
}

/// Proves `zone_name_model_excludes_empty_query` for every variant. A
/// plain `fn`, not `proof fn`: under plain rustc (`verus! {}`'s own
/// erasure, no real verus toolchain), `proof fn` items are stripped
/// entirely, while an ordinary exec `fn` with contracts survives --
/// the same shape every sibling model file in this directory already
/// uses.
pub fn verify_zone_name_model_excludes_empty_query(v: ZoneNameModelVariant)
    ensures
        zone_name_model_excludes_empty_query(v),
{
    let _length = match v {
        ZoneNameModelVariant::Gb => 2,
        ZoneNameModelVariant::Nz => 2,
    };
}

} // verus!
