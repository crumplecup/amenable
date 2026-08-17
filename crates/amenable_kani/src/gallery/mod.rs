//! Proof gallery: executable Kani experiments for verifier behavior itself.
//!
//! Production proofs answer "does this harness establish the intended claim?"
//! The gallery answers a different question: "what does Kani do with this
//! modeling pattern?" When a production-proof refinement fails, times out, or
//! raises a new modeling hypothesis, reduce that question here first.
//!
//! Each gallery case should document:
//!
//! - the verifier pattern under test
//! - the expected outcome (`passed`, `failed`, or `timeout`)
//! - whether the pattern is a `hypothesis`, `false_trail`, or `best_practice`
//!
//! Unlike the production queue, a gallery case whose expected outcome is
//! `failed` can still be a successful experiment if that failure is exactly
//! the behavior we needed to confirm.

mod atomic_ptr_compare_exchange;
mod derive_witness_generic_enum;
mod filesystem_observation_granularity;
mod iter_materialization;
mod ledger_account_id_comparison;
mod ledger_commit_contract_timeout;
mod replace_recommendations;
mod slice_escape_ascii;
mod slice_split_position;
mod string_drain;
mod utf8_validation_algorithm_cost;
mod vacuity;
