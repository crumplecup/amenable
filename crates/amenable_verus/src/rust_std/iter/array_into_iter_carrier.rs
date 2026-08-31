//! Verus accommodation model for `core::array::IntoIter<i32, 3>`.
//!
//! `vstd` gives array *values* (`[T; N]`) real spec support
//! (`vstd::array`, used directly by `amenable_verus`'s own module doc
//! comment), but not the owned `array::IntoIter<T, N>` type `[T; N]::
//! into_iter()` returns — the same kind of zero-coverage gap
//! `binary_heap_carrier.rs`/`linked_list_carrier.rs`/`cell_carrier.rs`
//! document for their own types. This carrier models the
//! yields-elements-by-value-in-order law a three-element `IntoIter` must
//! satisfy — not `array::IntoIter` itself — mirroring
//! `amenable_kani::rust_std::array`'s own
//! `verify_array_into_iter_yields_elements_in_order` harness (checking
//! the real `[T; N]::into_iter()` directly) for the identical claim,
//! which independently confirms this law.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The initial state law for the three-element array `IntoIter`
/// accommodation model.
pub open spec fn array_into_iter_model_starts_at_first_position(
    observed_first: i32,
    observed_second: i32,
    observed_third: i32,
    observed_position: u8,
    first: i32,
    second: i32,
    third: i32,
) -> bool {
    observed_first == first
        && observed_second == second
        && observed_third == third
        && observed_position == 0
}

/// The one-step transition law for the three-element array `IntoIter`
/// accommodation model.
pub open spec fn array_into_iter_advance_matches_position(
    old_position: u8,
    yielded: Option<i32>,
    final_position: u8,
    old_first: i32,
    old_second: i32,
    old_third: i32,
    final_first: i32,
    final_second: i32,
    final_third: i32,
) -> bool {
    (old_position == 0 ==> yielded == Some(old_first) && final_position == 1)
        && (old_position == 1 ==> yielded == Some(old_second) && final_position == 2)
        && (old_position == 2 ==> yielded == Some(old_third) && final_position == 3)
        && (old_position >= 3 ==> yielded is None && final_position == old_position)
        && final_first == old_first
        && final_second == old_second
        && final_third == old_third
}

/// The fixed three-item consuming-iterator law the model establishes.
pub open spec fn yields_three_values_in_order_then_ends(
    observed_first: Option<i32>,
    observed_second: Option<i32>,
    observed_third: Option<i32>,
    exhausted: Option<i32>,
    first: i32,
    second: i32,
    third: i32,
) -> bool {
    observed_first == Some(first)
        && observed_second == Some(second)
        && observed_third == Some(third)
        && exhausted is None
}

/// Models the yields-elements-by-value-in-order law a three-element
/// owned array iterator must satisfy — not `array::IntoIter` itself.
pub struct VerusArrayIntoIterModel {
    /// The array's first element.
    pub first: i32,
    /// The array's second element.
    pub second: i32,
    /// The array's third element.
    pub third: i32,
    /// The next index `next()` yields.
    pub position: u8,
}

impl VerusArrayIntoIterModel {
    /// Construct the model from three array elements, in order,
    /// positioned before the first element.
    pub fn from_array(first: i32, second: i32, third: i32) -> (result: Self)
        ensures
            array_into_iter_model_starts_at_first_position(
                result.first,
                result.second,
                result.third,
                result.position,
                first,
                second,
                third,
            ),
    {
        Self { first, second, third, position: 0 }
    }

    /// Advance and yield the next element by value, or `None` once every
    /// element has been yielded — the law the real
    /// `[T; 3]::into_iter()` is expected to refine.
    pub fn advance(&mut self) -> (result: Option<i32>)
        ensures
            array_into_iter_advance_matches_position(
                old(self).position,
                result,
                final(self).position,
                old(self).first,
                old(self).second,
                old(self).third,
                final(self).first,
                final(self).second,
                final(self).third,
            ),
    {
        let result = if self.position == 0 {
            Some(self.first)
        } else if self.position == 1 {
            Some(self.second)
        } else if self.position == 2 {
            Some(self.third)
        } else {
            None
        };

        if self.position < 3 {
            self.position += 1;
        }

        result
    }
}

/// `[a, b, c].into_iter()` yields `a`, then `b`, then `c`, by value, in
/// order, then `None` once exhausted.
pub fn verify_array_into_iter_model_yields_elements_in_order(a: i32, b: i32, c: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>, Option<i32>))
    ensures
        yields_three_values_in_order_then_ends(
            result.0,
            result.1,
            result.2,
            result.3,
            a,
            b,
            c,
        ),
{
    let mut it = VerusArrayIntoIterModel::from_array(a, b, c);

    let first = it.advance();
    let second = it.advance();
    let third = it.advance();
    let exhausted = it.advance();

    (first, second, third, exhausted)
}

} // verus!
