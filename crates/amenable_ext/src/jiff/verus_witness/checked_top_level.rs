//! Checked top-level `jiff::*` types -- real, hand-verified Verus
//! accommodation models (see each paragraph below for what each
//! model covers).

use super::bridge::{ExtCheckedProof, impl_verus_witness_checked_ext};
use crate::ExtStandard;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_checked_ext!(
    jiff::Error,
    "verify_error_classification_predicates_are_mutually_exclusive",
    "../../../../amenable_verus/src/jiff/error.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::Error>,
    concat!("amenable_ext::ExtStandard<", stringify!(jiff::Error), ">"),
    "error_classification_predicates_are_mutually_exclusive"
);

impl_verus_witness_checked_ext!(
    jiff::SignedDuration,
    "verify_signed_duration_new_model_normalizes_nanos_and_carries_into_secs",
    "../../../../amenable_verus/src/jiff/signed_duration.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::SignedDuration>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::SignedDuration),
        ">"
    ),
    "signed_duration_new_secs_headroom_holds"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::SignedDuration>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::SignedDuration),
        ">"
    ),
    "signed_duration_new_model_normalizes"
);

impl_verus_witness_checked_ext!(
    jiff::Span,
    "verify_span_unit_setters_model_round_trips",
    "../../../../amenable_verus/src/jiff/span.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::Span>,
    concat!("amenable_ext::ExtStandard<", stringify!(jiff::Span), ">"),
    "span_unit_setters_model_round_trip_holds"
);

impl_verus_witness_checked_ext!(
    jiff::SpanFieldwise,
    "verify_span_fieldwise_negation_model_negates_every_unit_getter",
    "../../../../amenable_verus/src/jiff/span_fieldwise.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::SpanFieldwise>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::SpanFieldwise),
        ">"
    ),
    "span_fieldwise_negation_model_holds"
);

impl_verus_witness_checked_ext!(
    jiff::TimestampSeries,
    "verify_timestamp_series_next_yields_start_then_advances_by_period",
    "../../../../amenable_verus/src/jiff/timestamp_series.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::TimestampSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::TimestampSeries),
        ">"
    ),
    [
        "timestamp_series_start_seconds_in_range",
        "timestamp_series_period_seconds_in_range"
    ]
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::TimestampSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::TimestampSeries),
        ">"
    ),
    "timestamp_series_next_yields_start_then_advances_by_period_holds"
);

impl_verus_witness_checked_ext!(
    jiff::Unit,
    "verify_unit_ordering_matches_discriminant_order_model",
    "../../../../amenable_verus/src/jiff/unit.rs"
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::Unit>,
    concat!("amenable_ext::ExtStandard<", stringify!(jiff::Unit), ">"),
    [
        "unit_model_ordering_holds",
        "unit_model_discriminant_exec_matches"
    ]
);

impl_verus_witness_checked_ext!(
    jiff::ZonedSeries,
    "verify_zoned_series_next_yields_start_then_advances_by_period_under_utc",
    "../../../../amenable_verus/src/jiff/zoned_series.rs"
);

amenable_derive::verus_requires_predicate!(
    ExtStandard<jiff::ZonedSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::ZonedSeries),
        ">"
    ),
    [
        "zoned_series_start_seconds_in_range",
        "zoned_series_period_seconds_in_range"
    ]
);

amenable_derive::verus_ensures_predicate!(
    ExtStandard<jiff::ZonedSeries>,
    concat!(
        "amenable_ext::ExtStandard<",
        stringify!(jiff::ZonedSeries),
        ">"
    ),
    "zoned_series_next_yields_start_then_advances_by_period_under_utc_holds"
);
