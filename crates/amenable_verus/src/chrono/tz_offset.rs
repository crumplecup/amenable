//! Verus accommodation model for `chrono_tz::TzOffset`'s consistency
//! claim: `fix() == base_utc_offset() + dst_offset()`.
//!
//! `chrono_tz` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all -- confirmed directly, not assumed: a real probe
//! referencing `chrono_tz::TzOffset` from a bare Verus file fails with
//! `E0433: cannot find module or crate 'chrono_tz'`, a flat, immediate
//! resolution error, not a resource wall. There is nothing to
//! reverse-engineer here, and no need to: the claim doesn't depend on
//! *how* the real offset is computed (binary search over a timespan
//! table, whatever), only on the structural fact that each real zone's
//! current offset has a `base`/`dst` decomposition whose sum is the
//! offset's own `fix()` value. This models exactly that structure,
//! minimally (four variants standing in for chrono-tz's real ~453
//! distinct discovered pairs -- `amenable_ext::DISCOVERED_OFFSET_TABLE`
//! -- with four genuine, real values drawn from it, not invented), with
//! two independently-written computations -- one assigning each
//! variant's `base`/`dst` components, one assigning its `fix` value
//! directly, as its own literal, not derived mechanically as their sum
//! -- so the proof is a real (if small) consistency check between them,
//! not a tautology by construction.
//!
//! This model's refinement premise -- that chrono_tz::TzOffset has this
//! same additive structure for every real offset it produces -- rests
//! on chrono-tz's own real source (`TzOffset`'s `fix()` computes
//! `FixedOffset::east_opt(utc_offset + dst_offset)` directly from the
//! same two fields `base_utc_offset()`/`dst_offset()` expose) and is
//! independently confirmed by `amenable_kani::chrono::tz_offset`'s real
//! proof (453 real discovered witnesses, proven by direct execution)
//! and `amenable_creusot::ext_chrono::tz_offset`'s real three-axiom
//! `extern_spec!` (proven against the real `fix`/`base_utc_offset`/
//! `dst_offset` trait impls, confirmed non-vacuous by a deliberately
//! broken re-run).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Four abstract variants, standing in for chrono_tz's real ~453
/// distinct discovered `(base, dst)` pairs. Each one below is a real
/// value from `amenable_ext::DISCOVERED_OFFSET_TABLE`, not invented:
/// `Utc` (0, 0), `Est` (America/New_York winter, -18000, 0), `Edt`
/// (America/New_York summer, -18000, 3600), `LondonDoubleSummer`
/// (Europe/London, WWII double summer time, 0, 7200).
pub enum TzOffsetModelVariant {
    Utc,
    Est,
    Edt,
    LondonDoubleSummer,
}

/// Each variant's own `base_utc_offset`, in seconds.
pub open spec fn tz_offset_model_base(v: TzOffsetModelVariant) -> int {
    match v {
        TzOffsetModelVariant::Utc => 0,
        TzOffsetModelVariant::Est => -18000,
        TzOffsetModelVariant::Edt => -18000,
        TzOffsetModelVariant::LondonDoubleSummer => 0,
    }
}

/// Each variant's own `dst_offset`, in seconds.
pub open spec fn tz_offset_model_dst(v: TzOffsetModelVariant) -> int {
    match v {
        TzOffsetModelVariant::Utc => 0,
        TzOffsetModelVariant::Est => 0,
        TzOffsetModelVariant::Edt => 3600,
        TzOffsetModelVariant::LondonDoubleSummer => 7200,
    }
}

/// Each variant's own `fix()` value, in seconds -- written as its own
/// literal, independently of `tz_offset_model_base`/
/// `tz_offset_model_dst` above (not their mechanical sum), so the
/// consistency proof below is a real check that the two agree.
pub open spec fn tz_offset_model_fix(v: TzOffsetModelVariant) -> int {
    match v {
        TzOffsetModelVariant::Utc => 0,
        TzOffsetModelVariant::Est => -18000,
        TzOffsetModelVariant::Edt => -14400,
        TzOffsetModelVariant::LondonDoubleSummer => 7200,
    }
}

/// The consistency law this model establishes: every variant's own
/// `fix` value equals its `base` plus its `dst`.
pub open spec fn tz_offset_consistency_holds(v: TzOffsetModelVariant) -> bool {
    tz_offset_model_fix(v) == tz_offset_model_base(v) + tz_offset_model_dst(v)
}

/// Proves `tz_offset_consistency_holds` for every variant. A plain
/// `fn`, not `proof fn`: under plain rustc (`verus! {}`'s own erasure,
/// no real verus toolchain), `proof fn` items are stripped entirely,
/// while an ordinary exec `fn` with contracts survives -- the same
/// shape every sibling model file in this directory already uses.
pub fn verify_tz_offset_consistency_model(v: TzOffsetModelVariant)
    ensures
        tz_offset_consistency_holds(v),
{
    let _base = match v {
        TzOffsetModelVariant::Utc => 0,
        TzOffsetModelVariant::Est => -18000,
        TzOffsetModelVariant::Edt => -18000,
        TzOffsetModelVariant::LondonDoubleSummer => 0,
    };
}

} // verus!
