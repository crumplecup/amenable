# Modularity: 500 LOC file-size cap

## Goal

Cap every source module at **500 lines**. The line count is a forcing
function, not the point: the real aim is clarity — better separation of
concerns, cohesive modules, and a file tree that reads like the design.
Splits follow real conceptual seams (a shared layer vs. its per-type
instances, one command family vs. another, discovery vs. rendering), never
blind line-count chopping.

This continues the work tracked in cordial's `modularity.checklist.md`. The
prior rounds (see `git log` on `dev`, ~20 `refactor(...): split ... into
cohesive submodules` commits) cleared every file cordial's z-score gate
flags. This round targets **every module ≥ 500 lines**, including the 8 that
sit above 500 but below cordial's statistical outlier threshold because
their crate has high size variance.

## Scope — 20 modules ≥ 500 lines

Snapshot from `~/.cordial/amenable/findings/modularity.csv` (regenerate with
`cordial quality` after each phase).

| # | Crate | Module | Lines | cordial-flagged | Validation |
| --: | --- | --- | --: | :-: | --- |
| 1 | `amenable_kani` | `rust_std::sync_atomic` | 871 | yes | `verify-kani` |
| 2 | `amenable_kani` | `rust_std::num::nonzero` | 810 | yes | `verify-kani` |
| 3 | `amenable_kani` | `rust_std::path` | 765 | yes | `verify-kani` |
| 4 | `amenable_kani` | `rust_std::ops` | 600 | yes | `verify-kani` |
| 5 | `amenable_kani` | `rust_std::cell` | 598 | yes | `verify-kani` |
| 6 | `amenable_kani` | `rust_std::alloc_vec` | 575 | yes | `verify-kani` |
| 7 | `amenable_kani` | `rust_std::slice::split_n_and_rsplit` | 574 | yes | `verify-kani` |
| 8 | `amenable_creusot` | `rust_std::num` | 631 | yes | `verify-creusot` |
| 9 | `amenable_gaap` | `ledger` | 539 | no | cross-backend (worked example) |
| 10 | `amenable_std` | `verus_witness::hash_ffi_collections_tail` | 953 | yes | `verify-verus` |
| 11 | `amenable_std` | `verus_witness::iter_adapters_b` | 765 | yes | `verify-verus` |
| 12 | `amenable_std` | `verus_witness::char_decode_slice_chunking` | 604 | yes | `verify-verus` |
| 13 | `amenable_std` | `verus_witness::io_and_sync_atomic` | 520 | no | `verify-verus` |
| 14 | `amenable_std` | `verus_witness::cell` | 508 | no | `verify-verus` |
| 15 | `amenable_std` | `verus_witness::panic_ops_time_future` | 507 | no | `verify-verus` |
| 16 | `amenable_std` | `verus_witness::machinery` | 503 | no | `verify-verus` |
| 17 | `amenable_std` | `verus_gallery` | 849 | yes | `verify-verus` |
| 18 | `amenable_derive` | `witness` | 811 | no | **all three verifiers** |
| 19 | `amenable_derive` | `state_machine` | 596 | no | **all three verifiers** |
| 20 | `amenable_derive` | `<crate>` (`lib.rs`) | 816 | no | **all three verifiers** |

## Risk / validation cost (per user)

- **`amenable_derive` is the highest risk**, not the lowest: it *derives
  proof code* for all three verifier backends, so any change needs Kani +
  Creusot + Verus all re-run, not just `cargo check`. Its `lib.rs` is also
  a `proc-macro = true` crate root — it can only export `#[proc_macro*]`
  items, so the entry-point fns can't move; only their bodies (already in
  `expand_*` submodules) and the module-level docs can be reorganized.
- **`amenable_std::verus_witness/*`** carry 10–27 `include_str!` literal
  compile-time paths each; moving a file down a directory level breaks
  those, and `#[cfg(verus_keep_ghost)]` code is invisible to `cargo check`
  — only `just verify-verus` validates the result.
- **`amenable_gaap::ledger`** is the shared worked example that all three
  backends prove `Ledger::validate` / `::commit` against.
- **`amenable_kani`** has the fastest validation loop and the most prior
  tooling; `amenable_creusot` needs the real Creusot translate.

## Phases

Work top-of-list order within each phase. Commit per file (or per cohesive
group of files split together). Regenerate `cordial quality` at the end of
each phase and reconcile the checklist.

- **Phase 1 — `amenable_kani` (items 1–7).** Per file: split along the
  shared-layer/per-type seam, `just check-all-package amenable_kani`, then
  one representative `amenable verify kani --proof ...` per new sub-file
  (one real kani compile validates every sibling compiles under
  `#[cfg(kani)]` too). Serialize kani calls; check for orphaned `cbmc`
  first.
