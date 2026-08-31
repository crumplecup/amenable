mod ascii_escape_carrier;
pub use ascii_escape_carrier::verify_escape_default_model_escapes_a_control_byte;
mod char_carrier;
pub use char_carrier::verify_char_roundtrip;
#[cfg(verus_keep_ghost)]
pub use char_carrier::{char_is_valid_unicode_scalar, char_roundtrip_preserves_value};
mod char_transform_carrier;
pub use char_transform_carrier::{
    verify_char_escape_debug_model_escapes_a_newline,
    verify_char_escape_default_model_escapes_a_newline,
    verify_char_escape_unicode_model_renders_the_codepoint_escape,
    verify_to_lowercase_model_maps_an_uppercase_ascii_letter,
    verify_to_uppercase_model_maps_a_lowercase_ascii_letter,
};
mod char_try_from_carrier;
#[cfg(verus_keep_ghost)]
pub use char_try_from_carrier::{
    ExCharTryFromError, ExTryFromCharError, char_fits_in_u8,
    char_try_from_u32_rejects_invalid_scalar, char_try_from_u32_succeeds_with_same_scalar,
    u8_try_from_char_rejects_out_of_range_scalar, u8_try_from_char_succeeds_with_same_scalar,
    u32_is_valid_unicode_scalar,
};
pub use char_try_from_carrier::{
    verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range,
    verify_try_from_char_error_occurs_exactly_when_out_of_range,
};
mod chars_carrier;
pub use chars_carrier::verify_chars_yields_characters_in_order;
#[cfg(verus_keep_ghost)]
pub use chars_carrier::{chars_input_is_ab, chars_iteration_yields_a_then_b_then_none};
mod decode_utf16_carrier;
pub use decode_utf16_carrier::{
    VerusDecodeUtf16Model, verify_decode_utf16_model_round_trips_and_reports_lone_surrogates,
};
#[cfg(verus_keep_ghost)]
pub use decode_utf16_carrier::{
    decode_utf16_bmp_unit_decodes_to_same_scalar, decode_utf16_lone_surrogate_reports_same_unit,
    decode_utf16_test_inputs_cover_both_cases, decode_utf16_unit_is_non_surrogate,
    decode_utf16_unit_is_surrogate,
};
mod escape_ascii_carrier;
pub use escape_ascii_carrier::verify_escape_ascii_model_leaves_printable_bytes_unescaped;
#[cfg(verus_keep_ghost)]
pub use escape_ascii_carrier::{
    escape_ascii_input_is_printable_ascii,
    escape_ascii_result_matches_printable_plus_newline_escape,
};
mod parse_char_error_carrier;
pub use parse_char_error_carrier::verify_parse_char_error_occurs_for_empty_or_multi_character_strings;
#[cfg(verus_keep_ghost)]
pub use parse_char_error_carrier::{ExParseCharError, char_from_str_result_matches};
mod str_ascii_iter_carrier;
#[cfg(verus_keep_ghost)]
pub use str_ascii_iter_carrier::{char_indices_first_offset_is_zero, numeric_cast_matches_char};
pub use str_ascii_iter_carrier::{
    verify_bytes_model_yields_the_utf8_encoding,
    verify_char_indices_model_pairs_each_char_with_its_byte_offset,
    verify_encode_utf16_model_yields_utf16_code_units,
};
mod str_escape_carrier;
pub use str_escape_carrier::{
    verify_str_escape_debug_model_escapes_control_characters,
    verify_str_escape_default_model_escapes_control_characters,
    verify_str_escape_unicode_model_renders_the_codepoint_escape,
};
mod str_lines_carrier;
pub use str_lines_carrier::{
    verify_lines_any_model_splits_on_any_line_ending, verify_lines_model_splits_on_line_endings,
};
mod str_pattern_match_carrier;
pub use str_pattern_match_carrier::{
    verify_str_match_indices_model_pairs_each_match_with_its_byte_offset,
    verify_str_matches_model_yields_every_non_overlapping_occurrence,
    verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back,
};
mod str_pattern_reverse_carrier;
pub use str_pattern_reverse_carrier::{
    verify_str_rsplit_model_yields_substrings_from_the_back,
    verify_str_rsplitn_model_limits_to_n_substrings_from_the_back,
};
mod str_pattern_split_carrier;
pub use str_pattern_split_carrier::{
    verify_str_split_inclusive_model_keeps_the_delimiter_attached,
    verify_str_split_model_yields_substrings_between_pattern_matches,
    verify_str_splitn_model_limits_to_n_substrings,
};
mod str_pattern_terminator_carrier;
pub use str_pattern_terminator_carrier::{
    verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back,
    verify_str_split_terminator_model_suppresses_a_trailing_empty_substring,
};
mod str_utf8_chunks_carrier;
#[cfg(verus_keep_ghost)]
pub use str_utf8_chunks_carrier::{
    invalid_byte_is_never_a_valid_utf8_lead_byte, utf8_chunk_invalid_byte_is_0xff,
    utf8_error_reports_length_and_span,
};
pub use str_utf8_chunks_carrier::{
    verify_utf8_chunk_model_separates_the_valid_prefix_from_invalid_bytes,
    verify_utf8_chunks_model_yields_one_chunk_for_wholly_valid_input,
    verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span,
};
mod str_whitespace_carrier;
pub use str_whitespace_carrier::{
    verify_split_ascii_whitespace_model_collapses_runs_of_whitespace,
    verify_split_whitespace_model_collapses_runs_of_whitespace,
};
mod string_carrier;
#[cfg(verus_keep_ghost)]
pub use string_carrier::string_roundtrip_result_matches;
pub use string_carrier::verify_string_roundtrip;
