//! The ASCII-byte-range precondition, named once instead of restated per
//! verifier and per call site.
//!
//! "This value is at most 127" is independently hand-written at 24 real
//! sites across all three verifier backends — five plain `kani::assume`
//! calls (`amenable_kani`), eighteen Verus `requires` clauses across six
//! carrier files (`amenable_verus`), and one Creusot `#[requires]`
//! attribute (`amenable_creusot`) — every one of them stating the same
//! precondition ("only a single-UTF-8-byte character is being tested
//! here") under a different local variable name. `AsciiByte` names this
//! contract type in the `amenable_core::Ensures`/`Requires` worklist: it
//! doesn't add a new proof, it names an existing precondition, once, even
//! though it needs several literal fragment spellings (one per variable
//! name actually used) to cover every real site.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A `u8` known to be in the ASCII range (`< 128`) — the precondition
/// every symbolic single-byte-character proof in this workspace assumes
/// before treating a byte as a one-UTF-8-byte `char`.
///
/// A derived claim about `u8`, not a fresh root authority — its evidence
/// chain rests on `u8`'s own already-registered standard-library
/// provenance ([`RustStdStandard<u8>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct AsciiByte {
    value: u8,
}

impl AsciiByte {
    /// Wrap a `u8` already known to be in the ASCII range.
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// The wrapped byte value.
    pub const fn value(&self) -> u8 {
        self.value
    }
}
