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
#[cfg(feature = "chrono")]
mod chrono_day_count_staging;
#[cfg(feature = "chrono")]
mod chrono_day_span_growth;
#[cfg(feature = "chrono")]
mod chrono_month_span_growth;
#[cfg(feature = "chrono")]
mod chrono_naive_week_span;
#[cfg(feature = "chrono")]
mod chrono_naive_week_span_contract_mechanism_test;
#[cfg(feature = "chrono")]
mod chrono_naive_week_span_partition_test;
#[cfg(feature = "chrono")]
mod chrono_naive_week_stub_composition;
#[cfg(feature = "chrono")]
mod chrono_year_partition_exhaustiveness_test;
#[cfg(feature = "chrono")]
mod chrono_year_partition_sweep;
#[cfg(feature = "chrono")]
mod chrono_year_span_growth;
#[cfg(feature = "chrono")]
mod chrono_year_span_with_free_month_day_growth;
mod derive_witness_generic_enum;
mod filesystem_observation_granularity;
mod iter_materialization;
#[cfg(feature = "jiff")]
mod jiff_error_drop_cost;
mod ledger_account_id_comparison;
mod ledger_commit_contract_timeout;
mod ledger_gaap_free_function_contract;
mod replace_recommendations;
mod slice_escape_ascii;
mod slice_split_position;
mod string_drain;
mod udp_inbox_removal_cost;
mod utf8_validation_algorithm_cost;
mod vacuity;
