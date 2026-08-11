# Naming Raw Requires/Ensures Bounds: Design Pattern and Workflow

**Status:** 🔲 Ongoing — mechanism redesigned and fully verified in an
earlier session (call-shape recognition replaced text matching);
`amenable_creusot` fully cleared (twice — see "History" below);
`amenable_kani` now in progress (two real `elicit_doc` matcher bugs
fixed, six clusters named, 771 → 520 sites — see "Current state");
`amenable_verus` not yet started under the new mechanism.

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
  submits the type's `ContractRecord` automatically.
- **Creusot**: a shared `#[logic(open)]` fn (or `#[logic(opaque)]` — see
  Gotchas) wrapped in `amenable_derive::harness!`, called directly from
  every real site; `amenable_std`'s `Ensures`/`Requires<CreusotVerifier>`
  impl returns the harness-captured `_SRC` const verbatim.
- **Verus**: a shared cross-file `spec fn`, same idea.

### `ContractRecord`: call-shape recognition, not text matching

`amenable_core::ContractRecord` (`{ evidence, verifier, kind, fragment }`)
is what makes a named bound *discoverable* outside the Rust binary that
defines it — `amenable dump-registry` dumps every registered record to
JSON, which `elicit_doc` reads to check real proof-site clauses against.

A real proof site is recognized as using a registered contract **only
when its clause is a real call to it** — never by comparing the clause's
text against the registered `fragment`'s text. This replaced an earlier
"two-tier" design (a canonical registration plus hand-typed
"supplementary" registrations, one per distinct call-site spelling,
`harnesses`-scoped to avoid coincidental cross-matches) that existed
purely to keep the old text-matching scanner quiet — ceremony that
caught nothing a real call-shape check doesn't already catch, and that
was actively **hiding** real debt (a Verus `NonNulByte` registration
turned out to be coincidentally text-matching real call sites that never
actually named it — see "History" below).

Two recognized shapes, one per family of verifier:

- **Kani** — `<TypePath>::ensures(...)` / `<TypePath>::requires(...)`:
  the call's last path segment must equal `"ensures"`/`"requires"`. The
  type prefix (everything before that segment, turbofish-stripped) is
  compared by **suffix** against every registered `evidence` string for
  that `(verifier, kind)` — a suffix match, not exact equality, because a
  real call site's type name is usually abbreviated by a `use` import
  (`RustStdStandard::<i32>::ensures(...)`) while `evidence` is always
  fully qualified (`amenable_std::rust_std::RustStdStandard<i32>`).
