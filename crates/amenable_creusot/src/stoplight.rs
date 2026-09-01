//! Creusot proof-function content for the `Stoplight`/`Exchange` worked
//! example (see `amenable_kani::stoplight` for the real Kani-side
//! implementation, and `EXCHANGE_PROOF_DERIVATION_PLAN.md` for the design
//! discussion this file is the outcome of).
//!
//! **A real, later correction, twice over.** This crate briefly carried a
//! real Cargo dependency directly on `amenable_kani`, to implement
//! `Witness<CreusotVerifier>` on the real `Green`/`Yellow`/`Red` types —
//! confirmed to compile cleanly, no ICE, since `creusot-rustc`'s
//! translator only sweeps items *local* to the crate it's directly
//! translating, not an ordinary dependency's own items (`ledger.rs`'s own
//! doc comment has the full finding). But that dependency itself was
//! wrong: it reintroduced exactly the backend-depends-on-backend edge the
//! "verifier backends never depend on each other" rule exists to forbid
//! (the same rule that got the old `amenable_kani -> amenable_creusot`
//! edge removed in the first place). The actual fix, matching the split
//! `amenable_gaap` already uses for `Pending`/`Validated`/`Committed`/
//! `Rejected<T>`: `Green`/`Yellow`/`Red` now live in `amenable_core` (see
//! `amenable_core::stoplight`'s own doc comment for why `amenable_core`
//! specifically, not a new dedicated crate) — a neutral crate both
//! `amenable_kani` and `amenable_creusot` depend on independently, with
//! no edge between the two backends at all. This crate's Cargo dependency
//! on `amenable_kani` has been removed entirely; the `amenable_std`/
//! `amenable_gaap` dependencies below remain (unrelated to this
//! correction — see `rust_std_witness.rs`/`ledger.rs` for those).
//!
//! **What still can't move, and why the mirror below still exists.** The
//! real transition bodies (`green_to_yellow`/`yellow_to_red`/
//! `red_to_green`) construct `Established::new(..)`/`YellowToken(())`/
//! `RedToken(())` directly — and those constructors are deliberately
//! *private* in `amenable_kani::stoplight`, the actual mechanism that
//! guarantees a token can never be disconnected from a real `establish()`
//! call. Widening them to `pub` so this crate could compile the real
//! bodies verbatim against the real types would be a genuine, permanent
//! weakening of that guarantee, not a free cleanup — so the transition
//! bodies still run against a local accommodation-model mirror
//! (`Green`/`Yellow`/`Red`/tokens/`Established<T, Token>` below), not the
//! real types. This is a deliberate, load-bearing split, not an
//! oversight: audit-only reporting (no construction involved) uses the
//! real types directly; anything that needs to *construct* a token still
//! needs its own mirror, matching whatever privacy the real constructors
//! actually have.
//!
//! The mirror types below genuinely implement the real `amenable_core`
//! trait family (`Evidence`, `Witness<V>`, `Sidecar<V>`, `Establish<C,
//! V>`, `ProofToken`), gated to `CreusotVerifier`, the same way the Kani
//! side implements them gated to `KaniVerifier` — `Provenance` stays
//! left out (this file has no use for it). None of those trait
//! *definitions* use the specific patterns that actually caused real
//! `creusot-rustc` translator crashes in this exact codebase (`Provenance`
//! impls returning `Box<dyn Iterator<..>>`; a return-position `impl
//! Trait` method; ungated `inventory::collect!`/`inventory::submit!` —
//! see `rust_std_witness.rs`'s doc comment and `amenable_std::
//! creusot_gallery`'s own confirmed findings).
//!
//! **The three per-edge transition bodies are generated, not hand-written
//! or hand-kept-in-sync.** An earlier version of this file hand-copied
//! `amenable_kani::stoplight`'s real transition logic per edge, needing
//! `stoplight_mirror_consistency_test.rs` to catch drift between the two
//! copies — a real, standing risk this whole `EXCHANGE_PROOF_DERIVATION_
//! PLAN.md` lineage otherwise exists to close. `#[amenable_derive::
//! exchange(..)]` now captures each real edge's transition body verbatim
//! at macro-expansion time and registers it, alongside its real type
//! names, as an `amenable_core::ExchangeEdgeRecord` — safe to do from
//! `amenable_kani` (an ordinary, never-translated Cargo crate) even
//! though this crate could never do the same for itself. `amenable`'s own
//! `emit-creusot-companions` CLI command (`amenable::creusot_export`)
//! reads that registry and *writes* the three `include!`d files below —
//! real, checked-in, `inventory`-free source `cargo creusot` just
//! compiles as ordinary static code, the same "generate from a safe
//! registry query, never call `inventory` inside the translated crate"
//! pattern `amenable::verus_export`/`emit-verus-witnesses` already uses
//! for the witness-composition system. Regenerate with `just
//! generate-creusot` after changing a real Kani-side transition; do not
//! hand-edit `src/generated/*.rs`.
//!
//! The state/token type *definitions* below stay hand-written (their own
//! field layout has no derive to collapse into, and isn't something a
//! different backend's real source could be captured verbatim *from* in
//! the first place -- `amenable_core`'s real `Green`/`Yellow` and
//! `amenable_kani`'s real `Established<T, Token>` are different,
//! concrete types from this file's own mirror, even though both names
//! and shapes match by convention). `Established<T, Token>`'s own
//! `Sidecar<CreusotVerifier>` impl is not hand-written, though --
//! `#[derive(amenable_derive::Sidecar)]` generates it, the same derive
//! `amenable_kani::stoplight`'s real `Established<T, Token>` now uses
//! for the identical shape, closing the hand-mirror duplication that
//! used to exist between the two.

