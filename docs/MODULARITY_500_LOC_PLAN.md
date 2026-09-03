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
- [ ] Phase 4 — `amenable_std` verus (0/8)
- [ ] Phase 5 — `amenable_derive` (0/3)

Baseline: the prior uncommitted split batch (compose / process_model /
utf8_model / char / fmt / option_result / verus_carrier / cli+assessment
commands) was committed in `12fbff0`, `2713edf`, `51ac997`, `3a10d24`,
`cfd7201` before this plan started.
