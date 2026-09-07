use amenable_core::{
    Certificate, ErasedEntry, Metadata, MetadataEntry, OwnedEntry, Provenance, Registry, Standard,
    Verifier,
};
use amenable_derive::Standard;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ManualProvenance {
    authority_kind: String,
    authority: String,
    source: String,
}

impl Metadata for ManualProvenance {
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("authority_kind", self.authority_kind.clone()),
            OwnedEntry::new("authority", self.authority.clone()),
            OwnedEntry::new("source", self.source.clone()),
        ]
    }
}

impl Provenance for ManualProvenance {}

#[derive(Debug, Clone, PartialEq, Eq, Default, Standard)]
#[standard(
    basis = "Self",
    provenance = "self.provenance.clone()",
    provenance_type = "ManualProvenance"
)]
struct ManualStandard {
    provenance: ManualProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ManualCertId(u64);

impl Display for ManualCertId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManualCertificate {
    id: ManualCertId,
    subject: String,
    entries: Vec<MetadataEntry>,
}

impl Certificate for ManualCertificate {
    type Id = ManualCertId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn subject(&self) -> &str {
        &self.subject
    }

    fn entries(&self) -> impl Iterator<Item = &MetadataEntry> {
        self.entries.iter()
    }
}

impl Display for ManualCertificate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Provenance certificate {} for {}", self.id, self.subject)?;

        for entry in &self.entries {
            write!(f, "\n{entry}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ManualRegistry {
    next_id: u64,
    issued: Vec<ManualCertificate>,
}

impl ManualRegistry {
    fn new() -> Self {
        Self {
            next_id: 1,
            issued: Vec::new(),
        }
    }
}

impl Registry for ManualRegistry {
    type Certificate = ManualCertificate;

    fn issue_provenance_certificate<P>(
        &mut self,
        subject: impl Display,
        provenance: &P,
    ) -> Self::Certificate
    where
        P: Provenance + ?Sized,
    {
        let certificate = ManualCertificate {
            id: ManualCertId(self.next_id),
            subject: subject.to_string(),
            entries: provenance
                .snapshot()
                .iter()
                .map(MetadataEntry::from)
                .collect(),
        };

        self.next_id += 1;
        self.issued.push(certificate.clone());

        certificate
    }

    fn issued(&self) -> impl Iterator<Item = &Self::Certificate> {
        self.issued.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct ManualVerifierMetadata;

impl Metadata for ManualVerifierMetadata {
    fn snapshot(&self) -> Vec<OwnedEntry> {
        const FACTS: &[(&str, &str)] =
            &[("verifier_family", "manual"), ("authority", "Test Fixture")];
        FACTS
            .iter()
            .map(|&(key, value)| OwnedEntry::new(key, value))
            .collect()
    }
}

impl Provenance for ManualVerifierMetadata {}

struct ManualVerifier;

impl Verifier for ManualVerifier {
    type Metadata = ManualVerifierMetadata;

    fn name() -> &'static str {
        "manual"
    }
}

#[test]
fn verifier_metadata_marker_is_zero_sized() {
    amenable_core::init_tracing();
    assert_eq!(std::mem::size_of::<ManualVerifierMetadata>(), 0);
}

#[test]
fn verifier_metadata_builds_entries_on_demand() -> miette::Result<()> {
    amenable_core::init_tracing();
    let metadata = ManualVerifier::metadata();

    assert!(!metadata.is_empty());
    assert_eq!(metadata.len(), metadata.snapshot().len());

    let entry = metadata
        .get("verifier_family")
        .ok_or_else(|| miette::miette!("verifier_family fact present"))?;
    assert_eq!(entry.value().to_string(), "manual");

    assert!(metadata.contains_key("authority"));
    assert!(!metadata.contains_key("nonexistent_key"));
    assert!(metadata.get("nonexistent_key").is_none());
    Ok(())
}

#[test]
fn provenance_exposes_rich_projected_metadata_views() {
    amenable_core::init_tracing();
    let provenance = ManualProvenance {
        authority_kind: "local_design".to_string(),
        authority: "UI Working Group".to_string(),
        source: "layout/decision-12".to_string(),
    };

    assert_eq!(
        provenance
            .keys()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["authority_kind", "authority", "source"]
    );
    assert_eq!(
        provenance
            .values()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["local_design", "UI Working Group", "layout/decision-12"]
    );
}

#[test]
fn provenance_report_renders_metadata_as_human_readable_lines() {
    amenable_core::init_tracing();
    let provenance = ManualProvenance {
        authority_kind: "external_standard".to_string(),
        authority: "Rust Project Developers".to_string(),
        source: "https://doc.rust-lang.org/std/primitive.i32.html".to_string(),
    };

    assert_eq!(
        provenance.report().to_string(),
        "authority_kind: external_standard\n\
authority: Rust Project Developers\n\
source: https://doc.rust-lang.org/std/primitive.i32.html"
    );
}

#[test]
fn registry_issues_standard_certificates_through_the_standard_surface() {
    amenable_core::init_tracing();
    let mut registry = ManualRegistry::new();
    let standard = ManualStandard {
        provenance: ManualProvenance {
            authority_kind: "external_standard".to_string(),
            authority: "Rust Project Developers".to_string(),
            source: "https://doc.rust-lang.org/std/primitive.i32.html".to_string(),
        },
    };

    assert_eq!(
        standard.certification(&mut registry, "i32").to_string(),
        "Provenance certificate 1 for i32\n\
authority_kind: external_standard\n\
authority: Rust Project Developers\n\
source: https://doc.rust-lang.org/std/primitive.i32.html"
    );
    assert_eq!(registry.len(), 1);
}

#[test]
fn registry_issues_and_tracks_provenance_certificates() {
    amenable_core::init_tracing();
    let mut registry = ManualRegistry::new();
    let provenance = ManualProvenance {
        authority_kind: "external_standard".to_string(),
        authority: "Rust Project Developers".to_string(),
        source: "https://doc.rust-lang.org/std/primitive.i32.html".to_string(),
    };
    let certificate = provenance.certification(&mut registry, "i32");

    assert_eq!(certificate.id(), &ManualCertId(1));
    assert_eq!(certificate.subject(), "i32");
    assert_eq!(
        certificate.to_string(),
        "Provenance certificate 1 for i32\n\
authority_kind: external_standard\n\
authority: Rust Project Developers\n\
source: https://doc.rust-lang.org/std/primitive.i32.html"
    );
    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry.report().to_string(),
        "Provenance certificate 1 for i32\n\
authority_kind: external_standard\n\
authority: Rust Project Developers\n\
source: https://doc.rust-lang.org/std/primitive.i32.html"
    );
}
