# cfg Hygiene Plan

**Status:** ✅ Step 0 done and verified. ✅ Step 3 done: per-crate tracing
policy implemented + tested in cordial, and a real `amenable_kani`
`--apply` (396 functions, 83 files) is now landed and fully verified —
`cargo check`/`clippy -D warnings`/`fmt --check`/`test` all clean, plus
two real `cargo kani` proof harnesses (including one exercising the
tuple-destructuring fix directly) still verify successfully with the
new gated instrumentation in place. Six real, pre-existing cordial
classifier bugs were found and fixed along the way (unrelated to the
policy layer itself) — see "Step 3 rollout" below, including a
workspace-wide call-graph reachability mechanism (not a hardcoded
trait-name list) that finds functions reachable only from proof-only
entry points, wherever they live in the workspace, and a fix for `syn`
never expanding the `amenable_derive::harness!` macro that wraps almost
every real Kani proof harness in the crate -- with it fixed, **every**
`amenable_core::Ensures`/`Requires` impl in `amenable_kani` is now
correctly excluded, not just the ones that happened to be called
outside a `harness!` block.
`amenable_creusot`/`amenable_verus` correctly received zero changes
(Skip policy). Not yet done: committing the `amenable_kani` apply
result, or generalizing `--apply` beyond tracing (Phase 1). Steps 1–2
not started at all.

## Why this exists

Two distinct, confirmed problems surfaced while investigating why
cordial's tracing `--apply` couldn't just be run blind against this
workspace (`#[instrument]` on any function reachable from a
`#[kani::proof]` harness causes real CBMC symbolic-closure-capture
timeouts — confirmed via `~/repos/elicitation`'s `KANI_FOR_VSMS.md` §6.3
gallery evidence, `gallery14a_ungated_debug` hangs / `gallery14b_gated_debug`
completes in ~8s).

**Problem 1 (fixed in Step 0):** the workspace declared every verifier's
cfg name (`kani`/`creusot`/`verus`/`verus_verify_core`/`verus_keep_ghost`)
as "expected" in *every* crate via one `[workspace.lints.rust]
unexpected_cfgs` union. A union can't distinguish "this crate legitimately
uses this cfg" from "this cfg is declared somewhere in the workspace" — a
verifier's cfg name copy-pasted into the wrong crate (e.g. an accidental
`#[cfg(creusot)]` in `amenable_kani`) would silently pass.

