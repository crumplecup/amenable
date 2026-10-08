//! JSON-serializable shapes for [`crate::EvidenceLink`] and its premises.

/// One [`crate::EvidenceLink`], owned for JSON serialization. Concrete links
/// serialize `bounds` and `premises` as empty; generic links carry both.
#[derive(serde::Serialize)]
pub(super) struct EvidenceLinkDump {
    pub(super) name: String,
    pub(super) basis: String,
    pub(super) index: usize,
    pub(super) bounds: Vec<String>,
    pub(super) premises: Vec<PremiseDump>,
}

/// One [`crate::Premise`], owned for JSON serialization.
#[derive(serde::Serialize)]
pub(super) struct PremiseDump {
    pub(super) id: String,
    pub(super) statement: String,
}
