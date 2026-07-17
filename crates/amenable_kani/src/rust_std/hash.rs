//! `KaniWitness` impls for `core::hash`.

use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<BuildHasherDefault<DefaultHasher>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_build_hasher_default_produces_consistent_hashers",
            claim: VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BuildHasherDefault<DefaultHasher>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BuildHasherDefault<DefaultHasher>>",
        verifier: "kani",
        describe: || <RustStdStandard<BuildHasherDefault<DefaultHasher>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC, {
        /// `BuildHasherDefault` fulfils `BuildHasher`'s core contract:
        /// two hashers it constructs (via `H::default()` each time)
        /// hash the same input to the same output.
        #[kani::proof]
        fn verify_build_hasher_default_produces_consistent_hashers() {
            use std::hash::{BuildHasher, Hasher};

            let value: u64 = kani::any();
            let builder = BuildHasherDefault::<DefaultHasher>::default();
            let mut h1 = builder.build_hasher();
            let mut h2 = builder.build_hasher();
            h1.write_u64(value);
            h2.write_u64(value);
            assert_eq!(
                h1.finish(),
                h2.finish(),
                "BuildHasherDefault produces hashers with consistent, deterministic output"
            );
        }
    }
}

// `SipHasher` is deprecated (in favor of `DefaultHasher`) but still stable
// and real; covering it is a coverage-completeness question, not a call
// to use it. `#[expect(deprecated)]` on a macro invocation is silently
// ignored by rustc rather than suppressing warnings from inside its
// expansion (confirmed empirically in the core::str batch's LinesAny
// proof), so `bridge_kani_witness!`/`inventory::submit!` below would
// otherwise warn on every mention. Routing through this locally
// `#[expect(deprecated)]`-attributed alias hides the deprecation at every
// downstream use site instead.
#[expect(
    deprecated,
    reason = "SipHasher itself is stable, only deprecated as a recommendation to use DefaultHasher; covering it is a coverage-completeness question, not a call to use it"
)]
type SipHasherAlias = std::hash::SipHasher;

impl KaniWitness for RustStdStandard<SipHasherAlias> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sip_hasher_produces_consistent_hashes",
            claim: VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SipHasherAlias>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SipHasher>",
        verifier: "kani",
        describe: || <RustStdStandard<SipHasherAlias> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC, {
        /// Same determinism contract as `BuildHasherDefault`: two
        /// freshly constructed `SipHasher`s hash the same input to the
        /// same output.
        #[expect(
            deprecated,
            reason = "SipHasher itself is stable, only deprecated as a recommendation to use DefaultHasher; covering it is a coverage-completeness question, not a call to use it"
        )]
        #[kani::proof]
        fn verify_sip_hasher_produces_consistent_hashes() {
            use std::hash::Hasher;

            let value: u64 = kani::any();
            let mut h1 = std::hash::SipHasher::new();
            let mut h2 = std::hash::SipHasher::new();
            h1.write_u64(value);
            h2.write_u64(value);
            assert_eq!(
                h1.finish(),
                h2.finish(),
                "SipHasher produces consistent, deterministic output"
            );
        }
    }
}
