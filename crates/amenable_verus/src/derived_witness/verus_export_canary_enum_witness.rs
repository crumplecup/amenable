//! Derived Verus closure for `amenable_std::verus_derive_canary::composites::VerusExportCanaryEnum<amenable_std::verus_derive_canary::leaves::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::leaves::TrustedVerusExportLeaf>`.

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

verus! {

// evidence: amenable_std::verus_derive_canary::composites::VerusExportCanaryEnum<amenable_std::verus_derive_canary::leaves::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::leaves::TrustedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_canary_enum_witness
// support: mixed (trivial=1, checked=1, trusted=2, opaque=0)

// checked leaf at variant Balanced -> member checked: calls crate::rust_std::char_carrier::verify_char_roundtrip
// trusted leaf at variant Balanced -> member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.
// trusted leaf at variant fallback -> member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.

pub enum VerusExportCanaryEnumWitnessSelector {
    Balanced,
    Fallback,
    Closed,
}

pub enum VerusExportCanaryEnumWitnessResult {
    Balanced(char),
    Fallback,
    Closed,
}

pub open spec fn verus_export_canary_enum_witness_ensures_holds(selector: VerusExportCanaryEnumWitnessSelector, result: VerusExportCanaryEnumWitnessResult, c: char) -> bool {
    match selector {
            VerusExportCanaryEnumWitnessSelector::Balanced => match result {
                VerusExportCanaryEnumWitnessResult::Balanced(r) => char_roundtrip_preserves_value(r, c) && char_is_valid_unicode_scalar(c),
                _ => false,
            },
            VerusExportCanaryEnumWitnessSelector::Fallback => match result {
                VerusExportCanaryEnumWitnessResult::Fallback => true,
                _ => false,
            },
            VerusExportCanaryEnumWitnessSelector::Closed => match result {
                VerusExportCanaryEnumWitnessResult::Closed => true,
                _ => false,
            },
    }
}

pub fn verify_verus_export_canary_enum_witness(selector: VerusExportCanaryEnumWitnessSelector, c: char) -> (result: VerusExportCanaryEnumWitnessResult)
    ensures
        verus_export_canary_enum_witness_ensures_holds(selector, result, c),
{
    match selector {
        VerusExportCanaryEnumWitnessSelector::Balanced => VerusExportCanaryEnumWitnessResult::Balanced(crate::rust_std::char_carrier::verify_char_roundtrip(c)),
        VerusExportCanaryEnumWitnessSelector::Fallback => VerusExportCanaryEnumWitnessResult::Fallback,
        VerusExportCanaryEnumWitnessSelector::Closed => VerusExportCanaryEnumWitnessResult::Closed,
    }
}

} // verus!
