//! Derived Verus closure for `amenable_std::verus_derive_canary::composites::VerusExportMultiCheckedStruct`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::char_is_valid_unicode_scalar;
#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::char_roundtrip_preserves_value;
#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::escape_ascii_input_is_printable_ascii;
#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::escape_ascii_result_matches_printable_plus_newline_escape;

verus! {

// evidence: amenable_std::verus_derive_canary::composites::VerusExportMultiCheckedStruct
// destination: crate::derived_witness::verus_export_multi_checked_struct_witness
// support: checked (trivial=0, checked=2, trusted=0, opaque=0)

// checked leaf at member first: calls crate::rust_std::str_and_char::verify_char_roundtrip
// checked leaf at member second: calls crate::rust_std::str_and_char::verify_escape_ascii_model_leaves_printable_bytes_unescaped

/// Proves `verus_export_multi_checked_struct_witness`'s own composed claim -- see this file's own
/// header comment.
pub fn verify_verus_export_multi_checked_struct_witness(c: char, printable: u8) -> (result: (char, (u8, u8, u8)))
    requires
        escape_ascii_input_is_printable_ascii(printable),
    ensures
        char_roundtrip_preserves_value(result.0, c),
        char_is_valid_unicode_scalar(c),
        escape_ascii_result_matches_printable_plus_newline_escape(printable, result.1),
{
    (crate::rust_std::str_and_char::verify_char_roundtrip(c), crate::rust_std::str_and_char::verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable))
}

} // verus!
