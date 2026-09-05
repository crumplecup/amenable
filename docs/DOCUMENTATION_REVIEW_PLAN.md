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

**Resolved (cross-crate fix, not scoped to one step):** the same "Step
8"/"Step 9" citations above turned out to be the source comments'
error, not the plan doc's. Stronger evidence surfaced mid-fix: the
commit that actually landed this work (`83a5907`, "GAAP ledger Step 7
— move Ledger's real methods to `amenable_gaap` for real, on all three
backends, **including reject/rollback**") explicitly titles the whole
span Step 7, and the plan doc's own prose (`GAAP_LEDGER_PLAN.md` line
1047) calls the Creusot/Verus re-pointing section "closing Step 7's
own follow-up," not a new step. Separately, `EXCHANGE_PROOF_DERIVATION_
PLAN.md` has its own real Step 8/9/10 headings (confirmed its Step 10
matches a different real commit message) — likely source of the
cross-contamination. Renumbered all 10 stray citations (9 files across
`amenable_gaap`/`amenable_kani`/`amenable_verus`/`amenable`) from
"Step 8"/"Step 9" to "Step 7", verified with `check-all-package` on
all four crates (comment-only change, no verifier re-run needed).

### Step 3 — `amenable_gaap` ✅ done

7 files, 1072 lines. Small, self-contained worked example (ledger/transfer).

- [x] Function docs: `contracts.rs`, `ledger/{mod,machine,types}.rs`,
      `tokens.rs`, `transfer.rs` — all read in full, cross-references
      verified. Clean (the Step 7/8/9 citations here were already fixed
      as part of the cross-crate renumbering above).
- [x] Module docs: all files + `lib.rs`. Clean.
- [x] README.md (132 lines) — fixed one broken link:
      `[ledger.rs](src/ledger.rs)` pointed at a file that no longer
      exists (split into `ledger/{mod,machine,types}.rs` during the
      500-LOC modularity pass) — repointed to the real files.

### Step 4 — `amenable_std` ✅ done

135 files, 20544 lines. Real std-type registrations bridging `amenable_core`
to all three backends.

- [x] Function docs (hand-written subset): all ~28 crate-root files
      (`cert.rs`, `error.rs`, `verus_call_shape_derive.rs`, `lib.rs`, and
      the 24 tiny proof-law modules) read in full and cross-referenced —
      clean, except a direct contradiction found across the law modules
      (below). `creusot_gallery`/`verus_gallery` (13 files, 2772 lines —
      much larger than the plan assumed): `mod.rs` for both read in full
      plus cross-reference spot-checks on the largest case files; no
      further drift found beyond the systemic issue below.
- [x] Module docs: same hand-written subset, plus `rust_std/mod.rs` and
      `verus_witness/mod.rs` (this crate has no thematic subdirectories
      under either, unlike `amenable_kani`/`amenable_verus`). Found and
      fixed a real rustdoc error (`[`VerusCheckedProof`]` unresolved in
      `verus_witness/call_shape.rs` — not in scope there, needed a
      fully-qualified path) plus an adjacent inaccurate count ("~280
      construction sites across this file" — that file has zero; they're
      spread across the whole module). Only caught under `--features
      verus`, which cordial's own default sweep doesn't exercise.
- [x] **Systemic finding, fixed across 18 files**: the ~24 law modules'
      "`X` is the Nth contract type in the `amenable_core::Ensures`/
      `Requires` worklist" claims directly contradict each other —
      confirmed multiple real conflicts (two files each claim "ninth",
      "tenth", "eleventh", "thirteenth"; three files all claim
      "fourteenth"; one cross-reference calls the same type both
      "seventh" and, in its own file, "tenth"). No authoritative worklist
      order exists to reconstruct the "true" numbering, and the ordinal
      itself carries no functional weight (unlike the substantive
      "collapses N real hand-written sites" claim beside it, which
      checked out via grep/cross-reference in every file read). Dropped
      the specific ordinal claim from all 18 affected files rather than
      inventing an unverifiable total order.
- [x] **Systemic finding, fixed across 4 files (12 sites)**: every
      `amenable_std::creusot_witness` reference is stale — that module
      really did live here once, but moved to `amenable_creusot::
      rust_std_witness` for a real Cargo-cycle reason (documented in its
      new home's own doc comment) that the 12 remaining references never
      picked up. Fixed `verus_witness/mod.rs`'s comparison paragraph (the
      orphan-rule mechanism genuinely changed, not just the path) and 8
      more mechanical sites across 3 `creusot_gallery` files (2 inside
      historical `claim` strings, reworded as "at the time, since moved
      again" rather than rewritten to erase the real two-hop history).
- [x] README.md (63 lines) — as suspected, the most stale: named the
      predecessor tool `elicit_doc` (renamed to `cordial` in August),
      cited `rust_std.rs`/`verus_witness.rs` as files (both are
      directories now), and its Coverage section's numbers (421/440,
      95.7%, "remaining 19") were superseded by real growth in the
      accountable-type universe (currently 422/457, 92.3%, 35 open —
      re-measured live via `cordial coverage --crate-name amenable_std`
      rather than trusted from memory). The two explained gap categories
      (`NonZero*`/`LayoutErr` tool false-negatives, the Windows cluster)
      still check out substantively; 16 newly-appeared open items
      (`core::range::*`, `ArrayWindows`, some `libc`/`os::unix` types)
      are honestly flagged as untriaged backlog rather than folded into
      either explained category without evidence. Reworded to point at
      the live command instead of a hand-copied snapshot, so this exact
      staleness class can't recur silently.
- Also fixed one unrelated pre-existing markdownlint failure hit while
  linting this step's README (`docs/MODULARITY_500_LOC_PLAN.md`'s MD004:
  a wrapped line starting with a literal `+` inside a `-`-style list,
  parsed as a list-style violation).

### Step 5 — `amenable_kani` ✅ done

226 files, 42208 lines — the largest crate. Hand-written subset: `*_model.rs`
files at crate root (`btree_model`, `net_model`, `utf8_model`, etc. — these
are real hand-built accommodation models, not templated carriers, despite
the naming similarity), `compose/`, `registry.rs`, `witness.rs`, `error.rs`,
`ledger/`, `gaap_ledger.rs`, `stoplight.rs`, `calculator.rs`, `gallery/`.

- [x] Function docs: all ~29 crate-root files read (most in full; the
      smaller, highly-formulaic accommodation models skimmed after the
      first ~10 showed zero drift), `compose/` and `gallery/`'s 27 files
      swept by citation-pattern grep after 3 full reads and spot-checks
      confirmed the self-registration boilerplate carries no external
      cross-references at risk of drift (only self-referential fully-
      qualified paths, which can't go stale independent of the code
      itself).
- [x] Module docs: same set, plus `rust_std/mod.rs` (no thematic
      subdirectories at this level, unlike `amenable_verus`).
- [x] Found and fixed: `lib.rs`/`witness.rs` each had a stale bare
      `` `rust_std.rs` `` (now `rust_std/`, split during the modularity
      pass); `rust_std/mod.rs` cited `num.rs` for what's now a genuine
      multi-file `num/` directory (`nonzero/{contracts,signed,unsigned}.rs`
      alone are 870 lines); 4 files had a stale `elicit_doc` tool-name
      reference; `gaap_ledger.rs`'s own module doc described itself as a
      "candidate that will eventually replace" `amenable_kani::ledger`'s
      own production `Ledger` — but `ledger/mod.rs`'s own doc comment
      already confirms that migration is done and calls `gaap_ledger.rs`
      the real, final harnesses; 2 gallery investigation files cited a
      bare `ledger.rs` from before both the Step 7 relocation to
      `amenable_gaap` and the later directory split — reworded as "this
      crate's own now-retired copy at the time" rather than corrected to
      a current path that doesn't apply retroactively.
- [x] README.md (48 lines) — as flagged, the most stale: 3 more
      file-vs-directory citations (`rust_std.rs`, `compose.rs`,
      `fs_model.rs`), and its harness count ("419") was stale — the real,
      CLI-verified current count is 445 (356 tracked production proofs
      via `amenable verify kani --list` + 89 gallery cases via
      `amenable gallery list`). Reworded to point at both live commands
      instead of a hand-copied number, matching `amenable_std`'s fix.

### Step 6 — `amenable_creusot` ✅ done

83 files, 13106 lines. Hand-written subset: `ledger/`, `stoplight.rs`,
`witness.rs`, `rust_std_witness/`, plus `generated/`'s own doc-comment
*mechanism* (not its per-file content, which is codegen output).

- [x] Function docs: `ledger/` (4 files), `stoplight.rs` (read in full —
      440 lines), `witness.rs`, `rust_std_witness/` (mod.rs read in full;
      32 leaf files, structurally a templated one-file-per-carrier tree
      despite being hand-written, swept by citation-pattern grep after
      spot-checking the largest 7).
- [x] Module docs: same set, plus `rust_std/mod.rs` (already touched
      during the systemic fix below) and `lib.rs`.
- [x] **Systemic finding, same class as Step 4/5**: `amenable_std::
      creusot_witness` turned out to have 8 *more* stale references
      beyond the 12 already fixed in `amenable_std` itself — a
      workspace-wide grep after this step's first hit found 4 in
      `amenable_creusot::rust_std_witness/mod.rs`'s own header (ironic:
      the module's own doc comment cited its own pre-move address) and 4
      more scattered across `rust_std`/`rust_std_witness` leaf files.
      Fixed all 20 sites workspace-wide now.
- [x] Found and fixed a real internal contradiction within
      `stoplight.rs` itself: the file's own header explains `Green`/
      `Yellow`/`Red` moved to `amenable_core` (with the real Cargo-cycle
      story), but 3 of 9 "Sanitized mirror of `amenable_kani::X`" struct
      doc comments two paragraphs later still attributed the moved types
      to `amenable_kani` — confirmed via `lib.rs`'s own export list
      (`Green`/`Yellow`/`Red` aren't there anymore) before fixing; the
      other 6 (tokens, `Established`, `Stoplight`, `StoplightError`)
      were correctly still attributed to `amenable_kani`, left alone.
- [x] Fixed ~10 more bare `ledger.rs`/`rust_std.rs`/`rust_std_witness.rs`
      file citations (now directories) across `lib.rs`, `witness.rs`,
      `stoplight.rs`, `ledger/mod.rs`, `rust_std_witness/mod.rs`.
- [x] README.md (97 lines) — same three file-vs-directory citations,
      plus a coverage count ("93 harness registrations") that underclaims
      the current real count by nearly half (174, confirmed via grep) —
      this crate roughly doubled in size since the README was last
      touched. Reworded to cite the counting command directly rather
      than a number that will go stale again.

### Step 7 — `amenable_verus` ✅ done

179 files, 14964 lines. Hand-written subset: `exchange_support.rs`,
`witness_accommodation.rs`, `gallery/` (investigation files — real prose,
central to how this crate documents its own findings), `lib.rs`'s `#[path]`
splice mechanism.

