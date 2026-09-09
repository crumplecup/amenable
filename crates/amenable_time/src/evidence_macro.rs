//! [`temporal_evidence!`] — declares aggregate temporal proof
//! propositions in bulk, the `elicit_temporal`
//! `proof_composition::structural_prop!` analog for the composed
//! (provable, not citation-only) half of the split.
//!
//! Crate-internal (`#[macro_use]` in `lib.rs`), the same as
//! [`temporal_standard!`](crate::temporal_standard). Where
//! `temporal_standard!` mints a [`Standard`](amenable_core::Standard)
//! carrying a [`TemporalProvenance`](crate::TemporalProvenance) citation,
//! this mints a bare [`Evidence`](amenable_core::Evidence) claim — one
//! that gets *proven* (a real `Witness<V>` in Phase 6), not asserted and
//! audited. No provenance record: a provable claim has no citation.

/// Declare one or more aggregate temporal proof propositions.
///
/// Each entry expands to a zero-sized `pub struct` that is
/// [`Evidence`](amenable_core::Evidence) via `#[derive(Evidence)]`
/// (`basis = "Self"`, `Audit = ()`), with an `EvidenceLink`
/// self-registration. These are the propositions a producer proves and a
/// consumer relies on; the `Establish` edges that mint them from
/// lower-order standards land alongside (`docs/AMENABLE_TIME_PLAN.md`,
/// Phase 3).
///
/// ```ignore
/// temporal_evidence! {
///     /// Aggregate proof that a complete calendar date is structurally valid.
///     CalendarDateValid;
///     /// Aggregate proof that an ISO 8601 date representation is structurally valid.
///     DateValid;
/// }
/// ```
macro_rules! temporal_evidence {
    ( $(
        $(#[$doc:meta])*
        $name:ident;
    )* ) => {
        $(
            $(#[$doc])*
            #[derive(
                Debug, Clone, Copy, Default, PartialEq, Eq, Hash,
                ::amenable_derive::Evidence,
            )]
            #[evidence(basis = "Self")]
            pub struct $name;
        )*
    };
}
