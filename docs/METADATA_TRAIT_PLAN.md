# Metadata Trait Family Plan

**Status:** 🚧 In progress. ✅ **Step 0 landed** (2026-09-07): the core
types (`MetadataValue` / `ErasedEntry` / `OwnedEntry` / `Metadata` /
`MetadataRecord` / `MetadataReport`) are in `amenable_core/src/metadata.rs`,
not wired to anything. `amenable_core` compiles clean (`clippy -D
warnings`, `fmt`, all tests); Verus `485 verified, 0 errors` (module
correctly excluded from the `#[path]` include); Creusot `--cfg creusot`
graph-wide + `cargo creusot --only=coma` translate clean; a sample Kani
harness passes with `amenable_core` compiled under kani codegen. The
`Arc<dyn>` risk did not materialize for the compile paths; the real
Creusot *translation* test comes in Step 5 when `amenable_creusot` adopts
`OwnedEntry` (dependency items are not translated). `Entry` + the
`Entry`→`ErasedEntry` blanket deferred to Step 1 (a coherence question
with `impl ErasedEntry for OwnedEntry`). Steps 1–8 remain.

The phased plan at the bottom is the implementation order. Supersedes the
two open `AMENABLE_PLAN.md` Phase 2 bullets on `Provenance` ("Redesign
`Provenance` from the current iter-only capability…") and the
`#[derive(Provenance)]` line, and reframes them around a dedicated
`Metadata` trait rather than growing `Provenance` itself.

Design review rounds 1–4 resolved (see "Open questions"): `Send + Sync`
on values; **`Provenance: Metadata` supertrait**, both kept as distinct
markers for distinct bounds — a provenance type is its own metadata
record, `#[derive(Provenance)]` reuses the `#[derive(Metadata)]` codegen
and appends `impl Provenance for T {}` (today `Provenance` adds only
`certification()`); `Metadata` has **no associated types** and returns
**owned** values (`snapshot(&self) -> Vec<OwnedEntry>`), which is what
lets ZSTs implement it; `report` lives on `Metadata`
(`MetadataReport` / `OwnedMetadataReport`), inherited by the rest;
`get`/`snapshot` perf deferred until there's something to benchmark; the
per-schema `match` enum is cut; the scalar `"value"` sentinel is
eliminated by an `#[entry]` (leaf) / `#[entry(nested)]` (sub-record)
attribute split; `OwnedEntry` holds `Arc<dyn MetadataValue>` (or the
`MetadataValueKind` enum if Step 0's Creusot spike fails).

## Why this exists

`amenable_core` has no trait interface for metadata. It has one concrete
struct, `MetadataEntry` (`{ key: String, value: String }`), and that
struct is the entire vocabulary: `Provenance::metadata()` yields
`MetadataEntry`, `ProvenanceCertificate` stores `Vec<MetadataEntry>`,
`ProvenanceReport` renders `MetadataEntry`, and all 37 `impl Provenance`
sites hand-build `MetadataEntry::new("k", v.to_string())` (~240 call
sites). Every value is stringified at the boundary.

Two things are wrong with that:

1. **`MetadataEntry` bears design weight it was never meant to.** It is a
   rendering leaf — the frozen key/value pair you put on a certificate —
   being used as the metadata *interface*. There is nothing to implement,
   no way for a caller to say "my metadata is shaped like *this*," and no
   way to get a typed value back out.

2. **Stringify-at-the-boundary loses the data.** A `jiff::Timestamp`
   field becomes `"2026-09-07T00:00:00Z"`; a `WidgetKind` enum becomes
   `"Servo"`. A caller that wants to *check a spec before operating on a
   widget* — "is `motor.max_rpm` under my rated ceiling?" — gets a string
   it has to re-parse, with no type safety and no guarantee the format
   round-trips.

The target use case, stated concretely: a complex widget composes
metadata from five sub-components (motor, battery, radio, chassis,
firmware), each with its own metadata schema. A programmer holds the
widget and wants to query its assembled specs — by key, getting
**structured typed values back** — as a pre-flight check before running
operations. `MetadataEntry` cannot express that. A trait family can.

## Design

### The shape of the problem

A single trait cannot be both:

- the typed contract an entry author writes (`type Value`, `fn value(&self)
  -> &Self::Value`), **and**
- the element type of a heterogeneous collection (`Vec<dyn Entry>` — which
  requires pinning `Value`, the exact thing that varies).

The resolution is two traits joined by a blanket impl — the same
`serde::Serialize` / `erased_serde::Serialize` split. The typed trait is
for authoring and generic (non-`dyn`) use; its object-safe projection is
what collections are made of.

### `MetadataValue` — the value carrier

```rust
/// A value carried by a metadata entry. Blanket-impl'd for every owned
/// `Display + Debug + 'static` (+ `Send + Sync`) type — never hand-written.
///
/// The accessors exist instead of `Display` / `Any` supertraits so that
/// `&dyn MetadataValue` projects without trait upcasting, which is not
/// stable on every verifier backend's pinned toolchain.
pub trait MetadataValue: Debug + Send + Sync + 'static {
    fn as_display(&self) -> &dyn Display;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Display + Debug + Send + Sync + 'static> MetadataValue for T {
    fn as_display(&self) -> &dyn Display { self }
    fn as_any(&self) -> &dyn Any { self }
}
```

`Any: 'static` is satisfied automatically by the workspace's "owned
strings in struct fields" rule — every `#[entry]` field type is already
`'static`. `Send + Sync` is required (resolved — plain owned data).

### `Entry` — the typed authoring contract

```rust
/// A typed metadata fact. Used as `impl Entry` / a generic bound. Never
/// used as `dyn Entry` — you would have to pin `Value`, and `Value` is
/// what varies across a schema's fields.
pub trait Entry {
    /// Key type. `str` before nesting, `String` after re-keying.
    type Key: ?Sized + AsRef<str>;
    /// Value type. `?Sized` so `str` and `dyn MetadataValue` qualify.
    type Value: ?Sized;

    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
}
```

Implemented by:

- the small reusable named entries a provenance vocabulary is built from
  (`Authority(String)`, `SourceDocument(Url)`, `Clause(String)`, …)
- `MetadataEntry` (`type Key = str; type Value = str`)
- `OwnedEntry` (`type Value = dyn MetadataValue`)

### `ErasedEntry` — the object-safe projection

```rust
/// Object-safe view of any entry. The element type every `Metadata` deals
/// in. Blanket-impl'd from `Entry`; also hand-impl'd by `OwnedEntry`.
pub trait ErasedEntry: Debug {
    /// Fully-qualified key, e.g. `"motor.max_rpm"` after nesting.
    fn key(&self) -> &str;

    /// Rendering path — always available.
    fn value(&self) -> &dyn Display;

    /// Structured path — `e.value_any().downcast_ref::<Rpm>()` returns the
    /// real value reference, not a stringification. Survives nesting.
    fn value_any(&self) -> &dyn Any;
}

impl<E> ErasedEntry for E
where
    E: Entry + Debug,
    E::Value: MetadataValue + Sized,
{
    fn key(&self) -> &str { self.key().as_ref() }
    fn value(&self) -> &dyn Display { Entry::value(self).as_display() }
    fn value_any(&self) -> &dyn Any { Entry::value(self).as_any() }
}
```

Object-safe: no associated types, no generics, no `Self` by value, only a
`Debug` supertrait.

### `Metadata` — the folder

`Metadata`'s `&self` methods return **owned** values. That is the load-
bearing choice: it lets a plain struct (`RustLanguageProvenance`) *and* a
ZST (`RustStdStandard<T>`) both implement `Metadata` directly — neither
stores an entry collection, both build one on call. A borrowed
`get(&self) -> Option<&Entry>` would force stored entries and rule the
ZSTs out (that was the round-2 assumption that forced `Provenance` into a
`type Metadata` associated type; owned returns dissolve it).

```rust
pub trait Metadata {
    /// Every entry, fully-qualified keys, deterministic source order.
    /// Owned, built fresh per call — not a hot path.
    fn snapshot(&self) -> Vec<OwnedEntry>;

    /// Point lookup. Default scans a fresh `snapshot`; keys are already
    /// fully qualified so this is *correct* at any nesting depth. A
    /// custom impl (map-backed, or key-splitting) can override it — not
    /// something the derive bothers with until there's something to
    /// benchmark.
    fn get(&self, key: &str) -> Option<OwnedEntry> {
        self.snapshot().into_iter().find(|e| e.key() == key)
    }

    /// Typed lookup — clones the one matching value out.
    fn get_as<T: Any + Clone>(&self, key: &str) -> Option<T> {
        self.get(key).and_then(|e| e.value_any().downcast_ref::<T>().cloned())
    }

    fn keys(&self) -> Vec<String> {
        self.snapshot().iter().map(|e| e.key().to_owned()).collect()
    }
    fn len(&self) -> usize { self.snapshot().len() }
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn contains_key(&self, key: &str) -> bool { self.get(key).is_some() }

    /// Render own facts as an audit surface. Reporting is `Metadata`'s
    /// job to define; `Provenance` / `Standard` reach it by inheritance
    /// or through `provenance()`. `MetadataReport` borrows; the owned
    /// form is `OwnedMetadataReport<Self>`.
    fn report(&self) -> MetadataReport<'_, Self> where Self: Sized {
        MetadataReport::new(self)
    }
}
```

No `type Entries` iterator and no `into_entries(self)` — `snapshot(&self)
-> Vec<OwnedEntry>` is the single primitive; everything else is built
from it. "Back it with a `HashMap`" is still available: a custom impl
overrides `get` (and builds `snapshot` from `self.map.values()`).

`MetadataReport` / `OwnedMetadataReport` are the renamed `ProvenanceReport`
/ `OwnedProvenanceReport` — same `Display` behavior (one `key: value`
line per entry, `(no metadata)` when empty), now generic over `M:
Metadata` instead of `P: Provenance`.

### `OwnedEntry` — the universal carrier

```rust
/// The one entry type every `Metadata` yields. Owns its key; holds its
/// value behind `Arc` so the entry is `Clone` (needed by `snapshot` and
/// by hand-built / merged records) while the concrete value type stays
/// reachable via `value_any()`.
#[derive(Debug, Clone)]
pub struct OwnedEntry {
    key: String,
    value: Arc<dyn MetadataValue>,
}

impl OwnedEntry {
    pub fn new(key: impl Into<String>, value: impl MetadataValue) -> Self { /* … */ }
    /// Namespace for nesting: `"max_rpm"` → `"motor.max_rpm"`. Applied to
    /// every entry of an `#[entry(nested)]` field. `Arc` untouched.
    pub(crate) fn prefixed(self, parent: &str) -> Self { /* … */ }
}

