# Kani Proof Gallery

## Goal

Add a first-class proof gallery to `amenable` so we can record and rerun
verifier-pattern experiments without mixing them into the production proof
queue.

The gallery is where we answer questions such as:

- why did this proof time out?
- is this modeling pattern a best practice or a false trail?
- does a reduced experiment support the refinement we want to make next?

## Architectural Boundary

The gallery is a third artifact class beside production verification and proof
assessment.

- `KaniProof` inventory remains the catalog of production proof harnesses.
- `artifacts/kani-verification-results.csv` remains the latest-result table for
  production Kani proof runs.
- `artifacts/proof-assessments.jsonl` remains the append-only reviewer ledger.
- the proof gallery is a separate inventory of non-production Kani experiments
  with its own run surface and its own result ledger.

This separation matters because a gallery case may be expected to fail or time
out. In the gallery, that can be a successful confirmation of a diagnosis. In
the production queue, the same outcome is simply a failing proof run.

## Gallery Case Shape

Each gallery case should be self-registering and carry:

- a stable fully-qualified ID
- the exact Kani harness selector
- the Cargo package
- a short title
- a disposition: `hypothesis`, `false_trail`, or `best_practice`
- an expected verifier outcome: `passed`, `failed`, or `timeout`

The long-form explanation lives with the source file itself in module and
harness documentation. The registry stays concise and executable.

## CLI Surface

Add a dedicated top-level CLI surface:

```text
amenable gallery list
amenable gallery run [--case <stable-gallery-id>] [--results <path>] [--harness-timeout <duration>]
```

`run` should call Kani directly with Kani's own native timeout flag, not an
outer `timeout` process wrapper.

## Ledger

The default gallery artifact is `artifacts/kani-gallery-results.csv`.

Each row records:

- gallery case ID
- Unix timestamp
- gallery disposition
- expected verifier outcome
- observed verifier outcome
- whether the observation matched the expectation

Unlike the proof-assessment ledger, this artifact only needs the latest result
per case. The source files and git history carry the reasoning; the CSV answers
"what happened last time we reran this experiment?"

## Initial Content

The first gallery cases should establish the boundary itself, not just the
mechanics:

- a vacuous pass caused by an unsatisfiable assumption (`false_trail`)
- an explicit counterexample that genuinely fails (`hypothesis`)
- a bounded satisfiable harness that genuinely passes (`best_practice`)

After that scaffold is in place, any ambiguous or failing production-proof
refinement should first be reduced into the gallery before we let it redirect
the main queue.

## First Substantive Gallery Loop

The first post-scaffold gallery reduction targets the iterator timeout pattern
encountered in the `rust_std::iter` queue around `flatten`.

The production harness shape is:

- two bounded symbolic range lengths
- `Vec<Range<i32>>` as the nested iterator source
- `Flatten<IntoIter<Range<i32>>>` as the subject
- direct semantic comparison against `(0..a).chain(0..b)`

The question is not whether `flatten` is semantically lawful; that is already
the intended production claim. The question is which observation strategy is
tractable for Kani.

The first two observed results sharpened that question:

- eager materialization into `Vec<i32>` does time out and is a false trail
- incremental `next()` comparison alone still times out for the same symbolic
  `flatten` shape, so that is also a false trail in this form

That leaves a narrower hypothesis to test next: does the same incremental
observation pass once the iterator lengths are concrete rather than symbolic?

The gallery should carry all three cases with their expected outcomes so later
refinements can cite an executable precedent rather than relying on memory.

## Acceptance Criteria

- gallery cases self-register through `inventory`
- `amenable gallery list` renders the compiled gallery catalog
- `amenable gallery run` executes one or all cases with Kani's native timeout
- gallery results are recorded separately from production proof results
- a case whose observed outcome matches its expected outcome counts as a
  successful gallery run, even when that outcome is `failed` or `timeout`

## Replace-Issue Taxonomy

The current `recommendation = "replace"` population is not one problem. It is
several distinct verifier-fit failures, and the gallery should track them by
pattern rather than by proof name.

The main issue classes so far are:

- unsupported foreign boundary reached by an otherwise sensible std proof
  (`backtrace`, anonymous `pipe`)
- unsupported panic boundary (`catch_unwind`)
- Kani model mismatch against a real process invariant (`env::args`,
  `env::args_os`)
- OS-backed filesystem boundary with real external state
  (`fs::{DirBuilder, File, Metadata, ReadDir, Permissions, ...}`)
- pure in-memory std implementation blow-up despite a crisp contract
  (`hash`, `fmt::Arguments`, buffered `io`, several iterator adapters)

The gallery should therefore preserve at least one reduced representative for
each class, so later reviews can say "this proof is replace-marked because it
matches gallery issue X" instead of rediscovering the failure mode from
scratch.
