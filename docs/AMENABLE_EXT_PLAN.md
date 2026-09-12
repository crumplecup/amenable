# amenable_ext Plan

## Goal

Third-party crate support for the `amenable` trait family, as **one
crate**, `amenable_ext`, not one crate per target. Each target lives in
its own directory behind a same-named feature flag, default `[]`
(empty): `cargo add amenable_ext --features jiff` sweeps in exactly the
`jiff` dependency and its registrations, nothing else.

**First target: jiff, and jiff only.** This plan is scoped tightly to
getting jiff support built and covered end-to-end — skeleton, type
registrations, per-backend witnesses, and a real `cordial` coverage
report — before anything else is added. `chrono` / `chrono-tz` are the
deliberate next target once jiff is done (they let `amenable_time` work
with more than the `std::time` canary — see [Later targets](#later-targets)),
not a target for this plan to design now. `uuid` is deferred further
still (see the same section) — it has nothing to do with `amenable_time`
and doesn't need to ride along with the jiff work.

## Why one crate, not `amenable_jiff` / `amenable_chrono` / …

`amenable_std`'s own doc comment gives the reason, and it applies
identically here: a trait meant to be implemented directly on a foreign
type must live in a crate visible to both the trait and the type — Rust's
orphan rule leaves no other option. There is no "interface crate plus
downstream consumer" split to make; `RustStdType`/`RustStdStandard<T>`
and the per-type registrations already live together in one crate. An
`amenable_ext` per external library would just be `amenable_std`'s
justification restated N times over, at N times the Cargo/CI surface.

Feature-gating inside one crate gets the same "only pull in what you use"
property a crate split would, without the fragmentation.

## Prior art

**Not** `elicit_jiff`/`elicit_chrono`/`elicit_uuid` in `~/repos/elicitation`
— those are a different concern (the MCP *elicitation* protocol: prompting
a human/LLM for structured input via `Elicitation`/`elicit_tool`), one
crate per library for historical reasons unrelated to proof content.
`elicit_jiff::workflow` has its own small `Prop`/`Established`/`And`
typestate, but it doesn't implement `elicit_temporal`'s traits and isn't
the pattern to copy.

**The real precedent is `elicitation` itself:**
`crates/elicitation/src/verification/types/datetimes.rs` — refinement-typed
wrapper types (`DateTimeUtcAfter`, …) for `chrono`, `jiff`, and `time`,
each gated `#[cfg(feature = "chrono")]` / `"jiff"` / `"time"` **in one
crate**, with a `#[cfg(kani)]` split (trust the upstream library, verify
only the wrapper's own logic — a `PhantomData` stand-in under Kani so the
harness never touches the real foreign type). `elicitation/Cargo.toml`'s
`full` feature already lists ~30 target libraries this way. This is the
shape to follow, not invent — re-check `datetime_jiff.rs`/
`datetime_specs.rs` for the jiff type list before Phase 1 starts (Open
decision 3).

**The in-repo precedent is `amenable_std`:** `RustStdType` (per-type
static metadata: source crate/module, doc URL, semantics summary) +
`RustStdStandard<T>` (the generic `Evidence` newtype) +
`impl_rust_std_type!` / `impl_rust_std_type_generic{1,2,3}` /
`impl_rust_std_type_lifetime{0,1}[_concrete]` (one macro per
arity/lifetime shape a real std type needs) +
`register_rust_std_standard_evidence!` (the `EvidenceLink` submission,
needed because `#[derive(Standard)]`'s auto-registration can't cover a
generic type). `impl_rust_std_type!`'s `$source_crate` parameter is
**already generic** — it just happens to always be called with `"std"`
today. `Witness<V>` for `RustStdStandard<T>` lands in each backend crate
(`amenable_kani::rust_std::macros::{bridge_kani_witness!,
impl_kani_witness_trusted!}` and siblings) — the same place
`ExtStandard<T>`'s `Witness<V>` impls will land, for the same orphan-rule
reason `KaniVerifier` etc. are local there.

## Architecture

```text
amenable_ext/
  Cargo.toml        # jiff = { workspace = true, optional = true }, features.jiff = ["dep:jiff"]
  src/
    lib.rs           # mod + pub use only; ExtType trait + ExtStandard<T> live at crate root
    ext_type.rs       # ExtType trait (source_crate, source_module, doc_url, semantics_summary)
                       # + ExtStandard<T> generic Evidence newtype (parallel to RustStdStandard<T>)
    macros.rs          # impl_ext_type! family, mirroring amenable_std's arity/lifetime macros
                        # + register_ext_standard_evidence!
    jiff/               # #[cfg(feature = "jiff")]
      mod.rs
      timestamp.rs      # impl_ext_type!(jiff::Timestamp, "jiff", "jiff", url, summary), etc.
      civil.rs          # jiff::civil::{Date, Time, DateTime}
      span.rs           # jiff::Span
      zoned.rs          # jiff::Zoned
      tz.rs             # jiff::tz::TimeZone
      verus_witness.rs  # #[cfg(feature = "verus")] Witness<VerusVerifier> bridge for ExtStandard<T>
```

`amenable_kani` / `amenable_creusot` gain a `#[cfg(feature = "jiff")]`-gated
module (`ext::jiff` or similar — see Open decision 2), mirroring
`amenable_kani::rust_std::std_time` — `Witness<V>` / `ClassifiedWitness<V>`
for `ExtStandard<T>` per concrete `T`, `checked` where a real proof
exists, `trusted` where the type is opaque to the verifier and we cite
upstream's own guarantee instead (same `impl_kani_witness_trusted!`-style
split `elicitation`'s `#[cfg(kani)]` wrapper-logic-only proofs already
use). The Verus bridge lives inside `amenable_ext::jiff` itself
(`Witness<VerusVerifier>` for `ExtStandard<T>`), not in a separate
backend crate — `VerusVerifier` lives in `amenable_core` (Phase 8 of
`AMENABLE_TIME_PLAN.md`), so this mirrors `amenable_std::verus_witness`'s
own placement: the bridge lives with the type registrations.

### Relationship to `amenable_time`

**Deliberately decoupled at the Evidence/Witness layer, one-directional
at the Cargo layer.** `amenable_time` is the trait/contract interface
only — it does not depend on `amenable_std` or `amenable_ext` (that
dependency was reversed in Phase 8: `amenable_std` now depends on
`amenable_time`, not the other way around, so its `std::time` backend
can live in `amenable_std` alongside the type registrations it's built
from). `amenable_ext` mirrors that: it takes `amenable_time` as an
*optional* dependency, gated specifically by the `jiff`/`chrono` features
(not by every `amenable_ext` feature — `uuid`, when it eventually lands,
has nothing to do with time and must not pull it in).

`amenable_ext::jiff` gives `jiff` types `Evidence` + `Witness<V>` — a
general-purpose fact about those types, not specific to temporal
contracts. A real jiff temporal backend (a `amenable_ext::jiff::backend`
module, replacing/extending the `std::time` canary with actual
calendar-date, zone, and RFC 3339 parse/format coverage) would consume
`amenable_ext::jiff`'s wrapped types as its `Temporal*Props` native
carriers and live in `amenable_ext` itself (the same "backend lives with
the type registrations" placement `amenable_std::StdTimeBackend`
follows) — tracked as an explicit phase below (Phase 3), not deferred
indefinitely, since it's the actual payoff of doing jiff first.

## cordial coverage tooling — the concrete path

This is not new invention: `cordial`'s existing coverage-plugin
machinery (`~/repos/cordial/src/{plugins/amenable.rs,plugin/coverage.rs,
framework_std/}`) already has almost every piece a jiff coverage report
needs, and most of the remaining piece is already generic despite living
under a `framework_std`-named module. Read this section against that
source before starting Phase 2 — the function/constant names below are
real, not sketched.

**Already generic, reusable as-is:**

- `CoverageTargetKind::UpstreamDep` and `CoverageTarget::upstream_dep(crate_name)`
  (`plugin/coverage.rs`) — a first-class target kind for "an upstream
  dependency crate, not a workspace member." `CoverageTarget::upstream_dep("jiff")`
  is exactly the target row a jiff coverage plugin needs; no new kind to
  add.
- `build_shadow_dep_rustdoc(project_root, store, shadow_crate, upstream_crate, force)`
  (`cargo_rustdoc/shadow_dep.rs`) — builds and caches rustdoc JSON for an
  upstream crate. Its feature-resolution step
  (`resolve_shadow_dep_build_config`) tries
  `collect_member_dep_build_config(project_root, shadow_crate, upstream_crate)`
  **first** — a plain `cargo_metadata` read of `shadow_crate`'s own
  `Cargo.toml` dependency edge on `upstream_crate` — and only falls back
  to the `elicitation`-specific tracked-shadow-pair lookup if that fails.
  Once `amenable_ext`'s `Cargo.toml` has a real
  `jiff = { workspace = true, optional = true }` dependency, calling
  `build_shadow_dep_rustdoc(project_root, store, "amenable_ext", "jiff", force)`
  **just works** — no `elicitation`-style shadow-pair registration
  needed at all. (The "shadow" in the function name is a legacy of its
  first caller; the mechanism itself is crate-name-parameterized.)
- `load_std_inventory_from_json(json_path, crate_name)` (`framework_std/inventory.rs`)
  — takes a raw rustdoc JSON path and a crate name; nothing about it is
  std-specific despite the module it lives in. Feed it the shadow-dep
  cache path and `"jiff"` and it returns the same `Vec<StdInventoryItem>`
  shape `load_merged_std_inventory(sysroot)` returns for std/core/alloc.
- `build_amenable_std_report(source_crate, items, impl_crate, registry, skip_map, proof_chain_subjects, include_nightly)`
  (`framework_std/amenable.rs`) — every parameter is already a plain
  string/slice, not a std-specific constant baked into the function
  body. Calling it with `source_crate = "jiff"`, `impl_crate =
  "amenable_ext"` produces a jiff coverage report today, with one caveat
  below.
- `load_verifier_skip_map(store, patch_set)` (`framework_std/verifier_skip.rs`)
  — already keyed by an arbitrary `patch_set` string; a `"amenable_ext_jiff"`
  patch set is a new skip-map file, not new code.

**The one real gap:** `framework_std/registry.rs`'s
`RUST_STD_STANDARD_PREFIX = "amenable_std::rust_std::RustStdStandard<"`
and `PROOF_CHAIN_RUST_STD_PREFIX = "RustStdStandard<"` are hardcoded
literals `parse_rust_std_standard_inner` strips before matching an
evidence name to an inventory type path. `ExtStandard<T>` evidence names
(`amenable_ext::jiff::ExtStandard<jiff::Timestamp>`, or however the real
registration renders) need the same treatment. The fix is mechanical:
generalize `parse_rust_std_standard_inner` to take a prefix pair (or a
small slice of prefix pairs, if a report ever needs to recognize both
wrapper families at once) instead of the two module-level constants, and
add the `ExtStandard<`-shaped pair alongside the existing
`RustStdStandard<`-shaped one. `evidence_for_std_type` /
`witness_verifiers_for_std_type` / `std_type_has_proof_test` all route
through this one function, so fixing it there fixes all three call
sites.

**Wiring** (mirrors `plugins/amenable.rs` + `plugins/mod.rs` exactly):

1. `framework_ext/` module (or extend `framework_std/` with an
   `amenable_ext` sibling file set) — `AmenableExtOptions`,
   `assess_amenable_ext_coverage(target_crate: &str, ...)`, thin wrappers
   around `build_shadow_dep_rustdoc` + `load_std_inventory_from_json` +
   `build_amenable_std_report` (renaming is optional — the report/entry
   types are already generic enough to reuse directly rather than
   forking).
2. A jiff-specific `AMENABLE_EXT_IMPL_CRATE = "amenable_ext"` /
   `AMENABLE_EXT_PATCH_SET = "amenable_ext_jiff"` pair, parallel to
   `amenable_run.rs`'s existing `AMENABLE_IMPL_CRATE`/`AMENABLE_PATCH_SET`.
3. `AmenableExtTargetProvider` (mirrors `AmenableStdTargetProvider` in
   `plugins/amenable.rs`) returning `vec![CoverageTarget::upstream_dep("jiff")]`
   plus the workspace members discovered via `discover_crate_targets`
   (same as today).
4. `AmenableExtCoverage` plugin (mirrors `AmenableStdCoverage`), id
   `"amenable-ext-coverage"`, registered in `plugins/mod.rs`'s
   `coverage_plugins()` behind a new `amenable_ext` cargo feature in
   `cordial`'s own `Cargo.toml` — same shape as the existing
   `amenable_std` feature gate.
5. `etiquettes/framework_ext/{probe,assessor,reporter}.rs` (mirrors
   `etiquettes/framework_std/{probe,assessor,amenable_reporter}.rs`),
   scoped to `ir.crate_name() == "amenable_ext"` instead of
   `"amenable_std"`.

This is real work but a mechanical mirror, not new design — every file
above has a same-shaped sibling already in the codebase to copy from.

**Sequencing payoff:** none of this needs a single real `ExtStandard<T>`
registration to produce a *useful* report. `CoverageTarget::upstream_dep("jiff")`
and the shadow-dep rustdoc loader work the moment `amenable_ext`'s
`Cargo.toml` names `jiff` as a dependency — the very first report is
"100% missing," which is exactly the checklist Phase 1's registrations
work down to zero, the same way `amenable_std`'s own coverage report
already drives its own backlog. So Phase 0 (skeleton with the jiff dep
declared) unblocks Phase 2 (cordial tooling) immediately; Phase 1 (real
registrations) and Phase 2 can then proceed in parallel, with the
coverage report itself guiding Phase 1's remaining work.

This is `cordial` work, tracked and executed in `~/repos/cordial`, not
this repo — coordinate there when Phase 2 (below) starts, and read the
real source at the paths named above first (per
`feedback_check_action_versions_live`'s sibling instinct: verify against
current code, don't trust a description of it — cordial changes weekly).

## Phases

### Phase 0 — skeleton

- [ ] `crates/amenable_ext` (`Cargo.toml`, `lib.rs` — mod + pub use only).
- [ ] `ExtType` trait + `ExtStandard<T>` generic `Evidence` newtype
      (`ext_type.rs`).
- [ ] `impl_ext_type!` macro family, mirroring `amenable_std`'s
      arity/lifetime variants — start with the plain (no generics) case
      only; add the others when a real jiff type needs them.
- [ ] `register_ext_standard_evidence!`.
- [ ] Workspace `Cargo.toml`: add `amenable_ext` member + `jiff`
      workspace dependency (optional, `amenable_ext/Cargo.toml` only —
      this single edge is what makes the whole cordial mechanism above
      resolve with zero extra registration).
- [ ] `just check-all-package amenable_ext` clean.

### Phase 1 — jiff type registrations and witnesses

- [ ] `#[cfg(feature = "jiff")] mod jiff` — `ExtType`/`ExtStandard<T>`
      registrations for `jiff::{Timestamp, Zoned, Span, tz::TimeZone,
      civil::{Date, Time, DateTime}}` (the set `elicitation`'s own jiff
      coverage judged worth wrapping — recheck against `elicitation`'s
      `datetime_jiff.rs`/`datetime_specs.rs` for what "worth wrapping"
      meant there before assuming this list is complete; Open decision 3).
- [ ] `amenable_kani::rust_std` (or a new sibling module) gains a
      `#[cfg(feature = "jiff")] mod jiff` — `Witness<KaniVerifier>` /
      `ClassifiedWitness<KaniVerifier>` per type, `trusted` by default
      (jiff is opaque to Kani), `checked` only where a real harness adds
      value over trusting jiff's own correctness.
- [ ] Same for `amenable_creusot`, and a `Witness<VerusVerifier>` module
      inside `amenable_ext::jiff` itself (feature-gated) — see
      Architecture above for why it lives there, not in a backend crate.
- [ ] `amenable_kani`/`amenable_creusot`/`amenable`(facade) gain a
      `jiff` feature toggling `amenable_ext/jiff` (+ the crate's own
      jiff witness module).
- [ ] Tests: a `temporal_composition_test`-style compile assertion (every
      registered jiff `ExtStandard<T>` is `ClassifiedWitness<V>` on every
      linked backend).

### Phase 2 — cordial coverage tooling

- [ ] In `~/repos/cordial`: generalize `registry.rs`'s
      `parse_rust_std_standard_inner` to accept the `ExtStandard<`
      prefix pair alongside `RustStdStandard<` (the one real gap
      identified above).
- [ ] `framework_ext` module / `AmenableExtOptions` /
      `assess_amenable_ext_coverage` (thin wrappers, see wiring steps
      1-2 above).
- [ ] `AmenableExtTargetProvider` + `AmenableExtCoverage` plugin,
      registered in `coverage_plugins()` behind a new `amenable_ext`
      cargo feature (wiring steps 3-4).
- [ ] `etiquettes/framework_ext/*` (wiring step 5).
- [ ] An `amenable_ext_jiff` skip-map / patch-set (empty to start;
      entries added as real exceptions are found and reviewed — never
      added unilaterally, per standing policy).
- [ ] `just` recipe in this repo to run the new coverage command,
      parallel to the existing `just cordial-gate`.
- [ ] Run it against the Phase 0 skeleton (before Phase 1's
      registrations land) to confirm the "100% missing" baseline report
      renders correctly, then again after Phase 1 to confirm it tracks
      real coverage.

### Phase 3 — a real jiff temporal backend (the payoff)

- [ ] `amenable_ext::jiff::backend` — a `TemporalDurationProps` /
      `TemporalInstantProps` / `TemporalReporter` / `Exchange` impl set
      against `amenable_ext::jiff`'s wrapped types, replacing/extending
      the `std::time` canary (`amenable_std::std_time_backend`) with
      real calendar-date, zone, and RFC 3339 parse/format coverage —
      jiff has an ISO 8601 parser `std::time` lacks, so the three
      `unsupported_parse` `Exchange` edges the canary reports as
      unsupported become real here.
- [ ] `temporal-coverage`-style test / capability declaration for the
      new backend, mirroring `amenable_std/tests/std_backend_test.rs`.

## Later targets

**chrono / chrono-tz**, once jiff is done: the deliberate next target,
precisely because it lets `amenable_time` work with a second real
calendar library (and `chrono-tz`'s IANA database gives a genuine
`supports_named_zone_round_trip` capability the `std::time` canary and a
bare `jiff` backend both report `false` for). Same shape as jiff
end-to-end (Phases 0-3 repeated with `chrono`/`chrono-tz` in place of
`jiff`); the cordial tooling from Phase 2 above should need no further
generalization — a second `AMENABLE_EXT_PATCH_SET` value and a second
`CoverageTarget::upstream_dep("chrono")` row, not new mechanism.

**uuid**, deferred further than chrono: already a vetted workspace
dependency (used by `amenable_kani`/`amenable_gaap`), a small, mostly-
opaque type surface (`Uuid`, `Builder`, `Timestamp`, the version/variant
enums), and unrelated to `amenable_time` — a good "second, unrelated
target" exercise for the `amenable_ext` architecture once it's proven on
two temporal libraries, not a reason to design it now.

## Open decisions

1. **`ExtType` metadata shape** — identical to `RustStdType` (source
   crate/module, doc URL, semantics summary), or does a third-party crate
   need an extra field (e.g. the target crate's own version pin, since
   "jiff 0.2" and "jiff 0.1" may have different guarantees)? Lean toward
   identical-to-`RustStdType` unless a real need shows up.
2. **Where the per-backend `jiff` witness modules live** — a new
   top-level `mod` in each backend crate (`amenable_kani::ext_jiff`?) or
   nested under the existing `rust_std` module despite jiff not being
   std? Lean toward a new sibling module (`amenable_kani::ext::jiff`) —
   `rust_std` should stay std-only by name.
3. **First jiff type set** — the plan lists a plausible starting set
   above; worth a quick pass against `elicitation`'s own jiff coverage
   (`datetime_jiff.rs`, `datetime_specs.rs`, `verification/types/
   datetimes.rs`) before Phase 1 starts, to avoid missing something that
   prior art already judged important (or wrapping something it judged
   not worth it).
