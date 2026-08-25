# cfg Hygiene Plan

**Status:** 🔲 Step 0 done and verified. Steps 1–3 not started — parked
deliberately until the tracing-instrumentation rollout (cordial's
`DERIVE-*`/tracing checklist work) lands, per explicit direction.

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
config-toggleable suite of etiquette applies, plus a `gate_cfg`/
`bare_compiler_crates` policy so tracing's own apply doesn't blindly
insert `#[instrument]` into Kani-reachable or Verus-spliced code).
Tracked here only for sequencing: **do this work first**, using
whatever `cfg-hygiene` (Step 2) already exists at that point to validate
its own output, then return to this plan's Steps 1–2 for the two
narrower, `amenable_derive`-specific and cordial-specific pieces.
