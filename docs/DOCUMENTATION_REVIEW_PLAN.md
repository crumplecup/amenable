# Documentation review: functions → modules → READMEs

## Goal

The core trait foundation (`amenable_core`) is now wired up to real std-library
coverage on all three verifier backends (Kani/Creusot/Verus), cordial's
quality checklists are clean, and `#![warn(missing_docs)]` already guarantees
every public item has *some* doc comment. This pass is about *accuracy and
usefulness*, not presence: catching drift (the `rust_std::task_and_thread`
intra-doc link broken by this session's own flatten is a real example of the
class of bug this catches), stale rationale, and thin docs that don't explain
*why*, not just *what*.

Per-crate order, bottom-up within each crate:

1. **Function docs** — hand-written, non-formulaic logic only (see Scope).
2. **Module docs** — the `//!` header on every module file, including each
   `rust_std`-style subdirectory's own `mod.rs` (the thematic index, hand-
   written even where its leaf carriers are templated).
3. **README.md** — written last per crate, so it's grounded in function/module
   detail already re-verified fresh, not copied from memory.

Crate order follows the dependency graph, foundation first: a crate's own
docs read better once the crate underneath it is confirmed accurate, since
"see `amenable_core::Ensures`'s own doc comment" is only worth writing once
that doc comment has itself been checked.

## Scope — what "function docs" covers

Per user decision: **hand-written logic only**, skipping the ~450 templated
one-file-per-std-type "carrier" files across `amenable_kani::rust_std`,
`amenable_std::{rust_std,verus_witness}`, `amenable_creusot::rust_std`, and
`amenable_verus::rust_std` — these already follow one consistent, established
doc pattern per crate (confirmed throughout this session), and a function-by-
function pass across near-identical files has a poor effort/value ratio.

Concretely, "hand-written logic" means:

- `amenable_core`, `amenable_derive`, `amenable_gaap`, `amenable` — all of it
  (no templated subtrees).
- Within `amenable_std`/`amenable_kani`/`amenable_creusot`/`amenable_verus`:
  registries, error types, compose/ledger/stoplight worked examples, gallery
  investigation files, macros, and each `rust_std`-family's own directory-
  level `mod.rs`/`mod/*.rs` grouping file — but not the individual leaf
  carrier files' own function docs.
- `derived_witness/` (amenable_verus) and `generated/` (amenable_creusot,
  amenable_verus's `gallery/generated`) are out of scope entirely — codegen
  output, never hand-edited (already true today; a leftover doc issue there
  is a codegen-tool bug, not a doc-review finding).

Module docs are reviewed more broadly than function docs: every module file's
`//!` header gets checked, including carrier leaf files' one-two line module
doc (cheap to skim, and these are exactly what stayed accurate across the
`task_and_thread` regression this session, unlike the parent's own doc) — but
a *rewrite* is only warranted where something is actually wrong or stale, not
a style pass for its own sake.

## Steps

### Step 1 — `amenable_core` ✅ done

23 files, 3493 lines. The trait foundation everything else cites by name
(`Ensures`, `Requires`, `Evidence`, `Witness<V>`, `ProofToken`, `Sidecar`,
`Verifier`, `StateMachine`, `Certificate`/`Registry`). Getting this crate's
docs right first means every later "see `amenable_core::X`'s own doc
comment" reference is trustworthy.

- [x] Function docs: `cert.rs`, `contract.rs`, `evidence.rs`, `exchange.rs`,
      `provenance.rs`, `roles.rs`, `state_machine.rs`, `verifier.rs`,
      `witness.rs`, `verus_carrier/discovery.rs`, `tracing_init.rs`,
      `calculation.rs`, `chain.rs`, `link.rs`, `state.rs`, `stoplight.rs`.
      All 23 files read in full — already excellent (recent 2026-09-01
      polish pass), no changes needed. Every cross-reference checked
      against real source (`docs/AMENABLE_PLAN.md`, `docs/
      STATE_MACHINE_DERIVATION_PLAN.md`, `docs/GAAP_LEDGER_PLAN.md`'s
      Step 7, `amenable::verus_source_directory`, `AmenableError`) —
      all still accurate.
- [x] Module docs: every file above, plus `lib.rs`'s crate-level `//!`.
      Same result: accurate throughout, no changes needed.
- [x] README.md (67 lines) — found one real stale reference: "`StateMachine`
      / `Amenable`" listed the deleted `Amenable` trait (confirmed gone via
      grep; `state_machine.rs`'s own doc comment says it "replaces the
      original StateMachine/Amenable trait pair entirely"). Fixed.

### Step 2 — `amenable_derive` ✅ done

22 files, 5222 lines. Proc macros generating real proof code for all three
backends — the highest-leverage crate to get right, since a stale doc here
misleads about what code three different verifiers will actually receive.

- [x] Function docs: all 22 files read in full. Found and fixed 5 real
      drift issues: `establish.rs`/`exchange.rs` cited a bare `ledger.rs`
      that's now a `ledger/` directory (split during the 500-LOC
      modularity pass) — repointed each to the real crate
      (`amenable_gaap::ledger` vs `amenable_kani::ledger`, confirmed by
      grepping which one actually carries the cited `reject`/`rollback`
      content); `proof_token.rs` cited `GreenToken`/`PendingToken` &c. as
      still-hand-written examples, but both have since been converted to
      the very derive being documented (confirmed via grep) — reworded
      with accurate current examples (`amenable_creusot::stoplight`, the
      `rust_std` corpus); `sidecar.rs` cited "`GAAP_LEDGER_PLAN.md`'s Step
      9", which doesn't exist as a heading (the doc's real headings only
      go to Step 7) — fixed to Step 7; `verus_fragment.rs` named
      `[crate::verus_carrier]`, a module that doesn't exist in this crate
      (it's `amenable_core::verus_carrier` — a proc-macro crate can't
      export non-macro items) — corrected.
