//! `TemporalVerusProof` — the proof artifact shared by every
//! `clock_bounds`/`calendar_*`/`interval_ordering`/`leap_year_core`/
//! `month_day_bounds` sibling module in this directory.

use derive_getters::Getters;
use derive_new::new;

/// Proof artifact naming an `amenable_verus` Verus spec function that
/// machine-checks a temporal contract's invariant, and carrying the
/// whole spec file it lives in verbatim (`include_str!`) — the same
/// one-claim-per-file granularity `amenable_derive::harness!` gives the
/// Kani/Creusot artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Getters, new)]
pub struct TemporalVerusProof {
    /// The Verus spec function that checks this contract's invariant.
    #[new(into)]
    harness: String,
    /// The spec's own source, verbatim.
    #[new(into)]
    claim: String,
}

impl core::fmt::Display for TemporalVerusProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        write!(f, "claim: {}", self.claim)
    }
}
