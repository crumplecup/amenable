//! The chrono-tz offset-discovery `Exchange`: a real, black-box scan of
//! every real `Tz` variant's public API over a given `(lo, hi, delta)`
//! grid, producing the complete set of distinct `(base_utc_offset,
//! dst_offset)` pairs any real zone can produce within that range.
//!
//! The method is real and public -- committing a discovered table (the
//! design this Exchange backs) means re-running it, by hand, is how that
//! table gets regenerated or re-verified later, not a one-off throwaway
//! script. The completeness bound this rests on is `BoundedStepWalk`
//! (proven on Kani, Creusot, and Verus): this Exchange's `Output` carries
//! that Standard as its own `Sidecar::Proposition`, established from a
//! fresh `amenable_time::TemporalInputToken` -- the same root every other
//! real-input boundary in this backend starts an `Establish` chain from.
//!
//! `BoundedStepWalk`'s own `Ensures` gives the honest bound this table can
//! claim: every grid point spaced `delta` apart was really checked, so
//! anything the table does *not* record must have had a real duration
//! strictly less than `delta` -- not "the table is exhaustive" outright,
//! which depends on `delta` being fine enough for chrono-tz's *actual*
//! data (an empirical fact, confirmed separately, not something this
//! Exchange's own types can state in general).

use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    NonPositiveDeltaSource, OffsetDiscoveryRangeReversedSource, TemporalError, TemporalErrorKind,
    TemporalInputToken,
};
use chrono::{Duration, NaiveDateTime};
use chrono_tz::{OffsetComponents, Tz};

use crate::BoundedStepWalk;
use crate::chrono::identity::{ChronoTimeBackend, ChronoVerifier};

/// A real zone's offset at a given instant: its `base_utc_offset()` and
/// `dst_offset()`, each in seconds.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tz, at)))]
fn offset_pair(tz: Tz, at: NaiveDateTime) -> (i64, i64) {
    use chrono::TimeZone;
    let offset = tz.offset_from_utc_datetime(&at);
    (
        offset.base_utc_offset().num_seconds(),
        offset.dst_offset().num_seconds(),
    )
}

// ── Primary payloads (plain data, not proofs) ───────────────────────

/// The discovery scan's own parameters: the grid's bounds and step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, amenable_derive::Evidence, derive_new::new)]
#[evidence(
    basis = "Self",
    basis_ctor = "Self::new(chrono::NaiveDateTime::default(), chrono::NaiveDateTime::default(), \
                  chrono::Duration::zero())"
)]
pub struct OffsetDiscoveryParams {
    lo: NaiveDateTime,
    hi: NaiveDateTime,
    delta: Duration,
}

impl OffsetDiscoveryParams {
    /// The grid's lower bound.
    #[must_use]
    pub const fn lo(&self) -> NaiveDateTime {
        self.lo
    }

    /// The grid's upper bound.
    #[must_use]
    pub const fn hi(&self) -> NaiveDateTime {
        self.hi
    }

    /// The grid's step -- the pigeonhole bound this table's completeness
    /// rests on: anything not recorded had a real duration strictly less
    /// than this.
    #[must_use]
    pub const fn delta(&self) -> Duration {
        self.delta
    }
}

/// One discovered `(base_utc_offset, dst_offset)` pair, with the first
/// concrete `(zone, date)` witness where the grid walk observed it.
pub type DiscoveredOffsetEntry = (Tz, NaiveDateTime, i64, i64);

/// The discovered table: the complete set of distinct offset pairs
/// observed on the grid described by `params`, each with one concrete
/// witness.
#[derive(
    Debug, Clone, PartialEq, amenable_derive::Evidence, derive_getters::Getters, derive_new::new,
)]
#[evidence(
    basis = "Self",
    basis_ctor = "Self::new(Vec::new(), OffsetDiscoveryParams::new(chrono::NaiveDateTime::default(), \
                  chrono::NaiveDateTime::default(), chrono::Duration::zero()))"
)]
pub struct DiscoveredOffsetTableData {
    /// The discovered entries, each `(zone, witness date, base_utc_offset
    /// seconds, dst_offset seconds)`.
    entries: Vec<DiscoveredOffsetEntry>,
    /// The scan's own parameters.
    params: OffsetDiscoveryParams,
}

