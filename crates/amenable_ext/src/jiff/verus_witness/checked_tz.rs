//! Checked `jiff::tz::*` types -- real, hand-verified Verus
//! accommodation models.
//!
//! `jiff::tz::AmbiguousTimestamp`'s model is scoped to the
//! `TimeZone::fixed` case only (no `DateTime`/`dt` payload modeled
//! at all — `civil::DateTime` stays opaque/trusted here by
//! deliberate Phase 1 design) — the same claim `amenable_creusot::
//! ext_jiff::tz_ambiguous_timestamp` checks against jiff's real API
//! (Kani can't check this claim at all — see `amenable_kani::
//! ext::jiff`'s own doc comment for the real `TimeZone`
//! `Repr`-dispatch CBMC wall).
//!
//! `jiff::tz::AmbiguousZoned`'s model is the same shape and scope —
//! the same claim `amenable_creusot::ext_jiff::tz_ambiguous_zoned`
//! checks against jiff's real API, also trusted for Kani (calls the
//! same already-confirmed-timing-out function directly).
//!
//! `jiff::tz::Dst`'s model is a plain, field-less two-variant enum
//! (matches `fmt_strtime_meridiem.rs`'s own `MeridiemModel` shape) —
//! the same claim `amenable_kani::ext::jiff::tz_dst` and
//! `amenable_creusot::ext_jiff::tz_dst` both check against jiff's
//! real `From<bool>`/`is_dst`/`is_std` API.
//!
//! `jiff::tz::OffsetConflict`'s model reproduces just the resolved
//! offset seconds directly (no `AmbiguousZoned`/`TimeZone`/
//! `DateTime` payload at all), scoped to `AlwaysOffset`/
//! `AlwaysTimeZone` only — the same claim `amenable_creusot::
//! ext_jiff::tz_offset_conflict` checks against jiff's real API,
//! also trusted for Kani (calls the same already-confirmed-
//! timing-out function directly).
//!
//! `jiff::tz::TimeZone`'s model reproduces `unknown`/`fixed` as a
//! plain struct (`{ is_unknown: bool, fixed_offset_seconds: i32 }`)
//! — the same claim `amenable_creusot::ext_jiff::tz_time_zone` checks
//! against jiff's real API, also trusted for Kani (the type
//! underlying every `Repr`-dispatch CBMC wall confirmed this
//! session).
//!
//! `jiff::tz::TimeZoneDatabase`'s model is scoped to `none()` only —
//! the same claim `amenable_kani::ext::jiff::tz_time_zone_database`
//! and `amenable_creusot::ext_jiff::tz_time_zone_database` both check
//! against jiff's real API.
//!
//! `jiff::tz::TimeZoneOffsetInfo<'static>` gets a real checked
//! property too (see `tz_time_zone_offset_info.rs`): a hand-verified
//! model of `TimeZone::fixed(offset).to_offset_info(ts)`, scoped
//! identically to `amenable_creusot::ext_jiff::
//! tz_time_zone_offset_info`'s own real `extern_spec!` — offset
//! seconds round-trip, DST always inactive. Trusted on Kani
//! specifically (the `TimeZone::Repr`-dispatch CBMC wall).
//!
//! `jiff::tz::TimeZonePrecedingTransitions<'static>` also stays
//! trusted, for consistency with the same content-free reasoning
//! already established for `TimeZoneFollowingTransitions` (a
//! confirmed structural mirror, not assumed — see `amenable_kani::
//! ext::jiff`'s own doc comment).

use super::bridge::{ExtCheckedProof, impl_verus_witness_checked_ext};
use crate::ExtStandard;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    jiff::tz::AmbiguousTimestamp,
    "verify_tz_ambiguous_timestamp_from_fixed_time_zone_is_always_unambiguous_model",
    "../../../../amenable_verus/src/jiff/tz_ambiguous_timestamp.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::AmbiguousTimestamp>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::AmbiguousTimestamp),
        ">"
    ),
    "ambiguous_timestamp_from_fixed_offset_seconds_holds"
);

impl_verus_witness_checked_ext!(
    jiff::tz::AmbiguousZoned,
    "verify_tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous_model",
    "../../../../amenable_verus/src/jiff/tz_ambiguous_zoned.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::AmbiguousZoned>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::AmbiguousZoned),
        ">"
    ),
    "ambiguous_zoned_from_fixed_offset_seconds_holds"
);

impl_verus_witness_checked_ext!(
    jiff::tz::Dst,
    "verify_tz_dst_from_bool_round_trips_model",
    "../../../../amenable_verus/src/jiff/tz_dst.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::Dst>,
    concat!("amenable_ext::ExtStandard<", stringify!(jiff::tz::Dst), ">"),
    [
        "dst_from_bool_holds",
        "dst_is_dst_holds",
        "dst_is_std_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::tz::OffsetConflict,
    "verify_tz_offset_conflict_always_offset_and_always_time_zone_model",
    "../../../../amenable_verus/src/jiff/tz_offset_conflict.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::OffsetConflict>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::OffsetConflict),
        ">"
    ),
    [
        "offset_conflict_always_offset_holds",
        "offset_conflict_always_time_zone_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::tz::TimeZone,
    "verify_tz_time_zone_unknown_and_fixed_round_trip_model",
    "../../../../amenable_verus/src/jiff/tz_time_zone.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::TimeZone>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::TimeZone),
        ">"
    ),
    ["time_zone_unknown_holds", "time_zone_fixed_holds"]
);

impl_verus_witness_checked_ext!(
    jiff::tz::TimeZoneDatabase,
    "verify_tz_time_zone_database_none_is_definitively_empty_model",
    "../../../../amenable_verus/src/jiff/tz_time_zone_database.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::TimeZoneDatabase>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::TimeZoneDatabase),
        ">"
    ),
    "time_zone_database_none_holds"
);

impl_verus_witness_checked_ext!(
    jiff::tz::TimeZoneOffsetInfo<'static>,
    "verify_tz_time_zone_offset_info_from_fixed_time_zone_model",
    "../../../../amenable_verus/src/jiff/tz_time_zone_offset_info.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::TimeZoneOffsetInfo<'static>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::TimeZoneOffsetInfo<'static>),
        ">"
    ),
    "time_zone_offset_info_from_fixed_offset_seconds_holds"
);

impl_verus_witness_checked_ext!(
    jiff::tz::Offset,
    "verify_offset_from_seconds_model_round_trips",
    "../../../../amenable_verus/src/jiff/offset.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::tz::Offset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::tz::Offset),
        ">"
    ),
    "offset_from_seconds_model_round_trip_holds"
);
