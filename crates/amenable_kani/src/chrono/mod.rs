//! Kani witnesses for chrono's registered types.
//!
//! Each witness is a full-domain harness over chrono's public API. Nothing is
//! narrowed to a sub-range to make CBMC finish: if a harness does not finish,
//! that is reported as a finding, not worked around.

mod civil_iso_week;
mod civil_naive_date;
mod civil_naive_date_time;
mod civil_naive_time;
mod civil_naive_week;
mod fixed_offset;
mod gregorian;
mod utc;
mod week_span_partitions;
