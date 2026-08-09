# Naming Raw Requires/Ensures Bounds: Design Pattern and Workflow

**Status:** 🔲 Ongoing — mechanism and tooling complete; `amenable_creusot`
fully cleared; `amenable_kani` partially cleared (largest duplicate
clusters resolved, long tail remains); `amenable_verus` not yet started.

**Purpose of this document:** a self-contained handoff so any agent (or
person) can pick this work back up without re-deriving the mechanism,
the tooling, or the lessons learned the hard way.

## The problem

A proof's `requires`/`ensures` bound can be stated two ways:

1. **Raw**, restated as a literal expression at every site that needs it
   (`assert!(byte != 0)`, `#[requires(value@ != 0)]`, `ensures result == c`
   — a different spelling per verifier, and often per site even within one
   verifier).
2. **Named**, as a single `amenable_core::{Ensures, Requires}` contract
   type with one real, callable predicate (a Kani `bool` fn, a Creusot
   `#[logic]` fn, a Verus `spec fn`) that every real site calls directly.

Raw restatement is the failure mode this whole mechanism exists to
retire: the same bound gets typed out slightly differently a dozen times
across a codebase, with no name tying the copies together, no single
place to fix it, and no way to tell "reused claim" from "coincidentally
similar text" without reading every site.

## The design pattern

### Contract types live in `amenable_std` (usually)

A contract type is a plain wrapper struct deriving `amenable_derive::Standard`,
its evidence chain resting on the wrapped primitive's own already-registered
`RustStdStandard<T>` provenance:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct NonNulByte {
    value: u8,
}
```

Two shapes, depending on how many real sites need the bound:

- **Shared claim across unrelated types** (e.g. `IndexingAndLength`,
  `IterYieldsValueOnceThenEnds`): needs its own purpose-built wrapper type,
  since the claim spans multiple real types that don't share a natural
  home.
- **Singleton, tied to one real type**: `impl Ensures<Verifier> for
  RustStdStandard<T>` directly — no new wrapper struct. This is also the
  fallback when a wrapper *would* make sense but the natural carrier's
  `Ensures`/`Requires` slot is already claimed by a different bound (a
  trait can only be implemented once per concrete `Self`+`Verifier` pair
  — see "Gotchas" below).

A handful of contract types live in `amenable_kani` itself instead of
`amenable_std` (e.g. `NonNegativeFd` in `fd_model.rs`) when the bound has
no Creusot/Verus coverage yet and there's no reason to move it upstream
until a second backend needs it.

### Per-verifier predicate shape

- **Kani**: `kani_requires!`/`kani_ensures!` macros
  (`amenable_kani/src/rust_std/macros.rs`) generate a real `impl
  Requires<KaniVerifier>`/`Ensures<KaniVerifier>` with `Bound = bool` —
  the proof site calls `Type::requires(x)`/`Type::ensures(x)` directly;
  that call *is* the check, not a restatement of it. The macro also
  submits a canonical `ContractRecord` (`fragment: || stringify!($expr)`,
  unscoped).
- **Creusot**: a shared `#[logic(open)]` fn (or plain `#[logic]` — see
  Gotchas) wrapped in `amenable_derive::harness!`, called directly from
  every real site; `amenable_std`'s `Requires<CreusotVerifier>` impl
  returns the harness-captured `_SRC` const verbatim.
- **Verus**: a shared cross-file `spec fn`, same idea.

### `ContractRecord`: two-tier registration

`amenable_core::ContractRecord` (`{ evidence, verifier, kind, fragment,
harnesses }`) is what makes a named bound *discoverable* outside the Rust
binary that defines it — `amenable dump-registry` dumps every registered
record to JSON, which `elicit_doc` reads to check real proof-site text
against.

Every contract type needs **two kinds** of registration:

1. **Canonical** (`harnesses: &[]`, unscoped): the predicate's own body
   text (`stringify!($expr)` for Kani, the harness-captured source for
   Creusot/Verus). Establishes the bound's own definition; matches
   nowhere on its own once real sites stop restating the raw expression.