- [x] Module docs: all 22 files, `lib.rs`. Same pass as function docs
      above (this crate's module docs and function docs are tightly
      interleaved).
- [x] README.md (450 lines) — found 2 more of the same drift classes:
      "every hand-written `ProofToken` impl" overclaim (same fix as
      `proof_token.rs`), and a "read its source (`ledger.rs`, ...)" tour
      pointing at a file that's now a directory.

**Open finding, not resolved (flagged, not silently fixed):**
`docs/GAAP_LEDGER_PLAN.md`'s own heading structure has only a `### Step
7` heading covering lines 963–1238, but 9+ source comments across 4
crates (`amenable_gaap`, `amenable_kani`, `amenable_creusot`,
`amenable_verus`, `amenable`) consistently cite specific sub-parts of
that same span as "Step 8" and "Step 9" — not a typo (confirmed via
git log: real historical commits titled "GAAP ledger Steps 6-8" and
"Step 10" show 8/9/10 existed as real, distinct steps at some point
before being consolidated under one "Step 7" heading in the current
doc, without the source comments' own numbering ever being updated to
match). Fixing this needs either splitting the heading to match the
source citations or renumbering 9+ source comments to match the
heading — a real decision, not something to guess silently. Left
exactly as found in both the doc and the source.

### Step 3 — `amenable_gaap`

7 files, 1072 lines. Small, self-contained worked example (ledger/transfer).

- [ ] Function docs: `contracts.rs`, `ledger.rs`, `tokens.rs`, `transfer.rs`.
- [ ] Module docs: all 4 + `lib.rs`.
- [ ] README.md (132 lines, last touched 2026-08-28).

### Step 4 — `amenable_std`

135 files, 20544 lines. Real std-type registrations bridging `amenable_core`
to all three backends.

- [ ] Function docs (hand-written subset): `cert.rs`, `error.rs`,
      `verus_call_shape_derive.rs`, `verus_derive_canary.rs`, `creusot_gallery.rs`,
      `verus_gallery.rs`, plus the small worked-example modules at crate root
      (`compose_*`, `array_into_iter_*`, `*_matches_*`, etc. — the ~24 tiny
      root-level proof-law modules, each a few lines, not templated carriers).
- [ ] Module docs: the same hand-written subset, plus every `rust_std`/
      `verus_witness` subdirectory's own `mod.rs` (not each leaf carrier).
- [ ] README.md (63 lines, last touched 2026-08-17 — likely the most stale
      relative to actual crate size; check it explains the witness/registry
      machinery, not just lists modules).

### Step 5 — `amenable_kani`

226 files, 42208 lines — the largest crate. Hand-written subset: `*_model.rs`
files at crate root (`btree_model`, `net_model`, `utf8_model`, etc. — these
are real hand-built accommodation models, not templated carriers, despite
the naming similarity), `compose.rs`, `registry.rs`, `witness.rs`, `error.rs`,
`ledger.rs`, `gaap_ledger.rs`, `stoplight.rs`, `calculator.rs`, `gallery/`.

- [ ] Function docs: every `*_model.rs` root file + the list above.
- [ ] Module docs: same set, plus every `rust_std` subdirectory's own
      `mod.rs`/`macros.rs`.
- [ ] README.md (48 lines, last touched 2026-08-07 — stale by two months of
      heavy work in this crate; needs a real rewrite, not a touch-up).

### Step 6 — `amenable_creusot`

83 files, 13106 lines. Hand-written subset: `ledger.rs`, `stoplight.rs`,
`witness.rs`, `rust_std_witness.rs`, plus `generated/`'s own doc-comment
*mechanism* (not its per-file content, which is codegen output).

- [ ] Function docs: `ledger.rs`, `stoplight.rs`, `witness.rs`,
      `rust_std_witness.rs`.
- [ ] Module docs: same set, plus `rust_std`'s own `mod.rs`.
- [ ] README.md (97 lines, last touched 2026-08-18).

### Step 7 — `amenable_verus`

179 files, 14964 lines. Hand-written subset: `exchange_support.rs`,
`witness_accommodation.rs`, `gallery/` (investigation files — real prose,
central to how this crate documents its own findings), `lib.rs`'s `#[path]`
splice mechanism.

