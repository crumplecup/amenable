//! Temporal contract interface: standards-anchored propositions for date,
//! time, offset, and timestamp interchange, expressed in `amenable`'s
//! trait family.
//!
//! Every citation-only contract is a [`Standard`](amenable_core::Standard)
//! carrying a [`TemporalProvenance`] record that cites its normative
//! source (ISO 8601, RFC 3339, RFC 9557, CalConnect, IANA TZDB, SI/BIPM,
//! LoC EDTF). Composed / provable propositions are
//! [`Evidence`](amenable_core::Evidence); trait methods become
//! [`Exchange`](amenable_core::Exchange)s. See `docs/AMENABLE_TIME_PLAN.md`
//! for the full design and the migration phases.
//!
//! This crate carries **no checked-in standards corpus** — the relevant
//! normative clause lives embedded in each contract's provenance
//! metadata, governed by a three-tier redistributability rule (see
//! [`provenance_vocab`]). Nothing paywalled is reproduced.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[macro_use]
mod standard_macro;

mod contracts;
mod error;
mod provenance;
mod provenance_vocab;
mod traits;
mod types;

pub use contracts::{
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    PrecisionReductionDeclared, RoundingModeDeclared, SubsecondDigitsPreserved,
};
pub use error::{TemporalError, TemporalErrorKind, TemporalResult};
pub use provenance::TemporalProvenance;
pub use provenance_vocab::{
    CrossCheck, NormativeDocument, NormativeQuotation, NormativeSection, NormativeStatus,
    StandardsBody,
};
pub use traits::TemporalReporter;
pub use types::{SerializationProfile, TemporalComponent};
