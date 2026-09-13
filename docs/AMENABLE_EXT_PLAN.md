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
      timestamp.rs      # impl_ext_type!(jiff::Timestamp, "jiff contributors", "jiff", "jiff", url, summary)
      zoned.rs          # jiff::Zoned
      civil.rs          # jiff::civil::DateTime
      verus_witness.rs  # #[cfg(feature = "verus")] Witness<VerusVerifier> bridge for ExtStandard<T>
```

Only `Timestamp`/`Zoned`/`civil::DateTime` so far — the verified-against-
`elicitation` set (Phase 1's own checklist entry below). `Span`/
`tz::TimeZone`/`civil::{Date, Time}` are not first-class registrations
here; they show up only as supporting types other methods use, per
that same recheck, and get added if/when Phase 3's backend actually
needs one as a native carrier.

`amenable_kani` / `amenable_creusot` gain a `#[cfg(feature = "jiff")]`-gated
`ext::jiff` module (Open decision 2, resolved — a new sibling module,
not nested under `rust_std`), mirroring `impl_kani_witness_trusted!`'s
own split: `checked` where a real proof exists, `trusted` where the type
is opaque to the verifier and we cite upstream's own guarantee instead
(the same split `elicitation`'s `#[cfg(kani)]` wrapper-logic-only proofs
already use). **`Witness<V>` only on Kani/Creusot, not
`ClassifiedWitness<V>`** — checked against the real precedent rather
than assumed: `RustStdStandard<T>`'s own trusted registrations don't
implement `ClassifiedWitness` either (only `amenable_time`'s contract
types do), so `ExtStandard<T>` follows the same real pattern it's
modeled on. The Verus bridge lives inside `amenable_ext::jiff` itself
(`Witness<VerusVerifier>` *and* `ClassifiedWitness<VerusVerifier>` for
`ExtStandard<T>` — Verus's own asymmetry with Kani/Creusot here is real,
matching `RustStdStandard<T>`'s own Verus registrations in
`amenable_std::verus_derive_canary::leaves`), not in a separate backend
crate — `VerusVerifier` lives in `amenable_core` (Phase 8 of
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
(`amenable_ext::ExtStandard<jiff::Timestamp>` — `ExtStandard` lives at
the crate root, not nested under a `jiff` submodule, so the prefix is
shorter than `RustStdStandard`'s own `amenable_std::rust_std::` nesting)
need the same treatment. The fix is mechanical:
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

- [x] `crates/amenable_ext` (`Cargo.toml`, `lib.rs` — mod + pub use only).
- [x] `ExtType` trait + `ExtStandard<T>` generic `Evidence` newtype
      (`ext_type.rs`); `ExtLanguageProvenance`/`ExtProvenance` (parallel
      to `RustLanguageProvenance`/`RustStdProvenance`, but with an
      explicit `authority` parameter per registration — see that
      module's own doc comment for why there's no crate-wide default).
- [x] `impl_ext_type!` macro family — the plain (no generics) case only,
      as planned; `register_ext_standard_evidence!`. Both gated behind
      `#[cfg(feature = "jiff")]` (extend to `any(feature = "jiff",
      feature = "chrono", ...)` once a second target lands) — with zero
      target modules yet, an ungated shared macro would be genuinely
      dead code.
- [x] Workspace `Cargo.toml`: `amenable_ext` member + `jiff` workspace
      dependency added; `amenable_ext/Cargo.toml`'s own `jiff = {
      workspace = true, optional = true }` + `jiff = ["dep:jiff",
      "dep:amenable_time"]` feature is the single edge the whole cordial
      mechanism above resolves against.
- [x] One real registration landed alongside the skeleton rather than
      after it: `jiff::Timestamp` (`src/jiff/timestamp.rs`) — needed to
      keep Phase 0 itself free of dead code (an unused shared macro with
      no consumer is exactly the kind of thing this codebase's own
      dead-code discipline flags), and it doubles as the first real
      exercise of the whole mechanism end to end. `tests/jiff_test.rs`
      (3 tests: derived provenance report, `Standard` impl, `EvidenceLink`
      registration) — note `std::any::type_name::<jiff::Timestamp>()`
      resolves to `jiff::timestamp::Timestamp` (jiff's internal defining
      module), not the public re-export path, the same property
      `RustStdType::provenance()` already lives with for std types.
- [x] `just check-all-package amenable_ext` clean (fmt, clippy
      `--all-features -D warnings`, test); `cargo test -p amenable_ext
      --features jiff` separately (the recipe's own `cargo test` step
      doesn't pass `--all-features`, matching every other crate's
      recipe use); full workspace `check`/`clippy --all-features
      --all-targets`/`fmt --check` all clean with the new member added.

### Phase 1 — jiff type registrations and witnesses

- [x] **Open decision 3 resolved (2026-09-13):** read `elicitation`'s
      real jiff coverage (`crates/elicitation/src/datetime_jiff.rs` +
      `verification/types/datetimes.rs` + `elicitation_kani/src/
      datetimes_jiff.rs`) rather than assuming the Architecture
      section's speculative list above. Finding: elicitation's own
      judgment of "worth wrapping" is narrower than that list —
      `Timestamp`, `Zoned`, and `civil::DateTime` get real `Elicitation`
      impls (the MCP-input-worthy carriers); only `Timestamp` gets a
      verification-focused refinement wrapper (`TimestampAfter`/
      `TimestampBefore`, `#[cfg(kani)]` trust-jiff/verify-wrapper-only
      split). `Span`/`tz::TimeZone`/`civil::{Date, Time}` appear only as
      supporting types inside other types' methods, never as a
      first-class wrapped carrier. **Registering exactly that verified
      set for now — `Timestamp` (done), `Zoned`, `civil::DateTime` —
      not the wider speculative list; `Span`/`TimeZone`/`Date`/`Time`
      get registered later if/when Phase 3's real backend actually
      needs one as a native carrier type**, per this crate's own
      dead-code discipline (a registration with no consumer is exactly
      what Phase 0 already flagged as a real problem, not a
      hypothetical one).
