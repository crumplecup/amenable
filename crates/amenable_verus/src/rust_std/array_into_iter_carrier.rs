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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Models the yields-elements-by-value-in-order law a three-element
/// owned array iterator must satisfy — not `array::IntoIter` itself.
pub struct VerusArrayIntoIterModel {
    pub first: i32,
    pub second: i32,
    pub third: i32,
    pub position: u8,
}

impl VerusArrayIntoIterModel {
    /// Construct the model from three array elements, in order,
    /// positioned before the first element.
    pub fn from_array(first: i32, second: i32, third: i32) -> (result: Self)
        ensures
            result.first == first,
            result.second == second,
            result.third == third,
            result.position == 0,
    {
        Self { first, second, third, position: 0 }
    }

    /// Advance and yield the next element by value, or `None` once every
    /// element has been yielded — the law the real
    /// `[T; 3]::into_iter()` is expected to refine.
    pub fn advance(&mut self) -> (result: Option<i32>)
        ensures
            old(self).position == 0 ==> result == Some(old(self).first) && final(self).position
                == 1,
            old(self).position == 1 ==> result == Some(old(self).second) && final(self).position
                == 2,
            old(self).position == 2 ==> result == Some(old(self).third) && final(self).position
                == 3,
            old(self).position >= 3 ==> result is None && final(self).position
                == old(self).position,
            final(self).first == old(self).first,
            final(self).second == old(self).second,
            final(self).third == old(self).third,
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
        result.0 == Some(a),
        result.1 == Some(b),
        result.2 == Some(c),
        result.3 is None,
{
    let mut it = VerusArrayIntoIterModel::from_array(a, b, c);

    let first = it.advance();
    let second = it.advance();
    let third = it.advance();
    let exhausted = it.advance();

    (first, second, third, exhausted)
}

} // verus!