- **Creusot/Verus** — a bare `name(...)` call (no receiver, exactly one
  path segment): `name` is compared against a function name **scanned
  out of the registered `fragment`'s own source text** — a literal `fn`
  token immediately followed by an identifier, not a full `syn::ItemFn`
  parse (Verus's real syntax, `pub open spec fn foo(...) -> bool { ... }`,
  has real keywords `syn::ItemFn` would reject as invalid Rust, but `fn`
  is still just a plain token to scan for). **`evidence` is not consulted
  at all for this match** — only the fragment's own extracted name
  matters, so one evidence value can legitimately carry more than one
  real fragment (see the `IterYieldsValueOnceThenEnds` example under
  "Gotchas": it names two distinct real predicates, one per shape of
  value it's proven over).

A fragment that isn't a real `fn`/`spec fn` definition — still a raw
restated expression under the hood, or a hand-typed string mimicking a
call site's text — **can never match**, under any circumstances, no
matter how its text compares to anything. That's not a bug to route
around; a fragment like that was never really naming a bound, it was
only passing the old text-equality check by coincidence.

## The elicit_doc tooling

### The rule: `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001`

Lives in `~/repos/elicit_doc/src/quality/antipatterns/contract_bounds.rs`.
Scans `amenable_creusot`/`amenable_verus`/`amenable_kani` only (the only
crates with a verifier-native raw-bound shape), skips any directory
literally named `gallery/` (verifier experiments and documented dead
ends, not production proofs — pruned via `WalkDir::filter_entry`, not
hand-excepted per site), and flags any `requires`/`ensures` clause that
[`ContractIndex::matches_named_call`] doesn't recognize as a real call to
some registered contract (see previous section) for its verifier.

Per-verifier clause shapes it recognizes:

- **Creusot**: `#[requires(...)]`/`#[ensures(...)]` attribute contents.
- **Verus**: `requires`/`ensures` clauses inside a `verus! { ... }`
  macro body, walked as a raw `proc_macro2::TokenStream` (the clauses are
  invisible to `syn::visit::Visit` — a macro body is opaque to `syn`).
- **Kani**: `assert!(EXPR, ..)` and `assert_eq!(A, B, ..)` (synthesizes
  the clause `A == B` — a direct transcription of the two comparands,
  not a guess) inside a `#[kani::proof]` fn body. **`kani::assume(EXPR)`
  is a plain function call, not a macro invocation** (`assume` has no
  `!`) — it has its own `syn::ExprCall` visitor, separate from the
  `assert!`/`assert_eq!` macro-node visitor.

  **`assert_eq!` can never be recognized as compliant, structurally, no
  matter what's registered.** It always synthesizes a binary comparison
  (`A == B`, a `syn::Expr::Binary`), never a call — even when one side of
  the comparison is itself a call to a real registered contract's
  `ensures`. A Kani postcondition that needs to be nameable has to use
  `assert!(Type::ensures(...), "message")`, never `assert_eq!`. This is a
  real, intentional, regression-tested behavior of the new mechanism, not
  a gap to work around.

A bare `true`/`false`/`result`/`result.N`/`result.N is None` clause is
treated as trivial and never flagged — see `is_trivial`'s doc comment in
`contract_bounds.rs` for the full reasoning per shape.

### The leverage tool: duplicate-cluster grouping

`write_duplicate_clusters` (in `~/repos/elicit_doc/src/quality/antipatterns/report.rs`)
groups `UnnamedContractBound001` findings by clause **shape** — every
non-call identifier and every literal blinded to a placeholder token
(`X`), so `map.is_empty()` and `dq.is_empty()` both collapse to
`X.is_empty()`. The checklist prints a `**Possible duplicate clusters**`
block, sorted by cluster size descending, **before** the flat per-site
list, for each crate's `UnnamedContractBound001` section. Unaffected by
the call-shape redesign — clustering operates on the raw clause shapes
regardless of how compliance is later checked.

This is the actual point of the whole exercise: naming a bound once only
pays off if you can see which raw sites share it. A cluster of size N
found at the top of the list means one contract type can resolve up to N
sites at once — that's the leverage worth pulling on first.

**Caveat, printed in the checklist itself and worth repeating**: a
coincidentally-identical shape is not guaranteed to be the same real
claim. `X == X` is the maximally generic shape — some sub-clusters
within it really were the same claim restated, but a good chunk were
genuinely unrelated (`Cell::replace`'s previous-value check vs.
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
     new type (see Gotchas: associated-type uniqueness) — or, if the two
     bounds are genuinely two shapes of the *same* claim on the *same*
     evidence (e.g. an `Option`-shaped and `Result`-shaped sibling
     predicate), register the second real fragment under the *same*
     evidence instead (see "Gotchas": one evidence, multiple fragments).
   - For Kani: write the `kani_ensures!`/`kani_requires!` call — the
     canonical `ContractRecord` comes for free.
   - For Creusot/Verus: write a named `#[logic(open)]` fn / `spec fn`
     whose body **is** the bound (via `amenable_derive::harness!`, so its
     source is capturable verbatim), and point the contract type's
     `Ensures`/`Requires` impl at that harness's `_SRC` constant directly
     — never at a hand-typed string. A fragment that isn't real `fn`
     source can never be recognized (see "`ContractRecord`: call-shape
     recognition" above).

5. **Rewrite the real call sites** to call the named predicate/type
   directly, replacing the raw expression:
   - Kani: `Type::ensures(...)`/`Type::requires(...)`.
   - Creusot/Verus: the bare predicate name, e.g. `#[ensures(bound_holds(value, result))]`.

   Preserve the original assertion message where one existed.

6. **Verify, in this order:**

   ```bash
   just check-package amenable_kani        # plain compile, fast
   cargo clippy -p amenable_kani --all-targets -- -D warnings
   cargo fmt -p amenable_kani -- --check
   just verify-kani <fully::qualified::harness_name>   # real cargo kani, per touched harness
   just test-package amenable_kani
   ```

   For Creusot, additionally run the real translator, not just `cargo
   check` — a Pearlite-only failure (visibility rules, unsupported
   syntax) won't show up under plain `rustc`:

   ```bash
   cargo creusot -- -p amenable_creusot
   ```

   `just verify-kani` needs the harness's fully-qualified path as it
   appears in the module tree (e.g. `rust_std::cell::verify_cell_get_set_replace_take_round_trip`,
   or `compose::proofs::verify_kani_compose_string_depths` if it's nested
   in an inner `mod`). **Never add `--unwind` or wrap the call in a
   shell `timeout`** — the recipe already carries `--harness-timeout 3m`.

7. **Rebuild the registry dump and rescan** (step 1 again) to confirm the
   specific sites you touched actually drop out of the checklist. Don't
   trust the rewrite until the real rescan confirms it — a clause that
   *looks* right can still fail to match: the registered `fragment` has
   to be real `fn`/`spec fn` source (not a hand-typed string), and the
   call site has to be a genuine call (not, e.g., buried inside an
   `assert_eq!` comparand).

8. **Commit** the batch with a message describing the sub-claims resolved
   and citing the real verification (harnesses passed, rescan counts
   before/after).

## Gotchas, found the hard way

- **A registered `fragment` must be real predicate source, not
  descriptive text.** `fragment_fn_name` only extracts a name when the
  fragment's own text contains a literal `fn` token followed by an
  identifier. A `fragment: || "some_predicate (value , result)"` closure
  — text that merely *reads like* a call — extracts no name and can
  never match anything, Kani-style call-shape aside. Always point
  `fragment` at a `harness!`-captured `_SRC` constant (or, for Kani, let
  `kani_ensures!`/`kani_requires!` generate it via `stringify!`).
- **One `evidence` can carry more than one real fragment.** Creusot/Verus
  matching ignores `evidence` entirely — it only cares whether *some*
  registered fragment's extracted name matches the call site. This means
  a single contract type can legitimately register two (or more) real,
  independent predicates under its own evidence string when it's proven
  over more than one real shape (`amenable_std::IterYieldsValueOnceThenEnds`
  registers both `iter_yields_value_once_then_ends`, for `Option`-typed
  proofs, and `iter_yields_ok_value_once_then_ends`, for `Result`-typed
  ones — two `ContractRecord` submissions, same evidence, each with its
  own real `fn`). Don't mistake this for redundancy when auditing.
- **`#[logic(open)]` can't call a less-visible item.** An `open`
  (transparent) Creusot logic fn is inlined wherever it's referenced, so
  Creusot requires everything it calls to be at least as visible as
  itself. If the predicate needs to call a module-private helper (most
  commonly a `#[trusted] #[logic(opaque)]` wrapper around an uncontracted
  std method, e.g. `nonzero_i16_get`), mark the *calling* predicate
  `#[logic(opaque)]` too rather than `open` — real error, not a guess:
  `Cannot make "..." transparent in "..." as it would call a
  less-visible item`. Since the harness it backs is usually already
  `#[trusted]` in this situation anyway, nothing is lost by not being
  transparent.
- **When auditing many `ContractRecord` sites at once, trust
  `cargo-expand`, not a source grep.** A canonical registration is often
  emitted by a macro (`kani_ensures!`, or a locally-defined
  `macro_rules!` that stamps out one `ContractRecord` per
  monomorphization) — invisible to a plain text search for
  `evidence: "..."` across the raw source. When retiring the old
  "supplementary" registrations this session, classifying by raw-source
  triples alone would have deleted a registration whose only sibling
  canonical source was generated through *nested* macro expansion
  (`impl_nonzero_get_round_trips_kani!` calling `kani_ensures!` inside
  itself). `cargo expand -p <crate> --lib` (plus `--features` for
  Creusot/Verus) gives the real, fully-expanded ground truth for exactly
  this kind of audit.
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

## History: the two-tier design and why it was replaced

The original design (see git history on this file for the full writeup)
matched a real proof-site clause against a registered `ContractRecord`
by comparing **normalized text**, not recognizing a real call. That
needed a "canonical" registration (the predicate's own body text) plus a
hand-typed "supplementary" registration per distinct real call-site
spelling — the exact literal text of each real site, `harnesses`-scoped
to avoid one type's registration coincidentally silencing an unrelated
site with matching text.

This was retired mid-session (not a gradual deprecation) after directly
investigating real Creusot vs. Verus call sites and finding the
asymmetry that motivated the whole redesign: Creusot's real sites were
genuinely wired to real named `#[logic]` calls, but at least one Verus
registration (`amenable_std::NonNulByte`'s `requires`) was not — its
`Requires<VerusVerifier>::requires()` impl returned the raw text `"byte
!= 0"`, which happened to text-match four real call sites that never
actually called anything, only restated the same literal comparison.
The old scanner reported those four sites as compliant; they were not.
That's a real correctness bug in the old mechanism, not just ceremony —
worth remembering if any future design temptation reaches for text
matching again.

The redesign itself was mechanical but large: `amenable_core::ContractRecord`
lost its `harnesses` field, and all 253 existing `ContractRecord`
submissions across `amenable_kani`/`amenable_std` were swept — 152
turned out to be pure duplicates of an already-canonical registration
(deleted), 96 were each the sole registration for their bound (kept,
`harnesses` field dropped). The sweep was classified against
`cargo-expand` ground truth, not raw source text (see "Gotchas" above
for why that mattered). `amenable_creusot` briefly went from "fully
cleared" to 7 real findings the moment the redesign landed — not a
regression, real debt the old mechanism had been silently hiding — and
was brought back to zero in three focused follow-up commits.

## Current state (last updated during this session)

- **`amenable_creusot`: fully cleared** — zero raw sites, confirmed by a
  real rescan after the redesign (not carried over from before it).
- **`amenable_kani`: in progress under the new mechanism.** Started this
  session; total is now **520** sites (was 771; eight intervening fixes
  landed, see below). Current top clusters, by size (re-run the scan
  before trusting these — this list will drift as work lands):
  - `X.pop_front() == Some(X)` — 11 sites
  - `X != X && X != X` — 9 sites
  - `X != X && X != X && X != X` — 9 sites
  - `X.checked_add(X).is_some()` — 9 sites
  - `X[X] == X` — 9 sites
  - and onward down the ranked list in the checklist itself.

  **A correction mid-session, worth recording so it isn't repeated:**
  the first pass through this list checked each top cluster for a real
  shared claim (reading a sample of sites — correct, per this doc's own
  "shape-clustering is a hint to investigate, not an automatic merge"
  caveat) but then *skipped* every cluster that came back heterogeneous
  (`X.next() == Some(X)`, `X.len() == X`, `X.pop_front() == Some(X)`) to
  keep chasing clusters that resolved with one shared type — leaving 60+
  real sites unnamed while reporting clean incremental progress. That
  contradicts this doc's actual goal: every `requires`/`ensures` bound
  gets named, not just the duplicated ones. **Whether a cluster is a
  shared claim decides *how* to name it (one generic type vs. N
  per-carrier `kani_ensures!` registrations), never *whether* to.**
  `X.next() == Some(X)` (40 sites, confirmed heterogeneous —
  `str.rs`'s `Some(byte)`, `alloc_vec.rs`'s `Some(2)`, `iter.rs`'s
  cycle-restart `Some(a)`/`Some(a + 1)`, each a different real value
  from that adapter's own logic) was gone back to and named per-carrier
  (item 8 below) once this was caught. `X.len() == X` and
  `X.pop_front() == Some(X)` are next in line for the same treatment,
  not skips.

  Eight things resolved the first 251 sites of the drop from 771:
  1. **A real bug in `elicit_doc`'s matcher, not a naming gap in
     `amenable`** (69 sites, no `amenable` source changes beyond a small
     `Cell` import cleanup): `ContractIndex::matches_named_call` compared
     a call site's type prefix against a registered `evidence` string
     after independently re-tokenizing both, and Rust's tokenizer
     collapses a bare `>>` in raw text into one `Shr` punct (no space)
     while `syn`'s real `Path` AST always emits nested closing generics
     as separate `Gt` tokens (with a space) — so *any* contract type with
     a nested generic parameter (`Cell<i32>`, `Box<i32>`, `Vec<i32>`,
     `NonZero<i32>`, ...) failed the suffix match even at its own
     canonical call site. Fixed in `elicit_doc` by
     `split_adjacent_gt` (see that repo's `contract_bounds.rs`), with a
     regression test. This is why the `X::<X::X::X<X>>::ensures((X.get(),
     X))` cluster (44 sites) from the original list above no longer
     exists at all — it wasn't a real naming gap, it was this bug.
  2. **`X.next() == X` (42 sites)**: the exhaustion postcondition
     (`assert_eq!(iter.next(), None, ...)`), independently restated
     across ~30 distinct concrete iterator adapter types. Named once as
     `amenable_kani::IteratorYieldsNoneWhenExhausted<T>`, generic over
     the item type with a single blanket `impl<T> Ensures<KaniVerifier>`
     — the first genuinely generic (not per-concrete-type) contract type
     in this codebase, since every real site's item type differs and the
     existing "fixed representative type" pattern
     (`IndexingAndLength`/`IterYieldsValueOnceThenEnds`) only works when
     real sites happen to already share one type. See that type's own
     doc comment in `rust_std/iter.rs` for the full mechanism (why the
     macros couldn't be reused, why call sites write no turbofish).
  3. **`X.load(X::X::X::X::X) == X` (29 sites)**: an atomic's `.load()`
     reflects the value most recently established by
     `new`/`store`/`swap`/`compare_exchange`/`fetch_add`, independently
     restated across all 11 `Atomic*` integer/bool types in
     `sync_atomic.rs` plus two unrelated-file call-counter sites. Named
     once as `amenable_kani::AtomicLoadReflectsTheLastWrite<T>`, the
     second genuinely generic contract type, same design as
     `IteratorYieldsNoneWhenExhausted`. Notably, `AtomicPtr<i32>`'s own
     `RustStdStandard<AtomicPtr<i32>>` carrier already had a *different*
     `Ensures<KaniVerifier>` bound occupying its slot (`.swap()`
     returning the previous value) — its four `.load()` sites could never
     have used a per-carrier registration even if every other `Atomic*`
     type had, confirming the generic-type approach isn't just
     convenient here, it's necessary.
  4. **`*X == X` (28 sites)**: dereferencing a smart pointer, guard, or
     reference recovers the value stored in (or borrowed by) it,
     independently restated across `Cow`, `Box`, `BinaryHeap::PeekMut`,
     `Rc`, `Arc`, `RefCell`'s `Ref`/`RefMut`, `ManuallyDrop`,
     `Option`/`Result`'s `IterMut`, `AssertUnwindSafe`, `Pin<Box<_>>`,
     shared/mutable references, `slice::IterMut`, and `Mutex`/`RwLock`'s
     guards. Named once as `amenable_kani::DerefReflectsTheStoredValue<T>`,
     the third generic contract type, same design as the two above.
     Checked a sample of real sites before starting (unlike
     `X.next() == Some(X)` below, which failed that same check) — every
     comparison really is a plain value-equality check regardless of
     which wrapper type derefs.
  5. **`X.next() == X.next()` (16 sites)**: an iterator adapter's
     sequence matches a directly-constructed reference iterator's
     sequence, step by step, independently restated across
     `verify_flat_map_flattens_each_generated_iterator` (`FlatMap` vs.
     calling its closure directly) and
     `verify_flatten_concatenates_the_inner_iterators` (`Flatten` vs. a
     direct `.chain()` concatenation). Named once as
     `amenable_kani::IteratorMatchesReferenceStepByStep<T>`, the fourth
     generic contract type. Both harnesses use identical
     receiver/comparand pairs throughout (only the assertion messages
     differ), confirmed before starting.
  6. **`X.is_empty()` (13 sites)**: a container that's had every element
     removed (via `drain`, repeated `pop`/`remove`, or iteration)
     reports itself empty afterward, independently restated across
     `BTreeMap`, `BTreeSet`, `LinkedList`, `VecDeque`, `BinaryHeap`, and
     `Vec`. Named once as `amenable_kani::EmptiedContainerReportsEmpty`
     — unlike the four generic types above, this one needs **no** type
     parameter: every real site already computes the `bool` before
     asserting it, so the ordinary `kani_ensures!`/`bridge_kani_witness!`
     macros work unmodified, same shape as `NonNegativeFd`/
     `IndexingAndLength`.
  7. **`!X::<X<X>>::ensures(X)` (12 sites)**: a real `elicit_doc`
     scanner completeness gap, not a naming gap — `matches_named_call`
     only recognized a bare `syn::Expr::Call`, never one wrapped in `!`,
     so every `NonZero::new`'s "fails only for zero" rejection check
     (already calling its own registered `Ensures<KaniVerifier>`
     directly, negated) was flagged despite naming a real contract. No
     `amenable` source changes at all this time — pure `elicit_doc` fix
     (strip a single leading `!` before matching either the Kani or
     Creusot/Verus shape; `!!x` stays unmatched). Confirmed the fix
     resolves exactly this cluster and nothing else.
  8. **`X.next() == Some(X)` (42 sites landed, of 40 originally
     flagged)**: the cluster wrongly skipped earlier this session (see
     the correction note above). Genuinely heterogeneous — every real
     site's expected value comes from that adapter's own domain logic,
     not a shared abstraction — so each of the ~25 distinct carrier
     types (`LinkedList`/`VecDeque`'s `IntoIter`, `Vec`'s `ExtractIf`/
     `Splice`, `array::IntoIter`, most of `iter.rs`'s adapters —
     `Chain`, `Rev`, `Cloned`, `Copied`, `Cycle`, `Fuse`, `Inspect`,
     `Peekable`, `Scan`, `StepBy`, `Take`, `TakeWhile`, `Once`, `Repeat`,
     `RepeatN`, `Successors`, `FromFn` — `option`/`result`'s `IntoIter`,
     and `str`'s `Bytes`/`Chars`/`Lines`/`LinesAny`) got its own
     `kani_ensures!(RustStdStandard<AdapterType>, ..., (actual,
     expected), |(actual, expected)| actual == expected)` registration
     — the same "trivial equality, real per-carrier registration"
     pattern `cell.rs` already used for
     `Cell<i32>`/`Cell<u32>`/`Cell<usize>`. Landed 2 more sites than the
     cluster's original 40: while already inside a harness for this
     cluster, also converted its mechanically-identical
     `X.next() == Some(X + X)` sites with the same registration (`Rev`,
     `Cycle`, `Peekable`, `Scan`) — matching this doc's own
     top-to-bottom-order exception for later items that are mechanically
     identical to the one just finished.
- **`amenable_verus`: not yet started under the new mechanism.** Total is
  now **663** sites, including the confirmed `NonNulByte` case from
  "History" above (register a real `spec fn` for it first — it's a
  concrete, already-understood fix, not exploratory). Current top
  clusters:
  - `X.X == X` — 145 sites
  - `X.X@ =~= X@` — 39 sites
  - `X.X == Some(X)` — 36 sites
  - `X == X` — 30 sites
  - `X != X` — 29 sites
  - `X == (X, X)` — 23 sites
  - and onward down the ranked list in the checklist itself.

  Also still worth checking before trusting the raw count: whether the
  Verus-side scanner's `verus! { ... }` token walk has been exercised
  against real duplicate volume the way Kani's `assert!`/`assert_eq!`
  visitor has — a scanner-completeness gap (missed clause shapes, not a
  naming-mechanism issue) would under-report here the same way the
  original `kani::assume` gap did for Kani before it was found and fixed.

To resume: run the workflow above starting from step 1, and continue
pulling from the top of whichever crate's duplicate-cluster list you're
working. `amenable_kani` and `amenable_verus` are both untouched under
the new mechanism — either is a reasonable place to start; `amenable_verus`
has the added wrinkle of the already-diagnosed `NonNulByte` fix as a
concrete first task.
