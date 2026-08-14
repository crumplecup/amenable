//! Derived Verus closure for `amenable_std::verus_derive_canary::VerusExportRawTemplateStruct`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;

verus! {

// evidence: amenable_std::verus_derive_canary::VerusExportRawTemplateStruct
// destination: crate::derived_witness::verus_export_raw_template_struct_witness
// support: checked (trivial=0, checked=1, trusted=0, opaque=0)

// checked leaf at member checked: calls crate::rust_std::ref_cell_carrier::verify_ref_cell_model_dynamic_borrow_rules

pub fn verify_verus_export_raw_template_struct_witness(initial: i32, updated: i32) -> (result: (bool, bool, bool, bool, bool, i32))
    ensures
        result.0,
        !result.1,
        result.2,
        !result.3,
        !result.4,
        observed_value_matches_input(result.5 as int, updated as int),
{
    crate::rust_std::ref_cell_carrier::verify_ref_cell_model_dynamic_borrow_rules(initial, updated)
}

} // verus!
