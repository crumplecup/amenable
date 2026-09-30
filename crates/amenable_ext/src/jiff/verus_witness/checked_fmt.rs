//! Checked `jiff::fmt::*` types -- real, hand-verified Verus
//! accommodation models.
//!
//! `jiff::fmt::StdFmtWrite<String>`'s model differs from every other
//! model in this list: it's not a from-scratch hand reproduction, but
//! uses `vstd`'s own already-contracted `String::append` directly
//! (real `vstd` coverage confirmed present, unlike jiff itself), and
//! states the FULL round-trip claim (`amenable_kani::ext::jiff::
//! fmt_std_fmt_write`'s own stronger claim) rather than
//! `amenable_creusot::ext_jiff::fmt_std_fmt_write`'s narrower
//! never-fails-only one — Verus never extern-specs jiff's actual
//! generic trait impl the way Creusot tried and hit a real "extern
//! spec generics don't match" wall on, so it has no reason to scope
//! down to match.
//!
//! `jiff::fmt::StdIoWrite<Vec<u8>>`'s model is the identical shape,
//! using `vstd`'s own already-contracted `Vec::extend_from_slice`
//! instead of `String::append` — also states the FULL round-trip
//! claim, unlike `amenable_creusot::ext_jiff::fmt_std_io_write`'s
//! narrower never-fails-only one.
//!
//! `jiff::fmt::friendly::FractionalUnit`'s model reuses `unit.rs`'s
//! existing `UnitModel` directly rather than modeling `Unit`'s ten
//! variants a second time — the same claim `amenable_kani::ext::jiff::
//! fmt_friendly_fractional_unit` and `amenable_creusot::ext_jiff::
//! fmt_friendly_fractional_unit` check against jiff's real
//! `From<FractionalUnit> for Unit` conversion.
//!
//! `jiff::fmt::strtime::BrokenDownTime`'s model is the first `&mut
//! self`-setter model in this checklist: unlike every prior model
//! (pure functions building a fresh value), it uses Verus's own
//! `old(self)`/final-`self` convention directly — no `mem::forget`/
//! `ManuallyDrop` workaround needed at all, since Verus has no
//! CBMC-style Drop-glue cost (confirmed: this model verifies
//! instantly, unlike `amenable_kani::ext::jiff::
//! fmt_strtime_broken_down_time`'s own real CBMC wall for the
//! identical claim).
//!
//! `jiff::fmt::strtime::Meridiem`'s model reproduces its documented
//! `hour < 12` threshold directly with its own small `MeridiemModel`
//! enum (`Am`/`Pm`) — the same claim `amenable_kani::ext::jiff::
//! fmt_strtime_meridiem` and `amenable_creusot::ext_jiff::
//! fmt_strtime_meridiem` check against jiff's real `From<civil::Time>
//! for Meridiem` conversion.
//!
//! `jiff::fmt::temporal::Pieces<'static>`'s model reproduces
//! `date`/`time` as plain tuples (`(i16, i8, i8)`/`Option<(i8, i8,
//! i8, i32)>`) rather than reusing `civil_date.rs`'s/`civil_time.rs`'s
//! own model types, exercising `with_date`/`with_time` independently
//! on their own fresh instance each — the same claim
//! `amenable_kani::ext::jiff::fmt_temporal_pieces` and
//! `amenable_creusot::ext_jiff::fmt_temporal_pieces` both check
//! against jiff's real API, scoped to `date`/`time` only (see either
//! module's own doc comment for why `offset`/`time_zone_annotation`
//! are deliberately excluded).
//!
//! `jiff::fmt::temporal::PiecesNumericOffset`'s model reproduces it as
//! `{ offset_seconds: i32, is_negative: bool }`, self-contained rather
//! than cross-importing `offset.rs`'s own range spec fn — the same
//! claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_numeric_offset` and `amenable_creusot::
//! ext_jiff::fmt_temporal_pieces_numeric_offset` both check against
//! jiff's real `From<Offset>`/`with_negative_zero` API.
//!
//! `jiff::fmt::temporal::PiecesOffset`'s model reproduces it as a
//! plain struct (`{ is_zulu: bool, numeric_seconds: i32 }`), not a
//! data-carrying `enum` (a first attempt tripped a real `missing_docs`
//! warning on a Verus-synthesized method — see that model's own doc
//! comment), self-contained rather than cross-importing
//! `fmt_temporal_pieces_numeric_offset.rs`'s own model — the same
//! claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_offset` and `amenable_creusot::ext_jiff::
//! fmt_temporal_pieces_offset` both check against jiff's real
//! `Zulu`/`From<Offset>`/`to_numeric_offset` API.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>`'s model
//! reproduces it as a plain struct (`{ is_named: bool,
//! offset_seconds: i32 }`), at the same NARROWER scope
//! `fmt_temporal_time_zone_annotation.rs`'s own Creusot doc comment
//! documents (no string content modeled at all) — the same claim
//! `amenable_kani::ext::jiff::fmt_temporal_time_zone_annotation` and
//! `amenable_creusot::ext_jiff::fmt_temporal_time_zone_annotation`
//! both check against jiff's real `From<&str>`/`From<Offset>` API.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationKind<'static>`'s model
//! reproduces it as the same plain struct shape (`{ is_named: bool,
//! offset_seconds: i32 }`, no `critical` field since this enum has
//! none) — the same claim `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_kind` and `amenable_creusot::
//! ext_jiff::fmt_temporal_time_zone_annotation_kind` both check
//! against jiff's real `From<&str>`/`From<Offset>` API.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationName<'static>`'s model is
//! a trivial `&str` wrapper — the real type's own round trip is a
//! store-and-return-back with no transformation, so the model is the
//! identity function over the borrowed string itself — the same
//! claim `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_name` and `amenable_creusot::
//! ext_jiff::fmt_temporal_time_zone_annotation_name` both check
//! against jiff's real `From<&str>`/`as_str` API.