/// The `#[cfg(creusot)]` mirror types (`Green`/`Yellow`/`Red`/tokens/
/// `Established`/`Stoplight`/`StoplightError`) and the traits they
/// implement, consolidated into one gate on this `mod` instead of one
/// per item (the file's original shape). The three `include!`d files
/// below stay at this file's own top level, not nested in here: each
/// carries its own mix of `#[cfg(creusot)]` (the real transition body)
/// and `#[cfg(not(creusot))]` (the `ProofRecord` registration) content,
/// so nesting them in a `#[cfg(creusot)]`-only module would make the
/// registration half unsatisfiable in every configuration. Instead this
/// module re-exports every name the includes reference by bare name
/// (types, traits, and the two Creusot attribute macros) once, right
/// below it.
#[cfg(creusot)]
mod mirror {
    pub(crate) use amenable_core::{Establish, Sidecar};
    use amenable_core::{Evidence, ProofToken, Witness};
    pub(crate) use creusot_std::macros::{ensures, requires};

    pub(crate) use crate::CreusotVerifier;

    /// Sanitized mirror of `amenable_kani::Green`.
    pub struct Green;

    impl Evidence for Green {
        type Basis = Self;
        type Audit = ();

        fn basis() -> Self::Basis {
            Self
        }

        fn audit(&self) -> Self::Audit {}
    }

    impl Witness<CreusotVerifier> for Green {
        type SupportingEvidence = Self;
        type ProofArtifact = ();

        fn proof() -> Self::ProofArtifact {}
    }

    /// Sanitized mirror of `amenable_kani::Yellow`.
    pub struct Yellow;

    impl Evidence for Yellow {
        type Basis = Self;
        type Audit = ();

        fn basis() -> Self::Basis {
            Self
        }

        fn audit(&self) -> Self::Audit {}
    }

    impl Witness<CreusotVerifier> for Yellow {
        type SupportingEvidence = Self;
        type ProofArtifact = ();

        fn proof() -> Self::ProofArtifact {}
    }

    /// Sanitized mirror of `amenable_kani::Red`.
    pub struct Red;

    impl Evidence for Red {
        type Basis = Self;
        type Audit = ();

        fn basis() -> Self::Basis {
            Self
        }

        fn audit(&self) -> Self::Audit {}
    }

    impl Witness<CreusotVerifier> for Red {
        type SupportingEvidence = Self;
        type ProofArtifact = ();

        fn proof() -> Self::ProofArtifact {}
    }

    /// Sanitized mirror of `amenable_kani::GreenToken`.
    pub struct GreenToken(());

    impl Clone for GreenToken {
        fn clone(&self) -> Self {
            GreenToken(())
        }
    }

    impl Copy for GreenToken {}

    impl ProofToken for GreenToken {
        type Proposition = Green;
    }

    /// Sanitized mirror of `amenable_kani::YellowToken`.
    pub struct YellowToken(());

    impl Clone for YellowToken {
        fn clone(&self) -> Self {
            YellowToken(())
        }
    }

    impl Copy for YellowToken {}

    impl ProofToken for YellowToken {
        type Proposition = Yellow;
    }

    /// Sanitized mirror of `amenable_kani::RedToken`.
    pub struct RedToken(());

    impl Clone for RedToken {
        fn clone(&self) -> Self {
            RedToken(())
        }
    }

    impl Copy for RedToken {}

    impl ProofToken for RedToken {
        type Proposition = Red;
    }

