//! Verus accommodation model for `jiff::civil::Era`'s classification
//! property (`jiff::civil::Date::era_year`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_era`'s and `amenable_creusot::ext_jiff::civil_era`'s own
//! harnesses for the identical claim, checked directly against
//! jiff's real `Date::era_year`, independently confirm.
//!
//! `Era` itself has no fields and no `Ord` (unlike `jiff::Unit`, see
//! `unit.rs`'s own local model enum) — this model represents it as a
//! plain two-variant `EraModel` enum, matching jiff's real shape
//! exactly rather than reducing it to a bare integer, since Verus (as
//! a self-contained model, not an extern_spec against real jiff) can
//! freely define its own enum with real variant-matching.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::civil::Era`'s two real variants.
#[derive(Clone, Copy)]
pub enum EraModel {
    /// Models `jiff::civil::Era::BCE`.
    Bce,
    /// Models `jiff::civil::Era::CE`.
    Ce,
}

/// A comfortably safe sub-range of `Date`'s real year range
/// (`-9999..=9999`), the same constant the Kani and Creusot proofs
/// for this type independently confirm.
pub open spec fn civil_era_year_in_range(year: int) -> bool {
    year >= -9999 && year <= 9999
}

/// The classification law this model establishes: `year >= 1` maps
/// to `(year, Ce)`; `year <= 0` maps to `(-year + 1, Bce)` — jiff's
/// own documented law, the same claim `amenable_kani::ext::jiff::
/// civil_era::verify_civil_era_year_classifies_bce_and_ce_correctly`
/// and `amenable_creusot::ext_jiff::civil_era::
/// verify_civil_era_year_classifies_bce_and_ce_correctly` check
/// against jiff's real API.
pub open spec fn civil_era_year_classifies_bce_and_ce_correctly(
    year: i16,
    result: (i16, EraModel),
) -> bool {
    if year >= 1 {
        result.0 == year
            && (match result.1 {
                EraModel::Ce => true,
                EraModel::Bce => false,
            })
    } else {
        result.0 == -year + 1
            && (match result.1 {
                EraModel::Ce => false,
                EraModel::Bce => true,
            })
    }
}

/// A model of `Date::new(year, 1, 1).era_year()`: classifies `year`
/// into its era-relative year and `Era` variant exactly as jiff
/// documents.
pub fn verify_civil_era_year_classifies_bce_and_ce_correctly_model(year: i16) -> (result: (
    i16,
    EraModel,
))
    requires
        civil_era_year_in_range(year as int),
    ensures
        civil_era_year_classifies_bce_and_ce_correctly(year, result),
{
    if year >= 1 {
        (year, EraModel::Ce)
    } else {
        (-year + 1, EraModel::Bce)
    }
}

} // verus!
