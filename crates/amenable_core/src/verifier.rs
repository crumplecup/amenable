//! Verifier backends recognized by the constitutional proof surface.

use crate::Provenance;

/// Marker trait for a verifier backend.
///
/// `amenable_core` names the interface only, never a concrete backend —
/// concrete verifier markers (Kani, Creusot, Verus, ...) live in their own
/// downstream crates. Rust's orphan rule requires it: bridging
/// `Witness<V>` to a foreign evidence type (like `amenable_std`'s
/// `RustStdStandard<T>`) is only legal when `V` is local to the crate
/// doing the bridging, so each verifier's marker has to be defined where
/// it's used, not centralized here.
pub trait Verifier: 'static {
    /// Structured reporting surface for backend-specific provenance and
    /// configuration disclosure.
    type Metadata: Provenance + Default;

    /// Canonical backend name for audit and display purposes.
    fn name() -> &'static str;

    /// Provenance metadata describing the verifier backend and its reporting
    /// surface. Query the result through the [`Provenance`] interface
    /// (`iter`, `get`, `contains_key`, ...) rather than expecting a
    /// particular collection back.
    fn metadata() -> Self::Metadata {
        Self::Metadata::default()
    }
}
