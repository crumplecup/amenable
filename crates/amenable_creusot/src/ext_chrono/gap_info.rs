//! Real Creusot proof content for `chrono_tz::GapInfo`'s own claim
//! (`chrono::gap_info` holds the `CreusotWitness` bridge that references
//! the `_SRC` constant this file's `harness!` call emits).
//!
//! The wrapper below cross-checks both halves of the claim by calling
//! `Tz::offset_from_local_datetime` and `GapInfoHandle::from_local` for
//! real: their `None`/`Some` agreement (against `logic.rs`'s
//! `is_local_gap_logic` axiom), and -- now that `chrono_tz::GapInfo`'s
//! own fields are reachable through `carrier::GapInfoHandle` without
//! ever naming `chrono_tz::GapInfo` in a translated signature -- the
//! bracketing property too (`begin`/`end` genuinely straddle the
//! witness, against `GapInfoHandle::from_local`'s own `#[trusted]`
//! contract). See `carrier.rs`'s own doc comment for why the newtype is
//! necessary at all: a confirmed Creusot/WHY3 keyword-escaping bug,
//! not a modeling choice (also recorded in
//! `amenable_std::creusot_gallery`'s
//! `public_field_foreign_struct_forces_full_record_translation` case).
//!
//! This is a *different* facet of `GapInfo`'s correctness than
//! `amenable_kani::chrono::gap_info`'s 47 binned harnesses check: Kani
//! checks the whole claim against 94 concrete, discovered real-world
//! witnesses; this proof instead states both halves as universal facts
//! over *any* `local`/`tz`, cross-checked by real execution, with no
//! table dependency.

mod carrier;
mod contracts;
mod logic;
#[cfg(creusot)]
use contracts::key_matches_naive_date_time_key;
#[cfg(creusot)]
use creusot_std::macros::{ensures, requires};

amenable_derive::harness! {
    creusot, VERIFY_GAP_INFO_MATCHES_OFFSET_FROM_LOCAL_DATETIME_SRC, {
        /// `Tz::offset_from_local_datetime` returns `LocalResult::None`
        /// exactly when `GapInfoHandle::from_local` returns `Some`, and
        /// when it does, its `begin`/`end` genuinely bracket `local` --
        /// both real functions run the same binary search over the same
        /// timespan table, confirmed by reading chrono-tz's own source.
        #[requires(true)]
        #[ensures(result)]
        fn verify_gap_info_matches_offset_from_local_datetime(
            local: chrono::NaiveDateTime,
            tz: chrono_tz::Tz,
        ) -> bool {
            use carrier::GapInfoHandle;
            use chrono::{Datelike, TimeZone, Timelike};

            // Nested pairs, not a flat 7-tuple, and never `NaiveDateTime::
            // cmp` directly: Creusot's own `PartialOrdLogic` only has a
            // blanket impl for 2-tuples, and `#[check(ghost)]`-marked
            // `Ord::cmp` needs `DeepModel` on `NaiveDateTime`, which this
            // crate has no coverage for (both confirmed by real `cargo
            // creusot` output) -- `.year()`/`.month()`/etc. are already
            // real, extern_spec'd elsewhere in this crate, and tuples of
            // plain integers already have `DeepModel`/`PartialOrdLogic`
            // coverage.
            //
            // The `#[ensures]` ties this real tuple to `naive_date_time_
            // key` (the logic-side version `naive_date_time_before`
            // itself is defined in terms of) explicitly: without it, the
            // caller only knows this function's return *type*, not that
            // its components equal the same opaque accessors `.year()`/
            // etc.'s own postconditions already name -- confirmed
            // necessary by real `why3find prove -X` output (the goal
            // `not begin_ok = False` stayed unproved without this link,
            // even with `naive_date_time_before` and this function
            // built from the exact same key shape).
            #[ensures(key_matches_naive_date_time_key(result, dt))]
            fn key(
                dt: &chrono::NaiveDateTime,
            ) -> (i32, (u32, (u32, (u32, (u32, (u32, u32)))))) {
                (
                    dt.year(),
                    (
                        dt.month(),
                        (dt.day(), (dt.hour(), (dt.minute(), (dt.second(), dt.nanosecond())))),
                    ),
                )
            }

            let is_gap = matches!(tz.offset_from_local_datetime(&local), chrono::LocalResult::None);
            let Some(handle) = GapInfoHandle::from_local(&local, &tz) else {
                return !is_gap;
            };
            if !is_gap {
                return false;
            }

            let local_key = key(&local);
            let begin_ok = match handle.begin_naive() {
                Some(begin_local) => key(&begin_local) < local_key,
                None => true,
            };
            let end_ok = match handle.end() {
                Some(end_zoned) => local_key < key(&end_zoned.naive_local()),
                None => true,
            };
            begin_ok && end_ok
        }
    }
}
