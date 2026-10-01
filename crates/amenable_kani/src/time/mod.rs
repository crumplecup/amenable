//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.
//!
//! Split by the same real domains as `amenable_creusot`'s analogous
//! `time/` module (cross-backend naming consistency, since both prove
//! the same 23 atomic contracts): `clock_bounds`,
//! `calendar_month_and_week`, `calendar_ordinal_bounds`,
//! `interval_ordering`, `leap_year_core` (further split into
//! `rule`/`year_length` since the combined file exceeded 500 lines),
//! `month_day_bounds` (further split into
//! `month_duration`/`day_bounds`, same reason).

mod calendar_month_and_week;
mod calendar_ordinal_bounds;
mod clock_bounds;
mod interval_ordering;
mod leap_year_core;
mod month_day_bounds;
