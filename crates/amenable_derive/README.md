# amenable_derive

> Proc macros for the `amenable` constitutional trait family.

## What this crate is

Mechanical, repetitive scaffolding the trait family needs across many
call sites — captured once here instead of hand-written per type.

## Macros

- `harness!(verifier, CONST_NAME, { item })` — defines a `#[cfg(...)]`-
  gated proof harness item and, alongside it, an always-available
  `&'static str` constant holding the harness's verbatim source
  (captured via `Span::source_text`, whitespace and all), so an audit
  report can show a proof exactly as its author wrote it and the two can
  never drift apart the way a hand-maintained description could.
- `#[derive(Provenance)]` — generates `Provenance` impls that project a
  struct or enum's own fields into structured, chain-derived metadata.
- `#[calculation]` — turns a method into a chain link in the evidence
  graph: it knows it has a method, knows it yields a token, and
  registers itself.
- `#[evidence]` — computes `is_root` for an `impl Evidence` block from
  its own `Basis` declaration, at compile time, with no `TypeId` and no
  `'static` bound needed.
- `#[derive(Standard)]` — generates both `Standard` and `Evidence` impls
  from a `#[standard(...)]` attribute, since they always share the same
  provenance value.
- `#[derive(KaniCompose)]` — composition scaffolding for Kani-side
  worked-example types.

## See also

- [Root README](../../README.md) for the project-wide overview.
- [`amenable_core`](../amenable_core/README.md) for the trait family
  these macros generate impls against.
