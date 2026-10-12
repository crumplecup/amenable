#![cfg(creusot)]
//! `chrono_tz::GapInfo`'s own `extern_spec!` content: `Tz::
//! offset_from_local_datetime` and `NaiveDateTime`'s `Ord`/`DateTime<T>::
//! naive_local` -- the pieces that never need `chrono_tz::GapInfo` to
//! appear in any translated signature at all.
//!
//! `GapInfo::new` itself is *not* extern_spec'd here: real `cargo
//! creusot` output confirmed (twice, including after removing all
//! literal field projection) that `chrono_tz::GapInfo` appearing
//! anywhere Creusot must translate a signature -- even just as an
//! extern_spec'd function's declared return type, with zero field
//! access -- forces Creusot to emit its full WHY3 record, and that
//! record's `begin` field name collides with a WHY3 reserved word
//! Creusot's own escaping table omits (see
//! `amenable_std::creusot_gallery`'s own
//! `public_field_foreign_struct_forces_full_record_translation` case for
//! both failed attempts). `GapInfo::new`'s real call, and the claim
//! about its result, instead live in the sibling `carrier` module's
//! `GapInfoHandle` -- a private-field newtype whose own signatures never
//! name `chrono_tz::GapInfo`, confirmed to avoid the bug.
//!
//! `Tz::offset_from_local_datetime` is tied to `is_local_gap_logic`
//! (`shared_trusted_accessors`), grounded by reading it side by side
//! with `GapInfo::new` in chrono-tz 0.10.4's `timezone_impl.rs`: both
//! run the identical `binary_search(0, timespans.len(), |i|
//! timespans.local_span(i).cmp(timestamp))` and treat `Err(_)` the same
//! way -- `GapInfoHandle::from_local`'s own (`#[trusted]`) contract is
//! stated against the same predicate, letting a real-execution wrapper
//! cross-check their `None`/`Some` agreement, the same
//! "independently-stated, cross-checked by a real call" shape
//! `TzOffset`'s three accessors use.
//!
//! `naive_date_time_before` (used by `GapInfoHandle::from_local`'s own
//! bracketing contract) is a lexicographic comparison derived entirely
//! from the seven already-trusted scalar accessors
//! (`naive_date_time_*_value`) -- not a new opaque primitive, since
//! `naive_date_time.rs`'s own doc comment already flagged that comparing
//! two foreign `NaiveDateTime`/`NaiveDate` values via `==`/`<` directly
//! has no `DeepModel` precedent in this crate; decomposing to the scalar
//! tuple sidesteps that gap instead of adding a new trusted comparison.

use crate::ext_chrono::shared_trusted_accessors::{
    date_time_tz_naive_local_value, is_local_gap_logic, naive_date_time_day_value,
    naive_date_time_hour_value, naive_date_time_minute_value, naive_date_time_month_value,
    naive_date_time_nanosecond_value, naive_date_time_second_value, naive_date_time_year_value,
};
use creusot_std::macros::{check, ensures, extern_spec, logic};

/// `a`'s own `(year, month, day, hour, minute, second, nanosecond)`
/// nested-pair key -- the exact same shape, component for component,
/// as `gap_info.rs`'s real exec `key()` helper (which builds it from
/// `.year()`/`.month()`/etc., each already tied to these same opaque
/// accessors by `naive_date_time.rs`'s own `extern_spec!`). Kept
/// nested, not a flat 7-tuple, for the same reason that wrapper does:
/// Creusot's `PartialOrdLogic` only has a blanket impl for 2-tuples.
#[logic(open)]
pub fn naive_date_time_key(
    dt: &chrono::NaiveDateTime,
) -> (i32, (u32, (u32, (u32, (u32, (u32, u32)))))) {
    pearlite! {
        (
            naive_date_time_year_value(dt),
            (
                naive_date_time_month_value(dt),
                (
                    naive_date_time_day_value(dt),
                    (
                        naive_date_time_hour_value(dt),
                        (
                            naive_date_time_minute_value(dt),
                            (naive_date_time_second_value(dt), naive_date_time_nanosecond_value(dt)),
                        ),
                    ),
                ),
            ),
        )
    }
}

/// `a` is strictly before `b`, by the same nested-pair key `<` the real
/// exec wrapper computes -- deliberately *not* a hand-written second
/// form (a separate if/else cascade over the same fields would need the
/// prover to show two independent formulations equivalent; sharing one
/// key function means the wrapper's real `key(a) < key(b)` connects to
/// this axiom by substitution alone, confirmed necessary by real
/// `why3find prove -X` output: the hand-written-cascade version left 3
/// goals unproved for exactly this reason).
#[logic(open)]
pub fn naive_date_time_before(a: &chrono::NaiveDateTime, b: &chrono::NaiveDateTime) -> bool {
    pearlite! { naive_date_time_key(a) < naive_date_time_key(b) }
}

extern_spec! {
    impl Ord for chrono::NaiveDateTime {
        #[check(ghost)]
        #[ensures(match result {
            std::cmp::Ordering::Less => naive_date_time_before(&self, other),
            std::cmp::Ordering::Greater => naive_date_time_before(other, &self),
            std::cmp::Ordering::Equal =>
                !naive_date_time_before(&self, other) && !naive_date_time_before(other, &self),
        })]
        fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    }

    impl<T: chrono::TimeZone> chrono::DateTime<T> {
        #[check(ghost)]
        #[ensures(result == date_time_tz_naive_local_value(&self))]
        fn naive_local(&self) -> chrono::NaiveDateTime;
    }

    impl chrono::TimeZone for chrono_tz::Tz {
        #[check(ghost)]
        #[ensures(
            (match result { chrono::LocalResult::None => true, _ => false })
                == is_local_gap_logic(&self, local)
        )]
        fn offset_from_local_datetime(
            &self,
            local: &chrono::NaiveDateTime,
        ) -> chrono::LocalResult<chrono_tz::TzOffset>;
    }
}
