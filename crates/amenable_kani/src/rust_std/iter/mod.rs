//! `KaniWitness` impls for `core::iter` adapters.
//!
//! Every harness here checks against a symbolic (`kani::any()`) element
//! value threaded through a single `.next()` step or a small fixed number
//! of steps, rather than a fixed literal example — the claim holds for
//! *any* value the adapter might see, not just one hand-picked case.
//! Closure parameters use bare `fn` pointer types rather than closures,
//! since closures have no nameable type to register evidence against (the
//! same pattern as `LazyCell<i32, fn() -> i32>` in `rust_std::cell`).
//! `Range<i32>` (or `Iter<'static, i32>`/`IntoIter<Range<i32>>` where an
//! adapter wraps an iterator of iterators or a borrowing source) is the one
//! representative source iterator this batch covers, matching the bare
//! names `amenable_std::rust_std::iter`'s evidence registration uses.
//!
//! `Filter`, `FilterMap`, and `FlatMap`'s outer source are the exceptions:
//! their `next()` routes through `Iterator::find`/`try_fold`, and
//! `Range<i32>::try_fold`'s loop bound depends on a runtime comparison
//! between symbolic endpoints that Kani's unwinder cannot conclude is
//! bounded — confirmed still timing out past 500 unwind iterations even for
//! an assumed single-item range. `std::array::IntoIter<i32, 1>` has the same
//! `find`-routed `next()` but a compile-time loop bound, so it resolves
//! immediately. See `gallery::iter_materialization` for the isolated
//! experiment and `gallery::replace_recommendations` for the direct
//! `Range<i32>`-sourced false trail this replaces.
//!
//! Split by adapter family, in roughly the same order the real
//! `core::iter` adapters are introduced: [`map_filter_flatten`],
//! [`chain_zip_enumerate`], [`copied_cycle_fuse`], [`scan_skip_step_take`],
//! [`take_while_map_while_once`], and [`repeat_empty_successors`].

mod chain_zip_enumerate;
mod copied_cycle_fuse;
mod map_filter_flatten;
mod repeat_empty_successors;
mod scan_skip_step_take;
mod take_while_map_while_once;

pub use map_filter_flatten::{
    IteratorMatchesReferenceStepByStep, VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC,
};
pub use repeat_empty_successors::IteratorYieldsNoneWhenExhausted;
