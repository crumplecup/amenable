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

# Required env vars for cargo-creusot/why3find; see CREUSOT_GUIDE.md in
# ~/repos/elicitation for the reference invocation this mirrors.
creusot_env := "PATH=" + home_directory() + "/.local/share/creusot/bin:${PATH} DUNE_DIR_LOCATIONS=why3find:lib:" + home_directory() + "/.local/share/creusot/share/why3find WHY3CONFIG=" + home_directory() + "/.config/creusot/why3.conf"

# `amenable_creusot` compiles fine on plain stable (its `creusot-std`
# dependency needs no nightly feature) -- these recipes just turn on the
# `creusot` feature, which is off by default (an experimental, still-partial
# verifier backend, not yet part of the default proof-chain surface).
check-creusot:
    cargo check -p amenable_creusot
    cargo check -p amenable_std --features creusot
    cargo check -p amenable --features creusot

clippy-creusot:
    cargo clippy -p amenable_creusot --all-targets -- -D warnings
    cargo clippy -p amenable_std --features creusot --all-targets -- -D warnings
    cargo clippy -p amenable --features creusot --all-targets -- -D warnings

test-creusot:
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
    env {{creusot_env}} cargo creusot -- -p amenable_creusot

# Full translate + prove for amenable_creusot's own contracts.
verify-creusot:
    env {{creusot_env}} cargo creusot prove -- -p amenable_creusot

# Cross-checks the Windows-gated std paths (std::os::windows, etc.) that
# only compile on a matching host otherwise. Requires `cross`
# (cargo install cross) and Podman as the container engine; no real
# Windows needed since this only compiles, never runs, the target code.
check-windows:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu --workspace

check-windows-package package:
    CROSS_CONTAINER_ENGINE=podman cross check --target x86_64-pc-windows-gnu -p {{package}}
