//! `contracts::iso_8601` — spot-checks that the ported group registers as
//! ISO-sourced `Standard`s carrying `ParaphraseOnly` provenance (tier C:
//! no ISO prose reproduced) with a real section pointer.

use amenable_core::{ErasedEntry, Metadata, Standard};
use amenable_time::{
    InstantIsPointOnTimeAxis, TimeAxisOrdersTimePointsByTemporalPosition,
    TimeOfDayOccursWithinCalendarDay,
};

#[test]
fn definitional_group_is_iso_sourced_and_paraphrase_only() {
    amenable_core::init_tracing();

    for report in [
        TimeAxisOrdersTimePointsByTemporalPosition
            .report()
            .to_string(),
        InstantIsPointOnTimeAxis.report().to_string(),
        TimeOfDayOccursWithinCalendarDay.report().to_string(),
    ] {
        assert!(
            report.contains("normative_document: ISO 8601-1:2019"),
            "{report}"
        );
        assert!(report.contains("standards_body: ISO"), "{report}");
        assert!(report.contains("normative_status: normative"), "{report}");
        assert!(
            report.contains(
                "normative_quotation: (paraphrase only — licensed source; see semantic_summary)"
            ),
            "tier-C contract must not embed verbatim ISO text:\n{report}"
        );
    }
}

#[test]
fn each_contract_names_its_clause() {
    amenable_core::init_tracing();
    let provenance = InstantIsPointOnTimeAxis.provenance();

    assert_eq!(
        provenance
            .get("normative_section")
            .expect("section present")
            .value()
            .to_string(),
        "3.1.1.3"
    );
}
