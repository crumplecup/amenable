//! JSON-serializable shape for [`crate::KaniProof`].

/// One [`crate::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize)]
pub(super) struct KaniProofDump {
    pub(super) id: String,
    pub(super) harness: String,
    pub(super) package: String,
}
