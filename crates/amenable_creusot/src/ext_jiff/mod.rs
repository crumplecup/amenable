//! Real Creusot proof content (`extern_spec!`/`harness!` Pearlite
//! functions) for `amenable_ext`'s jiff carriers — the proof-content
//! sibling to `ext::jiff`'s `CreusotWitness` bridge, exactly the same
//! split `rust_std`/`rust_std_witness` use and for the same reason (see
//! this crate's own root doc comment): `creusot-rustc`'s whole-crate
//! translation pass can't handle the ordinary Rust machinery
//! (`inventory::submit!`, trait dispatch) the witness bridge needs when
//! it's *local* to the translated crate, so that bridge stays
//! `#[cfg(not(creusot))]`-gated inside `ext`, and this module — pure
//! Pearlite proof content, the thing `cargo creusot` actually
//! translates — stays unconditional.
//!
//! Every leaf module below is private: none of them carry real
//! cross-file pub(crate) surface of their own (each is "one real
//! Creusot proof per file," with every item either `#[cfg(creusot)]`-
//! gated Pearlite content or a private `inventory::submit!` side
//! effect) -- the one thing the crate root needs from each, its
//! `harness!`-generated `VERIFY_..._SRC` constant, is re-exported
//! right here instead, flattened one level, matching how every real
//! consumer already imports it (`use crate::VERIFY_..._SRC`, never
//! `crate::ext_jiff::<module>::VERIFY_..._SRC`). `shared_trusted_
//! accessors` and `span` stay `pub(crate) mod` because real sibling
//! files (`span_fieldwise.rs`, several of the leaf modules below) do
//! reach into them by path.

mod civil_date;
mod civil_era;
mod civil_iso_week_date;
mod civil_time;
mod civil_time_series;
mod civil_weekday;
mod civil_weekdays_forward;
mod civil_weekdays_reverse;
mod date_series;
mod date_time_series;
mod error;
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
pub(crate) mod shared_trusted_accessors;
mod signed_duration;
pub(crate) mod span;
mod span_fieldwise;
mod timestamp_series;
mod tz_ambiguous_timestamp;
mod tz_ambiguous_zoned;
mod tz_dst;
mod tz_offset_conflict;
mod tz_time_zone;
mod tz_time_zone_database;
mod tz_time_zone_offset_info;
mod zoned_series;

pub use civil_date::VERIFY_CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_SRC;
pub use civil_era::VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC;
pub use civil_iso_week_date::VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC;
pub use civil_time::VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC;
pub use civil_time_series::VERIFY_CIVIL_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC;
pub use civil_weekday::VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC;
pub use civil_weekdays_forward::VERIFY_CIVIL_WEEKDAYS_FORWARD_NEXT_YIELDS_START_THEN_ITS_SUCCESSOR_SRC;
pub use civil_weekdays_reverse::VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC;
pub use date_series::VERIFY_DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC;
pub use date_time_series::VERIFY_DATE_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC;
pub use error::{
    ERROR_CLASSIFICATION_PREDICATES_ARE_MUTUALLY_EXCLUSIVE_HOLDS_SRC,
    VERIFY_ERROR_CLASSIFICATION_PREDICATES_ARE_MUTUALLY_EXCLUSIVE_SRC,
};
pub use fmt_friendly_fractional_unit::VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC;
pub use fmt_std_fmt_write::VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_SRC;
pub use fmt_std_io_write::VERIFY_FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_SRC;
pub use fmt_strtime_broken_down_time::VERIFY_FMT_STRTIME_BROKEN_DOWN_TIME_NUMERIC_SETTERS_ROUND_TRIP_SRC;
pub use fmt_strtime_meridiem::VERIFY_FMT_STRTIME_MERIDIEM_FROM_TIME_MATCHES_HOUR_THRESHOLD_SRC;
pub use fmt_temporal_pieces::VERIFY_FMT_TEMPORAL_PIECES_WITH_DATE_WITH_TIME_ROUND_TRIP_SRC;
pub use fmt_temporal_pieces_numeric_offset::VERIFY_FMT_TEMPORAL_PIECES_NUMERIC_OFFSET_FROM_AND_WITH_NEGATIVE_ZERO_SRC;
pub use fmt_temporal_pieces_offset::VERIFY_FMT_TEMPORAL_PIECES_OFFSET_ZULU_AND_FROM_OFFSET_ROUND_TRIP_SRC;
pub use fmt_temporal_time_zone_annotation::VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_FROM_NAME_AND_FROM_OFFSET_SRC;
pub use fmt_temporal_time_zone_annotation_kind::VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_KIND_FROM_NAME_AND_FROM_OFFSET_SRC;
pub use fmt_temporal_time_zone_annotation_name::VERIFY_FMT_TEMPORAL_TIME_ZONE_ANNOTATION_NAME_FROM_STR_ROUND_TRIPS_SRC;
pub use offset::{
    OFFSET_FROM_SECONDS_ROUND_TRIPS_HOLDS_SRC, VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC,
};
pub use signed_duration::{
    SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC,
    VERIFY_SIGNED_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
};
pub use span_fieldwise::VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC;
pub use timestamp_series::VERIFY_TIMESTAMP_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC;
pub use tz_ambiguous_timestamp::VERIFY_TZ_AMBIGUOUS_TIMESTAMP_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC;
pub use tz_ambiguous_zoned::VERIFY_TZ_AMBIGUOUS_ZONED_FROM_FIXED_TIME_ZONE_IS_ALWAYS_UNAMBIGUOUS_SRC;
pub use tz_dst::VERIFY_TZ_DST_FROM_BOOL_ROUND_TRIPS_SRC;
pub use tz_offset_conflict::VERIFY_TZ_OFFSET_CONFLICT_ALWAYS_OFFSET_AND_ALWAYS_TIME_ZONE_SRC;
pub use tz_time_zone::VERIFY_TZ_TIME_ZONE_UNKNOWN_AND_FIXED_ROUND_TRIP_SRC;
pub use tz_time_zone_database::VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC;
pub use tz_time_zone_offset_info::VERIFY_TZ_TIME_ZONE_OFFSET_INFO_FROM_FIXED_TIME_ZONE_SRC;
pub use zoned_series::VERIFY_ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_SRC;
