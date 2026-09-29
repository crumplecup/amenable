//! Creusot proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets a `#[logic]` postcondition, a `#[requires]`/`#[ensures]`-contracted
//! function Creusot machine-checks against it, and (on the
//! `#[cfg(not(creusot))]` side, matching `ledger::contract_bounds`) a
//! `Witness<CreusotVerifier>` + `Ensures<CreusotVerifier>` tying the real
//! `amenable_time` type to that Pearlite content. Structural contracts
//! stay `Standard`-only and never reach this module.

mod calendar_month_and_week;
mod calendar_ordinal_bounds;
mod clock_bounds;
mod interval_ordering;
mod leap_year_core;
mod month_day_bounds;
