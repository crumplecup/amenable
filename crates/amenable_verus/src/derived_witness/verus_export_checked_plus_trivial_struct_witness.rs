//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf>`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::char_carrier::char_is_valid_unicode_scalar;
#[cfg(verus_keep_ghost)]
use crate::rust_std::char_carrier::char_roundtrip_preserves_value;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_checked_plus_trivial_struct_witness
// support: checked (trivial=1, checked=1, trusted=0, opaque=0)

// checked leaf at member checked: calls crate::rust_std::char_carrier::verify_char_roundtrip

pub fn verify_verus_export_checked_plus_trivial_struct_witness(c: char) -> (result: char)
    ensures
        char_roundtrip_preserves_value(result, c),
        char_is_valid_unicode_scalar(c),
{
    crate::rust_std::char_carrier::verify_char_roundtrip(c)
}

} // verus!
