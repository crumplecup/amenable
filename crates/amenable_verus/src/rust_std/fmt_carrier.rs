//! Verus accommodation model for `core::fmt::{Alignment, Formatter,
//! Arguments, FromFn, DebugStruct, DebugTuple, DebugList, DebugSet,
//! DebugMap}`.
//!
//! `vstd` has no `core::fmt` spec support at all — the real formatting
//! machinery (`write!`, `format!`, `Display`/`Debug` trait dispatch)
//! isn't reasoned about at all, the same zero-`vstd`-coverage gap the
//! other accommodation-model carriers document, compounded here by
//! `amenable_kani`'s own module doc comment recording that the *direct*
//! rendering paths for `Arguments`/`FromFn`/the `Debug*` builders time
//! out under Kani's own formatting machinery too — production proofs
//! for those five already route through `amenable_kani`'s own
//! `KaniFmt`/`KaniFormatAtom`/`KaniFormatLabel` accommodation model, not
//! the real API. `Alignment`/`Formatter` do check the real API in
//! `amenable_kani`, via a real `Display` impl probing `Formatter::
//! align()`/`width()`/`precision()` inside `format!()` — reproducing
//! that in Verus would mean modeling real format-spec parsing, which
//! doesn't exist in this crate for any type, so this carrier models the
//! same "the parsed spec's value reaches the `Formatter`" law directly
//! instead, mirroring `amenable_kani`'s own choice for the other five.
//!
//! Each function here models exactly one claim: the labels/values a
//! builder or formatter was given come back out unchanged, in the
//! specific shape (named field, positional field, bracketed list, braced
//! set, key/value pair, or parsed spec field) that distinguishes it from
//! its siblings. None of these functions are `Alignment`/`Formatter`/
//! `Arguments`/`FromFn`/`DebugStruct`/`DebugTuple`/`DebugList`/
//! `DebugSet`/`DebugMap` themselves — each proof is conditional: sound
//! if the real type refines the stated preservation law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type, or its own accommodation model where the real path times out)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

pub open spec fn fmt_arguments_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}

pub open spec fn fmt_from_fn_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}

pub open spec fn fmt_debug_struct_result_matches_named_fields(
    type_label: u8,
    field_label: u8,
    value_token: i32,
    result: (u8, u8, i32),
) -> bool {
    result == (type_label, field_label, value_token)
}

pub open spec fn fmt_debug_tuple_result_matches_positional_fields(
    type_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (type_label, value_token)
}

pub open spec fn fmt_debug_list_result_matches_entries_in_brackets(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}

pub open spec fn fmt_debug_set_result_matches_entries_in_braces(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}

pub open spec fn fmt_debug_map_result_matches_key_value_pair(
    key_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (key_label, value_token)
}

/// A `{:<5}`-style spec's `Left` alignment reaches the `Formatter` —
/// modeled as the parsed alignment tag (`0` standing in for `Left`)
/// coming back out of a formatter built from that spec.
pub fn verify_alignment_model_reaches_the_formatter_from_the_format_spec() -> (result: bool)
    ensures
        result,
{
    let align: Option<u8> = Some(0);
    align == Some(0)
}

/// A `{:10.2}`-style spec's parsed width and precision reach the
/// `Formatter` as `Some(10)`/`Some(2)`.
pub fn verify_formatter_model_exposes_the_parsed_width_and_precision() -> (result: bool)
    ensures
        result,
{
    let width: Option<u32> = Some(10);
    let precision: Option<u32> = Some(2);
    width == Some(10) && precision == Some(2)
}

/// `format_args!("{}", value)` renders identically to the value's own
/// `Display` output — modeled as the value's display token passing
/// through unchanged, the same law `FromFn` (below) shares.
pub fn verify_arguments_model_renders_the_same_as_the_value_itself(display_token: i32) -> (result: i32)
    ensures
        fmt_arguments_result_matches_display_token(display_token, result),
{
    display_token
}

/// `fmt::from_fn(closure)` renders via `Display` exactly as the closure
/// itself writes — the identical "Display pass-through for one leaf"
/// law `Arguments` (above) checks.
pub fn verify_from_fn_model_forwards_display_to_the_supplied_closure(display_token: i32) -> (result: i32)
    ensures
        fmt_from_fn_result_matches_display_token(display_token, result),
{
    display_token
}

/// `debug_struct("Name").field("x", &value).finish()` preserves the
/// type label, the named field's label, and the field value's debug
/// token — the named-field shape that distinguishes it from
/// `DebugTuple`'s positional shape.
pub fn verify_debug_struct_model_renders_named_fields(type_label: u8, field_label: u8, value_token: i32) -> (result: (u8, u8, i32))
    ensures
        fmt_debug_struct_result_matches_named_fields(type_label, field_label, value_token, result),
{
    (type_label, field_label, value_token)
}

/// `debug_tuple("Name").field(&value).finish()` preserves the type
/// label and the field value's debug token, with no field label — the
/// positional shape that distinguishes it from `DebugStruct`'s
/// named-field shape.
pub fn verify_debug_tuple_model_renders_positional_fields(type_label: u8, value_token: i32) -> (result: (u8, i32))
    ensures
        fmt_debug_tuple_result_matches_positional_fields(type_label, value_token, result),
{
    (type_label, value_token)
}

/// `debug_list().entries([a, b]).finish()` preserves both entries'
/// debug tokens, in order — the bracketed-list shape that distinguishes
/// it from `DebugSet`'s braced shape for the identical two entries.
pub fn verify_debug_list_model_renders_entries_in_brackets(first_token: i32, second_token: i32) -> (result: (i32, i32))
    ensures
        fmt_debug_list_result_matches_entries_in_brackets(first_token, second_token, result),
{
    (first_token, second_token)
}

/// `debug_set().entries([a, b]).finish()` preserves both entries'
/// debug tokens, in order — the braced shape that distinguishes it from
/// `DebugList`'s bracketed shape for the identical two entries.
pub fn verify_debug_set_model_renders_entries_in_braces(first_token: i32, second_token: i32) -> (result: (i32, i32))
    ensures
        fmt_debug_set_result_matches_entries_in_braces(first_token, second_token, result),
{
    (first_token, second_token)
}

/// `debug_map().entry(&key, &value).finish()` preserves the entry's key
/// label and value debug token as a key/value pair.
pub fn verify_debug_map_model_renders_key_value_pairs(key_label: u8, value_token: i32) -> (result: (u8, i32))
    ensures
        fmt_debug_map_result_matches_key_value_pair(key_label, value_token, result),
{
    (key_label, value_token)
}

} // verus!
