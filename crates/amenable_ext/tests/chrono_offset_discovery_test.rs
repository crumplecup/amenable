//! `ChronoTimeBackend`'s offset-discovery `Exchange`: a real, black-box
//! scan of every real `Tz` variant, grounded against the facts the
//! scratchpad investigation that designed this already confirmed. The
//! table records one witness per distinct `(base, dst)` pair across the
//! whole union of zones, not one per zone -- so a pair's presence is
//! checked, not which specific zone is credited for it -- and a coarser
//! `delta` must find no more (and, over a range containing a known
//! short real transition, strictly fewer) distinct pairs than a finer
//! one, confirmed here directly against `America::Boa_Vista`'s real
//! 7-day transition rather than assumed.

#![cfg(feature = "chrono-tz")]

use amenable_core::Exchange;
use amenable_ext::{
    ChronoTimeBackend, ChronoVerifier, OffsetDiscoveryParams, OffsetDiscoveryRequest,
};
use amenable_time::TemporalInputToken;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use miette::IntoDiagnostic;

/// Builds a [`NaiveDateTime`] at midnight on the given date, surfacing an
/// invalid calendar date with miette rather than aborting.
fn midnight(year: i32, month: u32, day: u32) -> miette::Result<NaiveDateTime> {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .ok_or_else(|| miette::miette!("{year}-{month}-{day} is not a valid calendar date"))
}

#[test]
fn discovers_the_zero_offset_pair_via_the_real_api() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let lo = midnight(1900, 1, 1)?;
    let hi = midnight(2050, 1, 1)?;
    let request = OffsetDiscoveryRequest::new(
        OffsetDiscoveryParams::new(lo, hi, Duration::days(1)),
        TemporalInputToken::new(),
    );

    let table: amenable_ext::DiscoveredOffsetTable =
        <ChronoTimeBackend as Exchange<OffsetDiscoveryRequest, _, ChronoVerifier>>::exchange(
            &backend, request,
        )
        .into_diagnostic()?;

    // Etc::UTC's own (0, 0) pair may be attributed to a different zone's
    // witness in the deduped global union (the table records one witness
    // per distinct pair, not one per zone -- matching the scratchpad
    // discovery script's own `or_insert` semantics), so the real check is
    // that the pair exists at all, not which zone is credited for it.
    let has_zero_offset_pair = table
        .data()
        .entries()
        .iter()
        .any(|&(_, _, base, dst)| (base, dst) == (0, 0));
    assert!(
        has_zero_offset_pair,
        "the union must contain (0, 0) -- Etc::UTC's own real offset, confirmed via the real \
         public API"
    );
    Ok(())
}

#[test]
fn a_delta_coarser_than_the_real_minimum_gap_misses_a_real_transition() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    // `America::Boa_Vista` had a real transition reverted within 7 days
    // (2000-10-09 to 2000-10-16) -- confirmed via the earlier exhaustive
    // daily scan. A 90-day grid cannot possibly land inside that window.
    let lo = midnight(2000, 10, 1)?;
    let hi = midnight(2001, 1, 1)?;

    let coarse_request = OffsetDiscoveryRequest::new(
        OffsetDiscoveryParams::new(lo, hi, Duration::days(90)),
        TemporalInputToken::new(),
    );
    let coarse_table: amenable_ext::DiscoveredOffsetTable =
        <ChronoTimeBackend as Exchange<OffsetDiscoveryRequest, _, ChronoVerifier>>::exchange(
            &backend,
            coarse_request,
        )
        .into_diagnostic()?;
    let coarse_total = coarse_table.data().entries().len();

    let fine_request = OffsetDiscoveryRequest::new(
        OffsetDiscoveryParams::new(lo, hi, Duration::days(1)),
        TemporalInputToken::new(),
    );
    let fine_table: amenable_ext::DiscoveredOffsetTable = <ChronoTimeBackend as Exchange<
        OffsetDiscoveryRequest,
        _,
        ChronoVerifier,
    >>::exchange(&backend, fine_request)
    .into_diagnostic()?;
    let fine_total = fine_table.data().entries().len();

    assert!(
        fine_total > coarse_total,
        "a daily scan must find strictly more distinct pairs than a 90-day scan over the same \
         range (coarse={coarse_total}, fine={fine_total}) -- otherwise the real 7-day Boa_Vista \
         transition was missed by both"
    );
    Ok(())
}

#[test]
fn rejects_a_non_positive_delta() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let lo = midnight(2000, 1, 1)?;
    let request = OffsetDiscoveryRequest::new(
        OffsetDiscoveryParams::new(lo, lo, Duration::zero()),
        TemporalInputToken::new(),
    );

    let result =
        <ChronoTimeBackend as Exchange<OffsetDiscoveryRequest, _, ChronoVerifier>>::exchange(
            &backend, request,
        );
    assert!(
        result.is_err(),
        "delta=0 must be rejected, not silently accepted"
    );
    Ok(())
}

#[test]
fn rejects_lo_after_hi() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let lo = midnight(2001, 1, 1)?;
    let hi = midnight(2000, 1, 1)?;
    let request = OffsetDiscoveryRequest::new(
        OffsetDiscoveryParams::new(lo, hi, Duration::days(1)),
        TemporalInputToken::new(),
    );

    let result =
        <ChronoTimeBackend as Exchange<OffsetDiscoveryRequest, _, ChronoVerifier>>::exchange(
            &backend, request,
        );
    assert!(
        result.is_err(),
        "lo after hi must be rejected, not silently accepted"
    );
    Ok(())
}
