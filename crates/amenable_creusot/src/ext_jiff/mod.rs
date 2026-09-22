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

pub(crate) mod civil_date;
pub(crate) mod civil_era;
pub(crate) mod civil_iso_week_date;
pub(crate) mod civil_time;
pub(crate) mod civil_time_series;
pub(crate) mod civil_weekday;
pub(crate) mod civil_weekdays_forward;
pub(crate) mod civil_weekdays_reverse;
pub(crate) mod date_series;
pub(crate) mod date_time_series;
pub(crate) mod error;
pub(crate) mod fmt_friendly_fractional_unit;
pub(crate) mod fmt_std_fmt_write;
pub(crate) mod fmt_std_io_write;
pub(crate) mod fmt_strtime_broken_down_time;
pub(crate) mod fmt_strtime_meridiem;
pub(crate) mod fmt_temporal_pieces;
pub(crate) mod fmt_temporal_pieces_numeric_offset;
pub(crate) mod fmt_temporal_pieces_offset;
pub(crate) mod fmt_temporal_time_zone_annotation;
pub(crate) mod fmt_temporal_time_zone_annotation_kind;
pub(crate) mod fmt_temporal_time_zone_annotation_name;
pub(crate) mod offset;
pub(crate) mod signed_duration;
pub(crate) mod span;
pub(crate) mod span_fieldwise;
pub(crate) mod timestamp_series;
pub(crate) mod tz_ambiguous_timestamp;
pub(crate) mod tz_ambiguous_zoned;
pub(crate) mod tz_dst;
pub(crate) mod zoned_series;
