//! Verus specs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). One file per contract,
//! each holding exactly the `verus! { ... }` spec function(s) its real
//! invariant needs — matching `rust_std`'s one-claim-per-carrier shape,
//! so `amenable_time::verus_witness` can `include_str!` a single file as
//! that contract's whole `claim`.

mod calendar_month_carrier;
#[cfg(verus_keep_ghost)]
pub use calendar_month_carrier::calendar_month_is_enumerated;
pub use calendar_month_carrier::verify_calendar_month_in_range;