- [x] Function docs: `exchange_support.rs` (already reviewed/fixed
      earlier this session during the cfg-gate work), `witness_accommodation.rs`,
      all 12 `gallery/*.rs` + `gallery/ledger_exchange/*.rs` files (7 read
      in full, 5 swept by citation-pattern grep after the pattern showed
      zero drift beyond already-known classes).
- [x] Module docs: `gallery/mod.rs`, `gallery/support.rs`,
      `gallery/ledger_exchange/mod.rs`, `lib.rs` (already re-verified this
      session via the `task_and_thread`/`exchange_support` work).
- [x] **Real finding, documented but not acted on**: `witness_accommodation.rs`
      described itself as existing only because `amenable_core::witness`
      mixed clean trait mechanics with `inventory`-dependent registry code
      in one file, with a "when that's ever split, delete this and
      mod-include the real one" note. That split already happened (Step
      1's own review: `witness/{core_trait,registry,support,tree}.rs`).
      Investigated whether the described fix is now that simple: not
      quite — `core_trait.rs` itself has one real cross-file dependency
      the other eight mod-included files don't (`use super::support::
      WitnessSupportSummary`), so mod-including it needs `support.rs`
      alongside it, nested to preserve that relative path, not a flat
      swap. Documented precisely in the file itself; the actual migration
      is real, scoped, plausible, and not performed here (would need real
      `verus` re-verification of a structural change, beyond this pass's
      doc-accuracy scope).
- [x] Found and fixed a real internal contradiction in `stoplight.rs`
      (during Step 6, this crate's own `Green`/`Yellow`/`Red` structs
      correctly attributed to `amenable_core` there — cross-checked
      against this crate's real exports here, consistent).
- [x] README.md (60 lines) — as stale as `amenable_kani`: a proof-function
      count ("332 verified") that undercounts the real, `just verify-verus`-
      confirmed current total (485) by 45%. Reworded to point at the live
      command instead of a hand-copied number.

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