impl Entry for OwnedEntry {
    type Key = str;
    type Value = dyn MetadataValue;
    fn key(&self) -> &str { &self.key }
    fn value(&self) -> &dyn MetadataValue { &*self.value }
}
// ErasedEntry for OwnedEntry: hand-written (blanket needs Value: Sized).
```

### `MetadataRecord` — hand-authored / merged records only

Not the output of the derive (that's `impl Metadata for YourStruct`
directly). `MetadataRecord` is for code that assembles metadata by hand
or merges several sources:

```rust
#[derive(Debug, Clone, Default)]
pub struct MetadataRecord { entries: Vec<OwnedEntry> }

impl MetadataRecord {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, e: OwnedEntry) { self.entries.push(e); }
}
impl FromIterator<OwnedEntry> for MetadataRecord { /* … */ }
impl Extend<OwnedEntry> for MetadataRecord { /* … */ }

impl Metadata for MetadataRecord {
    fn snapshot(&self) -> Vec<OwnedEntry> { self.entries.clone() }   // Arc clones
    fn get(&self, key: &str) -> Option<OwnedEntry> {
        self.entries.iter().find(|e| e.key() == key).cloned()
    }
    fn len(&self) -> usize { self.entries.len() }
}
```

### `MetadataEntry` — reduced role

`MetadataEntry` stays exactly as it is on the wire (`String` / `String`,
`Clone + PartialEq + Eq + PartialOrd + Ord + Hash`, hand-written accessors
for the `verus` `#[path]`-include reason documented on it today). Its job
shrinks from "the metadata vocabulary" to "the frozen rendered snapshot":
the thing a `ProvenanceCertificate` stores, where you want cheap
comparison and hashing and never need the typed value back.

