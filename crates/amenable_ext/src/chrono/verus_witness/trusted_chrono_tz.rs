//! Trusted `chrono_tz::TzOffset` -- nothing beyond `Evidence::
//! basis().audit()` to rest on. Gated separately from `trusted`'s
//! `chrono::Utc`/`chrono::Local` since it needs the `chrono-tz` feature
//! specifically, not just `chrono`.
//!
//! `TzOffset` has no public constructor and no domain to range over --
//! see `amenable_kani::chrono::tz_offset`'s own doc comment for the full
//! reasoning (its own accessors are a restatement of the type's field
//! layout, true by construction; the real "is this offset consistent"
//! question is already checked for real by `amenable_ext::chrono::
//! zone`'s own `offset_is_consistent_with_named_zone`).

use crate::ExtProvenance;
use crate::ExtStandard;
use crate::ext_verus_witness::impl_verus_witness_trusted_ext;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_trusted_ext!(chrono_tz::TzOffset);
