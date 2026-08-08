//! The two `KaniWitness` dispositions every Rust standard-library carrier
//! falls into, plus the shared [`CheckedProof`] artifact type for the
//! "checked" case.

use amenable_core::Provenance;
use amenable_std::RustStdProvenance;

/// Mechanical bridge from `KaniWitness` to `Witness<KaniVerifier>` — every
/// type needs one, and it never varies, so it's generated rather than
/// hand-repeated per type.
macro_rules! bridge_kani_witness {
    ($ty:ty) => {
        impl amenable_core::Witness<crate::KaniVerifier> for $ty {
            type SupportingEvidence = <$ty as crate::KaniWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as crate::KaniWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as crate::KaniWitness>::proof()
            }
        }
    };
}

/// Register a `RustStdStandard<$ty>` whose `KaniWitness::proof()` is
/// "trusted": most carriers have no invariant beyond what the type system
/// already guarantees (every bit pattern of an `i8` is a valid `i8`), so
/// there is nothing for Kani to check. `proof()` returns the chain-derived
/// provenance reached through `SupportingEvidence::basis().audit()` and
/// nothing more — not a special case, just what a `proof()` implementation
/// looks like when there's no harness content to add.
///
/// Every trait method here is called through its fully-qualified path
/// (`Trait::method(&x)`, not `x.method()`) rather than relying on `use`
/// imports: dot-call method resolution needs the trait in scope wherever a
/// macro expands, which for `macro_rules!` is the *call site*, not this
/// definition — so a dot-call here would silently demand
/// `amenable_core::{Evidence, Provenance}` imports from every caller.
/// Fully-qualified calls carry their own trait path and need no import
/// anywhere, keeping this macro genuinely self-contained.
macro_rules! impl_kani_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl crate::KaniWitness for amenable_std::RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = amenable_std::RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    amenable_core::Evidence::audit(
                        &<Self::SupportingEvidence as amenable_core::Evidence>::basis(),
                    )
                }
            }

            crate::rust_std::macros::bridge_kani_witness!(amenable_std::RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "kani",
                    describe: || amenable_core::Provenance::report(
                        &<amenable_std::RustStdStandard<$ty> as crate::KaniWitness>::proof(),
                    ).to_string(),
                }
            }
        )*
    };
}

/// Defines a contract type's `Requires<KaniVerifier>` impl as the real,
/// callable check — `Bound = bool`, `Input` the value being checked. The
/// proof site calls `$ty::requires(x)` directly; that call *is* the bound,
/// not a restatement of it living somewhere else. A `ContractRecord` is
/// still registered (via `stringify!` on the identical expression) purely
/// as a derived audit artifact for `dump-registry` enumeration — it does
/// not compete with `requires()` as a second definition, since nothing
/// reads it back to check anything.
macro_rules! kani_requires {
    ($ty:ty, $evidence:literal, $param_ty:ty, |$param:pat_param| $expr:expr) => {
        impl amenable_core::Requires<crate::KaniVerifier> for $ty {
            type Input = $param_ty;
            type Bound = bool;

            fn requires($param: $param_ty) -> bool {
                $expr
            }
        }

        ::inventory::submit! {
            ::amenable_core::ContractRecord {
                evidence: $evidence,
                verifier: "kani",
                kind: "requires",
                fragment: || stringify!($expr),
                harnesses: &[],
            }
        }
    };
}

/// The `Ensures` counterpart of [`kani_requires`] — same one-source
/// guarantee, for a postcondition instead of a precondition.
macro_rules! kani_ensures {
    ($ty:ty, $evidence:literal, $param_ty:ty, |$param:pat_param| $expr:expr) => {
        impl amenable_core::Ensures<crate::KaniVerifier> for $ty {
            type Input = $param_ty;
            type Bound = bool;

            fn ensures($param: $param_ty) -> bool {
                $expr
            }
        }

        ::inventory::submit! {
            ::amenable_core::ContractRecord {
                evidence: $evidence,
                verifier: "kani",
                kind: "ensures",
                fragment: || stringify!($expr),
                harnesses: &[],
            }
        }
    };
}

pub(crate) use {bridge_kani_witness, impl_kani_witness_trusted, kani_ensures, kani_requires};

/// Proof artifact for a carrier with a real, machine-checked Kani harness:
/// names the harness, carries its verbatim source as `claim`, and still
/// rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Kani harness that checks this carrier's invariant.
    pub harness: String,
    /// The harness's own source — what it actually asserts, verbatim.
    pub claim: String,
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