use super::bridge::{ExtCheckedProof, impl_verus_witness_checked_ext};
use crate::ExtStandard;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    jiff::fmt::friendly::FractionalUnit,
    "verify_fmt_friendly_fractional_unit_from_matches_documented_mapping_model",
    "../../../../amenable_verus/src/jiff/fmt_friendly_fractional_unit.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::friendly::FractionalUnit>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::friendly::FractionalUnit),
        ">"
    ),
    "fractional_unit_model_conversion_holds"
);

// Hand-written, not `impl_verus_witness_checked_ext!`: the model's real
// source now spans three files (`model.rs` + `predicates.rs` +
// `setters.rs` -- `mod.rs` itself is just `mod`/`pub use`, split to
// stay under the modularity line limit), so the `claim` concatenates
// all three rather than the macro's single `include_str!`.
impl Witness<VerusVerifier> for ExtStandard<jiff::fmt::strtime::BrokenDownTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_strtime_broken_down_time_numeric_setters_round_trip_model".to_owned(),
            concat!(
                include_str!(
                    "../../../../amenable_verus/src/jiff/fmt_strtime_broken_down_time/model.rs"
                ),
                include_str!(
                    "../../../../amenable_verus/src/jiff/fmt_strtime_broken_down_time/predicates.rs"
                ),
                include_str!(
                    "../../../../amenable_verus/src/jiff/fmt_strtime_broken_down_time/setters.rs"
                ),
            )
            .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for ExtStandard<jiff::fmt::strtime::BrokenDownTime> {}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        concat!("amenable_ext::ExtStandard<", stringify!(jiff::fmt::strtime::BrokenDownTime), ">"),
        "verus",
        || <ExtStandard<jiff::fmt::strtime::BrokenDownTime> as Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::strtime::BrokenDownTime>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::strtime::BrokenDownTime),
        ">"
    ),
    [
        "broken_down_time_new_matches_new_spec",
        "broken_down_time_year_in_range",
        "broken_down_time_month_in_range",
        "broken_down_time_day_in_range",
        "broken_down_time_day_of_year_in_range",
        "broken_down_time_iso_week_in_range",
        "broken_down_time_week_number_in_range",
        "broken_down_time_hour_in_range",
        "broken_down_time_minute_or_second_in_range",
        "broken_down_time_subsec_nanosecond_in_range",
        "broken_down_time_year_set_or_preserved",
        "broken_down_time_month_set_or_preserved",
        "broken_down_time_day_set_or_preserved",
        "broken_down_time_day_of_year_set_or_preserved",
        "broken_down_time_iso_week_year_set_or_preserved",
        "broken_down_time_iso_week_set_or_preserved",
        "broken_down_time_week_sun_set_or_preserved",
        "broken_down_time_week_mon_set_or_preserved",
        "broken_down_time_hour_set_or_preserved",
        "broken_down_time_minute_set_or_preserved",
        "broken_down_time_second_set_or_preserved",
        "broken_down_time_subsec_nanosecond_set_or_preserved",
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::strtime::Meridiem,
    "verify_fmt_strtime_meridiem_from_time_matches_hour_threshold_model",
    "../../../../amenable_verus/src/jiff/fmt_strtime_meridiem.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::fmt::strtime::Meridiem>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::strtime::Meridiem),
        ">"
    ),
    "meridiem_hour_in_range"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::strtime::Meridiem>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::strtime::Meridiem),
        ">"
    ),
    "meridiem_model_from_hour_holds"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::Pieces<'static>,
    "verify_fmt_temporal_pieces_with_date_with_time_round_trip_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_pieces.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::Pieces<'static>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::Pieces<'static>),
        ">"
    ),
    [
        "pieces_from_date_holds",
        "pieces_with_date_holds",
        "pieces_with_time_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::PiecesNumericOffset,
    "verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_pieces_numeric_offset.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::PiecesNumericOffset),
        ">"
    ),
    "pno_offset_seconds_in_range"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::PiecesNumericOffset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::PiecesNumericOffset),
        ">"
    ),
    [
        "pno_from_offset_seconds_holds",
        "pno_with_negative_zero_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::PiecesOffset,
    "verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_pieces_offset.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::PiecesOffset>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::PiecesOffset),
        ">"
    ),
    [
        "pieces_offset_zulu_holds",
        "pieces_offset_from_offset_seconds_holds",
        "pieces_offset_to_numeric_seconds_holds",
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::TimeZoneAnnotation<'static>,
    "verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_time_zone_annotation.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotation<'static>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::TimeZoneAnnotation<'static>),
        ">"
    ),
    [
        "tz_annotation_from_name_holds",
        "tz_annotation_from_offset_seconds_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::TimeZoneAnnotationKind<'static>,
    "verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_time_zone_annotation_kind.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationKind<'static>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::TimeZoneAnnotationKind<'static>),
        ">"
    ),
    [
        "tz_annotation_kind_from_name_holds",
        "tz_annotation_kind_from_offset_seconds_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::TimeZoneAnnotationName<'static>,
    "verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips_model",
    "../../../../amenable_verus/src/jiff/fmt_temporal_time_zone_annotation_name.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::temporal::TimeZoneAnnotationName<'static>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::temporal::TimeZoneAnnotationName<'static>),
        ">"
    ),
    [
        "tz_annotation_name_from_name_holds",
        "tz_annotation_name_as_str_holds"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::fmt::StdFmtWrite<String>,
    "verify_fmt_std_fmt_write_write_str_model",
    "../../../../amenable_verus/src/jiff/fmt_std_fmt_write.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::StdFmtWrite<String>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::StdFmtWrite<String>),
        ">"
    ),
    "fmt_std_fmt_write_appends_str"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::StdIoWrite<Vec<u8>>,
    "verify_fmt_std_io_write_write_str_model",
    "../../../../amenable_verus/src/jiff/fmt_std_io_write.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::fmt::StdIoWrite<Vec<u8>>),
        ">"
    ),
    "fmt_std_io_write_appends_bytes"
);
