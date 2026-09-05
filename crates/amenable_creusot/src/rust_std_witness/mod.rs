//! `CreusotWitness` impls for Rust standard-library carriers.
//!
//! This file used to live in `amenable_std` itself, implementing `Witness<
//! CreusotVerifier>` from that side via an *optional* dependency back on
//! `amenable_creusot` (needed only to name `CreusotVerifier`, the trait's
//! own type parameter). That optional edge was a real problem: `amenable_
//! kani` depends on `amenable_std` unconditionally, so any crate wanting to
//! depend on `amenable_kani` (this one, eventually — see `ledger/`'s own
//! doc comment) would close a real Cargo cycle the moment it also touched
//! `amenable_std`, confirmed directly by a real `cyclic package dependency`
//! error. Moved here instead, matching `ledger/`'s own precedent for
//! `amenable_gaap`: `amenable_creusot` takes a real, ordinary, unconditional
//! Cargo dependency *on* `amenable_std`, and implements `Witness<
//! CreusotVerifier>` directly on the real `RustStdStandard<T>` from this
//! side. No accommodation-model mirror needed — confirmed empirically, not
//! assumed: `creusot-rustc`'s translator only sweeps items *local* to the
//! crate it's directly translating (`amenable_creusot` itself), not an
//! ordinary dependency's own items (a return-position `impl Trait` and a
//! `static`-generating `::inventory::submit!` are real, confirmed ICE
//! triggers when *local*, per this session's own findings — see `ledger/`
//! for the real counter-example and the fix, which still applies here:
//! anything `Vec`/`String`/`Display`-heavy stays `#[cfg(not(creusot))]`).
//!
//! Legal under Rust's orphan rule the *usual* way now (see `amenable_
//! creusot::witness`'s own doc comment): `CreusotVerifier`, the trait's own
//! type parameter, is local to this crate. `RustStdStandard<T>` being local
//! to `amenable_std` instead was the *old* file's own justification for the
//! reverse direction — no longer the mechanism in use, but still confirmed
//! true and still how `ledger/`'s own `Witness<CreusotVerifier> for
//! Validated`/`Committed` impls stay legal (`Validated`/`Committed` are
//! local to `amenable_gaap`, not here).
//!
//! One block per concrete type: a Creusot-checkable property doesn't
//! generalize across types the way provenance does, so there is no blanket
//! impl here — each type gets exactly the contract that's actually true of
//! it. The bridge to `Witness<CreusotVerifier>` is mechanical (delegates
//! straight to `CreusotWitness`), so it's generated per type by a macro
//! rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Creusot to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! contract content to add. `char` and `String` do carry a genuine
//! constraint, so their `proof()` also names the Creusot contract function
//! that checks it, alongside the same chain-derived provenance.
//!
//! Each type also registers a [`amenable_core::ProofRecord`] alongside its
//! `Witness` bridge, so `proof()`'s output is discoverable by name for
//! audit — see `amenable_core::chain::proof_chain`. The registered
//! `evidence` name is a hardcoded module-path literal matching
//! `RustStdStandard`'s own registration in `rust_std`, so both sides agree
//! on the same string without one computing it from the other.
//!
//! A "checked" carrier's [`CheckedProof::claim`] is the contract's own
//! verbatim source (`#[requires]`/`#[ensures]` included), captured via
//! [`amenable_derive::harness!`] *in `amenable_creusot`* — this crate only
//! imports the resulting `&'static str` constant, never the harness
//! function itself, so the claim can never drift from the real contract
//! without also touching the crate that's actually translated.
//!
//! Split by the real std module each file covers: [`trusted_leaf_types`]
//! (the single batch registration of every invariant-free carrier),
//! [`str_carriers`], [`char_carrier`], [`string_carrier`], [`hash`],
//! [`iter_flatten`], [`sync_atomic`], [`alloc_system`], [`backtrace`],
//! [`io_seek_from`], [`net_shutdown`], [`array_and_slice`],
//! [`tuple_fn_raw_pointer`], [`panic_assert_unwind_safe`],
//! [`reference_carriers`], [`cow_carrier`], [`btree`], [`binary_heap`],
//! [`linked_list`], [`vec_deque`], [`env_carriers`], [`os_str`], [`c_str`],
//! [`boxed_carrier`], [`duration_carrier`], [`ops_range_and_bound`],
//! [`num`], [`cmp_carriers`], [`option_and_result`], [`task_carriers`],
//! [`mem_carrier`], and [`os_windows_carrier`] (the Windows-handle cluster
//! that can never get a real impl here, on any platform — see that
//! module's own doc comment).

mod alloc_system;
mod array_and_slice;
mod backtrace;
mod binary_heap;
mod boxed_carrier;
mod btree;
mod c_str;
mod char_carrier;
mod cmp_carriers;
mod cow_carrier;
mod duration_carrier;
mod env_carriers;
mod hash;
mod io_seek_from;
mod iter_flatten;
mod linked_list;
mod mem_carrier;
mod net_shutdown;
mod num;
mod ops_range_and_bound;
mod option_and_result;
mod os_str;
mod os_windows_carrier;
mod panic_assert_unwind_safe;
mod reference_carriers;
mod str_carriers;
mod string_carrier;
mod sync_atomic;
mod task_carriers;
mod trusted_leaf_types;
mod tuple_fn_raw_pointer;
mod vec_deque;

/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    harness: String,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: amenable_std::RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use amenable_core::Provenance;

        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}
