//! Derived Verus closure for `amenable_std::verus_derive_canary::composites::VerusExportMultiCheckedEnum`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::char_carrier::char_is_valid_unicode_scalar;
#[cfg(verus_keep_ghost)]
use crate::rust_std::char_carrier::char_roundtrip_preserves_value;
#[cfg(verus_keep_ghost)]
use crate::rust_std::escape_ascii_carrier::escape_ascii_input_is_printable_ascii;
#[cfg(verus_keep_ghost)]
use crate::rust_std::escape_ascii_carrier::escape_ascii_result_matches_printable_plus_newline_escape;

verus! {

// evidence: amenable_std::verus_derive_canary::composites::VerusExportMultiCheckedEnum
// destination: crate::derived_witness::verus_export_multi_checked_enum_witness
// support: checked (trivial=1, checked=2, trusted=0, opaque=0)

// checked leaf at variant Active -> member first: calls crate::rust_std::char_carrier::verify_char_roundtrip
// checked leaf at variant Active -> member second: calls crate::rust_std::escape_ascii_carrier::verify_escape_ascii_model_leaves_printable_bytes_unescaped

pub enum VerusExportMultiCheckedEnumWitnessSelector {
    Active,
    Idle,
}

pub enum VerusExportMultiCheckedEnumWitnessResult {
    Active(char, (u8, u8, u8)),
    Idle,
}

pub open spec fn verus_export_multi_checked_enum_witness_ensures_holds(selector: VerusExportMultiCheckedEnumWitnessSelector, result: VerusExportMultiCheckedEnumWitnessResult, c: char, printable: u8) -> bool {
    match selector {
            VerusExportMultiCheckedEnumWitnessSelector::Active => match result {
                VerusExportMultiCheckedEnumWitnessResult::Active(r0, r1) => char_roundtrip_preserves_value(r0, c) && char_is_valid_unicode_scalar(c) && escape_ascii_result_matches_printable_plus_newline_escape(printable, r1),
                _ => false,
            },
            VerusExportMultiCheckedEnumWitnessSelector::Idle => match result {
                VerusExportMultiCheckedEnumWitnessResult::Idle => true,
                _ => false,
            },
    }
}

pub open spec fn verus_export_multi_checked_enum_witness_requires_holds(selector: VerusExportMultiCheckedEnumWitnessSelector, c: char, printable: u8) -> bool {
    match selector {
            VerusExportMultiCheckedEnumWitnessSelector::Active => escape_ascii_input_is_printable_ascii(printable),
            VerusExportMultiCheckedEnumWitnessSelector::Idle => true,
    }
}

pub fn verify_verus_export_multi_checked_enum_witness(selector: VerusExportMultiCheckedEnumWitnessSelector, c: char, printable: u8) -> (result: VerusExportMultiCheckedEnumWitnessResult)
    requires
        verus_export_multi_checked_enum_witness_requires_holds(selector, c, printable),
    ensures
        verus_export_multi_checked_enum_witness_ensures_holds(selector, result, c, printable),
{
    match selector {
        VerusExportMultiCheckedEnumWitnessSelector::Active => VerusExportMultiCheckedEnumWitnessResult::Active(crate::rust_std::char_carrier::verify_char_roundtrip(c), crate::rust_std::escape_ascii_carrier::verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable)),
        VerusExportMultiCheckedEnumWitnessSelector::Idle => VerusExportMultiCheckedEnumWitnessResult::Idle,
    }
}

} // verus!
