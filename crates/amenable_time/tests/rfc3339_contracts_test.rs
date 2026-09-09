//! `contracts::rfc3339` — tier-A verbatim embedding: an RFC 3339 contract
//! carries the actual clause text and a section-deep rfc-editor link.

use amenable_core::{ErasedEntry, Metadata, Standard};
use amenable_time::{Rfc3339FractionUsesDotSeparator, Rfc3339UnqualifiedLocalTimeForbidden};

#[test]
fn an_abnf_contract_embeds_the_production_verbatim() {
    amenable_core::init_tracing();
    let provenance = Rfc3339FractionUsesDotSeparator.provenance();

    assert_eq!(
        provenance
            .get("normative_quotation")
            .expect("quotation present")
            .value()
            .to_string(),
        "time-secfrac    = \".\" 1*DIGIT"
    );
    assert_eq!(
        provenance
            .get("source_url")
            .expect("url present")
            .value()
            .to_string(),
        "https://www.rfc-editor.org/rfc/rfc3339#section-5.6"
    );
    assert_eq!(
        provenance
            .get("standards_body")
            .expect("body present")
            .value()
            .to_string(),
        "IETF"
    );
}

#[test]
fn a_prose_contract_embeds_the_sentence_verbatim() {
    amenable_core::init_tracing();
    let report = Rfc3339UnqualifiedLocalTimeForbidden.report().to_string();

    assert!(
        report.contains(
            "the interoperability problems of unqualified local time are deemed unacceptable"
        ),
        "{report}"
    );
    // tier A, so NOT the paraphrase-only marker
    assert!(!report.contains("paraphrase only"), "{report}");
}
