//! The non-nul-byte precondition, named once instead of restated per
//! verifier and per call site.
//!
//! "This payload byte is not the C-string terminator" is independently
//! hand-written at 16 real sites across all three verifier backends — six
//! plain `kani::assume` calls (`amenable_kani::rust_std::{ffi,alloc_ffi}`),
//! four Verus `requires` clauses (`amenable_verus::rust_std::{cstr_carrier,
//! cstring_carrier, from_vec_with_nul_carrier}`), and six Creusot
//! `#[requires]` attributes (`amenable_creusot::rust_std`) — every one of
//! them under the identical local variable name `byte`, restating the same
//! precondition every `CStr`/`CString`-family proof in this workspace
//! assumes about its payload byte. `NonNulByte` names this contract type
//! in the `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A `u8` known not to be the C-string terminator (`!= 0`) — the
/// precondition every `CStr`/`CString`-family proof in this workspace
/// assumes about a payload byte before treating it as content rather than
/// a terminator.
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
pub struct NonNulByte {
    value: u8,
}

impl NonNulByte {
    /// Wrap a `u8` already known not to be the C-string terminator.
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// The wrapped byte value.
    pub const fn value(&self) -> u8 {
        self.value
    }
}
