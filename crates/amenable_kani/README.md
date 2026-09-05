# amenable_kani

> Kani verifier backend for the `amenable` constitutional trait family.

## What this crate is

`KaniVerifier` is defined here, not in `amenable_core` — there is only
one verifier Kani works with, Kani, so the marker belongs with the crate
that means it. That locality is what makes
`impl amenable_core::Witness<KaniVerifier> for amenable_std::RustStdStandard<T>`
legal under Rust's orphan rule, one concrete type at a time: the rule
requires *some* type in `Witness<KaniVerifier>`'s type list to be local,
and `KaniVerifier` now is. A blanket impl over a bare type parameter
still isn't legal (the parameter itself is never "covered"), which is
why each type gets its own `KaniWitness` impl plus a one-line mechanical
bridge, rather than one generic impl for all of them — see `rust_std/`.

## Coverage

**445 harnesses** as of this writing (356 tracked production proofs via
`amenable verify kani --list`, plus 89 proof-gallery cases via
`amenable gallery list`) — regenerate via those two commands for a
current count rather than trusting this one, since this crate is still
growing. `rust_std/`'s std-lib coverage accounts for most of it, plus
this crate's worked-example/gallery material (`calculator.rs`,
`stoplight.rs`, `compose/`, and friends).

Where a real API's timing, platform gating, or state-space size puts a
direct proof out of CBMC's reach, an Amenable-owned accommodation model
states the same law over a Linux-compilable stand-in instead —
`fs_model/`, `hash_model.rs`, `btree_model.rs`, `utf8_model.rs`, and
several others each document their own specific reason CBMC can't take
the direct real-type path. The proof's evidence is hand-linked to the
real type's registration by string rather than by naming the type.

`os_windows_model.rs` is the extreme case of that pattern:
`std::os::windows::*` types are `#[cfg(windows)]`-gated in std itself, so
they can't be named at all on this crate's Linux-only development/CI
host — not a matter of CBMC being unable to reason about them, but of
the type genuinely not existing in this compilation. The model plus
hand-linked evidence is the only way to supply a Kani witness for them at
all.

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_std`](../amenable_std/README.md) for the registry these
  witnesses bridge into, and overall coverage across all three verifiers.
- [`amenable_creusot`](../amenable_creusot/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for how the other two
  backends handle the same `std::os::windows` cluster.
