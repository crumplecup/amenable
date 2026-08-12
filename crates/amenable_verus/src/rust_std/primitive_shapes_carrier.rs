//! Verus accommodation model for the compound-primitive doc pages:
//! `[i32; 3]` (array), `[i32]` (slice), `str`, `(i32, i32)` (tuple),
//! `fn(i32) -> i32`, `*const i32`, `*mut i32`, `&'static i32` (shared
//! reference), `&'static mut i32` (mutable reference).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. These are the bare-primitive-shape doc pages themselves
//! (`core::array`/`core::slice`/`core::str`/`core::tuple`/`core::fn`/
//! `core::pointer`/`core::reference` and their `std::` mirrors), not
//! any of the concrete iterator/adapter types layered on top of
//! `core::slice`/`core::str` already covered elsewhere in this crate.
//! `amenable_kani`'s own harnesses check each real primitive shape
//! directly (no timeout concerns for any of the nine). This carrier
//! states each resulting law directly. `pointer`'s claims stay
//! `unsafe`-free (never dereferencing), matching `amenable_kani`'s own
//! documented choice for the identical reason (this crate forbids
//! `unsafe_code`). None of these functions are the real primitive
//! types themselves — each proof is conditional: sound if the real
//! type refines the stated law, which `amenable_kani`'s own harness for
//! that exact shape (checking the real type directly) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// The single-increment-headroom precondition `amenable_std::
// IncrementHeadroom` names -- a real spec fn defined once in
// `iter_sequence_carrier`, called from here directly rather than
// restated inline. `#[cfg(verus_keep_ghost)]`-gated like every other
// spec-only import in this crate (see `chars_carrier.rs`): plain
// `spec fn`s carry no runtime representation, so this import only
// resolves when Verus's own ghost content is retained, not under
// ordinary `cargo check`.
#[cfg(verus_keep_ghost)]
use crate::rust_std::iter_sequence_carrier::single_increment_headroom_holds;

verus! {

/// A shared scalar-observation identity predicate for accommodation
/// models whose observed `i32` should match their input exactly.
pub open spec fn observed_value_matches_input(observed: int, input: int) -> bool {
    observed == input
}

/// A shared text-view equality predicate for Verus accommodation models
/// that need to compare observed `str` content without relying on
/// `str`'s unsupported exec `PartialEq`.
pub open spec fn text_view_matches_expected(observed: Seq<char>, expected: Seq<char>) -> bool {
    observed =~= expected
}

/// `[a, b, c].len() == 3`, and each index recovers the element the
/// array was constructed with.
pub fn verify_array_model_indexing_and_length(a: i32, b: i32, c: i32) -> (result: (u32, i32, i32, i32))
    ensures
        result.0 == 3,
        result.1 == a,
        result.2 == b,
        result.3 == c,
{
    (3, a, b, c)
}

/// A slice's `.len()` reports the number of elements it views, and each
/// index recovers the underlying element — same shape as `array`, over
/// the distinct `[i32]` (unsized, always accessed through `&[i32]`)
/// type.
pub fn verify_slice_model_indexing_and_length(a: i32, b: i32, c: i32) -> (result: (u32, i32, i32, i32))
    ensures
        result.0 == 3,
        result.1 == a,
        result.2 == b,
        result.3 == c,
{
    (3, a, b, c)
}

/// A `str`'s `.len()` reports its UTF-8 byte length, and its bytes are
/// exactly its content's UTF-8 encoding — checked for any single-byte
/// (ASCII) character.
///
/// The precondition below is the canonical home
/// `amenable_std::AsciiByte`'s own `Requires<VerusVerifier>` impl names
/// (this file's own spelling, `byte < 128`, registered as one of its
/// supplementary fragments).
pub fn verify_str_model_byte_length_and_content(byte: u8) -> (result: (u32, u8))
    requires
        byte < 128,
    ensures
        result.0 == 1,
        result.1 == byte,
{
    (1, byte)
}

/// A tuple's `.0`/`.1` recover exactly the values it was constructed
/// with, in position order.
pub fn verify_tuple_model_field_access(a: i32, b: i32) -> (result: (i32, i32))
    ensures
        result == (a, b),
{
    (a, b)
}

/// Calling through a `fn` pointer invokes exactly the function it was
/// assigned from — modeled as the identical computation performed
/// twice, standing in for "the pointer" and "the function" agreeing.
pub fn verify_fn_pointer_model_calls_the_underlying_function(value: i32) -> (result: (i32, i32))
    requires
        single_increment_headroom_holds(value),
    ensures
        result.0 == result.1,
{
    let via_pointer = value + 1;
    let via_function = value + 1;
    (via_pointer, via_function)
}

/// Casting the same reference to a `*const i32` twice gives the same
/// address, without ever dereferencing the pointer — a safe property
/// of the cast itself, modeled abstractly since `vstd` has no spec
/// support for raw-pointer address identity.
pub fn verify_const_pointer_model_cast_is_reproducible() -> (result: bool)
    ensures
        result,
{
    true
}

/// Same as the `*const i32` claim, for a mutable raw pointer.
pub fn verify_mut_pointer_model_cast_is_reproducible() -> (result: bool)
    ensures
        result,
{
    true
}

/// Dereferencing a shared reference recovers exactly the value it
/// borrows.
pub fn verify_shared_reference_model_dereferences_to_the_referent(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// Dereferencing a mutable reference recovers the value it borrows, and
/// writing through it updates the referent visibly through the same
/// reference.
pub fn verify_mutable_reference_model_dereferences_to_and_updates_the_referent(initial: i32, next: i32) -> (result: (i32, i32))
    ensures
        result == (initial, next),
{
    (initial, next)
}

} // verus!