- [x] `Zoned` (`src/jiff/zoned.rs`) and `civil::DateTime`
      (`src/jiff/civil.rs`) registered, doc strings verified against
      jiff 0.2.35's own vendored source. `register_ext_standard_evidence!`
      extended for both.
- [x] **Open decision 2 resolved:** a new sibling module in each backend
      crate — `amenable_kani::ext::jiff`, `amenable_creusot::ext::jiff`
      — not nested under `rust_std`/`rust_std_witness`. `rust_std`
      stays std-only by name.
- [x] `amenable_kani::ext::jiff` — `Witness<KaniVerifier>` per type,
      `trusted` by default (jiff is opaque to Kani), via a new
      `impl_kani_witness_trusted_ext!` macro (`ext/macros.rs`, reusing
      the already-generic `bridge_kani_witness!` from `rust_std` —
      `pub(crate) use`d at `rust_std`'s own module boundary, not
      duplicated). **Not** `ClassifiedWitness<KaniVerifier>` — checked
      against the real precedent first: `RustStdStandard<T>`'s own
      trusted registrations (`impl_kani_witness_trusted!`) don't
      implement it either (only `amenable_time`'s contract types do);
      this plan's earlier text calling for it was aspirational, not
      matching the established pattern, so dropped to stay consistent
      with `RustStdStandard<T>`'s real treatment.
- [x] Same for `amenable_creusot::ext::jiff` (`bridge_creusot_witness!`/
      `impl_creusot_witness_trusted_ext!` defined locally in that one
      file, matching every other file in `rust_std_witness` — none of
      which share it from a common module either); module gated
      `#[cfg(not(creusot))]` in `lib.rs`, matching `rust_std_witness`'s
      own gate (both need `String`/`Vec`-returning closures and
      `inventory::submit!`, the exact shape that crate's own doc
      comment identifies as a real `creusot-rustc` ICE trigger when
      local to the crate being translated).
- [x] `Witness<VerusVerifier>` module inside `amenable_ext::jiff` itself
      (`verus_witness.rs`, `#[cfg(feature = "verus")]`) — implements
      `Witness<VerusVerifier>` directly on `ExtStandard<T>` (no
      intermediate `VerusWitness` trait, unlike `amenable_std`'s: there
      is no real per-type Verus harness to render a call-shape for
      yet), plus `ClassifiedWitness<VerusVerifier>` (this one *does*
      get it — `amenable_std`'s own canary/backend-style trusted
      impls set `support()` to `trusted_leaf()` and implement
      `ClassifiedWitness` alongside it; the asymmetry with Kani/Creusot
      above is real, not an oversight, since `RustStdStandard<T>`'s own
      Verus registrations follow the identical shape in
      `amenable_std::verus_derive_canary::leaves`).
- [x] `amenable_kani`/`amenable_creusot`/`amenable`(facade) gain a
      `jiff` feature. Facade: `amenable_kani/jiff` (unconditional dep)
      turned on directly; `amenable_creusot?/jiff` via Cargo's weak-dep
      `?` syntax so enabling `jiff` alone never force-enables the
      otherwise-optional `creusot` feature (verified via `cargo tree -e
      features`: `amenable_creusot` is absent from the resolved graph
      with `--features jiff` alone, and gains its own `jiff` feature
      only with `--features jiff,creusot` together); `amenable_ext`
      re-exported wholesale (`pub use amenable_ext;`), mirroring
      `amenable_time`'s own precedent.
- [x] Tests: `amenable_kani/tests/ext_jiff_test.rs` (4),
      `amenable_creusot/tests/ext_jiff_test.rs` (4),
      `amenable_ext/tests/jiff_verus_witness_test.rs` (3, using the same
      `assert_classified::<T>()` pattern the `temporal_composition_test`s
      use) — proof-value equality against `ExtType::provenance()` plus
      a real `ProofRecord`/`ClassifiedWitness` check per type, on all
      three backends.

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

1. **`ExtType` metadata shape — settled, identical to `RustStdType`**
   (source crate/module, doc URL, semantics summary; `ext_type.rs`).
   No extra field turned out to be needed for the three jiff types
   registered so far; revisit only if a real need shows up (e.g. a
   target crate's own version pin).
2. **Where the per-backend `jiff` witness modules live — settled,**
   `amenable_kani::ext::jiff` / `amenable_creusot::ext::jiff`, new
   sibling modules, not nested under `rust_std`/`rust_std_witness`.
3. **First jiff type set — settled** against `elicitation`'s real
   coverage (`datetime_jiff.rs`, `verification/types/datetimes.rs`,
   `elicitation_kani/src/datetimes_jiff.rs`): `Timestamp`, `Zoned`,
   `civil::DateTime` — narrower than this plan's own earlier
   speculative list. `Span`/`tz::TimeZone`/`civil::{Date, Time}` get
   registered later if/when Phase 3's real backend needs one as a
   native carrier, not preemptively.
