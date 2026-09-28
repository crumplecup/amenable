//! Verus accommodation models for jiff carriers, one file per real jiff
//! area that has earned one.

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
pub mod fmt_strtime_broken_down_time;
mod fmt_strtime_meridiem;
mod fmt_temporal_pieces;
mod fmt_temporal_pieces_numeric_offset;
mod fmt_temporal_pieces_offset;
mod fmt_temporal_time_zone_annotation;
mod fmt_temporal_time_zone_annotation_kind;
mod fmt_temporal_time_zone_annotation_name;
mod offset;
mod signed_duration;
pub mod span;
mod span_fieldwise;
mod timestamp_series;
mod tz_ambiguous_timestamp;
mod tz_ambiguous_zoned;
mod tz_dst;
mod tz_offset_conflict;
mod tz_time_zone;
mod tz_time_zone_database;
mod tz_time_zone_offset_info;
mod unit;
mod zoned_series;

pub use civil_date::verify_civil_date_new_year_month_day_round_trips_model;
#[cfg(verus_keep_ghost)]
pub use civil_date::{civil_date_fields_in_range, civil_date_new_model_round_trip_holds};

pub use civil_era::{EraModel, verify_civil_era_year_classifies_bce_and_ce_correctly_model};
#[cfg(verus_keep_ghost)]
pub use civil_era::{civil_era_year_classifies_bce_and_ce_correctly, civil_era_year_in_range};

pub use civil_iso_week_date::verify_civil_iso_week_date_new_year_week_weekday_round_trips_model;
#[cfg(verus_keep_ghost)]
pub use civil_iso_week_date::{
    civil_iso_week_date_new_model_round_trip_holds, civil_iso_week_date_week_in_range,
    civil_iso_week_date_year_in_range, weekday_offset_in_range,
};

pub use civil_time::verify_civil_time_new_hour_minute_second_subsec_round_trips_model;
#[cfg(verus_keep_ghost)]
pub use civil_time::{civil_time_fields_in_range, civil_time_new_model_round_trip_holds};

pub use civil_time_series::verify_civil_time_series_next_yields_start_then_advances_by_period;
#[cfg(verus_keep_ghost)]
pub use civil_time_series::{
    civil_time_series_next_yields_start_then_advances_by_period_holds,
    civil_time_series_period_nanos_in_range, civil_time_series_start_nanos_in_range,
};

pub use civil_weekday::verify_civil_weekday_monday_one_offset_round_trips_model;
#[cfg(verus_keep_ghost)]
pub use civil_weekday::{
    civil_weekday_monday_one_offset_model_round_trip_holds, weekday_monday_one_offset_in_range,
};

pub use civil_weekdays_forward::verify_civil_weekdays_forward_next_yields_start_then_its_successor;
#[cfg(verus_keep_ghost)]
pub use civil_weekdays_forward::{
    civil_weekdays_forward_next_yields_start_then_its_successor_holds,
    civil_weekdays_forward_start_offset_in_range,
};

pub use civil_weekdays_reverse::verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor;
#[cfg(verus_keep_ghost)]
pub use civil_weekdays_reverse::{
    civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds,
    civil_weekdays_reverse_start_offset_in_range,
};

pub use date_series::verify_date_series_next_yields_start_then_advances_by_period;
#[cfg(verus_keep_ghost)]
pub use date_series::{
    date_series_next_yields_start_then_advances_by_period_holds, date_series_period_days_in_range,
    date_series_start_days_in_range,
};

pub use date_time_series::verify_date_time_series_next_yields_start_then_advances_by_period;
#[cfg(verus_keep_ghost)]
pub use date_time_series::{
    date_time_series_next_yields_start_then_advances_by_period_holds,
    date_time_series_period_days_in_range, date_time_series_start_days_in_range,
};

#[cfg(verus_keep_ghost)]
pub use error::error_classification_predicates_are_mutually_exclusive;
pub use error::{ErrorClassModel, verify_error_classification_predicates_are_mutually_exclusive};

#[cfg(verus_keep_ghost)]
pub use fmt_friendly_fractional_unit::fractional_unit_model_conversion_holds;
pub use fmt_friendly_fractional_unit::{
    FractionalUnitModel, verify_fmt_friendly_fractional_unit_from_matches_documented_mapping_model,
};

#[cfg(verus_keep_ghost)]
pub use fmt_std_fmt_write::fmt_std_fmt_write_appends_str;
pub use fmt_std_fmt_write::verify_fmt_std_fmt_write_write_str_model;

#[cfg(verus_keep_ghost)]
pub use fmt_std_io_write::fmt_std_io_write_appends_bytes;
pub use fmt_std_io_write::verify_fmt_std_io_write_write_str_model;

