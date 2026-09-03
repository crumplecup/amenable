//! The Verus verifier, the `VerusWitness` trait, the
//! `bridge_verus_witness!`/`impl_verus_witness_trusted!` macros every
//! other file in this module invokes, and the `VerusCallShape` family
//! (structural call shapes a compositional renderer uses to emit a
//! literal call to, or citation of, a real Verus harness instead of
//! assuming its conclusion).

use amenable_core::{
    Evidence, MetadataEntry, Provenance, Verifier, WitnessArtifact, WitnessArtifactNode,
    WitnessSupportKind, WitnessSupportSummary,
};

use crate::{RustStdProvenance, RustStdStandard};

/// Verus-specific witness: identifies the Verus spec (if any) behind a
/// piece of evidence, without ever running it.
pub trait VerusWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Verus proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Verus proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}

pub(crate) trait VerusProofArtifactSupport {
    fn support() -> WitnessSupportSummary;
}

/// The Verus verifier, local to this crate: there is only one verifier
/// Verus works with — Verus. Being local here (not imported from
/// `amenable_core`) is what makes the per-type bridges below legal under
/// Rust's orphan rule — a blanket bridge over a bare type parameter is
/// not: the orphan rule requires every uncovered generic parameter to be
/// covered before the first local type, and `Self` in a blanket impl
/// never is.
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "verus"),
            ("authority", "Verus project"),
            ("source_url", "https://verus-lang.github.io/verus/"),
            ("proof_artifact", "Verus proof module token stream"),
            (
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            (
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ];
        FACTS
            .iter()
            .map(|&(k, v)| MetadataEntry::new(k, v))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "verus"
    }
}

/// Register explicit Verus witness exports for concrete instantiated
/// types.
///
/// Verus compiles a separate source tree and cannot discover every
/// derived witness automatically. This macro records the concrete types a
/// crate wants to materialize in that separate Verus pipeline while
/// keeping the registration itself in ordinary Rust macro expansion.
#[macro_export]
macro_rules! emit_verus_witnesses {
    ($($ty:ty),* $(,)?) => {
        ::amenable_core::register_witness_exports!(
            verifier = $crate::VerusVerifier;
            $($ty),*
        );
    };
}

// Every path below is fully qualified, not a bare name relying on
// hygienic definition-site resolution: this macro is invoked from every
// other file in this module (not just this one, the way it was before
// the split into a directory), and a plain `macro_rules!` macro
// re-exported via `pub(..) use` does not carry its defining module's
// unqualified names to the call site the way `#[macro_export]`'s own
// `$crate`-qualified convention already assumes -- confirmed empirically
// while doing this split: bare `VerusVerifier`/`Witness`/`ClassifiedWitness`/
// `WitnessSupportSummary`/`VerusProofArtifactSupport` all failed to
// resolve from any call site outside this file.
macro_rules! bridge_verus_witness {
    ($ty:ty) => {
        impl ::amenable_core::Witness<$crate::VerusVerifier> for $ty {
            type SupportingEvidence = <$ty as $crate::VerusWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as $crate::VerusWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as $crate::VerusWitness>::proof()
            }

            fn support() -> ::amenable_core::WitnessSupportSummary {
                <<$ty as $crate::VerusWitness>::ProofArtifact as $crate::verus_witness::machinery::VerusProofArtifactSupport>::support()
            }
        }

        // Every VerusWitness-bridged type resolves to a real VerusCheckedProof
        // or RustStdProvenance artifact -- both genuinely classified (Checked
        // or Trusted respectively, see VerusProofArtifactSupport below), never
        // the Witness::support() default (Opaque) -- so this impl is always
        // sound to add unconditionally alongside the bridge.
        impl ::amenable_core::ClassifiedWitness<$crate::VerusVerifier> for $ty {}
    };
}
// Every other file in this module invokes this macro on its own leaf
// types, so it needs to be path-addressable outside its own textual
// scope -- the same `pub(..) use` mechanism every other cross-file macro
// in this crate already uses for the identical reason.
pub(super) use bridge_verus_witness;

macro_rules! impl_verus_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_verus_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    "verus",
                    || <RustStdStandard<$ty> as VerusWitness>::proof().report().to_string(),
                )
            }
        )*
    };
}

impl_verus_witness_trusted!(
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
    std::convert::Infallible,
    core::ffi::c_void,
    std::cell::BorrowError,
    std::cell::BorrowMutError,
    std::fmt::Error,
    std::marker::PhantomData<i32>,
    std::marker::PhantomPinned,
    std::net::AddrParseError,
    std::str::ParseBoolError,
    std::io::Stderr,
    std::io::StderrLock<'static>,
    std::io::Stdin,
    std::io::StdinLock<'static>,
    std::io::Stdout,
    std::io::StdoutLock<'static>,
    std::process::ExitCode,
    std::thread::AccessError,
    std::thread::Builder,
    std::thread::JoinHandle<i32>,
    std::thread::Scope<'static, 'static>,
    std::thread::ScopedJoinHandle<'static, i32>,
    std::env::VarError,
    std::env::Vars,
    std::env::VarsOs,
    std::task::RawWaker,
    std::task::RawWakerVTable,
    core::panic::PanicInfo<'static>,
    core::panic::PanicMessage<'static>,
    core::time::TryFromFloatSecsError,
    ()
);

impl VerusProofArtifactSupport for RustStdProvenance {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trusted_leaf()
    }
}

impl WitnessArtifact for RustStdProvenance {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Trusted,
            WitnessSupportSummary::trusted_leaf(),
            self.report().to_string(),
            self.metadata(),
        )
    }
}

/// Proof artifact for a carrier with a real, machine-checked Verus spec:
/// names the spec function, carries its verbatim source as `claim`, and
/// still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct VerusCheckedProof {
    /// The Verus spec function that checks this carrier's invariant.
    harness: String,
    /// The spec's own source — the whole file it lives in, verbatim
    /// (`include_str!`, not a per-function extraction — Verus proof files
    /// in `amenable_verus` are kept to one carrier's spec function(s) each
    /// so this stays a tight, accurate claim, the same one-claim-per-
    /// carrier granularity `amenable_derive::harness!` gives Kani/Creusot
    /// by capturing one function at a time).
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: RustStdProvenance,
}

impl VerusProofArtifactSupport for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl WitnessArtifact for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Checked,
            WitnessSupportSummary::checked_leaf(),
            format!("harness: {}", self.harness),
            [
                MetadataEntry::new("verifier", VerusVerifier::name()),
                MetadataEntry::new("harness", self.harness.clone()),
                MetadataEntry::new("claim", self.claim.clone()),
            ],
        )
    }
}

impl std::fmt::Display for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}
