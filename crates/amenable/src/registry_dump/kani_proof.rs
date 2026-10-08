//! JSON-serializable shape for [`crate::KaniProof`].

/// One [`crate::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct KaniProofDump {
    id: String,
    harness: String,
    package: String,
}
