//! Verus accommodation model for `alloc::rc::Weak<i32>`/`alloc::sync::Weak<i32>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd`'s smart-pointer spec support (`std_specs/
//! smart_ptrs.rs`) covers `Rc::new`/`Arc::new` and their `Deref`, but not
//! `downgrade`/`Weak::upgrade`/the strong/weak reference-count machinery
//! at all. `VerusWeakModel` models `Weak`'s defining law directly: it
//! upgrades successfully while a strong reference is alive, and fails
//! once the strong count reaches zero — the same law
//! `amenable_kani`'s own `Rc`/`Arc` harnesses check against the real
//! types (`Rc::downgrade`/`Weak::upgrade`, `Arc::downgrade`/
//! `Weak::upgrade`). `Rc<i32>::Weak` and `Arc<i32>::Weak` check the
//! identical law in `amenable_kani` (only their thread-safety differs,
//! which this model doesn't address either way), so
//! `amenable_std::verus_witness` reuses this one carrier and harness for
//! both types, the same way `lazy_cell_carrier.rs` backs both `LazyCell`
//! and `LazyLock`. `VerusWeakModel` isn't `Weak` and doesn't claim to be
//! — the proof is conditional: sound if the real `Weak` types refine
//! this law, which `amenable_kani`'s own
//! `verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero`/
//! `verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero`
//! harnesses (checking the real types directly) already confirm
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{observed_value_matches_input, value_unchanged};

verus! {

/// Models `Weak`'s defining law — upgrades while a strong reference is
/// alive, fails once the strong count hits zero — not `Weak` itself.
pub struct VerusWeakModel {
    pub value: i32,
    pub strong_count: u32,
}

impl VerusWeakModel {
    /// A fresh model starts with one live strong reference.
    pub fn new(value: i32) -> (result: Self)
        ensures
            observed_value_matches_input(result.value as int, value as int),
            observed_value_matches_input(result.strong_count as int, 1int),
    {
        Self { value, strong_count: 1 }
    }

    /// Succeeds, exposing the stored value, while a strong reference is
    /// alive; fails once the strong count has reached zero.
    pub fn upgrade(&self) -> (result: Option<i32>)
        ensures
            self.strong_count > 0 ==> result == Some(self.value),
            self.strong_count == 0 ==> result is None,
    {
        if self.strong_count > 0 {
            Some(self.value)
        } else {
            None
        }
    }

    /// Drops the single strong reference this model tracks.
    pub fn drop_strong(&mut self)
        requires
            old(self).strong_count > 0,
        ensures
            final(self).strong_count == old(self).strong_count - 1,
            value_unchanged(old(self).value as int, final(self).value as int),
    {
        self.strong_count -= 1;
    }
}

/// A `Weak` upgrades successfully while a strong reference is alive,
/// exposing the original value, and fails once the last strong
/// reference is dropped — the law the real `Weak` types are expected to
/// refine.
pub fn verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero(value: i32) -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    let mut model = VerusWeakModel::new(value);

    let upgraded = model.upgrade();
    let upgrades_while_alive = upgraded == Some(value);

    model.drop_strong();
    let fails_after_last_strong_ref_drops = model.upgrade().is_none();

    (upgrades_while_alive, fails_after_last_strong_ref_drops)
}

} // verus!