2. **Supplementary** (`harnesses: &["fn_name", ...]`, scoped): the
   *exact* literal text of the real call site(s) — e.g.
   `"RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0))"`. This
   is what actually silences the scanner, since the scanner compares
   against literal source text, not against the predicate's semantic
   body.

   **Grouping insight**: if the exact same call-site text recurs
   verbatim across multiple harnesses (same local variable names, same
   literal), register it **once** with all those harness names in the
   `harnesses` list, rather than one registration per site. This mirrors
   exactly the duplicate-cluster leverage the checklist itself surfaces
   (see below) — e.g. `amenable_kani`'s `Cell<u32>` drop-counter idiom
   needed only 4 registrations (one per literal count: 0, 1, 2, 3) to
   cover 42 real sites across a dozen otherwise-unrelated harnesses,
   because every site uses the identical variable name `drop_count`.

   `ContractIndex::matches` (in elicit_doc) only checks `(verifier, kind,
   fragment_text)` plus harness scope — it never looks at the `evidence`
   field, so which type's name appears in `evidence` is purely
   descriptive.

## The elicit_doc tooling

### The rule: `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001`

Lives in `~/repos/elicit_doc/src/quality/antipatterns/contract_bounds.rs`.
Scans `amenable_creusot`/`amenable_verus`/`amenable_kani` only (the only
crates with a verifier-native raw-bound shape), skips any directory
literally named `gallery/` (verifier experiments and documented dead
ends, not production proofs — pruned via `WalkDir::filter_entry`, not
hand-excepted per site), and flags any `requires`/`ensures` clause whose
normalized text matches **no** registered `ContractRecord` fragment for
its verifier.

Per-verifier clause shapes it recognizes:

