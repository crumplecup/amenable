//! Derived Verus closure for `amenable_std::verus_derive_canary::composites::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::leaves::CheckedVerusExportLeaf>`.

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

verus! {

// evidence: amenable_std::verus_derive_canary::composites::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::leaves::CheckedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_checked_plus_trivial_struct_witness
// support: checked (trivial=1, checked=1, trusted=0, opaque=0)

// checked leaf at member checked: calls crate::rust_std::str_and_char::verify_char_roundtrip

/// Proves `verus_export_checked_plus_trivial_struct_witness`'s own composed claim -- see this file's own
/// header comment.
pub fn verify_verus_export_checked_plus_trivial_struct_witness(c: char) -> (result: char)
    ensures
        char_roundtrip_preserves_value(result, c),
        char_is_valid_unicode_scalar(c),
{
    crate::rust_std::str_and_char::verify_char_roundtrip(c)
}

} // verus!
