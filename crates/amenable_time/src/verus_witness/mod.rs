//! Verus `Witness` impls for `amenable_time`'s genuinely-checkable
//! temporal contracts (`AMENABLE_TIME_PLAN.md` Phase 6).
//!
//! Unlike Kani and Creusot, the Verus toolchain is invoked as a bare
//! compiler over `amenable_verus/src/lib.rs` — it never resolves Cargo
//! dependencies, so the *proof* content (the real `verus! { ... }` spec
//! functions) lives there. This module is the ordinary-Rust half: it
//! ties each `amenable_time` contract to the Verus spec file that
//! machine-checks it (`include_str!`, so the two can never drift), and
//! registers the `Witness<VerusVerifier>` the `proof_composition`
//! composites need to resolve under `VerusVerifier`.
//!
//! Split by contract family, matching `amenable_creusot`/`amenable_kani`'s
//! own `time/` directories: `clock_bounds`, `calendar_month_and_week`,
//! `calendar_ordinal_bounds`, `interval_ordering`, `leap_year_core`, and
//! `month_day_bounds`. `proof` holds the shared `TemporalVerusProof`
//! artifact every family's impls construct.

mod calendar_month_and_week;
mod calendar_ordinal_bounds;
mod clock_bounds;
mod interval_ordering;
mod leap_year_core;
mod month_day_bounds;
mod proof;

pub use proof::TemporalVerusProof;
