//! Inventory records for executable Kani proof harnesses.

/// One executable Kani proof harness compiled into `amenable_kani`.
///
/// This record is a catalog entry, not a verification result. It says which
/// proof Kani can run; the CLI's CSV ledger records whether a particular run
/// later passed, failed, or timed out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniProof {
    /// Stable, fully-qualified proof identifier.
    pub id: String,
    /// Exact Kani harness selector.
    pub harness: String,
    /// Cargo package containing the library target that defines this harness.
    pub package: String,
}

/// Static registration that constructs an owned [`KaniProof`] on discovery.
///
/// `inventory` entries must be static, so the registration holds only a
/// function pointer. The CLI calls it to obtain owned strings; no proof data
/// is represented by `&'static str` fields.
pub struct KaniProofRegistration {
    /// Construct this registration's executable Kani proof descriptor.
    pub proof: fn() -> KaniProof,
}

inventory::collect!(KaniProofRegistration);
