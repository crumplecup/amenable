//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::Utc>`.
//!
//! `Utc` is a zero-sized `TimeZone`. What makes it worth a real claim, rather than
//! a trusted-by-fiat stub, is that its `TimeZone` impl never produces the
//! ambiguity or gap a general offset can: `offset_from_local_date` always reports
//! `MappedLocalTime::Single(Utc)`, for every calendar date, over every `i32`
//! year and `u32` month and day. Nothing is narrowed to a sub-range. Alongside
//! that, `Utc`'s fixed offset is always zero (`Offset::fix`), which has no
//! symbolic input at all — chrono's own source makes it a constant fact, so it is
//! asserted directly rather than wrapped in a pointless proof over an empty
//! domain.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::{Datelike, MappedLocalTime, TimeZone, Utc};

use super::gregorian::days_in_month;
use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<Utc> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_utc_local_offset_is_always_single_and_fixed_offset_is_zero".to_owned(),
            VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<Utc>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::Utc>",
        "kani",
        || <ExtStandard<Utc> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<Utc>,
    "amenable_ext::ExtStandard<chrono::Utc>",
    (i32, u32, u32),
    |(year, month, day)| {
        let valid = (chrono::NaiveDate::MIN.year()..=chrono::NaiveDate::MAX.year()).contains(&year)
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day);
        match chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => matches!(
                Utc.offset_from_local_date(&date),
                MappedLocalTime::Single(Utc)
            ),
            None => !valid,
        }
    }
);

/// Gathers the one `cfg(kani)`-only item left after `Ensures`/the harness itself
/// into a single gate, rather than scattering `#[cfg(kani)]` across each item
/// individually (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use chrono::{FixedOffset, Offset, Utc};

    /// `Utc`'s own fixed-offset bound, named so the harness's second `assert!`
    /// points at a real, registered contract fragment instead of a raw equation.
    pub(super) fn utc_fixed_offset_is_zero_holds() -> bool {
        Utc.fix() == FixedOffset::east_opt(0).unwrap()
    }
}

#[cfg(kani)]
use kani_only::utc_fixed_offset_is_zero_holds;

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::utc::utc_fixed_offset_is_zero_holds",
        "kani",
        "ensures",
        || "Utc.fix() == FixedOffset::east_opt(0).unwrap()",
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTC_LOCAL_OFFSET_IS_ALWAYS_SINGLE_AND_FIXED_OFFSET_IS_ZERO_SRC, {
        /// For every valid calendar date over every `i32` year and `u32` month and
        /// day, `Utc`'s local offset is always `MappedLocalTime::Single(Utc)` — never
        /// ambiguous, never a gap. Separately, `Utc`'s fixed offset is always zero
        /// seconds: a constant fact with no symbolic input, asserted directly.
        #[kani::proof]
        fn verify_utc_local_offset_is_always_single_and_fixed_offset_is_zero() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            assert!(
                ExtStandard::<Utc>::ensures((year, month, day)),
                "Utc's local offset must always be Single(Utc), for every valid date"
            );
            assert!(
                utc_fixed_offset_is_zero_holds(),
                "Utc's fixed offset must always be zero"
            );
        }
    }
}
