//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf>`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportCheckedPlusTrivialStruct<amenable_std::verus_derive_canary::CheckedVerusExportLeaf>
// destination: crate::derived_witness::verus_export_checked_plus_trivial_struct_witness
// support: checked (trivial=1, checked=1, trusted=0, opaque=0)

pub open spec fn verus_export_checked_plus_trivial_struct_witness_member_marker_holds() -> bool {
    true
}

pub open spec fn verus_export_checked_plus_trivial_struct_witness_holds(verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds: bool) -> bool {
    verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds
        && verus_export_checked_plus_trivial_struct_witness_member_marker_holds()
}

// premise verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds: checked leaf at member checked; verifier = verus, harness = verify_char_roundtrip
pub proof fn verify_verus_export_checked_plus_trivial_struct_witness(verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds: bool)
    requires
        verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds,
    ensures
        verus_export_checked_plus_trivial_struct_witness_holds(verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds),
{
    assert(verus_export_checked_plus_trivial_struct_witness_holds(verus_export_checked_plus_trivial_struct_witness_member_checked_checked_holds));
}

} // verus!