    /// Sanitized mirror of `amenable_kani::Established<T, Token>`
    /// — genuinely generic and genuinely implements `Sidecar<CreusotVerifier>`,
    /// unlike an earlier, flattened version of this file. `amenable_derive::
    /// Sidecar` generates the same impl `amenable_kani::stoplight` now
    /// derives too, rather than hand-writing a second copy of the identical
    /// shape. Gated along with the rest of this module (`mirror`'s own
    /// `#[cfg(creusot)]`): `cfg`-stripping runs before derive expansion,
    /// so the derive is never even invoked outside a real Creusot build.
    #[derive(amenable_derive::Sidecar)]
    #[sidecar(verifier = "CreusotVerifier", constructor = "pub(super)")]
    pub struct Established<T, Token> {
        // Fully `pub`, not private -- see `ledger::Transfer`'s own fields'
        // doc comment for the real "less-visible item" transparency error
        // this avoids: `Sidecar::primary()` (a `pub` trait method) is more
        // visible than `new`'s own private constructor here, so even though
        // both derive-generated `ensures` clauses reach these fields, the
        // *more* visible one (`primary()`) is what sets the real floor.
        #[sidecar(primary)]
        pub primary: T,
        #[sidecar(token)]
        token: Token,
    }

    impl Establish<GreenToken, CreusotVerifier> for Yellow {
        type Token = YellowToken;

        fn establish(_credential: GreenToken) -> Self::Token {
            YellowToken(())
        }
    }

    impl Establish<YellowToken, CreusotVerifier> for Red {
        type Token = RedToken;

        fn establish(_credential: YellowToken) -> Self::Token {
            RedToken(())
        }
    }

    impl Establish<RedToken, CreusotVerifier> for Green {
        type Token = GreenToken;

        fn establish(_credential: RedToken) -> Self::Token {
            GreenToken(())
        }
    }

    /// Sanitized mirror of `amenable_kani::stoplight::Stoplight` — needed
    /// only because the generated companions below (`GAAP_LEDGER_PLAN.md`'s
    /// Step 6) now wrap each edge's captured body in a real `impl Stoplight
    /// { fn ..(&self, ..) }`, not a bare free function: `Ledger::validate`'s
    /// own real body was the first captured body to reference `self`/`Self`
    /// at all, forcing `amenable::creusot_export`'s own generator to add a
    /// real receiver, uniformly, to every edge it generates -- `Stoplight`'s
    /// own three edges never use `self` inside their bodies, but still need
    /// a real type to receive it now that the wrapper is unconditional.
    ///
    /// `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4: the Creusot half of
    /// the same canary `amenable_kani::Stoplight` already carries. Works
    /// unmodified because `amenable::creusot_export` (this file's own
    /// generated companions, below) now emits a real, concrete `impl
    /// amenable_core::Exchange<Input, Output, CreusotVerifier> for Stoplight`
    /// per edge, not just a same-named inherent method — the exact gap Step
    /// 5 already closed for `capture_exchange_body`, closed here for this
    /// generator instead. `#[derive(..)]` is gated by this module's own
    /// `#[cfg(creusot)]` -- cfg-stripping removes the whole module, derive
    /// included, before any macro runs, so no separate `cfg_attr` is
    /// needed.
    #[derive(amenable_derive::StateMachine)]
    #[state_machine(
        verifier = "CreusotVerifier",
        translator_cfg = "creusot",
        state("Green", "Established<Green, GreenToken>"),
        state("Yellow", "Established<Yellow, YellowToken>"),
        state("Red", "Established<Red, RedToken>"),
        edge("Green", "Yellow"),
        edge("Yellow", "Red"),
        edge("Red", "Green")
    )]
    pub struct Stoplight;

    /// Sanitized mirror of `amenable_kani::StoplightError` — a
    /// real, ordinary, constructible type, matching the real one's own
    /// justification exactly: an uninhabited error type (`std::convert::
    /// Infallible`) is incompatible with the same class of reconstruction
    /// concern (Kani's `stub_verified` needs `Arbitrary`; here, the
    /// generated body's own `Ok(..)` still needs *some* concrete `Err` type
    /// to name in its signature, even though no generated edge ever
    /// constructs one). Needed only because the generated bodies below are
    /// the real Kani bodies' own verbatim text, `Ok(..)` wrapper included —
    /// an earlier, hand-written version of this file simplified the return
    /// type to the bare `Ok` payload and dropped the wrapper entirely, which
    /// a *generated, unmodified* body can no longer do without becoming a
    /// silently different claim than the one actually captured.
    #[derive(Debug, Clone, Copy)]
    pub enum StoplightError {
        /// The one variant. Exists so `StoplightError` is an ordinary
        /// constructible type, not so any edge below constructs it.
        NotUsed,
    }
}
// Split by external reach, not by kind: `lib.rs` re-exports these seven
// names at the crate's own public boundary, so they need full `pub` here
// too (a re-export can never be more visible than what it re-exports);
// everything else only needs to reach the three `include!`d files below,
// so `pub(crate)` is enough.
#[cfg(creusot)]
pub(crate) use mirror::{
    CreusotVerifier, Establish, Sidecar, Stoplight, StoplightError, ensures, requires,
};
#[cfg(creusot)]
pub use mirror::{Established, Green, GreenToken, Red, RedToken, Yellow, YellowToken};

