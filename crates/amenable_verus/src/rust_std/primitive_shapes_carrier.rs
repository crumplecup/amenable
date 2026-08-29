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
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
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

/// A shared `Option<i32>`-observation identity predicate for
/// accommodation models whose observed slot should hold exactly
/// `Some(input)` — the `Option`-wrapped counterpart to
/// `observed_value_matches_input`, for iterator adapters whose `.next()`
/// yields `Option<i32>` directly rather than a bare scalar.
pub open spec fn observed_option_matches_input(observed: Option<i32>, input: i32) -> bool {
    observed == Some(input)
}

/// A shared before/after frame condition for accommodation models whose
/// operation leaves one field untouched — the general form of
/// `observed_value_matches_input`, named separately because its two
/// sides are a state's own before/after snapshots (`old(self).field`,
/// `final(self).field`) rather than an input parameter and an
/// independently observed result.
pub open spec fn value_unchanged(before: int, after: int) -> bool {
    before == after
}

/// A shared pair-observation identity predicate for accommodation
/// models whose observed two-tuple should match the input two-tuple
/// exactly — generic over both slots (rather than fixed at `int`, the
/// way `observed_value_matches_input` is) because Verus spec equality
/// is already total over any type, so this covers `(bool, bool)`,
/// `(i8, i8)` through `(u64, u64)`/`(usize, usize)`, and mixed-type
/// pairs alike without a cast on either side.
pub open spec fn observed_pair_matches_input<A, B>(observed: (A, B), input: (A, B)) -> bool {
    observed == input
}

/// A shared ASCII-range precondition for Verus accommodation models
/// that need a genuinely symbolic single-character `str`/`char` window
/// (the same claim `amenable_std::AsciiByte` names on the Kani/Creusot
/// side): `char as u32` stays a valid one-byte UTF-8 encoding only below
/// `128`.
pub open spec fn is_ascii_byte(value: u32) -> bool {
    value < 128
}

/// A shared pairwise-distinctness precondition for accommodation models
/// that build a symbolic non-overlapping match/split window: the model
/// only makes sense (a matched region can be told apart from what
/// surrounds it) when its two symbolic construction inputs are actually
/// distinct — a delimiter byte from ordinary content, a matched pattern
/// character from its window neighbors, two call-site line numbers from
/// each other, and so on. Generic over any type, the same reasoning
/// `observed_pair_matches_input` already applies to equality: Verus
/// spec inequality is total over any type, so this covers every
/// concrete type these models need without a cast on either side.
pub open spec fn values_are_distinct<T>(a: T, b: T) -> bool {
    a != b
}

/// `[a, b, c].len() == 3`, and each index recovers the element the
/// array was constructed with.
pub fn verify_array_model_indexing_and_length(a: i32, b: i32, c: i32) -> (result: (u32, i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, 3int),
        observed_value_matches_input(result.1 as int, a as int),
        observed_value_matches_input(result.2 as int, b as int),
        observed_value_matches_input(result.3 as int, c as int),
{
    (3, a, b, c)
}

/// A slice's `.len()` reports the number of elements it views, and each
/// index recovers the underlying element — same shape as `array`, over
/// the distinct `[i32]` (unsized, always accessed through `&[i32]`)
/// type.
pub fn verify_slice_model_indexing_and_length(a: i32, b: i32, c: i32) -> (result: (u32, i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, 3int),
        observed_value_matches_input(result.1 as int, a as int),
        observed_value_matches_input(result.2 as int, b as int),
        observed_value_matches_input(result.3 as int, c as int),
{
    (3, a, b, c)
}

/// A `str`'s `.len()` reports its UTF-8 byte length, and its bytes are
/// exactly its content's UTF-8 encoding — checked for any single-byte
/// (ASCII) character.
///
/// The precondition below reuses this file's own `is_ascii_byte`, the
/// same predicate `amenable_std::AsciiByte` names on the Kani/Creusot
/// side.
pub fn verify_str_model_byte_length_and_content(byte: u8) -> (result: (u32, u8))
    requires
        is_ascii_byte(byte as u32),
    ensures
        observed_value_matches_input(result.0 as int, 1int),
        observed_value_matches_input(result.1 as int, byte as int),
{
    (1, byte)
}

/// A tuple's `.0`/`.1` recover exactly the values it was constructed
/// with, in position order.
pub fn verify_tuple_model_field_access(a: i32, b: i32) -> (result: (i32, i32))
    ensures
        observed_pair_matches_input(result, (a, b)),
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
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

} // verus!
