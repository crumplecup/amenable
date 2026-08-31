//! Verus accommodation model for `core::cell::LazyCell<i32, fn() -> i32>`.
//!
//! Same zero-`vstd`-coverage gap the other cell-family carriers document.
//! `LazyCell`'s distinguishing law is deferred, run-once initialization:
//! the initializer only ever runs on the first deref, and every later
//! deref returns the cached result instead of re-invoking it. Modeling
//! genuine deferred closure invocation is out of scope for a small
//! accommodation model (Verus's exec function-pointer/closure support
//! isn't exercised elsewhere in this crate), so `VerusLazyCellModel`
//! instead takes each call's *already-computed* value as a parameter and
//! proves the caching law directly: `force` ignores every `computed`
//! argument after the first and returns the originally cached value —
//! the same nondeterminism trick `amenable_kani`'s own
//! `verify_lazy_cell_caches_its_initializer_result` harness uses
//! (`kani::any()` inside the initializer: if the two derefs' results
//! ever differed, that would mean the initializer ran twice). Two calls
//! to `force` with two *different* argument values still returning the
//! same result is exactly what "ran once, cached" means.
//! `VerusLazyCellModel` isn't `LazyCell` and doesn't claim to be — the
//! proof is conditional: sound if the real `LazyCell` refines this law,
//! which `amenable_kani`'s harness (checking the real
//! `LazyCell<i32, fn() -> i32>` directly) already confirms
//! independently, for the identical claim.
//!
//! `std::sync::LazyLock<i32, fn() -> i32>` checks its own real Kani
//! harness against the identical run-once-and-cache claim (only its
//! thread-safety is different, which this model doesn't address either
//! way), so `amenable_std::verus_witness` reuses this same carrier and
//! harness for both types rather than duplicating the model.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Models `LazyCell`'s run-once, cache-the-result law — not `LazyCell`
/// itself, and not real deferred closure invocation.
pub struct VerusLazyCellModel {
    pub cached: Option<i32>,
}

/// A freshly-constructed model has no cached value yet.
pub open spec fn lazy_cell_uninitialized_has_no_cached_value(result: VerusLazyCellModel) -> bool {
    result.cached is None
}

/// `force`'s first-call conjunct: an uncached cell stores and returns
/// exactly the given `computed` value.
pub open spec fn force_caches_on_first_call(old_cached: Option<i32>, computed: i32, result: i32, new_cached: Option<i32>) -> bool {
    old_cached is None ==> result == computed && new_cached == Some(computed)
}

/// `force`'s later-call conjunct: an already-cached cell returns the
/// cached value unchanged, ignoring whatever `computed` argument this
/// call passed.
pub open spec fn force_returns_cached_value_on_later_calls(old_cached: Option<i32>, result: i32, new_cached: Option<i32>) -> bool {
    old_cached is Some ==> result == old_cached->0 && new_cached == old_cached
}

impl VerusLazyCellModel {
    /// An unforced cell has no cached value yet.
    pub fn uninitialized() -> (result: Self)
        ensures
            lazy_cell_uninitialized_has_no_cached_value(result),
    {
        Self { cached: None }
    }

    /// The first call stores and returns `computed`; every later call
    /// ignores its own `computed` argument and returns the value the
    /// first call cached instead.
    pub fn force(&mut self, computed: i32) -> (result: i32)
        ensures
            force_caches_on_first_call(old(self).cached, computed, result, final(self).cached),
            force_returns_cached_value_on_later_calls(old(self).cached, result, final(self).cached),
    {
        match self.cached {
            Some(value) => value,
            None => {
                self.cached = Some(computed);
                computed
            },
        }
    }
}

/// Forcing the model twice with two different computed values still
/// yields the same result both times — the initializer only ever runs
/// once, and later derefs return the cached result — the law the real
/// `LazyCell<i32, fn() -> i32>` is expected to refine.
pub fn verify_lazy_cell_model_caches_its_initializer_result(first_computed: i32, second_computed: i32) -> (result: bool)
    ensures
        result,
{
    let mut lazy = VerusLazyCellModel::uninitialized();
    let first = lazy.force(first_computed);
    let second = lazy.force(second_computed);
    first == second
}

} // verus!
