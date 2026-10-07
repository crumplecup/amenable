//! Real Creusot proof content for `chrono::Utc`'s `TimeZone`/`Offset` trait impls
//! (`chrono::utc` holds the `CreusotWitness` bridge that references the `_SRC`
//! constant this file's `harness!` call emits).
//!
//! `Utc` is a zero-sized `TimeZone`. Unlike `NaiveDate`'s model-based proof
//! (chrono's internals aren't visible to Creusot, so a Pearlite reimplementation
//! stands in, under a stated refinement premise), `Utc`'s real behavior is
//! literally its trait impls' own one-line bodies — a direct `extern_spec!` on
//! `TimeZone for Utc`/`Offset for Utc` states that behavior as a trusted axiom
//! with no model layer in between, the same shape `FixedOffset`'s `extern_spec!`
//! uses, reusing its `fixed_offset_local_minus_utc_value` accessor for `fix()`'s
//! own return value.

mod logic;
#[cfg(creusot)]
use chrono::TimeZone as _;
#[cfg(creusot)]
use creusot_std::macros::{ensures, requires};

amenable_derive::harness! {
    creusot, VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC, {
        /// For every date, `Utc`'s local offset is always `Single`, and `Utc`'s
        /// fixed offset is always zero — the same claim
        /// `amenable_kani::chrono::utc::
        /// verify_utc_local_offset_is_always_single_and_fixed_offset_is_zero`
        /// checks by symbolic execution, restated as a real Creusot postcondition
        /// resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(result)]
        fn verify_utc_local_offset_is_always_single_and_fixed_offset_is_zero(
            date: chrono::NaiveDate,
        ) -> bool {
            let always_single =
                matches!(chrono::Utc.offset_from_local_date(&date), chrono::MappedLocalTime::Single(_));
            let fixed_zero = chrono::Offset::fix(&chrono::Utc).local_minus_utc() == 0;
            always_single && fixed_zero
        }
    }
}
