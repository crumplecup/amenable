# amenable_ext Plan

## Goal

Third-party crate support for the `amenable` trait family — `jiff`,
`uuid`, `chrono`, … — as **one crate**, `amenable_ext`, not one crate per
target. Each target lives in its own directory behind a same-named
feature flag, default `[]` (empty): `cargo add amenable_ext --features
jiff` sweeps in exactly the `jiff` dependency and its registrations,
nothing else.

First target: **jiff**. Second: **uuid** (already a vetted workspace
dependency, used by `amenable_kani`/`amenable_gaap`).

## Why one crate, not `amenable_jiff` / `amenable_uuid` / …

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
shape to follow, not invent.

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
impl_kani_witness_trusted!}` and siblings) — the same place `amenable_ext`
target types' `Witness<V>` impls will land, for the same orphan-rule
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
    uuid/               # #[cfg(feature = "uuid")]  (second target)
      mod.rs
      uuid_type.rs      # uuid::Uuid, uuid::Builder, ...
```

`amenable_kani` / `amenable_creusot` / `amenable_time::verus_witness` gain
a `#[cfg(feature = "...")]`-gated module per target, mirroring
`amenable_kani::rust_std::std_time` — `Witness<V>` / `ClassifiedWitness<V>`
for `ExtStandard<T>` per concrete `T`, `checked` where a real proof exists,
`trusted` where the type is opaque to the verifier and we cite upstream's
own guarantee instead (same `impl_kani_witness_trusted!`-style split
`elicitation`'s `#[cfg(kani)]` wrapper-logic-only proofs already use).

### Relationship to `amenable_time`

**Deliberately decoupled, and one-directional.** `amenable_time` is the
trait/contract interface only — it does not depend on `amenable_std` or
`amenable_ext` (that dependency was reversed: `amenable_std` now depends
on `amenable_time`, not the other way around, so its `std::time` backend
can live in `amenable_std` alongside the type registrations it's built
from). `amenable_ext` mirrors that: it takes `amenable_time` as an
*optional* dependency, gated specifically by the `jiff`/`chrono` features
(not by every `amenable_ext` feature — `uuid`, `url`, etc. have nothing to
do with time and must not pull it in).

`amenable_ext::jiff` gives `jiff` types `Evidence` + `Witness<V>` — a
general-purpose fact about those types, not specific to temporal
contracts. A real jiff temporal backend (a `amenable_ext::jiff::backend`
module, replacing/extending the `std::time` canary with actual
calendar-date, zone, and RFC 3339 parse/format coverage) would consume
`amenable_ext::jiff`'s wrapped types as its `Temporal*Props` native
carriers and live in `amenable_ext` itself (the same "backend lives with
the type registrations" placement `amenable_std::StdTimeBackend` follows)
— but that's separate, later work, tracked in `AMENABLE_TIME_PLAN.md`'s
optional follow-ons, not this plan. `uuid`/`url` support has nothing to
do with time at all; scoping `amenable_ext` to `amenable_time`'s needs
would be the wrong cut.

## cordial coverage tooling

`cordial`'s `src/framework_std/` (~1800 LOC) already does almost exactly
this job for `amenable_std`: `load_merged_std_inventory(sysroot)` enumerates
every std type from the sysroot's rustdoc JSON, cross-references against
the `amenable dump-registry` dump (`EvidenceLink`/`ProofRecord`/
`witness_exports`) via `build_amenable_std_report`, applies a skip-map of
documented exceptions (`load_verifier_skip_map`), and renders
checklist/gap CSVs + a summary MD — wired to `cordial coverage`.

The inventory source is the only sysroot-specific part, and cordial
already builds rustdoc JSON for an **upstream dependency crate** elsewhere
(`src/cargo_rustdoc/shadow_dep.rs`'s `build_shadow_dep_rustdoc` — a shadow
-member trick, not sysroot-specific). So the extension is real but
bounded:

1. A `load_merged_ext_inventory(crate_name)` alongside
   `load_merged_std_inventory(sysroot)`, sourced from
   `build_shadow_dep_rustdoc` instead of the sysroot loader.
2. `AMENABLE_IMPL_CRATE`/`AMENABLE_PATCH_SET`-shaped constants
   parameterized per target (`"jiff"` → `amenable_ext`'s `jiff` module +
   `amenable_kani`'s `jiff` module, etc.) instead of the single
   `amenable_std` hardcode.
3. One skip-map / patch-set per target crate (`jiff`, `uuid`, …), same
   shape as the existing `AMENABLE_PATCH_SET` skip-map.
4. `cordial coverage --target jiff` (or similar) alongside the existing
   std coverage command.

This is `cordial` work, tracked and executed in `~/repos/cordial`, not
this repo — coordinate there when Phase 2 (below) starts.

## Phases

### Phase 0 — skeleton

- [ ] `crates/amenable_ext` (`Cargo.toml`, `lib.rs` — mod + pub use only).
- [ ] `ExtType` trait + `ExtStandard<T>` generic `Evidence` newtype
      (`ext_type.rs`).
- [ ] `impl_ext_type!` macro family, mirroring `amenable_std`'s
      arity/lifetime variants — start with the plain (no generics) case
      only; add the others when a real jiff/uuid type needs them.
- [ ] `register_ext_standard_evidence!`.
- [ ] Workspace `Cargo.toml`: add `amenable_ext` member + `jiff`
      workspace dependency (optional).
- [ ] `just check-all-package amenable_ext` clean.

### Phase 1 — jiff, the first target

- [ ] `#[cfg(feature = "jiff")] mod jiff` — `ExtType`/`ExtStandard<T>`
      registrations for `jiff::{Timestamp, Zoned, Span, tz::TimeZone,
      civil::{Date, Time, DateTime}}` (the set `elicit_jiff`/
      `elicitation`'s own jiff coverage judged worth wrapping — recheck
      against `elicitation`'s `datetime_jiff.rs`/`datetime_specs.rs` for
      what "worth wrapping" meant there before assuming this list is
      complete).
- [ ] `amenable_kani::rust_std` (or a new sibling module) gains a
      `#[cfg(feature = "jiff")] mod jiff` — `Witness<KaniVerifier>` /
      `ClassifiedWitness<KaniVerifier>` per type, `trusted` by default
      (jiff is opaque to Kani), `checked` only where a real harness adds
      value over trusting jiff's own correctness.
- [ ] Same for `amenable_creusot` and for a `Witness<VerusVerifier>`
      module inside `amenable_ext` itself (feature-gated) — `VerusVerifier`
      now lives in `amenable_core`, so this mirrors
      `amenable_std::verus_witness`'s own placement (the bridge lives with
      the type registrations, not in `amenable_time`, which knows nothing
      about jiff types).
- [ ] `amenable_kani`/`amenable_creusot`/`amenable`(facade) gain a
      `jiff` feature toggling `amenable_ext/jiff` (+ the crate's own
      jiff witness module).
- [ ] Tests: a `temporal_composition_test`-style compile assertion (every
      registered jiff `ExtStandard<T>` is `ClassifiedWitness<V>` on every
      linked backend) + a coverage smoke test once Phase 2 lands.

### Phase 2 — cordial coverage tooling

- [ ] In `~/repos/cordial`: generalize `framework_std`'s inventory source
      per the design above; wire `AMENABLE_EXT_*` constants for `jiff`.
- [ ] A `jiff` skip-map / patch-set (empty to start; entries added as
      real exceptions are found and reviewed — never added unilaterally,
      per standing policy).
- [ ] `cordial coverage` (or a new subcommand) reports `jiff` type
      coverage the same way it reports std coverage today.
- [ ] `just` recipe in this repo to run it.

### Phase 3 — uuid, the second target

- [ ] `#[cfg(feature = "uuid")] mod uuid` in `amenable_ext` +
      per-backend witness modules, same shape as Phase 1. Shorter: `uuid`
      is a small, mostly-opaque type surface (`Uuid`, `Builder`,
      `Timestamp`, the version/variant enums) compared to jiff's.
- [ ] Extend the cordial tooling's target list, not its mechanism —
      Phase 2's generalization should make this a config addition, not
      new code.

## Open decisions

1. **`ExtType` metadata shape** — identical to `RustStdType` (source
   crate/module, doc URL, semantics summary), or does a third-party crate
   need an extra field (e.g. the target crate's own version pin, since
   "jiff 0.2" and "jiff 0.1" may have different guarantees)? Lean toward
   identical-to-`RustStdType` unless a real need shows up.
2. **Where the per-backend `jiff`/`uuid` witness modules live** — a new
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
