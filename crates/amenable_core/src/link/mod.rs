//! Static self-registration of evidence chain links.
//!
//! `Evidence::basis()` is a compile-time fact about a type, but Rust has no
//! reflection: nothing lets code discover every type implementing `Evidence`
//! across a compiled binary just by asking the type system. `inventory`
//! bridges that gap — each concrete evidence type submits a small
//! descriptor of itself once, and tooling can later walk the whole
//! collection to reconstruct an arbitrary chain without enumerating every
//! type by hand.
//!
//! `inventory::submit!` requires a `const`-evaluable value, which rules out
//! `std::any::type_name` (not yet stable as `const fn`). Build `name` and
//! `basis` from `concat!(module_path!(), "::", stringify!(Type))` instead —
//! both are compile-time text macros, not runtime calls:
//!
//! ```
//! # use amenable_core::EvidenceLink;
//! struct MyStandard;
//!
//! inventory::submit! {
//!     EvidenceLink::new(
//!         concat!(module_path!(), "::", stringify!(MyStandard)),
//!         concat!(module_path!(), "::", stringify!(MyStandard)),
//!         0,
//!     )
//! }
//! ```
//!
//! One file per registered record, grouped by what it carries: a link to a
//! basis type ([`evidence_link`]), a proof's presence ([`proof_record`]), a
//! named contract fragment ([`contract_record`]), a real `Exchange` edge's
//! captured source ([`exchange_edge_record`]), and a proof-token mint
//! ([`proof_token_mint_record`]).

mod contract_record;
mod evidence_link;
mod exchange_edge_record;
mod proof_record;
mod proof_token_mint_record;

pub use contract_record::ContractRecord;
pub use evidence_link::{EvidenceLink, Premise};
pub use exchange_edge_record::ExchangeEdgeRecord;
pub use proof_record::ProofRecord;
pub use proof_token_mint_record::ProofTokenMintRecord;
