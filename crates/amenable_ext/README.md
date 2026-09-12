# amenable_ext

Third-party crate support for the `amenable` trait family — one crate,
not one per target library. Each target lives in its own directory
behind a same-named feature flag, default `[]` (empty): `cargo add
amenable_ext --features jiff` sweeps in exactly the `jiff` dependency
and its registrations, nothing else.

`ExtType` (per-type static metadata: source crate/module, doc URL,
semantic summary) + `ExtStandard<T>` (a generic `Evidence` newtype) are
the third-party-crate counterpart of `amenable_std`'s `RustStdType` /
`RustStdStandard<T>` — the orphan rule forces the same "interface and
registrations live together in one crate" shape here as it does there.

First target: **jiff**, and jiff only for now. See
`docs/AMENABLE_EXT_PLAN.md` (repository root) for the full plan,
phasing, and the `cordial` coverage-tooling design this crate's
registrations are meant to be checked against.
