//! JSON-serializable shapes for [`crate::ProofRecord`] and
//! [`crate::ContractRecord`].

/// One [`crate::ProofRecord`], owned for JSON serialization. Never
/// invokes `describe()` — external tooling needs presence/absence per
/// `(evidence, verifier)`, not the rendered proof text, and calling every
/// registered `describe()` would be needlessly slow for a coverage check.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct ProofRecordDump {
    evidence: String,
    verifier: String,
}

/// One [`crate::ContractRecord`], owned for JSON serialization. Unlike
/// [`ProofRecordDump`], this carries the fragment text itself: external
/// tooling comparing real proof-site expressions against registered
/// contracts needs the literal bound, not just a presence/absence flag.
#[derive(serde::Serialize, derive_new::new)]
pub(super) struct ContractRecordDump {
    evidence: String,
    verifier: String,
    kind: String,
    fragment: String,
}
