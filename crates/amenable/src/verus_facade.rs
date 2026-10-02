//! One ungated re-export surface for every `verus`-feature-gated item
//! this facade exposes.
//!
//! Collapses five scattered, individually `#[cfg(feature = "verus")]`-
//! gated `pub use` lines in `lib.rs` (cordial's CFG-SCATTER finding)
//! into a single gate on this module's own declaration plus one on
//! re-exporting its contents — the fix cordial's own checklist
//! recommends ("extract the gated items into their own module and gate
//! the whole `mod` declaration once instead"). Every re-export here is
//! ungated: compiling this file at all already depends on the feature.

pub use crate::verus_exchange_export::write_verus_exchange_companions;
pub use crate::verus_export::write_verus_witness_modules;
pub use crate::verus_gaap_tokens_export::write_verus_gaap_token_companion;
pub use amenable_core::{VerusVerifier, VerusVerifierMetadata};
pub use amenable_std::{VerusCheckedProof, VerusWitness};