New impls:

```rust
impl Entry for MetadataEntry { type Key = str; type Value = str; /* … */ }

impl<E: ErasedEntry> From<&E> for MetadataEntry {
    fn from(e: &E) -> Self {
        MetadataEntry::new(e.key(), e.value().to_string())
    }
}
```

### `Provenance` — a `Metadata` that is a trust basis

`Provenance` and `Metadata` overlap on the surface (both answer key/value
queries) but are different in kind. `Metadata` is a mechanism — a flat
namespace of typed facts, no claim about meaning. `Provenance` is a
constitutional role, the first link in `Provenance → Standard → Evidence
→ Witness`: it records *why a trust decision is legitimate* (authority,
source, scope, rationale). `WidgetSpecs` is `Metadata` and must **not**
be `Provenance`; `RustLanguageProvenance` is both.

**`Provenance` is a supertrait of `Metadata`, and mostly a marker.** A
provenance type *is* its own metadata record; `Provenance` adds the role
and one method:

```rust
pub trait Provenance: Metadata {
    /// Issue a tracked certificate — the one thing unique to the role.
    /// A certificate certifies a *trust basis*, not arbitrary metadata,
    /// so this stays gated behind `Provenance`.
    fn certification<R: Registry>(&self, r: &mut R, subject: impl Display)
        -> R::Certificate where Self: Sized {
        r.issue_provenance_certificate(subject, self)
    }
}
```

