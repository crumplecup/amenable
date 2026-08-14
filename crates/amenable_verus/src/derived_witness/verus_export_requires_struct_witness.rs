//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportRequiresStruct`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::escape_ascii_carrier::escape_ascii_input_is_printable_ascii;
#[cfg(verus_keep_ghost)]
use crate::rust_std::escape_ascii_carrier::escape_ascii_result_matches_printable_plus_newline_escape;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportRequiresStruct
// destination: crate::derived_witness::verus_export_requires_struct_witness
// support: checked (trivial=0, checked=1, trusted=0, opaque=0)

// checked leaf at member checked: calls crate::rust_std::escape_ascii_carrier::verify_escape_ascii_model_leaves_printable_bytes_unescaped

pub fn verify_verus_export_requires_struct_witness(printable: u8) -> (result: (u8, u8, u8))
    requires
        escape_ascii_input_is_printable_ascii(printable),
    ensures
        escape_ascii_result_matches_printable_plus_newline_escape(printable, result),
{
    crate::rust_std::escape_ascii_carrier::verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable)
}

} // verus!
