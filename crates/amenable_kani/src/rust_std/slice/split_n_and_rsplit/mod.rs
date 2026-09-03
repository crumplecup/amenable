//! `KaniWitness` impls for the count-limited and reverse slice splitters,
//! split by direction/limit: `split_n` (`SplitN`/`SplitNMut`), `rsplit`
//! (`RSplit`/`RSplitMut`), and `rsplit_n` (`RSplitN`/`RSplitNMut`). Each is
//! proved through an Amenable-owned split observation rather than the
//! direct std iterator.

mod rsplit;
mod rsplit_n;
mod split_n;

// `slice::split`'s `SplitMut` harness reuses this SplitN harness's verbatim
// source as its own `claim` -- see that harness for why.
pub use split_n::VERIFY_SPLIT_N_CAPS_THE_NUMBER_OF_PIECES_SRC;