`Metadata` and `Provenance` are **distinct markers used as distinct
bounds** — that is the point of keeping them separate, more than any
method difference. `fn f<M: Metadata>` accepts any spec bag;
`fn g<P: Provenance>` demands a certified trust basis. Users derive
whichever the bound needs. `Provenance` may grow required semantic
entries or methods later; today it is a `Metadata` in a trenchcoat, and
that is fine.

This is *not* the round-2 forwarding anti-pattern. There is exactly one
`get` / `keys` / `len` / `report`, defined on `Metadata` and
**inherited** — no hand-written passthrough. `p.get_as::<Rpm>("…")` and
`p.report()` work directly on a provenance value; `p.certification(reg,
subj)` too.

Gone entirely:

- `Provenance::type MetadataIter` / `type Metadata` — no associated type.
  `impl Provenance for T {}` is empty once `impl Metadata for T` exists.
- `Provenance`'s hand-written `get` / `contains_key` / `len` / `is_empty`
  / `keys` / `values` / `iter` / `report` — inherited from `Metadata`.
- `ProvenanceReport` / `OwnedProvenanceReport` — renamed to
  `MetadataReport` / `OwnedMetadataReport`, moved to the `Metadata` layer.
- `Standard`'s `get` / `contains_key` / `len` / `is_empty` / `keys` /
  `values` / `metadata` passthrough (`roles.rs:26-58`). `Standard` keeps
  `provenance()` and `certification()`, plus a thin `report(&self) ->
  OwnedMetadataReport<Self::Provenance>` (its `provenance()` returns an
  owned temporary, so the owned report form is the one that fits).
  Callers reach the facts via `standard.provenance().get_as(…)`.

`Standard::Provenance: Provenance` is unchanged and now transitively
guarantees `Metadata`.

Both `#[derive(Metadata)]` and `#[derive(Provenance)]` ship: distinct
markers for distinct bounds. `#[derive(Provenance)]` = the `Metadata`
codegen + `impl Provenance for T {}`.

### Nesting

This is the load-bearing part for the widget use case. Two field kinds,
distinguished at the attribute — no runtime sentinel key:

- **`#[entry] part_number: PartNumber`** — a *leaf*. Bound: `PartNumber:
  MetadataValue`. Becomes exactly one entry, keyed by the field name.
  The derive emits `OwnedEntry::new("part_number", self.part_number.clone())`
  directly — no `.metadata()` call, no `Metadata` bound on the field.
