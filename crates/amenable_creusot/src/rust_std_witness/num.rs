use super::CheckedProof;

use std::num::{NonZero, Saturating, Wrapping};

use crate::{
    CreusotVerifier, CreusotWitness, FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_HOLDS_SRC,
    INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_HOLDS_SRC, NONZERO_I16_GET_ROUND_TRIPS_SRC,
    NONZERO_I16_NEW_SUCCEEDS_EXACTLY_WHEN_NONZERO_SRC,
    PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_HOLDS_SRC,
    PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_HOLDS_SRC, SATURATING_I32_ADD_CLAMPS_HOLDS_SRC,
    TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_HOLDS_SRC,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
    VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
    VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
    VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
    VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, WRAPPING_I32_ADD_WRAPS_HOLDS_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i16_roundtrips".to_string(),
            VERIFY_NONZERO_I16_ROUNDTRIPS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        "creusot",
        || <RustStdStandard<NonZero<i16>> as CreusotWitness>::proof().to_string(),
    )
}

/// `#[trusted]`'s two `#[ensures]` clauses each get their own
/// `ContractRecord` here — the same bound `Ensures<KaniVerifier>`
/// (`amenable_kani::rust_std::num`, `value != 0`) and
/// `Ensures<VerusVerifier>` (`amenable_std::verus_witness`, split into
/// `value != 0 ==> result`/`value == 0 ==> !result`) already name, restated
/// once more in Creusot's own `match`-expression form -- each returns its
/// own named predicate's `harness!`-captured source directly, not a
/// hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<NonZero<i16>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        NONZERO_I16_NEW_SUCCEEDS_EXACTLY_WHEN_NONZERO_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        "creusot",
        "ensures",
        || <RustStdStandard<NonZero<i16>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        "creusot",
        "ensures",
        || NONZERO_I16_GET_ROUND_TRIPS_SRC,
    )
}

impl CreusotWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_wrapping_i32_add_wraps".to_string(),
            VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        "creusot",
        || <RustStdStandard<Wrapping<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::WRAPPING_I32_ADD_WRAPS_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn wrapping_i32_add_wraps_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Wrapping<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        WRAPPING_I32_ADD_WRAPS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Wrapping<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_saturating_i32_add_clamps".to_string(),
            VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        "creusot",
        || <RustStdStandard<Saturating<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::SATURATING_I32_ADD_CLAMPS_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn saturating_i32_add_clamps_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Saturating<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SATURATING_I32_ADD_CLAMPS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Saturating<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::IntErrorKind, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::IntErrorKind>`).
impl CreusotWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_int_error_kind_classifies_parse_failures".to_string(),
            VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "creusot",
        || <RustStdStandard<core::num::IntErrorKind> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic] fn int_error_kind_classifies_parse_failures_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<core::num::IntErrorKind> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "creusot",
        "ensures",
        || <RustStdStandard<core::num::IntErrorKind> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::TryFromIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::TryFromIntError>`).
impl CreusotWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_from_int_error_occurs_exactly_when_out_of_range".to_string(),
            VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "creusot",
        || {
            <RustStdStandard<core::num::TryFromIntError> as CreusotWitness>::proof().to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// try_from_int_error_occurs_exactly_when_out_of_range_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<core::num::TryFromIntError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<core::num::TryFromIntError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::ParseIntError>`).
impl CreusotWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_int_error_reports_the_kind_of_the_failure".to_string(),
            VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "creusot",
        || {
            <RustStdStandard<core::num::ParseIntError> as CreusotWitness>::proof().to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic] fn parse_int_error_reports_the_kind_of_the_failure_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<core::num::ParseIntError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<core::num::ParseIntError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::FpCategory, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::FpCategory>`).
//
// `#[trusted]`, like `NonZero<i16>` above: `f64` has no `View` impl in
// `creusot-std`, and a bare float literal in Pearlite panics
// `creusot-rustc` outright — both confirmed real blockers, not a
// convenience shortcut; see `amenable_std::creusot_gallery`'s
// `f64_has_no_view_impl_at_all`/`float_literals_in_pearlite_ice_the_compiler`
// findings.
impl CreusotWitness for RustStdStandard<core::num::FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fp_category_matches_the_value_it_classifies".to_string(),
            VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "creusot",
        || {
            <RustStdStandard<core::num::FpCategory> as CreusotWitness>::proof().to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn fp_category_matches_the_value_it_classifies_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<core::num::FpCategory> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "creusot",
        "ensures",
        || <RustStdStandard<core::num::FpCategory> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseFloatError, ...)`, confirmed against the checklist's
// own `evidence_name` column: `RustStdStandard<core::num::ParseFloatError>`).
//
// `#[trusted]`: a real extern_spec for `FromStr for f64` translates
// cleanly but `why3find prove` doesn't discharge the harness's own goal
// against it — confirmed reproducible, not a convenience shortcut; see
// `amenable_std::creusot_gallery`'s
// `parse_float_error_extern_spec_translates_but_wont_discharge` finding.
impl CreusotWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_float_error_occurs_only_for_unparseable_input".to_string(),
            VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "creusot",
        || {
            <RustStdStandard<core::num::ParseFloatError> as CreusotWitness>::proof().to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// parse_float_error_occurs_only_for_unparseable_input_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<core::num::ParseFloatError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<core::num::ParseFloatError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
