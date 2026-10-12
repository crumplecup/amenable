//! Real Creusot proof content for `chrono_tz::TzOffset`'s own
//! consistency claim (`chrono::tz_offset` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits).
//!
//! Each of the three real public methods (`fix()`, `base_utc_offset()`,
//! `dst_offset()`) gets its own independent `extern_spec!` axiom
//! (`logic.rs`), not the combined relationship stated as one fact --
//! the wrapper below is what actually checks those three axioms are
//! consistent with each other, the same shape `ext_chrono::tz`'s own
//! `extern_spec!` + wrapper uses for `Tz::name()`/`FromStr`.
//!
//! This is the same claim `amenable_kani::chrono::tz_offset`'s 29
//! binned harnesses check by real execution over 453 discovered
//! witnesses -- restated here resting on the three `extern_spec!`
//! axioms above instead, which Creusot's own prover (not a human
//! restating the conclusion) confirms are mutually consistent.

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, requires};

amenable_derive::harness! {
    creusot, VERIFY_TZ_OFFSET_CONSISTENCY_SRC, {
        /// `fix().local_minus_utc()` always equals `base_utc_offset() +
        /// dst_offset()`, for any real `TzOffset`.
        #[requires(true)]
        #[ensures(result)]
        fn verify_tz_offset_consistency(offset: chrono_tz::TzOffset) -> bool {
            use chrono::Offset;
            use chrono_tz::OffsetComponents;

            let fix_secs = i64::from(offset.fix().local_minus_utc());
            let base_secs = offset.base_utc_offset().num_seconds();
            let dst_secs = offset.dst_offset().num_seconds();
            fix_secs == base_secs + dst_secs
        }
    }
}
