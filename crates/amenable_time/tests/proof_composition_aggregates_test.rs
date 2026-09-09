//! `proof_composition::aggregates` — spot-checks that the 93 aggregate
//! proof propositions land as `Evidence` roots (not `Standard`s — they
//! carry no `Provenance`; they get *proven*, not cited) and each
//! self-registers an `EvidenceLink`.

use amenable_core::{Evidence, EvidenceLink};
use amenable_time::{CalendarDateValid, TimestampRepresentsFixedInstant};

#[test]
fn an_aggregate_is_an_evidence_root() {
    amenable_core::init_tracing();

    assert!(<CalendarDateValid as Evidence>::is_root());
    assert!(<TimestampRepresentsFixedInstant as Evidence>::is_root());
    // `Audit = ()` — a provable claim has no citation to audit.
    let () = CalendarDateValid.audit();
}

#[test]
fn aggregates_are_not_provenance_backed() {
    amenable_core::init_tracing();

    // The `Standard` half of the split is `contracts::*`; aggregates are
    // plain `Evidence`. A compile-time check: `CalendarDateValid` has no
    // `Provenance` associated type, so this module can't name one. The
    // runtime shape check is that `basis()` round-trips the ZST.
    assert_eq!(<CalendarDateValid as Evidence>::basis(), CalendarDateValid);
}

#[test]
fn every_aggregate_self_registers_an_evidence_link() {
    amenable_core::init_tracing();

    let registered: Vec<&str> = inventory::iter::<EvidenceLink>()
        .map(EvidenceLink::name)
        .filter(|name| name.contains("proof_composition::aggregates::"))
        .collect();

    assert_eq!(
        registered.len(),
        93,
        "got {}: {registered:?}",
        registered.len()
    );
}