// The three per-edge `harness! { .. }` blocks (transition body + verbatim
// source constant) plus their `#[cfg(not(creusot))]`-gated `ProofRecord`
// registrations -- generated by `amenable emit-creusot-companions` from
// `amenable_core::ExchangeEdgeRecord`, not hand-written. See this file's
// own doc comment for the full mechanism and why hand-copying was
// dropped. `include!`, not `mod`: these share this file's own scope
// directly (`Green`/`Yellow`/`Established`/`Stoplight`/`StoplightError`
// above, already in scope via `mirror`'s own re-export), no `use
// super::*;`/explicit imports needed in the generated files themselves.
// Regenerate with `just generate-creusot` after changing a real Kani-side
// transition; do not hand-edit the included files.
include!("generated/green_to_yellow.rs");
include!("generated/yellow_to_red.rs");
include!("generated/red_to_green.rs");

/// Real `Witness<CreusotVerifier>` bridges for the *real* `amenable_core::
/// stoplight::{Green, Yellow, Red}` -- audit/registry only, no
/// construction involved, so (unlike the mirror module above) these don't
/// need their own accommodation model at all. `amenable_core`, not
/// `amenable_kani`: these markers moved there specifically so this file
/// could implement `Witness<CreusotVerifier>` on the real types without a
/// backend-to-backend Cargo edge -- see this file's own doc comment and
/// `amenable_core::stoplight`'s doc comment for the full rationale. The
/// mirror above still exists alongside these (the real transition bodies
/// still need private constructors this crate can't reach).
///
/// `#[cfg(not(creusot))]` on this whole module, matching `ledger.rs`'s own
/// `Witness` bridges: the real `MultiCheckProof` artifact carries `Vec`/
/// `String`/`Display` machinery that must never be *local* during actual
/// translation (confirmed the hard way -- see that type's own doc
/// comment). Fully-qualified `amenable_core::` paths throughout, not a
/// `use`, so these names never collide with `mirror`'s own re-exported
/// `Green`/`Yellow`/`Red` above -- both exist in the same source, gated to
/// mutually exclusive `cfg(creusot)` branches, so there is never a real
/// naming conflict at any single compilation. `super::VERIFY_*_SRC`
/// reaches the `harness!`-generated source constants the `include!`s
/// above define at this file's own top level -- always visible regardless
/// of `cfg(creusot)`, so reachable from this module in every
/// configuration it itself compiles under.
#[cfg(not(creusot))]
mod real_witness {
    use crate::CreusotVerifier;

    impl amenable_core::Witness<CreusotVerifier> for amenable_core::Yellow {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "green_to_yellow".to_owned(),
                super::VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC.to_owned(),
            )])
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_core::Yellow",
            "creusot",
            || <amenable_core::Yellow as amenable_core::Witness<CreusotVerifier>>::proof().to_string(),
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for amenable_core::Red {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "yellow_to_red".to_owned(),
                super::VERIFY_YELLOW_TO_RED_EXCHANGE_SRC.to_owned(),
            )])
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_core::Red",
            "creusot",
            || <amenable_core::Red as amenable_core::Witness<CreusotVerifier>>::proof().to_string(),
        )
    }

    /// `Green` is a root claim (see `amenable_core::Green`'s own
    /// doc comment), but this workspace's `Stoplight` cycles back to it too
    /// (`red_to_green`), so it still has a real Creusot proof backing the
    /// cycle-back edge -- named `"cycle_back"` matching the real Kani-side
    /// `ProofRecord`'s own `evidence_id` suffix convention for the exact
    /// same distinction (the *root* claim itself stays asserted, not
    /// proven; this is the separate claim that `Red` lawfully cycles back).
    impl amenable_core::Witness<CreusotVerifier> for amenable_core::Green {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "red_to_green".to_owned(),
                super::VERIFY_RED_TO_GREEN_EXCHANGE_SRC.to_owned(),
            )])
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_core::Green::cycle_back",
            "creusot",
            || <amenable_core::Green as amenable_core::Witness<CreusotVerifier>>::proof().to_string(),
        )
    }
}