- **`#[entry(nested)] motor: MotorSpecs`** — a *sub-record*. Bound:
  `MotorSpecs: Metadata` (`where` predicate added per nested field). Its
  entries are spliced in with a `motor.` prefix.

This replaces today's `#[derive(Provenance)]` model, where *every* field
is a sub-record, scalars get a blanket `impl Provenance` emitting an
entry keyed with the literal `"value"`, and the parent special-cases
that string to collapse it to the field name. That sentinel is fragile
(a nested field genuinely named `value` gets mangled) and invisible. The
attribute split states each field's role at the use site, gives the
derive two clean code paths, and lets `impl_scalar_provenance!` go away
entirely.

**Assembly.** A parent's `metadata()` builds its record by, for each field:

```text
// generated `fn snapshot(&self) -> Vec<OwnedEntry>`:

// #[entry] leaf:
out.push(OwnedEntry::new("<field>", self.<field>.clone()));

// #[entry(nested)] sub-record:
for entry in self.<field>.snapshot() {
    out.push(entry.prefixed("<field>"));         // "motor" + "max_rpm"
}
```

The generated `get` splits instead of snapshotting: `"motor.max_rpm"` →
match `"motor"` → `self.motor.get("max_rpm")` → `.prefixed("motor")`.

**The `Arc` is never opened during assembly.** `prefixed` rewrites the
key `String` and moves the `Arc<dyn MetadataValue>` through untouched, so
`widget.get_as::<Rpm>("motor.max_rpm")` works at any nesting depth —
`drivetrain.motor.max_rpm` composes the same way.

**Attributes:**

- `#[entry(skip)]` — omit the field.
- `#[entry(rename = "spec")]` — use `"spec"` as the key (leaf) or prefix
  (nested) instead of the field name.
- `#[entry(nested, flatten)]` — splice a sub-record's entries with **no**
  prefix (`"max_rpm"`, not `"motor.max_rpm"`). Collision handling becomes
  the programmer's problem.
- A `#[entry]` field type may itself be a closed enum with `#[derive(
  Metadata)]` — then mark it `#[entry(nested)]` to get variant + payload
  entries; leave it `#[entry]` to get one entry rendered via `Display`.

**Duplicate full keys** are allowed in `snapshot`, source order is
preserved deterministically, and `get` returns the first — matching
today's `find`-based `Provenance::get`.

### The per-schema enum — cut

The original sketch had `#[derive(Metadata)]` also generate `enum
WidgetSpecsEntry { … }` for exhaustive `match`, with `strum` for
iteration. With `get_as::<T>("key")` + `value_any()` as the typed path,
the enum is pure convenience, does not survive nesting, and
`strum::EnumIter` does not apply to data-carrying variants anyway.
**Dropped from the plan** (was Step 9).

### Worked example

```rust
#[derive(Debug, Clone, Metadata)]
pub struct MotorSpecs {
    #[entry] max_rpm: Rpm,
    #[entry] nominal_voltage: Voltage,
}

#[derive(Debug, Clone, Metadata)]
pub struct WidgetSpecs {
    #[entry]         part_number: PartNumber,   // leaf  → "part_number"
    #[entry(nested)] motor:       MotorSpecs,   // sub   → "motor.max_rpm", …
    #[entry(nested)] battery:     BatterySpecs,
    #[entry(nested)] radio:       RadioSpecs,
    #[entry(nested)] chassis:     ChassisSpecs,
    #[entry(nested)] firmware:    FirmwareSpecs,
}

impl Widget {
    fn spin_up(&self) -> Result<(), PreflightError> {
        // self.specs: WidgetSpecs, which *is* Metadata — query it directly.
        let rpm = self.specs.get_as::<Rpm>("motor.max_rpm")
            .ok_or_else(|| PreflightError::missing("motor.max_rpm"))?;
        if rpm > self.rated_ceiling {
            return Err(PreflightError::out_of_spec("motor.max_rpm", rpm));
        }

        let fw = self.specs.get_as::<SemVer>("firmware.version")
            .ok_or_else(|| PreflightError::missing("firmware.version"))?;
        if fw < Self::MIN_FIRMWARE {
            return Err(PreflightError::stale_firmware(fw));
        }

        // … structured owned values, checked, no string parsing …
        Ok(())
    }
}
```

