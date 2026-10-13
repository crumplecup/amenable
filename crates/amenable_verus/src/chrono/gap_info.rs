//! Verus accommodation model for `chrono_tz::GapInfo`'s consistency
//! claim: the gap's own width equals its post-transition total UTC
//! offset minus its pre-transition total UTC offset.
//!
//! `chrono_tz` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all -- confirmed directly for `TzOffset` (see
//! `tz_offset.rs`'s own doc comment) and unchanged for `GapInfo`: there
//! is no real path to a mechanical proof over the real type at all.
//! The claim doesn't depend on *how* a real gap is discovered (binary
//! search over a timespan table, whatever), only on the structural
//! fact that every real gap has a pre-transition total offset and a
//! post-transition total offset whose difference is the gap's own
//! width. This models exactly that structure, minimally (three
//! variants standing in for chrono-tz's real 94 distinct discovered
//! gap transitions -- `amenable_ext::DISCOVERED_GAP_TABLE` -- with
//! three genuine, real transitions drawn from it, not invented, chosen
//! to span the real range: an ordinary one-hour DST spring-forward, a
//! rarer two-hour double-DST jump, and a real International Date Line
//! transition -- one of exactly four real rows tied for `86400`
//! seconds, confirmed by computing every real row's width, the largest
//! any real zone produces),
//! with two independently-written computations -- one assigning each
//! variant's pre-/post-transition total offsets, one assigning its gap
//! width directly, as its own literal, not derived mechanically as
//! their difference -- so the proof is a real (if small) consistency
//! check between them, not a tautology by construction.
//!
//! This model's refinement premise -- that chrono_tz::GapInfo has this
//! same subtractive structure for every real gap it produces -- rests
//! on chrono-tz's own real source (a gap's `begin`/`end` bracket
//! exactly the pre-/post-transition offsets the real binary search
//! discovers, and the gap's width is exactly their difference by
//! construction) and is independently confirmed by
//! `amenable_kani::chrono::gap_info`'s real proof (94 real discovered
//! witnesses, proven by direct execution, binned across 47 harnesses)
//! and `amenable_creusot::ext_chrono::gap_info`'s real `GapInfoHandle`
//! accommodation model (proven against the real `GapInfo::new`,
//! confirmed non-vacuous).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Three abstract variants, standing in for chrono-tz's real 94
/// distinct discovered gap transitions
/// (`amenable_ext::DISCOVERED_GAP_TABLE`). Each one below is a real
/// row from that table, not invented (zone names resolved directly
/// against the real `chrono_tz::TZ_VARIANTS[index]`, not guessed):
/// `StandardDstSpringForward` (`Pacific/Enderbury`, row `(536, 1979,
/// 10, 1, ...)`, `-43200` -> `-39600`), `DoubleDstJump`
/// (`America/Goose_Bay`, row `(118, 1988, 4, 3, ...)`, `-14400` ->
/// `-7200`), and `DateLineJump` (`Kwajalein`, row `(516, 1993, 8, 21,
/// ...)`, `-43200` -> `43200` -- the real date the Marshall Islands
/// skipped a calendar day crossing the International Date Line).
pub enum GapInfoModelVariant {
    StandardDstSpringForward,
    DoubleDstJump,
    DateLineJump,
}

/// Each variant's own pre-transition total UTC offset (`base_utc_offset`
/// plus `dst_offset`), in seconds.
pub open spec fn gap_info_model_prev_offset(v: GapInfoModelVariant) -> int {
    match v {
        GapInfoModelVariant::StandardDstSpringForward => -43200,
        GapInfoModelVariant::DoubleDstJump => -14400,
        GapInfoModelVariant::DateLineJump => -43200,
    }
}

/// Each variant's own post-transition total UTC offset, in seconds.
pub open spec fn gap_info_model_next_offset(v: GapInfoModelVariant) -> int {
    match v {
        GapInfoModelVariant::StandardDstSpringForward => -39600,
        GapInfoModelVariant::DoubleDstJump => -7200,
        GapInfoModelVariant::DateLineJump => 43200,
    }
}

/// Each variant's own gap width, in seconds -- written as its own
/// literal, independently of `gap_info_model_prev_offset`/
/// `gap_info_model_next_offset` above (not their mechanical
/// difference), so the consistency proof below is a real check that
/// the two agree.
pub open spec fn gap_info_model_width(v: GapInfoModelVariant) -> int {
    match v {
        GapInfoModelVariant::StandardDstSpringForward => 3600,
        GapInfoModelVariant::DoubleDstJump => 7200,
        GapInfoModelVariant::DateLineJump => 86400,
    }
}

/// The consistency law this model establishes: every variant's own
/// width equals its post-transition offset minus its pre-transition
/// offset.
pub open spec fn gap_info_consistency_holds(v: GapInfoModelVariant) -> bool {
    gap_info_model_width(v) == gap_info_model_next_offset(v) - gap_info_model_prev_offset(v)
}

/// Proves `gap_info_consistency_holds` for every variant. A plain
/// `fn`, not `proof fn`: under plain rustc (`verus! {}`'s own erasure,
/// no real verus toolchain), `proof fn` items are stripped entirely,
/// while an ordinary exec `fn` with contracts survives -- the same
/// shape every sibling model file in this directory already uses.
pub fn verify_gap_info_consistency_model(v: GapInfoModelVariant)
    ensures
        gap_info_consistency_holds(v),
{
    let _prev = match v {
        GapInfoModelVariant::StandardDstSpringForward => -43200,
        GapInfoModelVariant::DoubleDstJump => -14400,
        GapInfoModelVariant::DateLineJump => -43200,
    };
}

} // verus!
