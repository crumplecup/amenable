//! Creusot proof content for chrono's registered types: the Pearlite model and
//! its harnesses, unconditionally compiled so `cargo creusot` translates them.
//! The `CreusotWitness` bridge that names these harnesses lives in
//! `crate::chrono`, behind `cfg(not(creusot))`.

mod civil_naive_date;
mod fixed_offset;
#[cfg(feature = "chrono-tz")]
mod gap_info;
mod iso_week;
mod naive_date_time;
mod naive_time;
mod naive_week;
#[cfg(feature = "chrono-tz")]
mod parse_error;
mod shared_trusted_accessors;
#[cfg(feature = "chrono-tz")]
mod tz;
#[cfg(feature = "chrono-tz")]
mod tz_offset;
mod utc;

pub use civil_naive_date::VERIFY_NAIVE_DATE_MODEL_ROUND_TRIPS_SRC;
pub use fixed_offset::VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC;
#[cfg(feature = "chrono-tz")]
pub use gap_info::VERIFY_GAP_INFO_MATCHES_OFFSET_FROM_LOCAL_DATETIME_SRC;
pub use iso_week::VERIFY_ISO_WEEK_ROUND_TRIPS_MODEL_SRC;
pub use naive_date_time::VERIFY_NAIVE_DATE_TIME_ROUND_TRIPS_SRC;
pub use naive_time::VERIFY_NAIVE_TIME_FROM_HMS_NANO_ROUND_TRIPS_SRC;
pub use naive_week::VERIFY_NAIVE_WEEK_SPAN_MODEL_SRC;
#[cfg(feature = "chrono-tz")]
pub use parse_error::VERIFY_ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC;
#[cfg(feature = "chrono-tz")]
pub use tz::VERIFY_TZ_NAME_AND_FROM_STR_ROUND_TRIP_SRC;
#[cfg(feature = "chrono-tz")]
pub use tz_offset::VERIFY_TZ_OFFSET_CONSISTENCY_SRC;
pub use utc::VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC;