## Laziness

"Lazy" here means **materialized on demand, never stored as a struct
field** — not pull-based streaming. `snapshot(&self)` allocates a fresh
`Vec<OwnedEntry>` each call; `KaniVerifierMetadata` et al. stay
zero-sized and build theirs from a `const` slice when asked. This is
already how the current design behaves (build a `Vec`, return
`.into_iter()`).

True streaming is rejected: a lazy iterator borrows `&self`, forcing
either a GAT on `Metadata` (the workspace has none — a large first) or
RPITIT (the documented creusot-rustc ICE). `get` avoids the full
`snapshot` by key-splitting; the per-call `Vec` in `snapshot`/`report`
is noise on an audit/pre-flight path.

## Backend constraints

| Constraint | Mitigation |
| --- | --- |
| No GATs exist in the workspace | `Metadata` has no associated types at all; every method returns owned / `&self`-borrowed concrete types |
| creusot-rustc ICEs on RPITIT at impl sites (`provenance.rs:16-29`) | No `impl Trait` in return position anywhere in `Metadata`; `snapshot` returns `Vec<OwnedEntry>` |
| Creusot `dyn` support "currently minimal" (`creusot_gallery/macro_and_type_translation.rs:257`) | `Arc<dyn MetadataValue>` in a Creusot-translated crate is the top spike risk — **Step 0 proves it or the design changes** (fallback: `MetadataValueKind` enum) |
| `provenance.rs` is `#[path]`-included into `amenable_verus`, compiled by raw `verus` with no `Cargo.toml` | Put `MetadataValue` / `OwnedEntry` / `Metadata` in a **new** `metadata.rs` that is *not* `#[path]`-included; `MetadataEntry` stays in `provenance.rs`. The verus carriers get a scalar-only `Metadata` path or stay `MetadataEntry`-only — **Step 0 proves it** |
| `Any: 'static` | Satisfied by the "owned fields only" rule already in force |
| Trait upcasting not stable on all pinned toolchains (esp. Kani's) | `MetadataValue` uses `as_display`/`as_any` methods, not `Display`/`Any` supertraits |

## Migration surface

- **37 `impl Provenance` sites** across ~36 files (`amenable_kani` ~30,
  `amenable_std` 3, `amenable_gaap` 1, `amenable_core` 3, tests).
- **~240 `MetadataEntry::new` call sites.**
- Mechanical pattern — this:

  ```text
  impl Provenance for T {
      type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;
      fn metadata(&self) -> Self::MetadataIter {
          Box::new(vec![MetadataEntry::new("k", v), …].into_iter())
      }
  }
  ```

  becomes this:

  ```text
  impl Metadata for T {
      fn snapshot(&self) -> Vec<OwnedEntry> {
          vec![OwnedEntry::new("k", v), …]
      }
  }
  impl Provenance for T {}
  ```

  `OwnedEntry::new` accepts `impl MetadataValue`, so existing `&'static
  str` / `String` values pass unchanged; `.to_string()` calls on already-
  typed values can optionally be dropped to carry the real type. No
  associated type to set anymore.
- `ProvenanceCertificate` / `cert.rs`: `entries` stays `Vec<MetadataEntry>`,
  built via `provenance.snapshot().iter().map(MetadataEntry::from).collect()`.
- `ProvenanceReport` → `MetadataReport`, `OwnedProvenanceReport` →
  `OwnedMetadataReport` (generic over `M: Metadata`); `Display::fmt`
  iterates `self.0.snapshot()`; output byte-identical. Update the
  `lib.rs` re-exports and every `.report()` call site's type annotation
  (`amenable_creusot::rust_std_witness`, `amenable_kani::rust_std`,
  `amenable_gaap` tests — the `.report()` calls themselves are unchanged,
  just inherited from `Metadata` now).
- `Standard` (`roles.rs`): drop the `get` / `contains_key` / `len` /
  `is_empty` / `keys` / `values` / `metadata` passthrough; keep
  `provenance()`, `certification()`, and a thin `report(&self) ->
  OwnedMetadataReport<Self::Provenance>`. Callers that used
  `standard.get("authority")` become
  `standard.provenance().get("authority")`.
- `impl_scalar_provenance!` (the `bool … String` blanket): **deleted**.
  It existed only so `#[derive(Provenance)]` could recurse uniformly; the
  `#[entry]` / `#[entry(nested)]` split removes the need. Any test
  asserting `5u32: Provenance` is rewritten or dropped.

## Phased implementation plan

Commit after each step (per `feedback_commit_between_plan_steps`). Serialize
all Kani runs (per `feedback_serialize_kani_calls`).

- [x] **Step 0 — de-risk the backends.** Landed `MetadataValue`,
  `ErasedEntry`, `OwnedEntry` (`Arc<dyn MetadataValue>`), `Metadata`,
  `MetadataRecord`, `MetadataReport` / `OwnedMetadataReport` in the new
  `amenable_core/src/metadata.rs` (not `#[path]`-included into
  `amenable_verus`; not wired to anything). `amenable_core` clean
  (`just check-all-package amenable_core`); `just verify-verus` → `485
  verified, 0 errors`; `just verify-creusot-translate` clean; `just
  verify-kani` sample harness passes. `Entry` + its `ErasedEntry` blanket
  moved to Step 1 (coherence with `impl ErasedEntry for OwnedEntry`).
  Deep Creusot translation of `Arc<dyn>` is exercised in Step 5, not here
  — `amenable_core` items are a dependency, not translated locally.
- [x] **Step 1 — `MetadataEntry` bridge + tests.** Added
  `impl<E: ErasedEntry + ?Sized> From<&E> for MetadataEntry` (coheres
  with the reflexive `From<T> for T`) and `Metadata::values()` for parity
  with `keys()`. `tests/metadata_test.rs` covers `get_as` typed
  recovery, `prefixed` nesting (value survives re-keying), `report`
  output incl. empty, `MetadataEntry::from`, and a `HashMap`-backed
  `Metadata` with a `get` override. The typed `Entry` trait is deferred
  again — nothing implements it yet, and `#[derive(Metadata)]` (Step 6)
  builds `Vec<OwnedEntry>` from fields without it. It lands with the
  first real named-entry vocabulary, and the `ErasedEntry` blanket /
  coherence question with it. `Provenance` still untouched.
- [ ] **Step 2 — flip `Provenance` and `Standard`.** `Provenance` becomes
  `trait Provenance: Metadata` — just `certification()`, losing
  `type MetadataIter` and every query/report method (inherited now).
  `Standard` loses its query passthrough, keeps a thin `report()`.
  Delete `impl_scalar_provenance!`. Migrate the 3 `amenable_core` impls
  (`stoplight.rs`, `provenance_test.rs`, scalar-macro fallout) and every
  `.report()` call site's type annotation. Re-verify `amenable_core` on
  all three backends.
- [ ] **Step 3 — `amenable_std`.** `cert.rs`, `verus_witness/machinery.rs`,
  `creusot_gallery/macro_and_type_translation.rs` (this file *documents* the
  RPITIT ICE — update its narrative to match the new shape). Re-verify.
- [ ] **Step 4 — `amenable_kani`.** ~30 impls across ~26 files. Purely
  mechanical. Spot-verify a representative sample of Kani harnesses plus the
  full `cargo test` (per `feedback_no_bash_gymnastics`).
- [ ] **Step 5 — the rest.** `amenable_gaap/transfer.rs`,
  `amenable_creusot/witness.rs`, `amenable/` integration tests, the
  `amenable_derive` test-support module and UI tests. Full workspace
  `just check-all` + `just check-features`.
- [ ] **Step 6 — `#[derive(Metadata)]`, struct support.** New expansion in
  `amenable_derive` reusing `attr_options.rs` (container `#[metadata(crate
  = "…")]`; field `#[entry]` leaf / `#[entry(nested)]` sub-record, with
  `skip` / `rename` / `flatten` modifiers). Generates `impl Metadata for T`
  — `snapshot` walks the fields, `get` key-splits — with leaf fields
  bounded `MetadataValue` and nested fields bounded `Metadata` in the
  `where` clause. `#[derive(Provenance)]` reuses this exact codegen and
  appends `impl Provenance for T {}` (see Open Questions on whether the two
  stay distinct). Convert the hand-written `amenable_core` impls the derive
  can now reach.
- [ ] **Step 7 — `#[derive(Metadata)]`, enum support.** Tagged variants +
  payload fields, mirroring today's `#[derive(Provenance)]` enum lowering.
- [ ] **Step 8 — docs.** Tick the `AMENABLE_PLAN.md` Phase 2 boxes, rewrite
  `provenance.rs` module docs, update the `amenable_core` / `amenable_std`
  READMEs, add the worked example to the appropriate crate.

## Fallback (if Step 0 fails on Creusot/Verus `dyn`)

1. **`enum MetadataValueKind`** — `OwnedEntry` holds this instead of
   `Arc<dyn MetadataValue>`: a closed sum of the value shapes the
   workspace actually uses (`Str(String)`, `Int(i128)`, `Uint(u128)`,
   `Bool(bool)`, `Timestamp(...)`, `Bytes(Vec<u8>)`) with an
   `Other(String)` escape. `Clone` for free, no `dyn`, no `Arc`. Loses
   open extensibility and arbitrary `downcast`; `get_as::<T>` matches the
   known shapes. Most likely the right call regardless — the open set
   buys little here.
2. **`cfg`-split**: `Arc<dyn MetadataValue>` for the Kani/std/normal
   build, `MetadataValueKind` under `#[cfg(any(creusot, verus))]`. More
   code, keeps the open interface where it can be carried.
3. Gate the whole `Metadata` layer out of the Creusot/Verus builds,
   leaving those backends on a `MetadataEntry`-only view (they never
   query specs — they emit witness metadata, `"key" => "string"` today,
   and that can stay).

## Open questions for review

1. ~~**`MetadataValue: Send + Sync`?**~~ **Resolved: yes.** Plain owned
   data, no interior mutability. Buys `OwnedEntry: Send + Sync` free.
2. ~~**`Metadata` vs `Provenance` — same trait?**~~ **Resolved:
   `Provenance: Metadata` (supertrait), both kept as distinct markers.**
   A provenance type *is* its own metadata record. The value is that they
   are **distinct bounds** — `<M: Metadata>` vs `<P: Provenance>` — not
   any current method difference (`Provenance` adds only `certification()`
   today). `WidgetSpecs` is `Metadata`, not `Provenance`. Both derives
   ship; `#[derive(Provenance)]` = `Metadata` codegen + `impl Provenance
   for T {}`. `Provenance::type Metadata` associated type is gone.
3. ~~**Per-schema `match` enum?**~~ **Resolved: cut.** `get_as` +
   `value_any` cover typed access; the enum doesn't survive nesting.
4. ~~**`type Entries` — pin to `Vec`?**~~ **Resolved: no associated type
   at all.** `snapshot(&self) -> Vec<OwnedEntry>` is the single required
   primitive; `get` is overridable for O(1) map backings. All `Metadata`
   returns are owned — that is what lets ZSTs implement it and what makes
   the supertrait in (2) possible.
5. ~~**Scalar `"value"` sentinel?**~~ **Resolved: eliminated.** The
   `#[entry]` (leaf) / `#[entry(nested)]` (sub-record) attribute split
   removes the need for a magic key and for scalar `Metadata` impls. See
   "Nesting".

### Still open

Nothing blocking Step 0. Deferred by explicit decision:

- **Extra `Provenance` methods / required semantic entries.** Added when
  a real bound needs them, not speculatively.
- **`get` / `snapshot` performance.** Provided `get` does a full
  `snapshot` scan; `MetadataRecord` scans linearly. Not optimized until
  there is a product to benchmark. Map-backed `Metadata` impls and a
  key-splitting `get` override remain available to anyone who needs them.
- **`MetadataReport` framing.** Currently one `key: value` line per
  entry. Whether provenance/standard want a titled variant is a
  rendering tweak for later.
