//! Default concrete certificate and registry implementation.

use std::fmt::{self, Display, Formatter};

use amenable_core::{Certificate, MetadataEntry, Provenance, Registry};

/// Stable identifier for an issued certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CertId {
    value: u64,
}

impl CertId {
    /// Create a new certificate identifier.
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Return the underlying numeric identifier.
    pub const fn value(&self) -> u64 {
        self.value
    }
}

impl Display for CertId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Concrete provenance certificate issued by the default registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceCertificate {
    id: CertId,
    subject: String,
    entries: Vec<MetadataEntry>,
}

impl ProvenanceCertificate {
    fn new(id: CertId, subject: impl Display, entries: Vec<MetadataEntry>) -> Self {
        Self {
            id,
            subject: subject.to_string(),
            entries,
        }
    }
}

impl Certificate for ProvenanceCertificate {
    type Id = CertId;

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

impl Display for ProvenanceCertificate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Provenance certificate {} for {}", self.id, self.subject)?;

        for entry in &self.entries {
            write!(f, "\n{entry}")?;
        }

        Ok(())
    }
}

/// Default tracked certificate registry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CertRegistry {
    next_id: u64,
    issued: Vec<ProvenanceCertificate>,
}

impl CertRegistry {
    /// Create an empty certificate registry.
    pub const fn new() -> Self {
        Self {
            next_id: 1,
            issued: Vec::new(),
        }
    }
}

impl Registry for CertRegistry {
    type Certificate = ProvenanceCertificate;

    fn issue_provenance_certificate<P>(
        &mut self,
        subject: impl Display,
        provenance: &P,
    ) -> Self::Certificate
    where
        P: Provenance + ?Sized,
    {
        let certificate = ProvenanceCertificate::new(
            CertId::new(self.next_id),
            subject,
            provenance.metadata().collect(),
        );

        self.next_id += 1;
        self.issued.push(certificate.clone());

        certificate
    }

    fn issued(&self) -> impl Iterator<Item = &Self::Certificate> {
        self.issued.iter()
    }
}
