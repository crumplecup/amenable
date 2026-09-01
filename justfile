set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    @just --list

check:
    cargo check --workspace

check-package package:
    cargo check -p {{package}}

test:
    cargo test --workspace

test-package package:
    cargo test -p {{package}}

fmt:
    cargo fmt --all --check

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

check-all:
    just fmt
    just clippy
    just test

check-all-package package:
    cargo fmt --all --check
    cargo clippy -p {{package}} --all-targets --all-features -- -D warnings
    cargo test -p {{package}}

# Canonical Kani verification entrypoint -- delegates to the `amenable
# verify kani` binary, never a raw `cargo kani` call, so it's registry-
# driven: `crates/amenable/src/kani.rs`'s `registered_proofs()` iterates
# only `KaniProofRegistration`, a real, separate registry from proof-
# gallery cases' own `KaniGalleryRegistration` (see `just gallery-run`
# for those) -- this can never sweep the gallery in by accident, unlike
# a bare `cargo kani` invocation with no `--harness` filter, which
# discovers and runs *every* `#[kani::proof]` compiled into the crate
# regardless of which registry (if any) it's in.
#
# With a harness name, runs exactly that one registered proof; with
# none, runs every registered proof and persists a CSV ledger (`amenable
# verify kani --help` for `--failed`/`--timeout` retry selectors). `-Z
# function-contracts -Z stubbing` are always enabled by `kani_command`
# (required for `kani::requires`/`kani::ensures`/`kani::proof_for_
# contract`/`kani::stub_verified` harnesses, harmless for ones that
# don't use them), so contract/stub-based harnesses need no separate
# recipe. Kani cannot place contracts on trait methods when the trait
# itself is generic (a real 0.67.0 tooling limitation, not a project
# convention): contracted logic lives on plain inherent methods instead,
# with trait impls reduced to single-expression delegation.
verify-kani harness="":
    cargo run -p amenable -- verify kani {{ if harness == "" { "" } else { "--proof " + harness } }} --harness-timeout 3m

# Required env vars for cargo-creusot/why3find; see CREUSOT_GUIDE.md in
# ~/repos/elicitation for the reference invocation this mirrors.
creusot_env := "PATH=" + home_directory() + "/.local/share/creusot/bin:${PATH} DUNE_DIR_LOCATIONS=why3find:lib:" + home_directory() + "/.local/share/creusot/share/why3find WHY3CONFIG=" + home_directory() + "/.config/creusot/why3.conf"

# `amenable_creusot` compiles fine on plain stable (its `creusot-std`
# dependency needs no nightly feature) -- these recipes just turn on the
# `creusot` feature, which is off by default (an experimental, still-partial
# verifier backend, not yet part of the default proof-chain surface).

# Regenerates `amenable_creusot/src/generated/*.rs` -- real `Exchange`-edge
# companions written from `amenable_core::ExchangeEdgeRecord` (registered
# by `#[amenable_derive::exchange(..)]` in `amenable_kani`), the same
# "regenerate before checking" discipline `emit-verus-witnesses` already
# uses. Run this after changing a real Kani-side transition; the generated
# files are checked in, not built fresh by `cargo creusot` itself.
# `rustfmt` invoked directly per file, not `cargo fmt -p amenable_creusot`
# -- confirmed empirically that `cargo fmt` never reaches `src/generated/
# *.rs` at all, since they're `include!`d into `stoplight.rs`, not `mod`-
# declared, and `cargo fmt`'s file discovery walks the module tree, not
# the literal file tree.
generate-creusot:
    cargo run -p amenable --features creusot -- creusot emit-companions
    rustfmt crates/amenable_creusot/src/generated/*.rs

check-creusot:
    just generate-creusot
    cargo check -p amenable_std
    cargo check -p amenable_creusot
    cargo check -p amenable --features creusot

clippy-creusot:
    just generate-creusot
    cargo clippy -p amenable_std --all-targets -- -D warnings
    cargo clippy -p amenable_creusot --all-targets -- -D warnings
    cargo clippy -p amenable --features creusot --all-targets -- -D warnings

test-creusot:
    just generate-creusot
    cargo test -p amenable_std
    cargo test -p amenable_creusot
    cargo test -p amenable --features creusot

check-all-creusot:
    just check-creusot
    just clippy-creusot
    just test-creusot

# Rust -> Why3/COMA translation only, no SMT solving -- fastest way to check
# that contracts actually parse under the real Creusot toolchain.
verify-creusot-translate:
    just generate-creusot
    env {{creusot_env}} cargo creusot -- -p amenable_creusot

# Full translate + prove for amenable_creusot's own contracts.
verify-creusot:
    just generate-creusot
    env {{creusot_env}} cargo creusot prove -- -p amenable_creusot

# `amenable_verus` compiles fine on plain stable (`verus_builtin_macros`/
# `vstd` are ordinary crates.io deps -- the `verus! {}` macro expands to
# plain Rust for ordinary rustc, spec clauses erased) -- these recipes just
# turn on the `verus` feature, which is off by default (an experimental,
# still-partial verifier backend, not yet part of the default proof-chain
# surface). `amenable_verus` itself is never gated: it's never a Cargo
# dependency of anything (see `amenable_std::verus_witness`'s own doc
# comment for why), so it has no feature to turn on.
emit-verus-witnesses:
    cargo run -p amenable --features verus -- verus emit-witnesses

# Regenerates `amenable_verus::gallery::stoplight_exchange`'s derived
# `Exchange`-edge companions from `amenable_core::ExchangeEdgeRecord` --
# the Verus-side counterpart to `generate-creusot`, same reason: real
# codegen from a registry read inside the ordinary, never-translated
# `amenable` binary, not a hand-copied mirror. `cargo fmt` does not
# discover `include!`d files (only walks the `mod` tree), so `rustfmt`
# runs directly on the generated files, matching `generate-creusot`.
generate-verus-exchange:
    cargo run -p amenable --features verus -- verus emit-exchange-companions
    rustfmt crates/amenable_verus/src/gallery/generated/*/*.rs

