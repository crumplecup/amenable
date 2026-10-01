//! Extended-year/decade/century emission-proof composites.
//!
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{CenturyValid, DecadeValid, ExtendedYearValid};

/// Emission proof for [`FormattedExtendedYear`](crate::FormattedExtendedYear) — the `format_extended_year` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ExtendedYearFormatted {
    /// The `extended_year_valid` sub-claim.
    extended_year_valid: ExtendedYearValid,
}
/// Emission proof for [`FormattedDecade`](crate::FormattedDecade) — the `format_decade` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct DecadeFormatted {
    /// The `decade_valid` sub-claim.
    decade_valid: DecadeValid,
}
/// Emission proof for [`FormattedCentury`](crate::FormattedCentury) — the `format_century` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct CenturyFormatted {
    /// The `century_valid` sub-claim.
    century_valid: CenturyValid,
}