// ── Sidecars ─────────────────────────────────────────────────────────

/// The discovery request: a neutral `(lo, hi, delta)` triple, paired with
/// a token for the trivial "a caller handed us this request" proposition
/// every real-input boundary starts from.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "amenable_time::TemporalInputReceived",
    constructor = "pub"
)]
pub struct OffsetDiscoveryRequest {
    #[sidecar(primary)]
    params: OffsetDiscoveryParams,
    #[sidecar(token)]
    token: TemporalInputToken,
}

/// Lawful token minted once [`BoundedStepWalk`] is established from a
/// proven [`TemporalInputToken`], by any backend that has registered a
/// real `Witness<V>` proof for it (Kani, Creusot, Verus; `ChronoVerifier`
/// trusts it by citation -- see `trusted_witness`).
#[derive(Debug, Clone, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::BoundedStepWalk")]
#[amenable_derive::establish(
    credential = "amenable_time::TemporalInputToken",
    proposition = "crate::BoundedStepWalk"
)]
pub struct BoundedStepWalkToken(());

/// The discovered table, paired with a token for the real
/// `BoundedStepWalk` completeness bound.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::BoundedStepWalk", constructor = "pub")]
pub struct DiscoveredOffsetTable {
    /// The discovered data.
    #[sidecar(primary)]
    data: DiscoveredOffsetTableData,
    #[sidecar(token)]
    token: BoundedStepWalkToken,
}

// ── The real discovery Exchange ─────────────────────────────────────

impl Exchange<OffsetDiscoveryRequest, DiscoveredOffsetTable, ChronoVerifier> for ChronoTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: OffsetDiscoveryRequest,
    ) -> Result<DiscoveredOffsetTable, TemporalError> {
        let params = *<OffsetDiscoveryRequest as Sidecar<ChronoVerifier>>::primary(&input);
        let (lo, hi, delta) = (params.lo(), params.hi(), params.delta());
        if delta <= Duration::zero() {
            return Err(TemporalError::new(TemporalErrorKind::NonPositiveDelta(
                NonPositiveDeltaSource::new(delta),
            )));
        }
        if lo > hi {
            return Err(TemporalError::new(
                TemporalErrorKind::OffsetDiscoveryRangeReversed(
                    OffsetDiscoveryRangeReversedSource::new(lo, hi),
                ),
            ));
        }

        // The real grid walk: `BoundedStepWalk`'s own one-step lemma
        // covers exactly this formula (`next = (prev + delta).min(hi)`),
        // so its proof directly backs the completeness claim this
        // produces -- no unwinding thousands of steps symbolically, the
        // same CBMC wall this whole investigation set out to avoid.
        let mut entries: Vec<DiscoveredOffsetEntry> = Vec::new();
        for &tz in chrono_tz::TZ_VARIANTS.iter() {
            let mut t = lo;
            let mut prev = offset_pair(tz, t);
            if !entries.iter().any(|&(_, _, b, d)| (b, d) == prev) {
                entries.push((tz, t, prev.0, prev.1));
            }
            while t < hi {
                t = (t + delta).min(hi);
                let cur = offset_pair(tz, t);
                if cur != prev {
                    if !entries.iter().any(|&(_, _, b, d)| (b, d) == cur) {
                        entries.push((tz, t, cur.0, cur.1));
                    }
                    prev = cur;
                }
                if t == hi {
                    break;
                }
            }
        }

        let data = DiscoveredOffsetTableData::new(entries, params);
        let credential = <OffsetDiscoveryRequest as Sidecar<ChronoVerifier>>::sidecar(&input);
        let token = <BoundedStepWalk as Establish<TemporalInputToken, ChronoVerifier>>::establish(
            credential,
        );
        Ok(DiscoveredOffsetTable::new(data, token))
    }
}
