//! Extended-year/decade/century parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    CenturyDescriptor, CenturyValidToken, DecadeDescriptor, DecadeValidToken,
    ExtendedYearDescriptor, ExtendedYearValidToken,
};

/// Output sidecar for the `parse_extended_year` exchange: [`ExtendedYearDescriptor`](crate::ExtendedYearDescriptor)
/// plus a token for [`ExtendedYearValid`](crate::ExtendedYearValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ExtendedYearValid", constructor = "pub")]
pub struct ParsedExtendedYear {
    #[sidecar(primary)]
    descriptor: ExtendedYearDescriptor,
    #[sidecar(token)]
    token: ExtendedYearValidToken,
}
/// Output sidecar for the `parse_decade` exchange: [`DecadeDescriptor`](crate::DecadeDescriptor)
/// plus a token for [`DecadeValid`](crate::DecadeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::DecadeValid", constructor = "pub")]
pub struct ParsedDecade {
    #[sidecar(primary)]
    descriptor: DecadeDescriptor,
    #[sidecar(token)]
    token: DecadeValidToken,
}
/// Output sidecar for the `parse_century` exchange: [`CenturyDescriptor`](crate::CenturyDescriptor)
/// plus a token for [`CenturyValid`](crate::CenturyValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::CenturyValid", constructor = "pub")]
pub struct ParsedCentury {
    #[sidecar(primary)]
    descriptor: CenturyDescriptor,
    #[sidecar(token)]
    token: CenturyValidToken,
}
