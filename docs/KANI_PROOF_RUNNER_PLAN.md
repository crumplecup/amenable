# Kani Proof Runner and Result Ledger

## Goal

Make every executable Kani harness self-register at compile time, then let
the `amenable` CLI discover, select, run, and record those harnesses.

The runner passes Kani's own per-harness timeout argument to every Kani
invocation. It must not wrap the whole Kani process in an operating-system
timeout: compilation and harness startup are outside the proof search budget.

The locally installed Kani accepts this as
`-Z unstable-options --harness-timeout <duration>`; for example, the
three-minute default is `--harness-timeout 3m`. The command for one Amenable
harness will therefore be:

```text
cargo kani -p amenable_kani --lib --all-features --exact \
  --harness <kani-harness-selector> -Z unstable-options --harness-timeout 3m
```

`--exact` keeps a registered selector from accidentally running another
harness whose fully-qualified name merely shares a prefix.

## Architectural Boundary

Static registration and dynamic verification are different facts.

- The inventory registry is the complete, compiled-in catalog of executable
  harnesses. It identifies how a harness is invoked, but never claims it
  passed.
- The Kani command is the authority that determines one run's result.
- A CSV ledger records the latest observed result for each stable proof ID:
  timestamp, status (`passed`, `failed`, or `timeout`), and the identity of
  the proof that produced it.

The ledger is operational state, not a Rust type-level property. A proof
must never become a different compile-time type merely because a previous
execution passed.

## Existing Seams

- `amenable_core::ProofRecord` registers rendered proof artifacts for audit.
  It deliberately has no executable command or harness selector.
- `amenable_derive::harness!` produces a `#[cfg(kani)]` harness item and the
  exact source text used in audit output.
- The `amenable` binary already owns command parsing and artifact writes.
- Kani harnesses currently live in the `amenable_kani` library target, so a
  registered selector must identify that target as well as the harness.

## Design

### Kani harness inventory

Add a Kani-specific inventory record owned by `amenable_kani`, rather than
putting Cargo/Kani invocation details in `amenable_core`.

Each record will contain:

- a stable proof ID, derived from module path and harness function name;
- the Kani harness selector, using Kani's crate-relative module path;
- the Cargo package (the library target is fixed by the runner command).

Extend the harness-generation path so a `kani` harness produces both its
cfg-gated function and its Kani inventory registration from the same
invocation. That makes the executable selector and the compiled harness
impossible to drift apart.

Trusted provenance-only witnesses are not executable Kani harnesses and do
not receive Kani harness registrations.

### CSV result ledger

Store the ledger at a deterministic workspace artifact path, with an
override for targeted or CI use. Its schema is deliberately small:

```text
proof_id,timestamp,status
amenable_kani::rust_std::array::verify_try_from_slice_rejects_a_length_mismatch,1784743231,passed
```

The file is a latest-result table: one row per proof ID. A completed run
updates that proof's row and preserves rows for proofs not selected in the
current invocation. Persist after each harness result so an interrupted
batch still leaves useful retry state.

`failed` covers Kani-reported verification failure and invocation failure;
`timeout` is reserved for Kani's own timeout outcome.

### CLI surface

Add a Kani verification command with mutually composable selectors:

```text
amenable verify kani
amenable verify kani --proof <stable-proof-id>
amenable verify kani --failed
amenable verify kani --timeout
```

The command tree and argument validation are derived with `clap`, including
its built-in help and conflict reporting.

No selector runs every registered executable harness. `--proof` selects one
exact stable ID, which the runner maps to Kani's separate exact harness
selector. `--failed` and `--timeout` select IDs from the CSV ledger; an ID
no longer registered is reported clearly rather than silently ignored.

Every selected proof becomes one Kani invocation, with the configured
three-minute Kani timeout passed directly to Kani. The CLI reports a
non-success exit if any selected proof fails or times out, after recording
every result reached during the batch.

## Implementation Steps

1. Use the locally verified Kani invocation syntax: `cargo kani -p
   amenable_kani --lib --all-features --exact --harness <kani-harness-selector> -Z
   unstable-options --harness-timeout <duration>`. Classify Kani's own
   timeout report distinctly from ordinary verification failure; do not use
   GNU `timeout` or infer timeout from its exit code.
2. Add the Kani-specific record and collection in `amenable_kani`, plus tests
   proving a representative harness is discoverable with its stable ID and
   selector.
3. Extend the harness generation path and migrate existing executable Kani
   harnesses mechanically, without registering provenance-only witnesses.
4. Add a small CSV ledger module to the CLI crate: parse, validate, upsert,
   and atomically persist latest results. Test malformed rows, replacement,
   and persistence after partial batches.
5. Add CLI selection and Kani command construction. Unit-test command
   arguments so the Kani timeout is present and no outer timeout program is
   introduced.
6. Run a representative proof through the actual local Kani toolchain, then
   exercise single-proof, failed-retry, and timeout-retry workflows.
7. Document the command, ledger location/schema, and the distinction between
   registered harnesses and recorded verification results.

## Acceptance Criteria

- Every executable Kani harness is discoverable from inventory without source
  scanning.
- A selected run invokes Kani with its native per-harness timeout argument.
- The CLI can run all, one, prior failures, and prior timeouts.
- The CSV has one current row per proof ID and is updated after each result.
- Inventory registration never implies that a proof passed.
- A real Kani run validates the command shape and timeout classification.
