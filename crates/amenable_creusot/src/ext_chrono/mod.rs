//! Creusot proof content for chrono's registered types: the Pearlite model and
//! its harnesses, unconditionally compiled so `cargo creusot` translates them.
//! The `CreusotWitness` bridge that names these harnesses lives in
//! `crate::chrono`, behind `cfg(not(creusot))`.

mod civil_naive_date;
mod fixed_offset;
mod shared_trusted_accessors;
mod utc;

pub use civil_naive_date::VERIFY_NAIVE_DATE_MODEL_ROUND_TRIPS_SRC;
pub use fixed_offset::VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC;
pub use utc::VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC;