# Regenerates `amenable_verus::gallery::ledger_exchange`'s derived
# proof-token companion (`generated/ledger_tokens.rs`) from `amenable_
# core::ProofTokenMintRecord` -- the token-layer counterpart to
# `generate-verus-exchange` (`GAAP_LEDGER_PLAN.md`'s Step 8). Previously
# only ever run by hand via `cargo run -p amenable --features verus --
# verus emit-gaap-tokens`, with no `just` recipe wrapping it at all -- a
# real gap (`CLAUDE.md`'s own "reference `just` recipes, never raw
# `cargo` commands" rule), closed here rather than left in place once
# `reject`'s/`rollback`'s own tokens needed regenerating too (`GAAP_
# LEDGER_PLAN.md`'s Step 7, revisited).
generate-verus-gaap-tokens:
    cargo run -p amenable --features verus -- verus emit-gaap-tokens
    rustfmt crates/amenable_verus/src/gallery/generated/ledger_tokens.rs

check-verus:
    just emit-verus-witnesses
    just generate-verus-gaap-tokens
    just generate-verus-exchange
    cargo check -p amenable_verus
    cargo check -p amenable_std --features verus
    cargo check -p amenable --features verus

clippy-verus:
    just emit-verus-witnesses
    just generate-verus-gaap-tokens
    just generate-verus-exchange
    cargo clippy -p amenable_verus --all-targets -- -D warnings
    cargo clippy -p amenable_std --features verus --all-targets -- -D warnings
    cargo clippy -p amenable --features verus --all-targets -- -D warnings

test-verus:
    just emit-verus-witnesses
    just generate-verus-gaap-tokens
    just generate-verus-exchange
    cargo test -p amenable_std --features verus
    cargo test -p amenable --features verus

check-all-verus:
    just check-verus
    just clippy-verus
    just test-verus

# Real Verus verification: invoked as a bare compiler over a single file
# tree (never reads Cargo.toml), so `amenable_verus` must depend on
# nothing but `verus_builtin_macros`/`vstd` -- see `amenable_verus::lib`'s
# own doc comment. Requires `verus` on PATH (~/.cargo/bin/verus after the
# usual `verus-lang/verus` build); see VERUS_GUIDE.md in ~/repos/elicitation
# for the reference invocation this mirrors.
verify-verus:
    just emit-verus-witnesses
    just generate-verus-gaap-tokens
    just generate-verus-exchange
    status=0; verus --crate-type=lib crates/amenable_verus/src/lib.rs || status=$?; rm -f liblib.rlib; exit $status

# Cross-checks the Windows-gated std paths (std::os::windows, etc.) that
# only compile on a matching host otherwise. Requires `cross`
# (cargo install cross) and Podman as the container engine; no real
# Windows needed since this only compiles, never runs, the target code.
check-windows:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu --workspace

check-windows-package package:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu -p {{package}}
