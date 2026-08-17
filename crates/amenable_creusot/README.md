# amenable_creusot

> Creusot verifier backend for the `amenable` constitutional trait family.

## What this crate is

`CreusotVerifier` and the `CreusotWitness` trait are defined here — there
is only one verifier Creusot works with, Creusot, so the marker belongs
with the crate that means it. The impls bridging `CreusotWitness`/
`Witness<CreusotVerifier>` to concrete std carriers (`RustStdStandard<T>`)
also live here now, in `rust_std_witness.rs` — that surface used to live
in `amenable_std` (`creusot_witness.rs`, behind a `creusot` feature) but
was migrated wholesale once a real, empirically-confirmed finding made
the old split unnecessary: `creusot-rustc`'s whole-crate translation pass
only sweeps items *local* to the crate it's directly translating, not an
ordinary Cargo dependency's own items. `amenable_creusot` now carries a
real, unconditional Cargo dependency on `amenable_std` (the direction
used to be reversed, and optional) with no translator conflict, so the
witness-bridge surface could move to sit alongside the proofs it
witnesses instead of the other way around.

That doesn't mean `creusot-rustc`'s translator has no real constraints —
it chokes on ordinary Rust infrastructure that's completely unremarkable
to plain `rustc`: a return-position `impl Trait` on a local `impl`
panicked its intrinsics-gathering pass outright (a real ICE, confirmed
empirically, not a hypothetical), `Box<dyn Iterator<..>>` as a concrete
associated-type value is rejected outright ("forbidden dyn type... dyn
support is currently minimal"), and `inventory::collect!`/`inventory::
submit!`'s generated `static` items hit "unsupported definition kind" /
"unsupported constant value" the same way — but only for items *local*
to this crate. All three are avoidable with precise
`#[cfg(not(creusot))]` gating *in place*, applied at the actual
definition site, not just at the point of use — a real, separately
confirmed ICE hit twice in this crate when only a re-export or a usage
site was gated, not the struct/impl definition itself.

`rust_std.rs`/`rust_std_witness.rs` hold `amenable_std`'s harness
functions and their witness bridge, respectively; `ledger.rs` holds
`amenable_gaap`'s real, non-trivial Pearlite predicates (`AmountPositive`/
`SufficientFunds`/`AccountsDistinct`/`BalancedEntries`), implemented
directly on the real `Validated`/`Committed` types via a real Cargo
dependency on `amenable_gaap` — no accommodation-model mirror needed,
for the same translator-sweep-scope reason as `amenable_std` above.
`stoplight.rs` is the one exception: `Stoplight`'s tokens/
`Established<T, Token>`/transition bodies still need a hand-written
accommodation-model mirror, because the real constructors they'd
otherwise call (`Established::new`, the token tuple fields) are
deliberately private to `amenable_kani` — the audit-only half of that
file, though, implements `Witness<CreusotVerifier>` directly on the
real `amenable_core::{Green, Yellow, Red}` evidence markers (moved
there specifically so neither this crate nor `amenable_kani` has to
depend on the other — verifier backends never depend on each other,
full stop). `witness.rs` holds the shared trait/marker definitions
these all implement against.

## Coverage

**93 harness registrations** in `rust_std.rs`. Most carry a real,
machine-checked Pearlite `requires`/`ensures` contract, discharged by SMT
via `cargo creusot prove` — confirmed by actually running the prover
locally, not just the translator. Where `creusot-std` has no contract
surface for the underlying std call (or the call touches OS-backed state
Creusot can't reason about at all), the harness states the same claim
under an explicit `#[trusted]` boundary instead of quietly claiming
coverage it doesn't have.

`std::os::windows::*` types can't even be named here at all —
`creusot-rustc` has no Windows target, and its whole-crate translator
can't tolerate ordinary `inventory::submit!`-based witness wiring left
ungated (see above). That cluster's four harnesses instead prove a law
over a synthetic `isize`/`u64`/`u32` model — real and fully
Creusot-proved, not a `#[trusted]` stub — with the evidence hand-linked
to the real types' registrations by string in `rust_std_witness.rs`,
mirroring `amenable_kani::os_windows_model`'s identical bypass for the
identical reason (Kani/CBMC don't run on Windows either).

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_std`](../amenable_std/README.md) for `RustStdType`'s own
  registrations, and overall coverage across all three verifiers.
- [`amenable_gaap`](../amenable_gaap/README.md) for the GAAP ledger's
  own evidence types this crate's `ledger.rs` proves against.
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for the other two
  backends.
