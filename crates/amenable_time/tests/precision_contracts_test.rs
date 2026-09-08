//! Phase 0 pattern-setter: the five `contracts::precision` contracts are
//! `Standard` + `Evidence` roots carrying a queryable `TemporalProvenance`,
//! and the three-tier quotation rule holds (ISO → paraphrase only, RFC →
//! verbatim ABNF).

use amenable_core::{ErasedEntry, Evidence, EvidenceLink, Metadata, Standard};
use amenable_time::{
    FractionalSecondPrecisionDeclared, NormativeDocument, NormativeStatus, RoundingModeDeclared,
    SubsecondDigitsPreserved, TemporalProvenance,
};

#[test]
fn a_contract_is_a_provenance_backed_standard() {
    amenable_core::init_tracing();
    let provenance: TemporalProvenance = FractionalSecondPrecisionDeclared.provenance();

    assert_eq!(
        provenance.get_as::<NormativeDocument>("normative_document"),
        Some(NormativeDocument::new("ISO 8601-1:2019/Amd 1:2022"))
    );
    assert_eq!(
        provenance.get_as::<NormativeStatus>("normative_status"),
        Some(NormativeStatus::Normative)
    );
    assert_eq!(
        provenance
            .get("normative_section")
            .expect("section entry present")
            .value()
            .to_string(),
        "5.3.1.4"
    );
}

#[test]
fn contracts_are_evidence_roots() {
    amenable_core::init_tracing();
    assert!(<RoundingModeDeclared as Evidence>::is_root());
    assert_eq!(<RoundingModeDeclared as Evidence>::chain().len(), 1);
}

#[test]
fn tier_c_iso_contracts_reproduce_no_prose() {
    amenable_core::init_tracing();
    for report in [
        FractionalSecondPrecisionDeclared.report().to_string(),
        RoundingModeDeclared.report().to_string(),
    ] {
        assert!(
            report.contains(
                "normative_quotation: (paraphrase only — licensed source; see semantic_summary)"
            ),
            "tier-C contract must not embed verbatim ISO text:\n{report}"
        );
        assert!(report.contains("standards_body: ISO"));
    }
}

#[test]
fn tier_a_rfc_contract_embeds_the_verbatim_abnf_and_a_deep_link() {
    amenable_core::init_tracing();
    let provenance = SubsecondDigitsPreserved.provenance();

    assert_eq!(
        provenance
            .get("normative_quotation")
            .expect("quotation entry present")
            .value()
            .to_string(),
        "time-secfrac    = \".\" 1*DIGIT"
    );
    assert_eq!(
        provenance
            .get("source_url")
            .expect("url entry present")
            .value()
            .to_string(),
        "https://www.rfc-editor.org/rfc/rfc3339#section-5.6"
    );
}

#[test]
fn informative_cross_checks_are_projected() {
    amenable_core::init_tracing();
    let provenance = FractionalSecondPrecisionDeclared.provenance();

    assert_eq!(
        provenance
            .get("cross_check_0")
            .expect("cross-check entry present")
            .value()
            .to_string(),
        "RFC 3339 §5.6 (informative)"
    );
}

#[test]
fn every_precision_contract_self_registers_an_evidence_link() {
    amenable_core::init_tracing();
    let registered: Vec<&str> = inventory::iter::<EvidenceLink>()
        .map(EvidenceLink::name)
        .filter(|name| name.contains("contracts::precision::"))
        .collect();

    assert_eq!(registered.len(), 5, "got: {registered:?}");
}
