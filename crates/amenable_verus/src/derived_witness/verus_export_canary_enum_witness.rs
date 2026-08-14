//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportCanaryEnum<amenable_std::verus_derive_canary::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::TrustedVerusExportLeaf>`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportCanaryEnum<amenable_std::verus_derive_canary::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::TrustedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_canary_enum_witness
// support: mixed (trivial=1, checked=1, trusted=2, opaque=0)

pub open spec fn verus_export_canary_enum_witness_variant_balanced_holds(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds: bool, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds: bool) -> bool {
    verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds
        && verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds
}

pub open spec fn verus_export_canary_enum_witness_variant_fallback_holds(verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds: bool) -> bool {
    verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds
}

pub open spec fn verus_export_canary_enum_witness_variant_closed_holds() -> bool {
    true
}

pub open spec fn verus_export_canary_enum_witness_holds(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds: bool, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds: bool, verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds: bool) -> bool {
    verus_export_canary_enum_witness_variant_balanced_holds(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds)
        && verus_export_canary_enum_witness_variant_fallback_holds(verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds)
        && verus_export_canary_enum_witness_variant_closed_holds()
}

// premise verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds: checked leaf at variant Balanced -> member checked; verifier = verus, harness = verify_char_roundtrip
// premise verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds: trusted leaf at variant Balanced -> member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.
// premise verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds: trusted leaf at variant fallback -> member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.
pub proof fn verify_verus_export_canary_enum_witness(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds: bool, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds: bool, verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds: bool)
    requires
        verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds,
        verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds,
        verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds,
    ensures
        verus_export_canary_enum_witness_holds(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds, verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds),
{
    assert(verus_export_canary_enum_witness_holds(verus_export_canary_enum_witness_variant_balanced_member_checked_checked_holds, verus_export_canary_enum_witness_variant_balanced_member_trusted_trusted_holds, verus_export_canary_enum_witness_variant_fallback_member_trusted_trusted_holds));
}

} // verus!
