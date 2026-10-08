//! JSON-serializable shapes for [`crate::EvidenceLink`] and its premises.

/// One [`crate::EvidenceLink`], owned for JSON serialization. Concrete links
/// serialize `bounds` and `premises` as empty; generic links carry both.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct EvidenceLinkDump {
    name: String,
    basis: String,
    index: usize,
    bounds: Vec<String>,
    premises: Vec<PremiseDump>,
}

/// One [`crate::Premise`], owned for JSON serialization.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct PremiseDump {
    id: String,
    statement: String,
}