- **Phase 2 — `amenable_creusot` (item 8).** Split `rust_std::num`, then
  `just verify-creusot-translate` and `just verify-creusot`.
- **Phase 3 — `amenable_gaap` (item 9).** Split `ledger.rs`, then
  `just check-all` + Kani `gaap_ledger` harnesses + `verify-creusot` +
  `verify-verus` on the gaap tokens.
- **Phase 4 — `amenable_std::verus_witness` + `verus_gallery` (items
  10–17).** Batch. Rebuild the scratchpad `split_top_level.py` tooling if
  needed. For every `include_str!` whose path changes: adjust the literal
  or move the fragment. `just verify-verus` after each file (or each small
  group). This is the fragile phase — expect `include_str!` path fixups
  and lost `#[cfg(windows)]` gates.
- **Phase 5 — `amenable_derive` (items 18–20).** Last, and most carefully.
  Split `witness.rs` / `state_machine.rs` bodies into `expand_*` helper
  submodules; reorganize `lib.rs` docs without moving the `#[proc_macro*]`
  entry points. After **each** file: `just check-all`, `just verify-kani`,
  `just verify-creusot`, `just verify-verus` — the full matrix, since this
  crate emits all of their proof code.

## Recurring post-split fix classes

(carried from `project_modularity_checklist_status` memory — expect all of
these again)

1. Shared items landing in the wrong file under an "attach to preceding
   item" grouping rule → cross-file `use super::<file>::<item>;`, sometimes
   needing `pub(crate)` widening first.
2. A `///` doc comment for the *next* item orphaned at the tail of the
   *previous* file when a boundary lands mid-comment — diff every
   reconstructed file against `git show HEAD:<path>`.
3. `cfg(kani)` / `cfg(verus_keep_ghost)` / `cfg(creusot)` imports that only
   the gated proof bodies use — invisible to `cargo check`, caught only by
   the real verifier. Re-add whatever the verifier names, gated.
4. `include!` / `include_str!` relative paths need re-depthing when the
   including file moves down a level (check whether the codegen tool uses a
   fixed absolute base first).
5. `harness!`-derived proof IDs are `module_path!()`-derived: splitting a
   harness-containing file churns `artifacts/kani-verification-results.csv`
   (accepted; append-only). `gallery_harness!` cases' `harness` selector
   field needs the new submodule name inserted or `amenable gallery run`
   fails.

## Status

- [x] **Phase 1 — `amenable_kani` (7/7)** — all under 500, each verified with
  check-all-package + representative harness re-runs.
  - [x] `rust_std::sync_atomic` 871 → boolean/signed/unsigned/pointer/ordering (max 282) — `3141457`
  - [x] `rust_std::num::nonzero` 810 → signed/unsigned/contracts (max 352) — `3883ed4`
  - [x] `rust_std::path` 765 → components/path_buf/display/windows_prefix (max 265) — `50e3502`
  - [x] `rust_std::ops` 600 → ranges/bound/control_flow (max 337) — `0ff975a`
  - [x] `rust_std::cell` 598 → cell_family/ref_cell/once_lazy_unsafe (max 290) — `e22dbe7`
  - [x] `rust_std::alloc_vec` 575 → vec_core/iterators (max 326) — `1c94010`
  - [x] `rust_std::slice::split_n_and_rsplit` 574 → split_n/rsplit/rsplit_n (max 218) — `7b0fa5a`
- [x] **Phase 2 — `amenable_creusot` (1/1)** — `rust_std::num` 631 →
  nonzero/wrapping_saturating/parse/float (max 303). `verify-creusot-translate`
  clean, `verify-creusot` → Proved (150 files). Commit after `cb50d1f`.
- [x] **Phase 3 — `amenable_gaap` (1/1)** — `ledger` 539 → types (64) / machine
  (468). All three backends: 4 kani gaap_ledger harnesses pass, verify-creusot
  Proved (150 files), verify-verus 485 verified 0 errors, no companion diffs.
- [x] **Phase 4 — `amenable_std` verus (8/8)** — all peeled as sibling files at
  the same directory depth so no `include_str!` paths moved (verus_gallery is the
  one exception — a directory, but its claims are string literals). check-all-package
  amenable_std clean after each; verify-creusot-translate clean; verify-verus pending.
  - [x] `verus_witness::machinery` 503 → machinery + call_shape
  - [x] `verus_witness::cell` 508 → (moved from_utf8 fragments to ascii_and_drain) → 486
  - [x] `verus_witness::io_and_sync_atomic` 520 → io_tail + sync_atomic
  - [x] `verus_witness::panic_ops_time_future` 507 → (peeled time.rs) → 383
  - [x] `verus_witness::char_decode_slice_chunking` 604 → char_decode + slice_chunking
  - [x] `verus_witness::iter_adapters_b` 765 → iter_markers + iter_adapters_b + iter_adapters_d
  - [x] `verus_gallery` 849 → infra + numeric_cases + spec_cases + binder_cases (dir)
  - [x] `verus_witness::hash_ffi_collections_tail` 953 → primitives_and_pointers +
    hash_ffi_collections_tail + os_windows_handles
