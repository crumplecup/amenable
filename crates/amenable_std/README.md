# amenable_std

> `RustStdType`: interface and concrete registrations for Rust
> standard-library types.

## What this crate is

Traits meant to be implemented directly on foreign standard-library types
must live in the crate that defines them — Rust's orphan rules leave no
other option, since neither the trait nor `bool`/`i32`/`String`/etc. is
local anywhere else. So rather than an interface crate plus a downstream
consumer, this crate defines `RustStdType` and its full std-lib coverage
together, alongside the default concrete certificate and registry types,
serving as the canonical gold-standard registrations other crates can
depend on instead of re-registering the same std types themselves.

This is also where two of the three verifier backends' witnesses
converge: `amenable_kani` and `amenable_verus` are each architecturally
constrained to stay pure proof content (Kani/CBMC's symbolic-execution
model and Verus's bare-compiler invocation each rule out ordinary
bridging machinery living alongside the proofs themselves — see each
crate's own README for the specifics). The bridge from each of those
proofs to a concrete `RustStdStandard<T>` — the `Witness` impl, the
registered `ProofRecord` — lives here instead, in `rust_std/` and
`verus_witness/` (behind the `verus` feature; both grew from a single
file into a directory as coverage widened, one file per carrier
cluster). Creusot's own bridge used to live here too
(`creusot_witness.rs`, behind a `creusot` feature), but
`creusot-rustc`'s translator turns out not to share Kani's/Verus's
constraint — it only sweeps items *local* to the crate it's directly
translating, not an ordinary Cargo dependency's own items — so that
~90-carrier surface moved wholesale into `amenable_creusot::
rust_std_witness` instead, a real Cargo dependency on this crate rather
than the reverse; see that crate's own README.

## Coverage

Per the project's own audit tool (`cordial coverage --crate-name
amenable_std`), **422 of 457 accountable stable `std`/`core` types
(92.3%) have complete evidence plus every applicable verifier's
witness**, as of this writing — regenerate via that command (or read
`std.checklist.md` in cordial's own findings store) for a current
count rather than trusting this one; the accountable-type universe
grows as the standard library does, and this snapshot will drift. Of
the 35 open items:

- 13 are confirmed false-negatives in the audit tool's own type-alias
  resolution (`core::num::NonZero*`, `core::alloc::LayoutErr`) — real
  Verus coverage already exists
  (`amenable_verus::rust_std::num::non_zero_carrier`), confirmed by
  direct compile-test, not assumed.
- 6 are the `std::os::windows` cluster (`EncodeWide`, `BorrowedHandle`,
  `BorrowedSocket`, `HandleOrInvalid`, `OwnedHandle`, `OwnedSocket`),
  whose Verus proofs are real (`rust_std/os_windows.rs`'s
  `#[cfg(windows)]` registrations, `amenable_verus::rust_std::
  os_windows_carrier`'s `assume_specification` axioms) but only
  checkable on the `verus-windows` GitHub Actions workflow
  (`workflow_dispatch`-triggered, `windows-latest`), never on this
  crate's Linux development host. Kani and Creusot already cover this
  same cluster — see their own READMEs for how, given that neither of
  those two verifiers can run on Windows at all.
- The remaining 16 (the `core::range::*` family, `core::slice::
  ArrayWindows`, three `libc`/`std::os::unix` raw-type aliases,
  `std::os::linux::raw::stat`, and `std::os::unix::prelude::
  {Borrowed,Owned}Fd`) are open, real backlog — not yet triaged against
  either category above, unlike the two groups documented before this
  paragraph existed.

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_creusot`](../amenable_creusot/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for each backend's own
  coverage and architecture.
