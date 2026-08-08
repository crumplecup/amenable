//! The argv-includes-the-program-path precondition/postcondition pair,
//! named once instead of restated per real type.
//!
//! `env::args()` and `env::args_os()` both guarantee the identical
//! shape: the process's own argv always has at least one element -- the
//! program's own slot -- so neither ever yields an empty sequence, and
//! the reported count is exactly one more than however many extra
//! arguments were supplied. Independently hand-written at two real
//! Creusot proof sites (`amenable_creusot::rust_std::{
//! verify_args_reports_at_least_the_program_path,
//! verify_args_os_reports_at_least_the_program_path}`) -- one `requires`
//! headroom precondition and two `ensures` clauses, restated verbatim at
//! both. `ArgvIncludesProgramPath` is the ninth contract type in the
//! `amenable_core::Ensures`/`Requires` worklist.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A count of "extra" arguments beyond the program's own path, known to
/// leave enough headroom below `usize::MAX` for `1 + extra` to compute
/// without overflow, and to produce an argv report that always includes
/// at least the program path.
///
/// A derived claim about `usize`, not a fresh root authority -- its
/// evidence chain rests on `usize`'s own already-registered standard-
/// library provenance ([`RustStdStandard<usize>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<usize>",
    basis_ctor = "RustStdStandard::<usize>::new()",
    provenance = "<usize as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ArgvIncludesProgramPath {
    extra: usize,
}

impl ArgvIncludesProgramPath {
    /// Wrap a count of extra arguments already known to have enough
    /// headroom for `1 + extra`.
    pub const fn new(extra: usize) -> Self {
        Self { extra }
    }

    /// The wrapped extra-argument count.
    pub const fn extra(&self) -> usize {
        self.extra
    }
}
