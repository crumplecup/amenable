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

verify-kani harness:
    cargo kani -p amenable_kani --lib --all-features --output-format terse --exact --harness {{harness}} -Z unstable-options --harness-timeout 3m

# For `#[kani::proof_for_contract(...)]` harnesses (real function contracts
# via DFCC, see EXCHANGE_PROOF_DERIVATION_PLAN.md). `-Z function-contracts`
# is required for any harness using `kani::requires`/`kani::ensures`/
# `kani::proof_for_contract`; `-Z stubbing` additionally for
# `kani::stub_verified` composition harnesses (not yet used, staged here
# for when Step 4 needs it). Harness names for contract harnesses include
# their module path (see `cargo kani list -Z function-contracts` from
# inside `crates/amenable_kani` to discover exact names) -- `--exact` still
# applies. Kani cannot place contracts on trait methods when the trait
# itself is generic (a real 0.67.0 tooling limitation, not a project
# convention): contracted logic lives on plain inherent methods instead,
# with trait impls reduced to single-expression delegation.
verify-kani-contract harness:
    cargo kani -p amenable_kani --lib --all-features --output-format terse --exact --harness {{harness}} -Z unstable-options -Z function-contracts -Z stubbing --harness-timeout 3m

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
    cargo run -p amenable --features creusot -- emit-creusot-companions
    rustfmt crates/amenable_creusot/src/generated/*.rs

check-creusot:
    just generate-creusot
    cargo check -p amenable_creusot
    cargo check -p amenable_std --features creusot
    cargo check -p amenable --features creusot

clippy-creusot:
    just generate-creusot
    cargo clippy -p amenable_creusot --all-targets -- -D warnings
    cargo clippy -p amenable_std --features creusot --all-targets -- -D warnings
    cargo clippy -p amenable --features creusot --all-targets -- -D warnings

test-creusot:
    just generate-creusot
    cargo test -p amenable_creusot
    cargo test -p amenable_std --features creusot
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
    cargo run -p amenable --features verus -- emit-verus-witnesses

check-verus:
    just emit-verus-witnesses
    cargo check -p amenable_verus
    cargo check -p amenable_std --features verus
    cargo check -p amenable --features verus

clippy-verus:
    just emit-verus-witnesses
    cargo clippy -p amenable_verus --all-targets -- -D warnings
    cargo clippy -p amenable_std --features verus --all-targets -- -D warnings
    cargo clippy -p amenable --features verus --all-targets -- -D warnings

test-verus:
    just emit-verus-witnesses
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
    verus --crate-type=lib crates/amenable_verus/src/lib.rs

# Cross-checks the Windows-gated std paths (std::os::windows, etc.) that
# only compile on a matching host otherwise. Requires `cross`
# (cargo install cross) and Podman as the container engine; no real
# Windows needed since this only compiles, never runs, the target code.
check-windows:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu --workspace

check-windows-package package:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu -p {{package}}