- [ ] Function docs: `exchange_support.rs`, `witness_accommodation.rs`,
      every `gallery/*.rs` file (not `gallery/generated/`).
- [ ] Module docs: same set, plus `rust_std`'s own `mod.rs` and each
      subdirectory `mod.rs` (already partly re-verified this session via the
      `task_and_thread`/`exchange_support` work — confirm the rest).
- [ ] README.md (60 lines, last touched 2026-08-07 — same staleness as
      amenable_kani; this crate gained `mod_thin_skip` exemptions, the
      `exchange_support` cfg-gate fix, and the `task_and_thread` flatten
      since, none reflected yet).

### Step 8 — `amenable` (top-level facade/CLI)

43 files, 6038 lines. Reviewed last: the front door, and the one README a
new reader hits first (it's what `docs.rs`/crates.io show), so it should
summarize a workspace whose lower layers have just been re-verified.

- [ ] Function docs: all 43 files (`assessment/`, `cli/`, `cli_output.rs`,
      `creusot_export.rs`, `verus_export.rs`, `verus_exchange_export.rs`,
      `verus_gaap_tokens_export.rs`, `error.rs`, `gallery.rs`, `kani.rs`,
      `paths.rs`, `registry_dump.rs`).
- [ ] Module docs: all of the above, `lib.rs`, `main.rs`.
- [ ] README.md (75 lines, last touched 2026-09-01 — the most recently
      touched; confirm it still reads as the workspace's front door after
      Steps 1–7 land, and cross-link the other 7 READMEs where useful).

## Validation per step

Doc-only changes don't need a verifier re-run *unless* a doc fix touches
code inside a `verus! {}`/`#[kani::proof]` body (rare, but the
`task_and_thread` intra-doc-link fix earlier this session shows module docs
can legitimately need a real `verify-verus`/`verify-kani` run when the fix
is more than prose — check case by case). Baseline for every step:

- `just check-all-package <crate>` (fmt/clippy/test clean, zero warnings).
- `RUSTDOCFLAGS="-D warnings" cargo doc -p <crate> --no-deps` (catches
  intra-doc link breakage the way this session's real bug was caught).
- `cordial quality --crate-name <crate>`: `doc-warnings` and `derives`
  checklists stay at 0 open items.
- Commit once per crate (function + module + README together), not per
  sub-phase — these three sub-steps are one coherent unit of review per
  crate, unlike the multi-week modularity-split work.
