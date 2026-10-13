//! `KaniWitness` for `amenable_ext::ExtStandard<chrono_tz::ParseError>`.
//!
//! The claim: a string naming no real IANA time zone makes `Tz::
//! from_str` return `Err(ParseError(()))` -- the negative direction of
//! `Tz`'s own name/`FromStr` round trip (`amenable_kani::chrono::tz`'s
//! 10 binned harnesses only exercise the positive direction: every
//! real variant's own name round-trips). `ParseError` itself carries no
//! information (confirmed by reading chrono-tz's real source: `pub
//! struct ParseError(());`), so the only real claim possible about it
//! is exactly this: when it occurs.
//!
//! Three concrete, hand-picked witness strings, not a symbolic one:
//! `Tz`'s own module doc comment already found that `.parse()` hits a
//! real cost wall the moment even one symbolic *byte* reaches
//! chrono-tz's `siphasher`-based static lookup table, regardless of
//! domain size -- the same wall a symbolic string here would hit
//! immediately, for the same reason. A finite, concrete sample is
//! sufficient (not exhaustive) for this type's claim, the same
//! convention `rust_std::num::parse`'s own int-parsing claims use.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono_tz::ParseError;

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<ParseError> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "check_tz_parse_error_occurs_for_unknown_names".to_owned(),
            CHECK_TZ_PARSE_ERROR_OCCURS_FOR_UNKNOWN_NAMES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<ParseError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::ParseError>",
        "kani",
        || <ExtStandard<ParseError> as crate::KaniWitness>::proof().to_string(),
    )
}

/// Gathers the one `cfg(kani)`-only item this claim needs into a single
/// gate, rather than scattering `#[cfg(kani)]` across each item
/// individually (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    /// Named contract type for the claim, so the harness's `assert!`
    /// points at a real, registered `Ensures` impl (a fully-qualified
    /// `<Type as Ensures<V>>::ensures(..)` call) instead of a bare
    /// function call cordial's own antipattern check can't recognize
    /// as pointing at a contract fragment -- the same shape
    /// `amenable_kani::chrono::utc::UtcFixedOffsetIsZero` already uses.
    pub(super) struct TzFromStrRejectsUnknownName;

    crate::rust_std::kani_local_claim!(TzFromStrRejectsUnknownName);

    impl amenable_core::Ensures<crate::KaniVerifier> for TzFromStrRejectsUnknownName {
        type Input = &'static str;
        type Bound = bool;

        /// The claim, for one witness string: it names no real IANA
        /// zone, so `Tz::from_str` genuinely returns `Err`.
        fn ensures(name: &'static str) -> bool {
            use std::str::FromStr;
            chrono_tz::Tz::from_str(name).is_err()
        }
    }
}
#[cfg(kani)]
use kani_only::TzFromStrRejectsUnknownName;

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::parse_error::TzFromStrRejectsUnknownName",
        "kani",
        "ensures",
        || "chrono_tz::Tz::from_str(name).is_err()",
    )
}

amenable_derive::harness! {
    kani, CHECK_TZ_PARSE_ERROR_OCCURS_FOR_UNKNOWN_NAMES_SRC, {
        /// Three concrete witness strings, none naming a real IANA
        /// zone (confirmed by real `cargo kani` execution against
        /// chrono-tz's own `TIMEZONES` lookup table, not assumed).
        /// Looped over, not three separate `assert!` call sites: the
        /// same claim, applied to different literals, is one check --
        /// cordial's own `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001`
        /// flagged the original three call sites as a duplicate
        /// cluster and recommended naming it once, the same shape
        /// `amenable_kani::chrono::tz`'s own binned harnesses already
        /// use (one loop over a table, not N hand-written calls). An
        /// indexed loop (`WITNESS_NAMES[i]`), not `for name in
        /// WITNESS_NAMES`: the iterator form regressed CBMC to a
        /// genuine timeout (confirmed by real `cargo kani` output, not
        /// a guess) -- indexed access matches `amenable_kani::chrono::
        /// tz`'s own proven bin shape exactly.
        #[kani::proof]
        fn check_tz_parse_error_occurs_for_unknown_names() {
            const WITNESS_NAMES: [&str; 3] =
                ["Definitely/Not/A/Real/Zone", "", "Etc/Not_A_Real_Offset_999"];
            for i in 0..WITNESS_NAMES.len() {
                assert!(TzFromStrRejectsUnknownName::ensures(WITNESS_NAMES[i]));
            }
        }
    }
}
