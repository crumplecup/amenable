//! Verus spec for `core::char::ParseCharError`.

#[cfg(verus_keep_ghost)]
use std::char::ParseCharError;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExParseCharError(ParseCharError);

pub assume_specification [<char as std::str::FromStr>::from_str] (s: &str) -> (result: Result<char, <char as std::str::FromStr>::Err>)
    ensures
        s@.len() == 0 ==> result is Err,
        s@.len() == 2 ==> result is Err,
        s == "a" ==> result == Ok('a'),
;

/// A string parses as `char` only when it holds exactly one character;
/// an empty or multi-character string fails with `ParseCharError` —
/// the same claim the Kani harness checks. Calls `FromStr::from_str`
/// directly rather than through `str::parse`'s generic wrapper (`verus`
/// has no spec for `str::parse` itself, only the underlying `from_str`
/// it delegates to). Takes `empty`/`two_chars` as `requires`-constrained
/// parameters (the same "parameter, not inline literal" shape as
/// `option_carrier.rs`) for the length-based cases, but the exact
/// single-character case uses the literal `"a"` directly since `verus`
/// does connect a literal `&str` to `==` comparison against another
/// literal fine (unlike `@.len()`, which needed the parameter form in
/// `int_error_kind_carrier.rs`).
pub fn verify_parse_char_error_occurs_for_empty_or_multi_character_strings(empty: &str, two_chars: &str) -> (result: (bool, bool, bool))
    requires
        empty@.len() == 0,
        two_chars@.len() == 2,
    ensures
        result.0,
        result.1,
        result.2,
{
    let empty_result = <char as std::str::FromStr>::from_str(empty);
    assert(empty_result is Err);
    let empty_rejected = empty_result.is_err();

    let two_chars_result = <char as std::str::FromStr>::from_str(two_chars);
    assert(two_chars_result is Err);
    let multi_rejected = two_chars_result.is_err();

    let single_result = <char as std::str::FromStr>::from_str("a");
    assert(single_result is Ok);
    let single_char = single_result.unwrap();
    let single_accepted = single_char == 'a';

    (empty_rejected, multi_rejected, single_accepted)
}

} // verus!
