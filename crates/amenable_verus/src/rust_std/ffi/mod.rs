mod cstr_carrier;
#[cfg(verus_keep_ghost)]
pub use cstr_carrier::{
    ExCStr, ExFromBytesUntilNulError, ExFromBytesWithNulError, cstr_bytes_contain_a_nul,
    cstr_bytes_contain_no_nul, cstr_bytes_have_an_interior_nul,
    cstr_bytes_have_only_a_trailing_nul, cstr_from_bytes_until_nul_result_matches_nul_presence,
    cstr_from_bytes_with_nul_result_matches_bytes, cstr_to_bytes_matches_model,
    cstr_until_nul_test_inputs_cover_both_cases, non_nul_byte_value_is_nonzero,
};
pub use cstr_carrier::{
    verify_cstr_excludes_the_terminating_nul_from_to_bytes,
    verify_from_bytes_until_nul_requires_a_nul_byte_somewhere,
    verify_from_bytes_with_nul_requires_the_nul_only_at_the_end,
};
mod cstring_carrier;
pub use cstring_carrier::verify_cstring_excludes_the_terminator_and_rejects_interior_nul;
#[cfg(verus_keep_ghost)]
pub use cstring_carrier::{
    ExCString, ExNulError, cstring_input_has_a_preterminal_nul,
    cstring_input_has_no_preterminal_nul, cstring_new_result_matches_input_bytes,
    cstring_test_byte_is_nonzero, into_vec_u8_spec_matches_input_vec,
};
mod from_utf16_error_carrier;
pub use from_utf16_error_carrier::verify_from_utf16_rejects_a_lone_surrogate;
#[cfg(verus_keep_ghost)]
pub use from_utf16_error_carrier::{
    ExFromUtf16Error, from_utf16_case_results_match_accept_reject_triple,
    from_utf16_inputs_cover_valid_and_lone_surrogate_cases,
    from_utf16_result_matches_single_unit_examples,
};
mod from_utf8_error_carrier;
pub use from_utf8_error_carrier::{
    VerusFromUtf8ErrorModel, verify_from_utf8_error_model_recovers_the_original_bytes,
};
#[cfg(verus_keep_ghost)]
pub use from_utf8_error_carrier::{
    from_utf8_error_model_as_bytes_preserves_bytes,
    from_utf8_error_model_into_bytes_preserves_bytes, from_utf8_error_model_new_preserves_bytes,
};
mod from_vec_with_nul_carrier;
pub use from_vec_with_nul_carrier::verify_from_vec_with_nul_requires_the_nul_only_at_the_end;
#[cfg(verus_keep_ghost)]
pub use from_vec_with_nul_carrier::{
    ExFromVecWithNulError, from_vec_with_nul_result_matches_nul_placement,
    from_vec_with_nul_test_byte_is_nonzero,
};
mod into_string_error_carrier;
pub use into_string_error_carrier::verify_into_string_error_recovers_the_original_cstring;
#[cfg(verus_keep_ghost)]
pub use into_string_error_carrier::{
    ExIntoStringError, as_bytes_matches_cstring_bytes_spec,
    into_string_error_recovers_the_original_bytes, into_string_rejects_a_leading_0xff_byte,
    probe_starts_with_0xff_and_second_byte_nonzero,
};
mod std_ffi_carrier;
#[cfg(verus_keep_ghost)]
pub use std_ffi_carrier::os_str_len_fits_the_two_byte_buffer;
pub use std_ffi_carrier::{
    verify_os_str_display_model_renders_valid_utf8_content_unchanged,
    verify_os_str_model_valid_utf8_content_round_trips_through_to_str,
    verify_os_string_model_push_appends_to_the_existing_content,
};
