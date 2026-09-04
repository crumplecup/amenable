//! `KaniWitness` impls for `core::convert`.
//!
//! `Infallible` is uninhabited — there is no value to construct and
//! therefore nothing for a harness to check a property of. This is the
//! canonical "trusted" case: the type system already guarantees everything
//! there is to guarantee, so `proof()` is chain-derived provenance alone.

use std::convert::Infallible;

use crate::rust_std::impl_kani_witness_trusted;

impl_kani_witness_trusted!(Infallible);
