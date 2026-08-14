//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportTupleStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::TrustedVerusExportLeaf>`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportTupleStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf, amenable_std::verus_derive_canary::TrustedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_tuple_struct_witness
// support: mixed (trivial=1, checked=1, trusted=1, opaque=0)

pub open spec fn verus_export_tuple_struct_witness_member_marker_holds() -> bool {
    true
}

pub open spec fn verus_export_tuple_struct_witness_holds(verus_export_tuple_struct_witness_member_0_checked_holds: bool, verus_export_tuple_struct_witness_member_trusted_trusted_holds: bool) -> bool {
    verus_export_tuple_struct_witness_member_0_checked_holds
        && verus_export_tuple_struct_witness_member_trusted_trusted_holds
        && verus_export_tuple_struct_witness_member_marker_holds()
}

// premise verus_export_tuple_struct_witness_member_0_checked_holds: checked leaf at member 0; verifier = verus, harness = verify_char_roundtrip
// premise verus_export_tuple_struct_witness_member_trusted_trusted_holds: trusted leaf at member trusted; rust.authority_kind = external_standard, rust.authority = Rust Project Developers, rust.source_crate = core, rust.source_module = core::primitive, source_url = https://doc.rust-lang.org/std/primitive.bool.html, type_name = bool, semantic_summary = The boolean carrier admits exactly the truth values false and true.
pub proof fn verify_verus_export_tuple_struct_witness(verus_export_tuple_struct_witness_member_0_checked_holds: bool, verus_export_tuple_struct_witness_member_trusted_trusted_holds: bool)
    requires
        verus_export_tuple_struct_witness_member_0_checked_holds,
        verus_export_tuple_struct_witness_member_trusted_trusted_holds,
    ensures
        verus_export_tuple_struct_witness_holds(verus_export_tuple_struct_witness_member_0_checked_holds, verus_export_tuple_struct_witness_member_trusted_trusted_holds),
{
    assert(verus_export_tuple_struct_witness_holds(verus_export_tuple_struct_witness_member_0_checked_holds, verus_export_tuple_struct_witness_member_trusted_trusted_holds));
}

} // verus!