pub use fmt_strtime_meridiem::{
    MeridiemModel, verify_fmt_strtime_meridiem_from_time_matches_hour_threshold_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_strtime_meridiem::{meridiem_hour_in_range, meridiem_model_from_hour_holds};

pub use fmt_temporal_pieces::{
    PiecesModel, verify_fmt_temporal_pieces_with_date_with_time_round_trip_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_pieces::{
    pieces_from_date_holds, pieces_with_date_holds, pieces_with_time_holds,
};

pub use fmt_temporal_pieces_numeric_offset::{
    PiecesNumericOffsetModel,
    verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_pieces_numeric_offset::{
    pno_from_offset_seconds_holds, pno_offset_seconds_in_range, pno_with_negative_zero_holds,
};

pub use fmt_temporal_pieces_offset::{
    PiecesOffsetModel, verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_pieces_offset::{
    pieces_offset_from_offset_seconds_holds, pieces_offset_to_numeric_seconds_holds,
    pieces_offset_zulu_holds,
};

pub use fmt_temporal_time_zone_annotation::{
    TimeZoneAnnotationModel,
    verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_time_zone_annotation::{
    tz_annotation_from_name_holds, tz_annotation_from_offset_seconds_holds,
};

pub use fmt_temporal_time_zone_annotation_kind::{
    TimeZoneAnnotationKindModel,
    verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_time_zone_annotation_kind::{
    tz_annotation_kind_from_name_holds, tz_annotation_kind_from_offset_seconds_holds,
};

pub use fmt_temporal_time_zone_annotation_name::{
    TimeZoneAnnotationNameModel,
    verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips_model,
};
#[cfg(verus_keep_ghost)]
pub use fmt_temporal_time_zone_annotation_name::{
    tz_annotation_name_as_str_holds, tz_annotation_name_from_name_holds,
};

pub use offset::verify_offset_from_seconds_model_round_trips;
#[cfg(verus_keep_ghost)]
pub use offset::{offset_from_seconds_model_round_trip_holds, offset_seconds_in_range};

pub use signed_duration::verify_signed_duration_new_model_normalizes_nanos_and_carries_into_secs;
#[cfg(verus_keep_ghost)]
pub use signed_duration::{
    signed_duration_new_model_normalizes, signed_duration_new_secs_headroom_holds,
};

#[cfg(verus_keep_ghost)]
pub use span_fieldwise::span_fieldwise_negation_model_holds;
pub use span_fieldwise::verify_span_fieldwise_negation_model_negates_every_unit_getter;

pub use timestamp_series::verify_timestamp_series_next_yields_start_then_advances_by_period;
#[cfg(verus_keep_ghost)]
pub use timestamp_series::{
    timestamp_series_next_yields_start_then_advances_by_period_holds,
    timestamp_series_period_seconds_in_range, timestamp_series_start_seconds_in_range,
};

#[cfg(verus_keep_ghost)]
pub use tz_ambiguous_timestamp::ambiguous_timestamp_from_fixed_offset_seconds_holds;
pub use tz_ambiguous_timestamp::{
    AmbiguousTimestampModel,
    verify_tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_model,
};

#[cfg(verus_keep_ghost)]
pub use tz_ambiguous_zoned::ambiguous_zoned_from_fixed_offset_seconds_holds;
pub use tz_ambiguous_zoned::{
    AmbiguousZonedModel, verify_tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous_model,
};

pub use tz_dst::{DstModel, verify_tz_dst_from_bool_round_trips_model};
#[cfg(verus_keep_ghost)]
pub use tz_dst::{dst_from_bool_holds, dst_is_dst_holds, dst_is_std_holds};

#[cfg(verus_keep_ghost)]
pub use tz_offset_conflict::{
    offset_conflict_always_offset_holds, offset_conflict_always_time_zone_holds,
};
pub use tz_offset_conflict::{
    resolve_always_offset_seconds, resolve_always_time_zone_seconds,
    verify_tz_offset_conflict_always_offset_and_always_time_zone_model,
};

pub use tz_time_zone::{TimeZoneModel, verify_tz_time_zone_unknown_and_fixed_round_trip_model};
#[cfg(verus_keep_ghost)]
pub use tz_time_zone::{time_zone_fixed_holds, time_zone_unknown_holds};

#[cfg(verus_keep_ghost)]
pub use tz_time_zone_database::time_zone_database_none_holds;
pub use tz_time_zone_database::{
    TimeZoneDatabaseModel, verify_tz_time_zone_database_none_is_definitively_empty_model,
};

#[cfg(verus_keep_ghost)]
pub use tz_time_zone_offset_info::time_zone_offset_info_from_fixed_offset_seconds_holds;
pub use tz_time_zone_offset_info::{
    TimeZoneOffsetInfoModel, verify_tz_time_zone_offset_info_from_fixed_time_zone_model,
};

pub use unit::{UnitModel, verify_unit_ordering_matches_discriminant_order_model};
#[cfg(verus_keep_ghost)]
pub use unit::{
    unit_model_discriminant, unit_model_discriminant_exec_matches, unit_model_ordering_holds,
};

pub use zoned_series::verify_zoned_series_next_yields_start_then_advances_by_period_under_utc;
#[cfg(verus_keep_ghost)]
pub use zoned_series::{
    zoned_series_next_yields_start_then_advances_by_period_under_utc_holds,
    zoned_series_period_seconds_in_range, zoned_series_start_seconds_in_range,
};
