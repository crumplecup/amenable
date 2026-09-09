//! `proof_composition` — spot-checks that the 93 aggregate `*Valid`
//! propositions land as composite `Evidence` (their fields are the
//! sub-claims folded in from the `elicit_temporal` `*Evidence` bundle)
//! and each self-registers an `EvidenceLink`. Leaf `Witness<V>` impls for
//! the member `Standard`s live in the backend crates, so `Witness`
//! resolution is not exercised here.

use amenable_core::{Evidence, EvidenceLink};
use amenable_time::{CalendarDateValid, DateValid, ZonedDateTimeHasNamedZone};

#[test]
fn a_folded_aggregate_is_a_composite_evidence_root() {
    amenable_core::init_tracing();

    // `CalendarDateValid`'s fields ARE its five sub-claims — the
    // `ProvableFrom<CalendarDateEvidence>` edge became field containment.
    let valid = CalendarDateValid::default();
    let () = valid.audit();
    assert!(<CalendarDateValid as Evidence>::is_root());
    assert_eq!(
        <CalendarDateValid as Evidence>::basis(),
        CalendarDateValid::default()
    );
}

#[test]
fn a_multi_credential_aggregate_is_an_enum_over_its_decompositions() {
    amenable_core::init_tracing();

    // `ZonedDateTimeHasNamedZone` had two `ProvableFrom` credentials, so
    // it is an enum with one variant per decomposition.
    let z = ZonedDateTimeHasNamedZone::default();
    assert!(matches!(
        z,
        ZonedDateTimeHasNamedZone::NamedZoneAttachment(_)
    ));

    // `DateValid` folds an `elicit_temporal` branch enum.
    assert!(matches!(DateValid::default(), DateValid::Calendar { .. }));
}

#[test]
fn every_aggregate_self_registers_an_evidence_link() {
    amenable_core::init_tracing();

    let registered: Vec<&str> = inventory::iter::<EvidenceLink>()
        .map(EvidenceLink::name)
        .filter(|name| name.contains("proof_composition::composites"))
        .collect();

    assert_eq!(
        registered.len(),
        93,
        "got {}: {registered:?}",
        registered.len()
    );
}
