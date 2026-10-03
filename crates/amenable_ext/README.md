# amenable_ext

> Third-party crate support for the `amenable` trait family: the
> registration layer for foreign types, and — for jiff — a real,
> three-backend-verified implementation built on top of it.

## The registration problem

Rust's orphan rule says a trait impl must live in the crate that
defines the trait *or* the crate that defines the type. `amenable_core`
defines the roles (`Standard`, `Evidence`, …); `jiff` defines
`jiff::Timestamp`. Neither crate can host an impl connecting the two,
so something has to — a small, dedicated crate that depends on both.
`amenable_std` plays this role for the Rust standard library;
`amenable_ext` plays it for everything else, one target crate at a
time, each behind its own same-named feature flag:

```toml
[dependencies]
amenable_ext = { version = "...", features = ["jiff"] }
amenable_time = "..." # pulled in transitively by the jiff feature
```

`ExtType` (per-type metadata: maintaining authority, source module, doc
URL, a one-line semantic summary) and `ExtStandard<T>` (a generic
`Evidence` wrapper) are the third-party counterpart of `amenable_std`'s
`RustStdType` / `RustStdStandard<T>`. Registering a type costs one
macro call naming its authority and documentation:

```rust
use amenable_core::Standard;
use amenable_ext::ExtStandard;

let cite = ExtStandard::<jiff::Timestamp>::new().provenance();
println!("{cite}");
// authority_kind: external_standard
// authority: jiff contributors
// source_crate: jiff
// source_module: jiff
// source_url: https://docs.rs/jiff/latest/jiff/struct.Timestamp.html
// type_name: jiff::Timestamp
// semantic_summary: A Timestamp is an instant in time represented as a
//   signed count of nanoseconds since the Unix epoch, …
```

`cargo add amenable_ext --features jiff` sweeps in exactly the `jiff`
dependency and its registrations — nothing else. jiff is the first
target, and currently the only one; a second (`chrono`, say) would get
its own directory and feature flag the same way, with no change to how
jiff is wired.

## More than a registry: a real backend

Registration alone would make this a bookkeeping crate. What actually
lives here is bigger: `JiffTimeBackend`, a working implementation of
most of [`amenable_time`](../amenable_time/README.md)'s contract-backed
trait surface, built on real jiff calendar arithmetic — not a
hand-rolled stand-in and not a mock. Where `amenable_std`'s own
`StdTimeBackend` is an honest **canary**, deliberately scoped to the
narrow slice `std::time` can back at all (no calendars, no zones, no
text parsing), `JiffTimeBackend`'s `Exchange` bodies call jiff's actual
parser, its actual `Span` arithmetic, its actual IANA time zone
database lookups.

Concretely: realizing a duration means turning an ISO 8601 duration
descriptor into a real `jiff::Span`, carrying proof that the result
satisfies `amenable_time`'s duration contract — not just returning a
value, but a value *with the proof attached*. Trimmed from the real
test suite (`tests/jiff_backend_test.rs`):

```rust
use amenable_core::Exchange;
use amenable_ext::{JiffSpan, JiffTimeBackend};
use amenable_time::{DurationDescriptorBuilder, ProvenDurationCarrier, ReflectedDuration};

let descriptor = DurationDescriptorBuilder::default()
    .years(1u32).months(2u32).weeks(3u32).days(4u32)
    .hours(5u32).minutes(6u32).seconds(7u32)
    .build()?;

let backend = JiffTimeBackend;
let carrier: ProvenDurationCarrier<JiffSpan> =
    backend.exchange(ReflectedDuration::new(descriptor, /* proof token */))?;

let span = **carrier.carrier(); // a real jiff::Span, proof-carrying
assert_eq!(span.get_years(), 1);
assert_eq!(span.get_months(), 2);
// … every component round-trips exactly; jiff::Span carries years and
// months natively, richer than std::time::Duration's whole-seconds-only shape
```

(The elided proof token is built through `amenable_core::Establish` —
see the real test for the full, uncut version; it's a few lines, not
hidden complexity.) The same real-arithmetic treatment extends across
duration, instant, civil dates, named time zones with full IANA
disambiguation, UTC conversion, and both parsing and formatting for
every ISO 8601 / RFC 3339 / RFC 9557 form jiff can represent. What jiff
*can't* represent — the CalConnect / ISO 8601-2 "extension" family:
masked digits, seasons, temporal sets, recurrence rules — gets an
honest `Unsupported` error, not a silent approximation. Over 3,500
lines of tests across a dozen files exercise this directly; read
`tests/jiff_backend_*.rs` for the full surface.

## Three backends, for a third-party crate too

The standard library's "prove it three independent ways" discipline
(see the [root README](../../README.md)) doesn't stop at `std`. jiff's
own types carry real Kani harnesses, real Creusot Pearlite contracts,
and real Verus postconditions, each checking jiff's documented
behavior — civil date/time round trips, time zone disambiguation,
duration field-wise arithmetic — the same way `amenable_std` does for
`char` or `String`. This is what makes `JiffTimeBackend` more than a
convenience wrapper: the native carrier types it hands back aren't
just believed to behave correctly, they're checked to.

## Using it

1. Add the dependency with the feature for your target crate:
   `cargo add amenable_ext --features jiff` (add `amenable_time`
   explicitly too — it's pulled in transitively, but you'll want its
   types in scope directly, as the example above does).
2. Construct a `JiffTimeBackend` and call its `Exchange` impls, or
   implement `amenable_time`'s traits yourself against jiff's types —
   `JiffTimeBackend`'s own source is the reference for how.
3. Enable `--features verus` if you also want the Verus witness bridge
   for jiff's carrier types linked into your build (off by default,
   like every verifier feature in this workspace).

## See also

- [Root README](../../README.md) for the project-wide overview and the
  three-backend verification story.
- [`amenable_time`](../amenable_time/README.md) for the contract and
  trait-interface layer this crate implements.
- [`amenable_std`](../amenable_std/README.md) for the sibling crate
  playing the same registration role for the standard library.
