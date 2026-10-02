//! `#[derive(Witness)]`: structural closure over already-witnessed members.
//!
//! The derived artifact is a new nominal proof type for the enclosing data
//! shape. Product types fold child proofs into one larger product proof
//! (`product`); sum types fold per-variant proofs into one larger sum proof
//! (`sum`). Shared field/bounds/marker helpers live in `helpers`.

mod expand;
mod helpers;
mod product;
mod sum;

pub use expand::expand_witness;
