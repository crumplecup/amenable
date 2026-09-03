//! `NonZero<T>` `KaniWitness` impls, split into `signed` / `unsigned` per-width
//! harness blocks and the shared `contracts` module (the `value != 0`
//! construction precondition and the `NonZeroGetRoundTrips` accessor
//! postcondition).

mod contracts;
mod signed;
mod unsigned;

pub use contracts::NonZeroGetRoundTrips;
