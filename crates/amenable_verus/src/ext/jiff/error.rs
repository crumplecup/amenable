//! Verus accommodation model for `jiff::Error`'s classification
//! predicates (`is_range`/`is_invalid_parameter`/`is_crate_feature`).
//!
//! Same zero-`vstd`-coverage gap `offset.rs` documents. `Error`'s real
//! value isn't a simple bounded scalar `offset.rs`'s model can mirror
//! directly — its classification depends on a private `ErrorKind` enum
//! this crate has no access to. `ErrorClassModel` reproduces jiff's
//! real classification exhaustively (confirmed by reading jiff
//! 0.2.35's real match arms directly, not assumed: `is_range`/
//! `is_invalid_parameter`/`is_crate_feature` match on three disjoint
//! `ErrorKind` variant sets, plus everything else) and derives all
//! three predicates from one model value, making mutual exclusivity a
//! structural consequence of the match, not a separately-asserted
//! axiom — the same claim `amenable_kani::ext::jiff::error`'s Kani
//! finding named as the honest limit on that backend (every real
//! accessor times out) and `amenable_creusot::ext_jiff::error`'s real
//! `extern_spec!` independently confirms against jiff's actual API.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's real, exhaustive classification of `ErrorKind`, as far as
/// the three public `is_*` predicates can distinguish it.
pub enum ErrorClassModel {
    /// Models `is_range() == true` — a value out of jiff's supported range.
    Range,
    /// Models `is_invalid_parameter() == true` — an invalid parameter
    /// configuration.
    InvalidParameter,
    /// Models `is_crate_feature() == true` — a disabled crate feature.
    CrateFeature,
    /// Every other `ErrorKind`, for which all three predicates are `false`.
    Other,
}

/// The mutual-exclusivity law this model establishes — the same claim
/// `amenable_creusot::ext_jiff::error`'s real `extern_spec!` checks
/// against jiff's actual `is_range`/`is_invalid_parameter`/
/// `is_crate_feature`.
pub open spec fn error_classification_predicates_are_mutually_exclusive(
    flags: (bool, bool, bool),
) -> bool {
    !(flags.0 && flags.1) && !(flags.0 && flags.2) && !(flags.1 && flags.2)
}

/// A model of `(err.is_range(), err.is_invalid_parameter(),
/// err.is_crate_feature())`: whichever classification `class` models,
/// exactly one flag (or none, for `Other`) comes back `true`.
pub fn verify_error_classification_predicates_are_mutually_exclusive(
    class: ErrorClassModel,
) -> (result: (bool, bool, bool))
    ensures
        error_classification_predicates_are_mutually_exclusive(result),
{
    match class {
        ErrorClassModel::Range => (true, false, false),
        ErrorClassModel::InvalidParameter => (false, true, false),
        ErrorClassModel::CrateFeature => (false, false, true),
        ErrorClassModel::Other => (false, false, false),
    }
}

} // verus!
