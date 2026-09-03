//! `KaniWitness` impls for `alloc::vec`, split into `vec_core` (the
//! `Vec<i32>` carrier, its push/pop harness, and the
//! `VecLengthTracksPushesAndPops` / `PopRecoversTheStoredValue` contract
//! types) and `iterators` (`Drain`/`IntoIter`/`ExtractIf`/`Splice`).

mod iterators;
mod vec_core;

pub use vec_core::{PopRecoversTheStoredValue, VecLengthTracksPushesAndPops};
