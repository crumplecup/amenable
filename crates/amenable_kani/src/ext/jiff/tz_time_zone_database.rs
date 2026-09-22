//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::tz::
//! TimeZoneDatabase>` — a real, checked property over jiff's actual
//! public API (`none`/`is_definitively_empty`), not a trusted stub.
//!
//! `TimeZoneDatabase` has a substantial public surface (`bundled`/
//! `from_env`/`from_dir`/`from_concatenated_path`/`get`/`available`/
//! `reset`/etc.), but only `none()`/`is_definitively_empty()` are
//! checked here: real jiff source confirms both are genuinely cheap
//! and dispatch-free for the `Repr::Empty` case (`none()` just
//! constructs the `Empty` tag; `is_definitively_empty()`'s `match
//! self.inner { Repr::Empty => true, .. }` returns immediately, no
//! `TimeZone::Repr`-style pointer-tagged dispatch at all) — checked
//! directly, not assumed safe by resemblance to `TimeZone`'s own
//! Kani-uncheckable dispatch wall. `get()`/`bundled()`/etc. touch
//! real IANA tzdb data or `jiff::Error` construction, disproportionate
//! to add here.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::tz::TimeZoneDatabase> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_tz_time_zone_database_none_is_definitively_empty".to_owned(),
            VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::tz::TimeZoneDatabase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::TimeZoneDatabase>",
        "kani",
        || <ExtStandard<jiff::tz::TimeZoneDatabase> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::tz::TimeZoneDatabase>,
    "amenable_ext::ExtStandard<jiff::tz::TimeZoneDatabase>",
    (),
    |()| jiff::tz::TimeZoneDatabase::none().is_definitively_empty()
);

amenable_derive::harness! {
    kani, VERIFY_TZ_TIME_ZONE_DATABASE_NONE_IS_DEFINITIVELY_EMPTY_SRC, {
        /// `TimeZoneDatabase::none().is_definitively_empty()` is
        /// always `true` — a fixed fact about jiff's real
        /// implementation, not an assumed one.
        #[kani::proof]
        fn verify_tz_time_zone_database_none_is_definitively_empty() {
            let unit: () = kani::any();
            assert!(
                ExtStandard::<jiff::tz::TimeZoneDatabase>::ensures(unit),
                "TimeZoneDatabase::none() must always be definitively empty"
            );
        }
    }
}
