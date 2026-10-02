/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    harness: String,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: amenable_std::RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use amenable_core::Metadata;

        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}
