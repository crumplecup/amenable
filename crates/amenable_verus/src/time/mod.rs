//! Verus specs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). One file per contract,
//! each holding exactly the `verus! { ... }` spec function(s) its real
//! invariant needs — matching `rust_std`'s one-claim-per-carrier shape,
//! so `amenable_time::verus_witness` can `include_str!` a single file as
//! that contract's whole `claim`.

mod calendar_month_carrier;
mod hour_in_range_zero_to_twenty_four_carrier;
mod minute_in_range_zero_to_fifty_nine_carrier;
mod second_in_range_zero_to_sixty_carrier;
mod utc_offset_hour_in_range_zero_to_twenty_three_carrier;
mod utc_offset_minute_in_range_zero_to_fifty_nine_carrier;
#[cfg(verus_keep_ghost)]
pub use calendar_month_carrier::calendar_month_is_enumerated;
pub use calendar_month_carrier::verify_calendar_month_in_range;
#[cfg(verus_keep_ghost)]
pub use hour_in_range_zero_to_twenty_four_carrier::hour_in_range_zero_to_twenty_four_holds;
pub use hour_in_range_zero_to_twenty_four_carrier::verify_hour_in_range_zero_to_twenty_four;
#[cfg(verus_keep_ghost)]
pub use minute_in_range_zero_to_fifty_nine_carrier::minute_in_range_zero_to_fifty_nine_holds;
pub use minute_in_range_zero_to_fifty_nine_carrier::verify_minute_in_range_zero_to_fifty_nine;
#[cfg(verus_keep_ghost)]
pub use second_in_range_zero_to_sixty_carrier::second_in_range_zero_to_sixty_holds;
pub use second_in_range_zero_to_sixty_carrier::verify_second_in_range_zero_to_sixty;
#[cfg(verus_keep_ghost)]
pub use utc_offset_hour_in_range_zero_to_twenty_three_carrier::utc_offset_hour_in_range_zero_to_twenty_three_holds;
pub use utc_offset_hour_in_range_zero_to_twenty_three_carrier::verify_utc_offset_hour_in_range_zero_to_twenty_three;
#[cfg(verus_keep_ghost)]
pub use utc_offset_minute_in_range_zero_to_fifty_nine_carrier::utc_offset_minute_in_range_zero_to_fifty_nine_holds;
pub use utc_offset_minute_in_range_zero_to_fifty_nine_carrier::verify_utc_offset_minute_in_range_zero_to_fifty_nine;
