//! Derived Verus closure for `amenable_std::verus_derive_canary::composites::VerusExportNestedStruct`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::escape_ascii_input_is_printable_ascii;
#[cfg(verus_keep_ghost)]
use crate::rust_std::str_and_char::escape_ascii_result_matches_printable_plus_newline_escape;

verus! {

// evidence: amenable_std::verus_derive_canary::composites::VerusExportNestedStruct
// destination: crate::derived_witness::verus_export_nested_struct_witness
// support: mixed (trivial=0, checked=1, trusted=1, opaque=0)

// checked leaf at member inner -> member checked: calls crate::rust_std::str_and_char::verify_escape_ascii_model_leaves_printable_bytes_unescaped
// trusted leaf at member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.

/// Proves `verus_export_nested_struct_witness`'s own composed claim -- see this file's own
/// header comment.
pub fn verify_verus_export_nested_struct_witness(printable: u8) -> (result: (u8, u8, u8))
    requires
        escape_ascii_input_is_printable_ascii(printable),
    ensures
        escape_ascii_result_matches_printable_plus_newline_escape(printable, result),
{
    crate::rust_std::str_and_char::verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable)
}

} // verus!