- [x] **Phase 5 — `amenable_derive` (3/3)** — all pure code moves, generated macro
  output identical. Full 3-backend re-verify: kani (stoplight Witness+StateMachine+
  harness!, atomic_ptr) pass, verify-creusot Proved (150 files), verify-verus 485
  verified 0 errors, no generated diffs.
  - [x] `lib.rs` 816 → extracted inline `#[derive(Provenance)]` impl to `provenance.rs`
    (plus shared attr parsing to `attr_options.rs`); lib.rs now 378, entry points only
  - [x] `state_machine.rs` 596 → mod / emit / parse
  - [x] `witness.rs` 811 → mod / product / sum / helpers

## Result

**Zero modules ≥ 500 lines.** cordial's modularity checklist: 13 → 1 (the one
remaining is `amenable_verus::rust_std` "Rebalance" — 11353 lines, 80% of the
crate's sibling mass; a directory already subdivided into 12 thematic groups,
flagged because `rust_std` is genuinely the crate's bulk — a judgment call, not
one of the 20 files here). The tracing-attr apply below briefly regressed this
to 3 → 1 (pushed `amenable_kani::ledger` and
`amenable_std::verus_witness::char_ffi_errors` from 494 to 502 lines each, +8
instrument lines); both re-split in `346e014` (`ledger` → `ledger/{mod,mirror}.rs`;
`char_ffi_errors` → `char_conversion.rs` + `cstring_ffi.rs`, also relocating a
stray `Rc`-witness const to its actual sole consumer) — confirmed back to 1.

### Follow-on findings the splits surfaced

Regenerating `cordial quality` after Phase 5 showed two secondary areas move:

- **Tracing instrumentation 0 → 24 → 704 → 0 (DONE).** The first pass (`0 → 24
  → 0`) instrumented the 19 `KaniWitness::proof` / `Establish::establish`
  methods in `amenable_kani` and 5 `VerusWitness::proof` methods in
  `amenable_std` that the census flagged — but that "24" was itself an
  undercount. cordial's tracing etiquette recorded every `impl Trait for Type`
  method under `{Trait}::{method}`, dropping the self type, so N types
  implementing one trait in a module collapsed to a single IR node: `--apply`
  fixed one and reported zero, leaving every sibling impl silently
  uninstrumented. Fixed in cordial `d2d8a10` (record trait-impl methods under
  `<Type as Trait>::method`, generics kept). The corrected sweep surfaced
  **704** real gaps — 427 `amenable_kani`, 260 `amenable_std`, 12
  `amenable_core`, 4 `amenable_gaap`, 1 `amenable_derive`; ~624 the
  proof-witness family (`KaniWitness`/`VerusWitness`/`Establish`/`KaniCompose`),
  the rest getters / `Display::fmt` / constructors on the state-marker types.
  `cordial quality --apply` added `#[cfg_attr(not(kani),
  tracing::instrument(...))]` to all 704 across 168 files (0 unresolved). No
  behavior change — the attr is stripped under `--cfg kani` /
  `--cfg verus_keep_ghost`. Verified: `check-all-package` clean on all 5
  crates; `verify-creusot` Proved (150 files); `verify-verus` 485 verified, 0
  errors; 5 canary Kani proofs across 5 touched files pass (full
  `just verify-kani` deferred to a dedicated run).
- **Derive patterns 0 → 4 → 0 (DONE).** `ProvenanceContainerOptions` /
  `WitnessContainerOptions` / `MemberOptions` / `ProofField` in `amenable_derive`:
  private structs that had to become `pub(crate)` / `pub(super)` for
  cross-submodule access during Phase 5, which lifted cordial's fully-private
  exemption and exposed their `pub` fields. Fixed in `f15dd31` — private
  fields, plus `#[derive(derive_getters::Getters)]` (its first use in the
  workspace; no new transitive deps). Construction/mutation stay in each
  struct's own module
  where private fields are still reachable; only the cross-module reads move to
  `.field()` calls. check-all-package + fixture-corpus tests confirm emitted
  tokens unchanged; verify-creusot Proved(150), verify-verus 485/0, 3 canary
  Kani proofs pass.

Baseline: the prior uncommitted split batch (compose / process_model /
utf8_model / char / fmt / option_result / verus_carrier / cli+assessment
commands) was committed in `12fbff0`, `2713edf`, `51ac997`, `3a10d24`,
`cfd7201` before this plan started.
