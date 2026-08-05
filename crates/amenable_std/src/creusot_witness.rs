//! `CreusotWitness` impls for Rust standard-library carriers.
//!
//! This lives here, not alongside the proof functions in `amenable_creusot`,
//! because `creusot-rustc`'s translator sweeps every local item in a
//! `creusot-std`-dependent crate — including ones no `#[cfg(creusot)]` gate
//! protects, since Rust items don't need to *run* to be enumerated. Ordinary
//! infrastructure that's completely unremarkable to plain `rustc` turned out
//! to be unsupported there: a return-position `impl Trait` on a local `impl`
//! panicked its intrinsics-gathering pass outright (a real ICE, not a
//! hypothetical), and the `static` item `::inventory::submit!` expands to
//! hit "unsupported definition kind ... Static" the same way. So
//! `amenable_creusot` stays pure Pearlite proof-function content — the thing
//! `cargo creusot -- -p amenable_creusot` actually needs to translate — and
//! everything else about *finding* those proofs (the witness bridge, the
//! registry) moves to the crate that already owns the types being proved
//! about.
//!
//! This is legal under Rust's orphan rule for a reason distinct from every
//! other verifier backend's bridge (see `amenable_creusot::witness`'s own
//! doc comment for the usual justification: the verifier marker type is
//! local): here, `RustStdStandard<T>` — the `Self` type — is local to this
//! crate instead. The orphan rule only requires *one* of {trait, Self type,
//! trait's own type parameters} to be local to the implementing crate, not
//! any particular one, and a real 3-crate test confirmed a Self-type-local
//! justification compiles exactly the same as a trait-parameter-local one.
//!
//! One block per concrete type: a Creusot-checkable property doesn't
//! generalize across types the way provenance does, so there is no blanket
//! impl here — each type gets exactly the contract that's actually true of
//! it. The bridge to `Witness<CreusotVerifier>` is mechanical (delegates
//! straight to `CreusotWitness`), so it's generated per type by a macro
//! rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Creusot to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! contract content to add. `char` and `String` do carry a genuine
//! constraint, so their `proof()` also names the Creusot contract function
//! that checks it, alongside the same chain-derived provenance.
//!
//! Each type also registers a [`amenable_core::ProofRecord`] alongside its
//! `Witness` bridge, so `proof()`'s output is discoverable by name for
//! audit — see `amenable_core::chain::proof_chain`. The registered
//! `evidence` name is a hardcoded module-path literal matching
//! `RustStdStandard`'s own registration in `rust_std`, so both sides agree
//! on the same string without one computing it from the other.
//!
//! A "checked" carrier's [`CheckedProof::claim`] is the contract's own
//! verbatim source (`#[requires]`/`#[ensures]` included), captured via
//! [`amenable_derive::harness!`] *in `amenable_creusot`* — this crate only
//! imports the resulting `&'static str` constant, never the harness
//! function itself, so the claim can never drift from the real contract
//! without also touching the crate that's actually translated.

use std::num::{NonZero, Saturating, Wrapping};
use std::time::{Duration, TryFromFloatSecsError};

use amenable_core::{Evidence, Provenance, Witness};
use amenable_creusot::{
    CreusotVerifier, CreusotWitness, VERIFY_CHAR_ROUNDTRIP_SRC,
    VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
    VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
    VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, VERIFY_STRING_ROUNDTRIP_SRC,
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
};

use crate::{RustStdProvenance, RustStdStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}

macro_rules! impl_creusot_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreusotWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_creusot_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "creusot",
                    describe: || <RustStdStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_creusot_witness_trusted!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    TryFromFloatSecsError
);

/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    pub claim: &'static str,
    /// The chain-derived provenance this claim still rests on.
    pub provenance: RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

impl CreusotWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        describe: || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_roundtrip",
            claim: VERIFY_STRING_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        describe: || <RustStdStandard<String> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_duration_new_normalizes_nanos_and_carries_into_secs",
            claim: VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Duration>",
        verifier: "creusot",
        describe: || <RustStdStandard<Duration> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i16_roundtrips",
            claim: VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "creusot",
        describe: || <RustStdStandard<NonZero<i16>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_kani::rust_std::cmp` and
// `amenable_std::rust_std::cmp`'s own registration exactly: there's also
// a `core::sync::atomic::Ordering`, so the evidence string must say
// `std::cmp::Ordering`, not the bare name, or alias resolution won't
// match this proof to the checklist row.
impl CreusotWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ordering_reverse_swaps_less_and_greater",
            claim: VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        verifier: "creusot",
        describe: || <RustStdStandard<std::cmp::Ordering> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_wrapping_i32_add_wraps",
            claim: VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Wrapping<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_saturating_i32_add_clamps",
            claim: VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Saturating<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::IntErrorKind, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::IntErrorKind>`).
impl CreusotWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_int_error_kind_classifies_parse_failures",
            claim: VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::num::IntErrorKind> as CreusotWitness>::proof().to_string(),
    }
}
