//! `Witness<KaniVerifier>` registrations for `amenable_ext::ExtStandard<T>`
//! over jiff's registered carriers, split one file per real jiff area
//! that has earned a checked harness, plus the trusted carriers that
//! haven't (below).

mod macros;

mod civil_date;
mod civil_era;
mod civil_iso_week_date;
mod civil_time;
mod civil_weekday;
mod civil_weekdays_forward;
mod civil_weekdays_reverse;
mod fmt_friendly_fractional_unit;
mod fmt_std_fmt_write;
mod fmt_std_io_write;
mod fmt_strtime_broken_down_time;
mod fmt_strtime_meridiem;
mod fmt_temporal_pieces;
mod fmt_temporal_pieces_numeric_offset;
mod fmt_temporal_pieces_offset;
mod fmt_temporal_time_zone_annotation;
mod fmt_temporal_time_zone_annotation_kind;
mod fmt_temporal_time_zone_annotation_name;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod tz_dst;
mod tz_time_zone_database;
mod tz_time_zone_name_iter;
mod unit;

mod trusted_civil;
mod trusted_fmt;
mod trusted_top_level;
mod trusted_tz;