**Problem 2 (Steps 1–3, not started):** `amenable_derive`'s own proc
macros generate code containing raw `#[cfg(kani)]`/`#[cfg_attr(#cfg,
...)]` tokens that get spliced directly into whatever crate *applies*
the macro — the exact bug class `~/repos/elicitation`'s
`UNEXPECTED_CFGS.md` white paper documents and burned real time fixing
there ("the library pissing on the user and telling them to bring a
towel"). Confirmed live in four of `amenable_derive`'s macros (below),
not just theoretical — safe *today* only because the sole current
consumer of all four happens to be `amenable_kani`, which now declares
`kani` itself (Step 0). That safety is incidental: the moment any
internal crate uses one of these macros with a different verifier
without its own declaration, or any external consumer of the published
`amenable_derive` crate applies any of them at all, the bug is live for
them with zero way to know why.

## Step 0 — replace the workspace cfg union with per-crate `build.rs` ✅

**Status:** Done, verified, committed (`1dd1985`).

Removed `[workspace.lints.rust].unexpected_cfgs.check-cfg` entirely.
Cargo rejects `[lints] workspace = true` (needed for the shared clippy
list) coexisting with a crate-local `[lints.rust]` override in the same
manifest — confirmed directly (Cargo 1.97.1: "cannot override
`workspace.lints` in `lints`"). A `build.rs` sidesteps this: `cargo::
rustc-check-cfg=cfg(name)` is independent of the `[lints]` table
entirely, so `workspace = true` and a crate-scoped check-cfg list both
hold at once (confirmed with a real scratch crate before applying here).

Per-crate needs were verified empirically, not copied from the old
list — ran `cargo clippy --workspace --all-targets --all-features` with
an empty check-cfg list and read exactly which crate/file/name each
real warning named:

| Crate | `build.rs` declares | Real site(s) |
| --- | --- | --- |
| `amenable` | `kani` | `lib.rs` (1) |
| `amenable_core` | `kani`, `verus_keep_ghost` | `stoplight.rs` (kani, not spliced anywhere); `evidence.rs` (verus_keep_ghost, **is** `#[path]`-spliced into `amenable_verus`, so `amenable_verus`'s own `build.rs` declares it too — independently, since Cargo lints each compilation unit separately) |
| `amenable_creusot` | `creusot` | 332 sites, matches its identity |
| `amenable_derive` | *(none)* | zero real hits — every earlier grep hit was inside `quote!{}` text or a doc comment, not a real attribute on this crate's own code |
| `amenable_gaap` | `kani` | 10 sites — wasn't on the old list at all |
| `amenable_kani` | `kani` | 624 sites, clean |
| `amenable_std` | *(none)* | uses `#[cfg(feature = "...")]` instead of bare cfg names |
| `amenable_verus` | `verus_keep_ghost` | 125 sites + the spliced `evidence.rs` copy |

`verus`/`verus_verify_core` were on the old list but are used nowhere in
the workspace today — dropped rather than carried speculatively.

Sanity-checked the fix actually closes the gap it targets: added a bare
`#[cfg(creusot)]` to `amenable_kani/src/lib.rs`, confirmed it now warns
("expected names are: `docsrs`, `feature`, `kani`, ... and 32 more" —
`creusot` absent), reverted. Re-verified `cargo check`/`clippy --all-
features` (incl. `--features creusot`/`--features verus` variants)/
`fmt`/`test --workspace` all clean; real `verus --crate-type=lib`
(`491 verified, 0 errors`) and `cargo creusot prove` (`Proved (153
files) ✔`) both re-confirmed clean afterward — `build.rs` is inert to
both (verus never runs build scripts at all; check-cfg only controls
whether a warning fires, never what a `#[cfg(...)]` block compiles to,
so it can't affect Kani's real proof behavior either).

## Step 1 — fix the four confirmed live macro-injection sites

Not started. Each needs one of `~/repos/elicitation`'s two proven
wrapper patterns from `UNEXPECTED_CFGS.md` (read directly before
starting, not summarized from memory — the gallery test
(`crates/elicitation_derive/tests/cfg_allow_gallery_test.rs`) is the
source of truth for which placements actually suppress the lint; sibling
allow does NOT work, confirmed there via 13 real test cases):

| Macro | Injection | Real site | Fix shape |
| --- | --- | --- | --- |
| `#[amenable_derive::exchange(cfg = ..., ...)]` | `#[cfg_attr(#cfg, #cfg::ensures(...))]` on the transformed method — `#cfg` is caller-parameterized | `crates/amenable_derive/src/exchange.rs:296-306` | Pattern 2 (`#[allow(unexpected_cfgs)] mod _compat { ... } + pub use`) — transforms an existing function that must stay reachable at its original path |
| `#[amenable_derive::capture_exchange_body(...)]` | `#[cfg_attr(kani, kani::requires(...))]` + `#[cfg_attr(kani, kani::ensures(...))]`, hardcoded to `kani` | `crates/amenable_derive/src/capture_exchange_body.rs:280-296` | Pattern 2, same shape as `exchange` |
| `#[derive(amenable_derive::StateMachine)]` | `#[cfg(not(#cfg_ident))]` / `#[cfg(#cfg_ident)]`, two-branch `audit_surface()` | `crates/amenable_derive/src/state_machine.rs:358-373` | Pattern 1 (`#[allow(unexpected_cfgs)] const _: () = { ... };`) if the two branches can live in a `const` block (need to confirm `audit_surface()`'s trait-impl-method placement tolerates this — a derive emits a whole `impl` block, closer to Pattern 1's designed use case than Pattern 2's) |
| `#[derive(amenable_derive::KaniCompose)]` | `#[cfg(kani)]` on the generated `impl KaniCompose for #name` block | `crates/amenable_derive/src/kani_compose.rs:43-62` | Pattern 1 — textbook case, matches elicitation's own `enum_impl.rs`/`struct_impl.rs` precedent exactly |

`#[derive(amenable_derive::Witness)]`, `#[amenable_derive::establish(...)]`,
and `#[amenable_derive::proof_token]` were checked and are clean — no
raw cfg token injection, nothing to fix there.

For each site: isolate in a throwaway local test crate first (mirroring
elicitation's own gallery-test discipline — do NOT test proof-macro
`cfg` wrapper syntax in this repo's own scratch files per this project's
established "no scratchpad proof probing" convention; a fresh, disposable
crate outside `~/repos/amenable` is the right isolation boundary here,
not a file inside it), confirm the wrapper actually suppresses
`unexpected_cfgs` for an unregistered cfg name, then apply to the real
macro and re-verify the real `Stoplight`/`Ledger` call sites still
compile and (for `exchange`/`capture_exchange_body`) still pass their
real `cargo kani`/`cargo creusot prove` checks non-vacuously (the
existing injected-panic regression-check discipline this workspace
already uses throughout `EXCHANGE_PROOF_DERIVATION_PLAN.md`/
`VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`).

## Step 2 — cordial: new `cfg-hygiene` etiquette

Not started. Two rules, own `mod.rs`/`scan.rs`/`assessor.rs`/etc. file
layout matching the existing `cfg_scatter` etiquette's precedent
(`~/repos/cordial/src/etiquettes/cfg_scatter/`) — a sibling, not an
extension of it (different concern: `cfg_scatter` is about
organizational DRY-ness, this is about lint-hygiene correctness):

- **UNEXPECTED-CFG-001**: any `cfg(X)`/`cfg_attr(X, ...)` (including
  nested in `all()`/`any()`/`not()`) where `X` isn't declared in any
  check-cfg source reachable by that crate (workspace `[lints.rust]`,
  crate-local `[lints.rust]`, or `build.rs`-emitted). The literal
  elicitation-flavor bug, generalized for any cordial-scanned project —
  finds nothing in `amenable` today (everything's declared after Step
  0), but protects it going forward and is directly reusable by
  `elicitation` itself or any other cordial-scanned proof-heavy project.
- **CFG-VERIFIER-MISMATCH-001**: a crate using a verifier cfg name that
  isn't its own — the real gap a union-based check-cfg list creates and
  can never self-detect (confirmed: `creusot` is declared "expected"
  workspace-wide in `amenable`'s old config, so `rustc` itself could
  never catch a misplaced `#[cfg(creusot)]` in `amenable_kani`). Needs a
  config table, e.g. `[cfg_hygiene] crate_verifier = { amenable_kani =
  "kani", amenable_creusot = "creusot", amenable_verus = "verus" }`, so
  cordial knows each crate's expected identity.

This etiquette is also the safety net for cordial's own apply pipeline
(tracing `--apply` and any future `derives --apply`): it can validate
whatever a code-insertion feature just wrote, rather than needing a
one-off manual `Cargo.toml` read the way this plan's own Step 0
investigation did by hand.

## Step 3 — cordial: `--apply` generalization + verifier-safety gating for tracing

Not started — this is the plan already discussed for the tracing
rollout itself (generalizing `--apply` from tracing-only to a
config-toggleable suite of etiquette applies, plus a per-crate policy so
tracing's own apply doesn't blindly insert `#[instrument]` into
Kani-reachable, Creusot-translated, or Verus-spliced code). Tracked here
only for sequencing: **do this work first**, using whatever
`cfg-hygiene` (Step 2) already exists at that point to validate its own
output, then return to this plan's Steps 1–2 for the two narrower,
`amenable_derive`-specific and cordial-specific pieces.

**The real per-crate policy, confirmed empirically before writing any
code** — the naive two-bucket model (gate Kani-reachable code, skip
Verus-spliced code) turned out wrong on one axis: Creusot isn't a milder
version of the same risk, it's a *harder* failure than Kani's.

- `amenable_kani` → **Gated**, `#[cfg_attr(not(kani), tracing::instrument(...))]`.
  Compiles fine under ordinary `cargo kani`; the risk is a CBMC
  symbolic-closure-capture timeout at proof time (confirmed via
  `~/repos/elicitation`'s `KANI_FOR_VSMS.md` §6.3 gallery evidence —
  `gallery14a_ungated_debug` hangs, `gallery14b_gated_debug` completes
  in ~8s), not a compile failure.
- `amenable_creusot` → **Skip entirely**. Tested directly: added a bare
  `#[tracing::instrument(level = "debug")]` to one ordinary, really-
  translated function (`amenable_creusot::stoplight::Green::basis`, not
  a generated companion, not `#[cfg(not(creusot))]`-only) and ran the
  real `cargo creusot -- -p amenable_creusot` translation pass. Hard
  compile failure, not a milder or narrower-scoped one:
  `creusot-std`'s `DeepModel` trait isn't implemented for
  `tracing::Level`/`LevelFilter` (both types the `#[instrument]`
  expansion touches), and independently: `error: Unsupported constant
  value: Scalar(alloc191) of type &'?36 tracing::callsite::DefaultCallsite`
  — creusot-rustc's translator can't handle the static `DefaultCallsite`
  reference `tracing::span!` embeds, the same "ordinary Rust
  infrastructure the translator has never seen" failure class as the
  already-documented `Box<dyn Iterator>`/RPITIT ICEs. Reverted
  immediately after confirming (`git checkout --
  crates/amenable_creusot/{Cargo.toml,src/stoplight.rs} Cargo.lock`);
  never committed. This means the elicitation white paper's narrower
  "generated companions need tracing-free bodies" framing does not
  apply here as written — the real constraint is crate-wide, any real
  translated function, not just the generated companion surface.
- `amenable_verus` → **Skip entirely**. `tracing` isn't resolvable at
  all under the real, bare-compiler `verus --crate-type=lib` invocation
  (same root cause already confirmed by the `MetadataEntry`/
  `TransitionAudit` incident this session — the real binary never reads
  `Cargo.toml`).
- Any file `#[path]`-spliced into a **Skip** crate is **Skip** there too,
  independent of its own owning crate's own policy — reuses
  `path_inclusion.rs`'s existing splice graph (see the original
  plan discussion: it needs a small extension, a per-crate "toolchain
  never reads `Cargo.toml`" fact for Verus specifically, since that
  can't be inferred from the dependency graph the way Creusot's
  ordinary-`cargo`-driven build can).
- Everything else → **Bare**, `#[instrument(...)]` as cordial already
  generates it today.

**Implementation status:** done in cordial (not yet in this repo's own
history — cordial is a sibling project). `TracingApplyPolicy` (`Bare`/
`Gated(Vec<cfg>)`/`Skip`) lives in `~/repos/cordial/src/etiquettes/
tracing/apply/verifier_policy.rs`, resolved from two new
`PathInclusionFacts` methods (`splice_consumers`, `transitive_dependents`)
and two new `TracingThresholds` config fields (`apply_gate_crates`,
`apply_skip_crates`), threaded into `apply_gap`/`InstrumentApplySummary`
(new `skipped_policy` count). 4 new integration tests cover bare/gated/
skip/dependency-propagation/splice-propagation; all 22 tests in
`tests/tracing_apply.rs` plus the full `cargo test --all-features`
suite pass. `amenable/cordial.toml` now declares the real policy:
`apply_gate_crates = { amenable_kani = "kani" }`, `apply_skip_crates =
["amenable_creusot", "amenable_verus"]`.

**Step 3 rollout — four real classifier bugs found and fixed across
three attempts, apply now landed and fully verified:**

Ran `cordial quality --apply --crate-name amenable_kani` for real (not
dry-run) after rebuilding/reinstalling cordial and adding `tracing` as a
dependency (workspace `[workspace.dependencies]` + `amenable_kani`'s own
`[dependencies]` — the crate had zero prior `tracing` usage, so this was
a real, necessary prerequisite, not scope creep). The **gating mechanism
itself worked correctly**: all 530 changed functions got
`#[cfg_attr(not(kani), instrument(...))]`, zero bare `#[instrument]`
leaked through. But the resulting tree failed to compile
(`cargo check -p amenable_kani`: 612 errors) for two reasons unrelated to
gating — both would occur identically under a `Bare` policy in any
crate, Kani-aware or not:

1. **`#[instrument]` proposed on `const fn`s.** `tracing::instrument`
   categorically rejects `const fn` (`error: the #[instrument]
   attribute may not be used with const fns`). Real site:
   `KaniProofRegistration::new`/`KaniGalleryRegistration::new` in
   `crates/amenable_kani/src/registry.rs` (both hand-written `const fn`,
   per that file's own doc comment — deliberately not derived). Cordial's
   tracing scanner/classifier doesn't check `const`-ness before
   recommending instrumentation. Because these two constructors are
   called from nearly every proof/gallery registration site in the
   crate, this single classifier miss cascaded into 530 downstream
   "associated function `new` not found" errors once the two real
   definitions failed to compile — not 530 independent bugs, one root
   cause times two call sites.
2. **`err(level = ...)` proposed on functions whose error type doesn't
   implement `Display`.** `tracing::instrument`'s `err` option requires
   the error type to implement `std::fmt::Display` (it renders the
   error via `tracing_core::field::display`). Real sites: `KaniSendError`,
   `KaniRecvError`, `KaniRecvTimeoutError`, `KaniJoinPathsError`,
   `KaniFromUtf8Error`, `KaniUtf8BufferError`, `KaniUtf8PositionError`,
   `KaniWindowsInvalidHandleError`, `StoplightError` — accommodation-
   model error types that mirror real `std` error shapes and don't (all)
   carry `derive_more::Display`. Cordial's classifier decides `err()` is
   warranted purely from "function returns `Result`," without checking
   whether the `Err` payload is `Display`.

Reverted immediately (`git checkout -- crates/amenable_kani`, then
re-added the `tracing` dependency edit was reverted along with it — the
Cargo.lock/root-`Cargo.toml` `tracing = "0.1"` workspace-dependency
declaration was left in place since it's harmless and will be needed
again). Full workspace `cargo check --workspace` re-confirmed clean
afterward. Both bugs are cordial classifier gaps (`~/repos/cordial/src/
etiquettes/tracing/`, the recipe-building logic, not the apply/policy
layer built in this step).

**Both fixed in cordial:**

1. `scan.rs::record_fn` now returns immediately (records nothing) when
   `sig.constness.is_some()`, before `classify()`/`recipe()` ever run --
   a `const fn` never enters the checklist or IR inventory at all, for
   either the scan path (`enricher.rs` → probes → checklist) or the
   direct-scan apply path (`apply/mod.rs` → `scan_rust_source`). A
   stale checklist row referencing a const fn (from before this fix)
   self-heals too: `apply`'s `recipe_for_gap` finds no matching record
   and counts it `unresolved` rather than writing anything.
2. New `~/repos/cordial/src/etiquettes/tracing/display_types.rs`:
   per-file `DisplayTypeFacts`, built once per scanned file (both scan
   paths already parse the whole file with `syn`, so this is a second
   pass over the same already-parsed AST, not a new file read) —
   collects locally defined types with a real `Display` impl
   (`#[derive(..Display)]`, matched on the derive path's *last*
   segment so `derive_more::Display` counts, not just a bare
   `Display`; and hand-written `impl ..Display for X`), plus local
   `type Alias<..> = Result<T, E>;` aliases so an aliased return type
   still resolves to its real `E`. `recipe.rs::fallible_err` now
   requires `ctx.err_is_displayable` in addition to `returns_result` —
   unresolvable `Err` types (foreign, cross-file, generic) are treated
   as **not** displayable by default, since a missed `err()` is a minor
   omission and a proposed `err()` that can't compile is the actual
   bug. A small, deliberately narrow well-known-safe list (`String`,
   bare `Error`) covers the cases this codebase's own existing cordial
   test fixtures already relied on (`Result<(), String>`), confirmed by
   running the full pre-existing test suite unchanged afterward.
   Scoped intentionally to single-file resolution, not a crate-wide
   registry: every real failing case found (`KaniJoinPathsError`,
   `KaniUtf8BufferError`, `StoplightError`, and 6 others) has the error
   type's own definition in the same file as the function returning
   it — a crate-wide pass would be more complete but wasn't needed for
   what's actually broken today.

2 new regression tests added to `~/repos/cordial/tests/
tracing_etiquette.rs` (`tracing_const_fn_is_never_flagged`,
`tracing_err_recipe_requires_confirmed_display`); all pre-existing
tests (`tracing_etiquette.rs`, `tracing_apply.rs`, full `cargo test
--all-features`) re-confirmed passing unchanged. `cargo clippy
--all-features --all-targets -D warnings` clean.

**Re-running the real `amenable_kani` `--apply` with both fixes in
place found two more real bugs, both fixed too:**

1. **`#[instrument]` proposed on a tuple-destructured generic
   parameter with no `Debug` bound, silently un-skippable.** Real
   site: `fn ensures((actual, expected): (T, T)) -> bool` (7 sites
   across `alloc_collections.rs`, `cell.rs`, `num.rs`, `primitives.rs`,
   `slice.rs`, `str.rs`, `sync_atomic.rs` — every `Ensures`/`Requires`
   impl following this shape). `tracing::instrument`'s real expansion
   records each binding *inside* a destructured pattern individually
   via `Debug` (`actual`, `expected`), not "the parameter" as one
   opaque unit — but cordial's own `param_names`/`unrecordable_params`
   (`classify.rs`, `recordable.rs`) only ever recognized a bare
   `Pat::Ident` parameter; a `Pat::Tuple` parameter was invisible to
   both, so it could never be named in a `skip(..)` list at all,
   silently proposing an `#[instrument(level = "trace")]` that
   couldn't compile whenever `T` lacked `Debug`. Fixed with a new
   `pattern_bindings(pat, ty)` in `recordable.rs`: zips a `Pat::Tuple`
   against its matching `Type::Tuple` element-wise (and unwraps
   `Pat::Reference`/`Pat::Paren`), recursing to find every real
   binding name and, where the shapes line up, its real sub-type;
   anything it can't structurally correlate (struct/slice/or-patterns)
   falls back to a generic `syn::visit::Visit`-based ident collector
   with an *unknown* type, which `unrecordable_params` then treats as
   **not recordable by default** — the same "unresolvable means don't
   propose it" bias as bug 2's `Display` check.
2. **A gated attribute's short `instrument(..)` form leaves a real
   `unused_imports` warning whenever the instrumented function only
   exists under an outer `#[cfg(<verifier>)]` ancestor.** Real sites:
   `compose.rs` and `gallery/ledger_gaap_free_function_contract.rs` —
   every instrumented function in both files sits inside a real
   `#[cfg(kani)]`-only trait/module (Kani proof harnesses and contract
   wrappers, which structurally can't exist under an ordinary build at
   all), so the `use tracing::instrument;` cordial inserted for the
   file was never actually reachable under a plain `cargo check`.
   Cordial had no way to know a function's *enclosing* items were
   already `cfg`-excluded. Fixed by sidestepping the question entirely
   rather than adding ancestor-cfg tracking: a `Gated` policy attribute
   now always renders fully qualified (`tracing::instrument`, or
   `::tracing::instrument` when a local `mod tracing;` shadows the
   crate name) instead of relying on the short form, and the plain
   `use tracing::instrument;` insertion now only fires when at least
   one applied gap in the file actually used `Bare` policy (the only
   policy that ever writes the short form) -- both in
   `apply/instrument.rs`/`apply/mod.rs`.

3 more regression tests (`tracing_skip_covers_tuple_destructured_
generic_params` in `tracing_etiquette.rs`;
`apply_gates_function_nested_in_outer_cfg_without_unused_import` in
`tracing_apply.rs`, plus updating the two existing gated-attribute
assertions to the new qualified form). Full `cargo test --all-features`
and `cargo clippy --all-features --all-targets -D warnings` both
re-confirmed clean after all four fixes together.

**Real `amenable_kani` `--apply` re-run with all four fixes in place —
clean:** 519 functions across 86 files, all `Gated(kani)`, zero
un-gated, zero skipped-by-policy, zero unresolved. `cargo check -p
amenable_kani`: zero errors, zero warnings. `cargo fmt -p amenable_kani`
needed a normal re-wrap pass afterward (the fully-qualified
`tracing::instrument` form is longer than the short form, pushing some
`cfg_attr(..)` lines past rustfmt's width — expected, not a bug, the
same as any codegen tool's output needing a `cargo fmt` pass). After
that: `cargo clippy -p amenable_kani --all-targets --all-features -D
warnings` clean, `cargo fmt -p amenable_kani --check` clean, `cargo test
-p amenable_kani` all passing, `cargo check --workspace` clean.
`amenable_creusot`/`amenable_verus` received zero file changes, exactly
as Skip policy requires. Re-verified two real `cargo kani` proofs
non-vacuously (serialized, one at a time, matching this workspace's
established Kani-call discipline): `stoplight::
verify_full_cycle_composes` (the crate's flagship full-cycle
composition proof, needs `-Z function-contracts -Z stubbing` for its
`#[kani::stub_verified]` harnesses) — `0 of 52 failed`, 0.55s; and
`rust_std::primitives::verify_tuple_field_access`, chosen specifically
because it directly calls `FieldAccessRecoversTheStoredValue::ensures`
(the exact tuple-destructured-parameter function bug 3 fixed) — `0 of
14 failed`, 0.04s. Both `VERIFICATION:- SUCCESSFUL`.

**A fifth bug, found by asking the right question about the 519-function
result above.** Every `#[cfg(kani)]`-nested function (46 of the 519,
confirmed by a real `syn`-based census) can never actually receive
`#[instrument]` in *any* build -- `Gated` policy suppresses it when
`kani` *is* active, and the item doesn't exist when it isn't -- so
gating (rather than skipping) them is dead weight. The same is true,
transitively, of anything reachable *only* from those functions:
`amenable_core::Ensures`/`Requires` impl methods are called either
directly inside a `#[cfg(kani)]` harness or via a `#[cfg_attr(kani,
kani::ensures(..))]`-attached contract (itself kani-only), confirmed by
reading every real call site and the trait's own doc comment ("`ensures
()` is the real check, called directly at the proof site").

A first fix recognized `Ensures`/`Requires` by trait name specifically
(a `proof_only_traits` config list). Correctly rejected: that's a
special case tied to this one workspace's own trait names, not a
mechanism any other cordial user gets for free. The real, reusable
invariant is call-graph reachability -- a function whose *every* real
caller, transitively, bottoms out in a proof-only entry point is
exactly as dead-to-tracing as the entry point itself, whatever trait
(if any) it happens to implement, and wherever in the workspace it
lives (not scoped to the crate being scanned -- a proof-only helper
type could just as easily live in a dependency crate another user's
proof harnesses call into).

Rebuilt as `~/repos/cordial/src/etiquettes/tracing/call_graph.rs`: a
new `CallGraphFacts`, computed once per workspace (cached like
`PathInclusionFacts`) by walking every workspace crate's source twice
-- once to collect function definitions (seeding `excluded` with every
function nested in an ancestor `#[cfg(<crate's own gate cfg>)]`, via
the same `apply_gate_crates`-derived cfg set the policy layer already
computes, factored out as `crate_gate_cfgs`), once to resolve each
function body's call sites (`Type::method(..)`/`Trait::method(..)`/
`bare_fn(..)` -- unambiguous path syntax only; `receiver.method(..)`
calls are never resolved, since without real type inference guessing
would risk a false exclusion, and a missed edge only risks under-
excluding, never the reverse) against a workspace-wide registry.
Fixed point: add a function to `excluded` once it has at least one
known caller and *all* of them are already `excluded`; a function with
zero known callers (`pub` API an external crate might call, or
genuinely dead code) is never added. One real, load-bearing correctness
fix along the way: `syn` never expands macros, and `assert!(..);`
written as a whole statement parses as `Stmt::Macro`, not `Stmt::Expr
(Expr::Macro(..), ..)` -- almost every real `Ensures`/`Requires` call
site in `amenable_kani` is wrapped in `assert!(..)`, so the visitor had
to hook the shared `visit_macro` (which both statement- and expression-
position macros delegate to), not `visit_expr_macro` alone, or it would
have silently found nothing.

4 new regression tests (`tracing_never_flags_function_called_only_from_
proof_context`, `tracing_still_flags_function_with_an_ordinary_caller_
too` in `tracing_etiquette.rs`; `apply_skips_function_called_only_from_
proof_context` and updating the trait-name test in `tracing_apply.rs`)
-- including a positive control confirming a function with *any* real
ordinary caller stays `Gated` even if it also happens to be called from
a proof context. Full `cargo test --all-features` and `cargo clippy
--all-features --all-targets -D warnings` re-confirmed clean.

**First re-run with the call-graph mechanism, 473 functions across 84
files -- a real number, but wrong in a way that took a direct user
challenge to surface.** ("the majority of Kani functions are requires
or ensures, so how did so many end up instrumented?") Checked: only 4
of 15 real `Ensures`/`Requires` impl blocks had actually been excluded
-- the rest, including `FieldAccessRecoversTheStoredValue::ensures`
(which an earlier, sloppier grep had wrongly reported as excluded),
stayed `Gated`. Root cause, confirmed with a real `syn::parse_file`
probe: `amenable_derive::harness!` -- the macro almost every real Kani
proof harness in `amenable_kani` is declared through (482 invocations
workspace-wide) -- is a function-like macro invoked at item position.
`syn` never expands macros; it parses `harness! { kani, NAME, { ..real
items.. } }` as an opaque `Item::Macro`, so every `#[kani::proof] fn`
inside one, and every `assert!(SomeType::ensures(..))` call inside
*that*, was completely invisible to the call-graph's collector -- not
a subtle edge case, the dominant shape almost every real call site
actually uses.

**Fixed properly, not by hardcoding `harness!`'s name** (the same
generalization lesson as the `proof_only_traits` mistake earlier):
`call_graph.rs` now extracts the trailing brace-delimited block of any
item-position macro invocation whose tokens end in one (`syn::Block::
parse_within` on the last brace group, filtering for `Stmt::Item`),
recursing into the extracted items exactly as if they'd been written
directly at that position -- works for `harness!` without knowing its
name, and for any future macro shaped the same way. A second, paired
fix: since `harness!`-generated functions never carry an explicit
`#[cfg(kani)]` in their own source text (the gating is baked into the
macro's real expansion, invisible to a source scan), a new `has_
verifier_attr` check seeds `ancestor_seed` from any attribute whose
path's *first* segment matches the crate's own gate cfg name --
`#[kani::proof]`, `#[kani::proof_for_contract(..)]` -- the same
generalization: not a hardcoded attribute name, but the verifier's own
real namespace, which is exactly what `apply_gate_crates`'s cfg name
already identifies.

1 new regression test (`apply_finds_calls_inside_a_harness_style_macro_
invocation`, modeling the exact `harness! { kani, NAME, { #[kani::
proof] fn .. } }` shape with a made-up macro name, confirming the fix
isn't tied to `amenable_derive::harness!`'s own name). Full `cargo test
--all-features` and `cargo clippy --all-features --all-targets -D
warnings` re-confirmed clean.

**Real `amenable_kani` `--apply` re-run with the fix in place: 396
functions across 83 files** (down from 473/84 -- 77 more real
exclusions once `harness!` bodies were actually visible). Verified
directly, not assumed: `amenable_kani`'s own checklist section now has
**zero** remaining `Ensures::ensures`/`Requires::requires` rows at all
(the 3 that first looked like stubborn leftovers turned out to belong
to `amenable_creusot`'s own, separate `ledger.rs`/`rust_std_witness.rs`
-- a Skip-policy crate, correctly untouched, found via an unscoped
`grep` across the whole checklist file rather than `amenable_kani`'s
own section). `cargo check`/`clippy -D warnings`/`fmt --check`/`test -p
amenable_kani` all clean; `cargo check --workspace` clean. The same two
real `cargo kani` proofs re-verified non-vacuously:
`stoplight::verify_full_cycle_composes` (`0 of 52 failed`) and
`rust_std::primitives::verify_tuple_field_access` (`0 of 14 failed`,
now directly exercising the correctly-excluded
`FieldAccessRecoversTheStoredValue::ensures`). `amenable_creusot`/
`amenable_verus` received zero file changes.

Not yet done: committing the `amenable_kani` apply result (`tracing`
added as a dependency in root `Cargo.toml`/`amenable_kani/Cargo.toml`,
396 functions instrumented across 83 files) — the user's call, not
done automatically per this project's commit policy.
