//! Trusted `chrono::Utc` -- nothing beyond `Evidence::basis().audit()`
//! to rest on.
//!
//! Unlike `FixedOffset` (a real round-trip over a bounded `i32`, worth a
//! hand-verified model), `Utc`'s own claim has zero input-dependent
//! content for Verus to model: its local offset is unconditionally
//! `Single`, and its fixed offset is unconditionally zero, for every
//! input, with no domain to range over at all. A Verus model of a
//! constant fact would be the same tautology this crate's own policy
//! (see `amenable_kani::chrono::utc`'s doc comment, where the identical
//! reasoning led to a real `extern_spec!` against the trait impl instead
//! -- an escape hatch Verus has no equivalent of, since it never reaches
//! chrono's actual code at all) explicitly rejects building a thin fake
//! model for. `amenable_kani::chrono::utc` and `amenable_creusot::
//! ext_chrono::utc` already check this claim against chrono's real
//! `TimeZone`/`Offset` trait impls directly.

use crate::ExtProvenance;
use crate::ExtStandard;
use crate::ext_verus_witness::impl_verus_witness_trusted_ext;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_trusted_ext!(chrono::Utc);
