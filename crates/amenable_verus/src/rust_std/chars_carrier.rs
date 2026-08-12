//! Verus spec for `core::str::Chars<'_>`.
//!
//! Unlike `alloc::vec::IntoIter`/`VecDeque::Iter` (both only specced
//! through the generic `Iterator` trait-extension machinery, where a
//! direct `.next()` call is itself unsupported — see
//! `vec_into_iter_carrier.rs`'s module doc comment), `vstd` gives
//! `Chars<'_>` its own dedicated, per-type `next` spec
//! (`vstd/string.rs`'s `next_postcondition`/`next_post`), so this
//! carrier proves order preservation directly via `.next()`, not
//! `.collect()`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::iter::IteratorSpec;
#[cfg(verus_keep_ghost)]
use vstd::string::group_string_axioms;

verus! {

pub open spec fn chars_input_is_ab(s: &str) -> bool {
    s@.len() == 2 && s@[0] == 'a' && s@[1] == 'b'
}

pub open spec fn chars_iteration_yields_a_then_b_then_none(
    result: (Option<char>, Option<char>, Option<char>),
) -> bool {
    result.0 == Some('a') && result.1 == Some('b') && result.2 is None
}

/// `str::chars()` yields a two-character str's characters by value, in
/// order, then `None` once exhausted. Takes `s` as a `requires`-constrained
/// parameter rather than an inline literal — the same "parameter, not
/// literal" shape `parse_char_error_carrier.rs` documents needing for
/// `@.len()`/`@[i]` facts about a `&str`.
pub fn verify_chars_yields_characters_in_order(s: &str) -> (result: (Option<char>, Option<char>, Option<char>))
    requires
        chars_input_is_ab(s),
    ensures
        chars_iteration_yields_a_then_b_then_none(result),
{
    broadcast use group_string_axioms;

    let mut it = s.chars();
    assert(it.remaining() == s@);

    let first = it.next();
    assert(first == Some('a'));
    let second = it.next();
    assert(second == Some('b'));
    let exhausted = it.next();

    (first, second, exhausted)
}

} // verus!
