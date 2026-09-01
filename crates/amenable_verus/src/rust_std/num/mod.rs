mod fp_category_carrier;
pub use fp_category_carrier::verify_fp_category_matches_the_value_it_classifies;
mod int_error_kind_carrier;
pub use int_error_kind_carrier::{
    verify_int_error_kind_classifies_parse_failures,
    verify_parse_int_error_model_reports_the_kind_of_the_failure,
};
mod non_zero_carrier;
pub use non_zero_carrier::{
    verify_non_zero_i8_model_round_trips_iff_nonzero,
    verify_non_zero_i16_model_round_trips_iff_nonzero,
    verify_non_zero_i32_model_round_trips_iff_nonzero,
    verify_non_zero_i64_model_round_trips_iff_nonzero,
    verify_non_zero_i128_model_round_trips_iff_nonzero,
    verify_non_zero_isize_model_round_trips_iff_nonzero,
    verify_non_zero_u8_model_round_trips_iff_nonzero,
    verify_non_zero_u16_model_round_trips_iff_nonzero,
    verify_non_zero_u32_model_round_trips_iff_nonzero,
    verify_non_zero_u64_model_round_trips_iff_nonzero,
    verify_non_zero_u128_model_round_trips_iff_nonzero,
    verify_non_zero_usize_model_round_trips_iff_nonzero,
};
mod ordering_carrier;
pub use ordering_carrier::verify_ordering_reverse_swaps_less_and_greater;
mod parse_float_error_carrier;
pub use parse_float_error_carrier::verify_parse_float_error_occurs_only_for_unparseable_input;
mod reverse_carrier;
pub use reverse_carrier::{ExReverse, verify_reverse_field_roundtrips_the_constructed_value};
mod saturating_carrier;
pub use saturating_carrier::{
    ExSaturating, verify_saturating_field_roundtrips_the_constructed_value,
};
mod try_from_int_error_carrier;
pub use try_from_int_error_carrier::verify_try_from_int_error_occurs_exactly_when_out_of_range;
mod try_from_slice_carrier;
pub use try_from_slice_carrier::verify_try_from_slice_rejects_a_length_mismatch;
mod wrapping_carrier;
pub use wrapping_carrier::{ExWrapping, verify_wrapping_field_roundtrips_the_constructed_value};

/// Ghost/spec-only re-exports, one `#[cfg(verus_keep_ghost)]` gate on this
/// `mod` instead of scattered per-carrier ones -- see `misc::mod`'s own
/// doc comment for the full rationale.
#[cfg(verus_keep_ghost)]
mod ghost_reexports {
    pub use super::fp_category_carrier::{
        ExFpCategory, fp_category_classify_result_matches_special_value_categories,
        fp_category_inputs_cover_nan_and_infinite_cases,
        fp_category_results_match_nan_and_infinite_cases,
    };
    pub use super::int_error_kind_carrier::{
        ExIntErrorKind, ExParseIntError, from_str_empty_reports_empty_kind,
        from_str_lowercase_reports_invalid_digit_kind, parse_int_error_kind_matches,
        starts_with_lowercase_ascii_letter,
    };
    pub use super::non_zero_carrier::{non_zero_new_accepts_nonzero, non_zero_new_rejects_zero};
    pub use super::ordering_carrier::ordering_reverse_swaps_less_and_greater;
    pub use super::parse_float_error_carrier::{
        ExParseFloatError, parse_float_examples_match_expected_outcome,
    };
    pub use super::try_from_slice_carrier::{ExTryFromSliceError, try_from_slice_result_matches};
}
#[cfg(verus_keep_ghost)]
pub use ghost_reexports::*;
