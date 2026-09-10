//! `amenable temporal-coverage` — the per-contract proof-coverage report
//! over the temporal contract graph. The facade links `amenable_kani`
//! always and `amenable_creusot` / the Verus witnesses under their
//! features, so which columns are populated depends on the build.

use amenable::TemporalCoverage;

#[test]
fn the_report_covers_every_atomic_temporal_contract() {
    amenable::init_tracing();
    let coverage = TemporalCoverage::snapshot();

    // 322 citation-only structural contracts + 23 machine-checked.
    assert_eq!(coverage.total(), 345, "the atomic temporal contract count");
    assert_eq!(
        coverage.checked() + coverage.citation_only(),
        coverage.total()
    );

    // Kani is always linked, and every machine-checked contract has a Kani
    // harness, so 23 rows are checked regardless of the other features.
    assert_eq!(coverage.checked(), 23, "machine-checked atomic contracts");

    let row = |name: &str| {
        coverage
            .rows()
            .iter()
            .find(|row| row.contract() == name)
            .unwrap_or_else(|| panic!("`{name}` should be in the coverage report"))
    };

    let month = row("CalendarMonthInRangeOneToTwelve");
    assert!(month.is_checked() && month.kani());

    let structural = row("CalendarDateUsesGregorianCalendar");
    assert!(!structural.is_checked(), "a shape fact stays citation-only");
}

#[cfg(feature = "creusot")]
#[test]
fn creusot_proves_every_machine_checked_contract() {
    amenable::init_tracing();
    let coverage = TemporalCoverage::snapshot();
    for row in coverage.rows().iter().filter(|row| row.is_checked()) {
        assert!(
            row.creusot(),
            "{} is machine-checked but has no Creusot proof",
            row.contract()
        );
    }
}
