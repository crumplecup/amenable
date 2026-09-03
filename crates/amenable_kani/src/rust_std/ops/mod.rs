//! `KaniWitness` impls for `core::ops`, split into `ranges` (the six range
//! types), `bound` (`Bound` + `BoundHasNoEndpoint`), and `control_flow`
//! (`ControlFlow` + its two variant-check claim types).

mod bound;
mod control_flow;
mod ranges;
