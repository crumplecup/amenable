//! Verus spec for `core::alloc::Layout`.
//!
//! Not the size/align half of Kani's claim (`Layout::new::<i32>()`
//! reports size 4, align 4) — genuinely unprovable under Verus, not a
//! convenience shortcut: `vstd::layout` deliberately treats
//! `size_of::<V>()`/`align_of::<V>()` as fully opaque (`uninterp spec
//! fn`, no axiom pinning any concrete type, primitives included, to a
//! known value), with its own doc comment explaining why ("we are NOT
//! creating an axiom that size_of fits in usize"). See
//! `amenable_std::verus_gallery`'s `layout_new_size_and_align_are_
//! opaque_even_for_primitives` finding.
//!
//! The alignment-rejection half is independent of that opacity — a pure
//! combinatorial fact about `from_size_align`'s own validation, not
//! about any type's real layout — and is proved here in full.

use std::alloc::Layout;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExLayoutError(std::alloc::LayoutError);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExLayout(Layout);

pub open spec fn is_power_of_two_spec(value: usize) -> bool {
    value > 0 && (value & ((value - 1) as usize)) == 0
}

#[verifier::when_used_as_spec(is_power_of_two_spec)]
pub assume_specification [usize::is_power_of_two] (value: usize) -> (result: bool)
    ensures
        result == is_power_of_two_spec(value),
;

pub assume_specification [Layout::from_size_align] (size: usize, align: usize) -> (result: Result<Layout, std::alloc::LayoutError>)
    ensures
        !is_power_of_two_spec(align) ==> result is Err,
;

/// `Layout::from_size_align` rejects an alignment that isn't a power of
/// two — the same claim the Kani harness checks (the size/align-value
/// half of that harness isn't provable here; see the module doc
/// comment). Takes `size`/`align` as `requires`-constrained parameters
/// (the same "parameter, not inline literal" shape as
/// `option_carrier.rs`) rather than the literal `(4, 3)`, which `verus`
/// doesn't connect to `is_power_of_two_spec` automatically.
pub fn verify_layout_from_size_align_rejects_a_non_power_of_two_alignment(size: usize, align: usize) -> (result: bool)
    requires
        !is_power_of_two_spec(align),
    ensures
        result,
{
    Layout::from_size_align(size, align).is_err()
}

} // verus!
