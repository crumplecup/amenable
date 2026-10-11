//! Verus accommodation model for `chrono_tz::Tz`'s `name()`/`FromStr`
//! round trip.
//!
//! `chrono_tz` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all -- unlike `FixedOffset`/`NaiveDate`/`IsoWeek`
//! (where the real chrono algorithm could be read from source and
//! hand-ported to Pearlite/Verus spec code), Verus has no way to even
//! see what chrono-tz's real ~600 variants or their real names ARE, let
//! alone its real `siphasher`-based lookup. There is nothing to
//! reverse-engineer here, and no need to: the claim doesn't depend on
//! *how* the real lookup is implemented (hash table, linear scan,
//! whatever), only on the structural fact that a finite set of
//! distinctly-named variants has an exhaustive, correctly-inverting
//! name lookup. This models exactly that structure, minimally (four
//! variants standing in for chrono-tz's real ~600), with two
//! independently-written match statements -- one assigning names, one
//! looking them up -- so the proof is a real (if small) consistency
//! check between them, not a tautology by construction.
//!
//! This model's refinement premise -- that chrono_tz::Tz has this same
//! structure (distinct names, from_str as an exhaustive match against
//! exactly those names) -- rests on chrono-tz's own generation process
//! (the `Tz` enum and its `name()`/`FromStr` are generated together from
//! the same canonical IANA zone list, guaranteeing the correspondence by
//! construction) and is independently confirmed by
//! `amenable_kani::chrono::tz`'s real proof (all 597 real variants,
//! proven by direct execution) and
//! `amenable_creusot::ext_chrono::tz`'s real two-pillar `extern_spec!`
//! (proven against the real `FromStr`/`name()` trait impls).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Four abstract variants, standing in for chrono_tz::Tz's real ~600.
pub enum TzModelVariant {
    A,
    B,
    C,
    D,
}

/// Each variant's own name, as a distinct integer identifier -- the
/// claim's structure doesn't depend on names being strings
/// specifically, only on them being distinct per variant.
pub open spec fn tz_model_name(v: TzModelVariant) -> int {
    match v {
        TzModelVariant::A => 17,
        TzModelVariant::B => 42,
        TzModelVariant::C => 99,
        TzModelVariant::D => 3,
    }
}

/// Look up a variant by name: the exhaustive match any real `from_str`
/// performs, written independently of `tz_model_name` above (not its
/// mechanical inverse), so the round-trip proof below is a real check
/// that the two agree.
pub open spec fn tz_model_find_by_name(name: int) -> Option<TzModelVariant> {
    if name == 17 {
        Some(TzModelVariant::A)
    } else if name == 42 {
        Some(TzModelVariant::B)
    } else if name == 99 {
        Some(TzModelVariant::C)
    } else if name == 3 {
        Some(TzModelVariant::D)
    } else {
        None
    }
}

/// The round-trip law this model establishes: every variant's own name
/// looks back up to that same variant.
pub open spec fn tz_name_round_trip_holds(v: TzModelVariant) -> bool {
    tz_model_find_by_name(tz_model_name(v)) == Some(v)
}

/// Proves `tz_name_round_trip_holds` for every variant. A plain `fn`,
/// not `proof fn`: under plain rustc (`verus! {}`'s own erasure, no
/// real verus toolchain), `proof fn` items are stripped entirely, while
/// an ordinary exec `fn` with contracts survives -- the same shape
/// every sibling model file in this directory already uses.
pub fn verify_tz_name_round_trips_model(v: TzModelVariant)
    ensures
        tz_name_round_trip_holds(v),
{
    let _name = match v {
        TzModelVariant::A => 17,
        TzModelVariant::B => 42,
        TzModelVariant::C => 99,
        TzModelVariant::D => 3,
    };
}

} // verus!