- **Creusot**: `#[requires(...)]`/`#[ensures(...)]` attribute contents,
  compared at the token level (not parsed as `syn::Expr`, since Pearlite
  isn't valid plain Rust).
- **Verus**: `requires`/`ensures` clauses inside a `verus! { ... }`
  macro body, walked as a raw `proc_macro2::TokenStream` (the clauses are
  invisible to `syn::visit::Visit` — a macro body is opaque to `syn`).
- **Kani**: `assert!(EXPR, ..)` and `assert_eq!(A, B, ..)` (synthesizes
  the clause `A == B` — a direct transcription of the two comparands,
  not a guess) inside a `#[kani::proof]` fn body. **`kani::assume(EXPR)`
  is a plain function call, not a macro invocation** (`assume` has no
  `!`) — it needs its own `syn::ExprCall` visitor, separate from the
  `assert!`/`assert_eq!` macro-node visitor. Getting this wrong (as the
  scanner originally did) means every Kani `requires`-shaped bound is
  silently never checked, regardless of content.

A bare `true`/`false`/`result`/`result.N` clause is treated as trivial
and never flagged — see `is_trivial`'s doc comment in `contract_bounds.rs`
for the full reasoning per shape.

### The leverage tool: duplicate-cluster grouping

`write_duplicate_clusters` (in `~/repos/elicit_doc/src/quality/antipatterns/report.rs`)
groups `UnnamedContractBound001` findings by clause **shape** — every
non-call identifier and every literal blinded to a placeholder token
(`X`), so `map.is_empty()` and `dq.is_empty()` both collapse to
`X.is_empty()`. The checklist prints a `**Possible duplicate clusters**`
block, sorted by cluster size descending, **before** the flat per-site
list, for each crate's `UnnamedContractBound001` section.

This is the actual point of the whole exercise: naming a bound once only
pays off if you can see which raw sites share it. A cluster of size N
found at the top of the list means one contract type can resolve up to N
sites at once — that's the leverage worth pulling on first.

**Caveat, printed in the checklist itself and worth repeating**: a
coincidentally-identical shape is not guaranteed to be the same real
claim. `X == X` is the maximally generic shape — some sub-clusters
within it really were the same claim restated (NonZero's `value == 0`
across twelve widths, five different `RustStdStandard<T>::ensures`
round-trip checks all comparing `first == second`), but a good chunk
were genuinely unrelated (`Cell::replace`'s previous-value check vs.
`LazyCell`'s cache-once check vs. a raw pointer-cast reproducibility
check) that only look alike because `assert_eq!(a, b)` is a common
idiom. **Always read the actual site before assuming a shared claim** —
shape-clustering is a hint to investigate, not an automatic merge.

## The workflow, step by step

1. **Build the registry dump and run the scan:**

   ```bash
   cd /home/erik/repos/amenable
   cargo run -p amenable --features creusot,verus -- dump-registry --out /tmp/dump.json
   cd /home/erik/repos/elicit_doc
   ./target/release/elicit_doc quality antipatterns --project /home/erik/repos/amenable
   ```

   (`elicit_doc quality antipatterns` re-runs `dump-registry` itself into
   its own cache, so the first command above is really just for a quick
   manual sanity check of `contract_records` count — the second command
   is the one that actually regenerates the checklist.)

2. **Read the duplicate-cluster block** at the top of
   `~/.elicit_doc/amenable/quality/antipatterns.checklist.md`'s
   `amenable_kani`/`amenable_creusot`/`amenable_verus` sections. Pick the
   top (largest) cluster — that's "pull from the top of the list" applied
   to leverage, not just file order.

3. **Pull the full site list for that cluster** (crate/fn/file/line) and
   **read each real site**. Group by what they're actually checking, not
   just by shape. A single shape bucket often splits into several real
   sub-claims (see the `X == X` example above) — handle each sub-claim as
   its own mini-batch.

4. **For each sub-claim, find or build the contract type:**
   - Check whether the real call sites already share a registered
     `RustStdStandard<T>` (or crate-local carrier) witness type for the
     harness in question — most do, since every real proof already
     registers a `KaniWitness`/`ProofRecord` for its own carrier.
   - Check whether that type's `Ensures`/`Requires` slot for this
     verifier is already taken by a *different* bound. If so, you need a
     new type (see Gotchas: associated-type uniqueness).
   - Write the `kani_ensures!`/`kani_requires!` (or Creusot/Verus
     equivalent) call, plus the canonical `ContractRecord` it generates
     automatically.
   - Add supplementary `ContractRecord`s for the *exact* real call-site
     text, one per distinct literal text, each `harnesses`-scoped to
     every harness that uses that exact text (see the grouping insight
     above — check for recurring identical text before writing N
     separate registrations for N sites).

5. **Rewrite the real call sites** to call the named type's
   `::ensures(...)`/`::requires(...)` directly, replacing the raw
   expression. Preserve the original assertion message where one existed.

6. **Verify, in this order:**

   ```bash
   just check-package amenable_kani        # plain compile, fast
   cargo clippy -p amenable_kani --all-targets -- -D warnings
   cargo fmt -p amenable_kani -- --check
   just verify-kani <fully::qualified::harness_name>   # real cargo kani, per touched harness
   just test-package amenable_kani
   ```

   `just verify-kani` needs the harness's fully-qualified path as it
   appears in the module tree (e.g. `rust_std::cell::verify_cell_get_set_replace_take_round_trip`,
   or `compose::proofs::verify_kani_compose_string_depths` if it's nested
   in an inner `mod`). **Never add `--unwind` or wrap the call in a
   shell `timeout`** — the recipe already carries `--harness-timeout 3m`.

7. **Rebuild the registry dump and rescan** (step 1 again) to confirm the
   specific sites you touched actually drop out of the checklist. Don't
   trust the rewrite until the real rescan confirms it — a text mismatch
   between the supplementary `ContractRecord` fragment and the real
   call-site text (e.g. forgetting a `amenable_std::` prefix, or writing
   `Cell<u32>` when the site actually spells it `std::cell::Cell<u32>`)
   silently leaves the site flagged with no compile error.

8. **Commit** the batch with a message describing the sub-claims resolved
   and citing the real verification (harnesses passed, rescan counts
   before/after).

## Gotchas, found the hard way

- **`#[cfg(kani)]` import gating.** A real proof's fn body only exists
  under `cfg(kani)` (the `harness!` macro gates `#[cfg(#cfg_name)]` on
  the item, not the module) — so any `use amenable_core::Ensures;` (or
  `Requires`) needed only inside that body must *also* be
  `#[cfg(kani)]`-gated, or a normal (non-kani) build leaves it unused
  and clippy flags it. Watch out for accidentally duplicating an
  existing `#[cfg(kani)]` attribute when inserting a new import next to
  one that's already gated — the duplicate silently detaches the
  original gate from its intended item and the import becomes
  unconditional (caught once via a stray `warning: unused import` on a
  plain `cargo check`).
- **Associated-type uniqueness.** `impl Ensures<KaniVerifier> for
  RustStdStandard<NonZero<i8>>` can exist exactly once per concrete
  type. If the natural carrier already carries a *different* bound
  (e.g. `NonZero<T>`'s construction precondition `value != 0` already
  occupies that slot), a *second*, distinct claim about the same type
  (e.g. `.get()` round-tripping the wrapped value) needs its own new
  wrapper type — it cannot reuse the existing carrier's slot.
- **Attribute string literals can't be built via `concat!`/`stringify!`.**
  `#[standard(basis = "...")]` is parsed directly as a `syn::LitStr` by
  a third-party derive macro — passing `concat!("RustStdStandard<",
  stringify!($ty), ">")` as the attribute value does not work (it stays
  an unexpanded macro-call token, which fails `LitStr` parsing). Pass
  the fully-computed literal string as its own macro argument instead.
  The **one exception**: `#[doc = concat!(...)]` *is* supported, since
  `#[doc]` is a compiler-native attribute with special macro-expansion
  handling — useful for generating a real, distinct doc comment per
  type in a macro that stamps out several structs at once.
- **Bit-preserving casts unify otherwise-incompatible widths.** When a
  claim is genuinely one relationship but the real sites use different
  integer widths (e.g. `SeekFrom::Start(u64)` vs. `SeekFrom::End(i64)`),
  casting both sides `as i64` before comparing is sound — `(a as i64)
  == (b as i64)` iff `a == b`, since a same-width-or-narrower unsigned
  cast is a bit-reinterpreting bijection — letting one contract type
  cover every variant instead of minting a second type just for the
  odd-width variant.
- **`amenable_kani::gallery` is not production code.** Its own module
  doc comment says so explicitly: "Production proofs answer 'does this
  harness establish the intended claim?' The gallery answers a different
  question." `compose.rs`'s `#[cfg(kani)] mod proofs { ... }` and
  `os_windows_model.rs`, by contrast, *are* real production code (self-
  tests of a derive macro's own guarantees, and a real Kani-side
  Windows accommodation model respectively) — don't assume everything
  outside `rust_std/` is gallery-equivalent; read the module doc comment.

## Current state (last updated during this session)

- `amenable_creusot`: **fully cleared** — zero raw sites remain.
- `amenable_kani`: two largest duplicate clusters cleared this session
  (`X.get() == X`, 59 sites; `X == X`, 30 sites — the latter after
  splitting into ~10 genuinely distinct sub-claims), plus the earlier
  `compose.rs` self-test cluster (9 sites) and the `os_windows_model.rs`
  sentinel-rejection singleton (1 site), plus 20 sites from a
  `NonZero`/enum-round-trip sweep. Total resolved this session: 119
  sites. **~700 sites remain** — next up by cluster size:
  `X.next() == X` (42), `X.next() == Some(X)` (40),
  `X.load(X::X::X::X::X) == X` (29), `*X == X` (28), and onward down the
  ranked list.
- `amenable_verus`: **not yet started.** Almost certainly has its own
  version of the `kani::assume`-style scanner gap worth checking before
  trusting its raw count (Verus's `requires`/`ensures` are real spec
  clauses inside a macro body, not runtime asserts, so the specific gap
  won't recur verbatim, but don't assume the Verus-side scanner has been
  exercised against real duplicate volume the way Kani's has).

To resume: run the workflow above starting from step 1, and continue
pulling from the top of whichever crate's duplicate-cluster list you're
working.
