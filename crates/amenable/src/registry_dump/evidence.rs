//! JSON-serializable shape for [`crate::EvidenceLink`].

/// One [`crate::EvidenceLink`], owned for JSON serialization.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct EvidenceLinkDump {
    name: String,
    basis: String,
    index: usize,
}
