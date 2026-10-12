//! Named contract predicates for `gap_info`'s own claims: each cites
//! exactly the boolean expression that used to sit inline in
//! `carrier.rs`'s `#[trusted]` `#[ensures]` clauses (or the parent
//! module's nested `key()` helper) -- cordial's `ANTIPATTERN-UNNAMED-
//! CONTRACT-BOUND-001` flagged all five as raw, unnamed equations once
//! real `cargo creusot` output confirmed the proof itself (see
//! `carrier.rs`'s own doc comment). The fix is the same one
//! `naive_date_time_round_trips` (`naive_date_time.rs`) already uses: a
//! named `#[logic(open)]` predicate, wrapped in `harness!` for its own
//! always-available source text, registered via `ContractRecord` so a
//! site naming it by a real call (not just matching its text) is
//! recognized as using it.

#[cfg(creusot)]
use super::carrier::{self, GapInfoHandle};
#[cfg(creusot)]
use super::logic::{naive_date_time_before, naive_date_time_key};
#[cfg(creusot)]
use crate::ext_chrono::shared_trusted_accessors::{
    date_time_tz_naive_local_value, is_local_gap_logic,
};
#[cfg(creusot)]
use creusot_std::macros::logic;

amenable_derive::harness! {
    creusot, FROM_LOCAL_GAP_DETECTION_AGREES_SRC, {
        /// `GapInfoHandle::from_local`'s own claim: it returns `Some`
        /// exactly when `is_local_gap_logic` says `local` falls in a
        /// gap for `tz`.
        #[logic(open)]
        pub(crate) fn from_local_gap_detection_agrees(
            outcome: Option<GapInfoHandle>,
            tz: &chrono_tz::Tz,
            local: &chrono::NaiveDateTime,
        ) -> bool {
            pearlite! {
                (match &outcome { Some(_) => true, None => false })
                    == is_local_gap_logic(tz, local)
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::gap_info::contracts::from_local_gap_detection_agrees",
        "creusot",
        "ensures",
        || FROM_LOCAL_GAP_DETECTION_AGREES_SRC,
    )
}

amenable_derive::harness! {
    creusot, FROM_LOCAL_BRACKETS_LOCAL_SRC, {
        /// `GapInfoHandle::from_local`'s other claim: whenever it
        /// returns `Some`, the handle's `begin`/`end` genuinely bracket
        /// `local`.
        #[logic(open)]
        pub(crate) fn from_local_brackets_local(
            outcome: Option<GapInfoHandle>,
            local: &chrono::NaiveDateTime,
        ) -> bool {
            pearlite! {
                match &outcome {
                    Some(h) => (
                        match carrier::gap_info_handle_begin_naive_value(h) {
                            Some(ref b) => naive_date_time_before(b, local),
                            None => true,
                        }
                        &&
                        match carrier::gap_info_handle_end_value(h) {
                            Some(ref e) => naive_date_time_before(
                                local,
                                &date_time_tz_naive_local_value(e),
                            ),
                            None => true,
                        }
                    ),
                    None => true,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::gap_info::contracts::from_local_brackets_local",
        "creusot",
        "ensures",
        || FROM_LOCAL_BRACKETS_LOCAL_SRC,
    )
}

amenable_derive::harness! {
    creusot, BEGIN_NAIVE_MATCHES_HANDLE_SRC, {
        /// `GapInfoHandle::begin_naive`'s own claim: it returns exactly
        /// the handle's real `begin` field's own `NaiveDateTime`.
        #[logic(open)]
        pub(crate) fn begin_naive_matches_handle(
            outcome: Option<chrono::NaiveDateTime>,
            h: &GapInfoHandle,
        ) -> bool {
            pearlite! { outcome == carrier::gap_info_handle_begin_naive_value(h) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::gap_info::contracts::begin_naive_matches_handle",
        "creusot",
        "ensures",
        || BEGIN_NAIVE_MATCHES_HANDLE_SRC,
    )
}

amenable_derive::harness! {
    creusot, END_MATCHES_HANDLE_SRC, {
        /// `GapInfoHandle::end`'s own claim: it returns exactly the
        /// handle's real `end` field's own `DateTime<Tz>`.
        #[logic(open)]
        pub(crate) fn end_matches_handle(
            outcome: Option<chrono::DateTime<chrono_tz::Tz>>,
            h: &GapInfoHandle,
        ) -> bool {
            pearlite! { outcome == carrier::gap_info_handle_end_value(h) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::gap_info::contracts::end_matches_handle",
        "creusot",
        "ensures",
        || END_MATCHES_HANDLE_SRC,
    )
}

amenable_derive::harness! {
    creusot, KEY_MATCHES_NAIVE_DATE_TIME_KEY_SRC, {
        /// The real exec `key()` helper's own claim: its result equals
        /// `naive_date_time_key`, the exact opaque-accessor-built tuple
        /// `naive_date_time_before` (the axiom being exercised) is
        /// itself defined in terms of.
        #[logic(open)]
        pub(crate) fn key_matches_naive_date_time_key(
            outcome: (i32, (u32, (u32, (u32, (u32, (u32, u32)))))),
            dt: &chrono::NaiveDateTime,
        ) -> bool {
            pearlite! { outcome == naive_date_time_key(dt) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::gap_info::contracts::key_matches_naive_date_time_key",
        "creusot",
        "ensures",
        || KEY_MATCHES_NAIVE_DATE_TIME_KEY_SRC,
    )
}
