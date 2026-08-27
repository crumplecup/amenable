//! `#[amenable_derive::evidence]` on a hand-written `impl Evidence`
//! block: computes `is_root()` from the block's own `Basis` declaration,
//! leaving everything else exactly as authored.

use amenable_core::Evidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RootEvidence;

#[amenable_derive::evidence]
impl Evidence for RootEvidence {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

#[test]
fn bare_evidence_attribute_computes_is_root_from_basis() {
    amenable_core::init_tracing();
    assert!(RootEvidence::is_root());
}
