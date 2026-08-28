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
///
/// `new` stays private (`#[new(visibility = "")]`): the derive can't
/// replicate the original `subject: impl Display` parameter (`#[new(
/// into)]` only widens to `impl Into<String>`, a different bound), so
/// [`CertRegistry::issue_provenance_certificate`] converts via
/// `.to_string()` before calling `new` instead.
#[derive(Debug, Clone, PartialEq, Eq, derive_new::new)]
#[new(visibility = "")]
pub struct ProvenanceCertificate {
    id: CertId,
    #[new(into)]
    subject: String,
    entries: Vec<MetadataEntry>,
}

impl Certificate for ProvenanceCertificate {
    type Id = CertId;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn id(&self) -> &Self::Id {
        &self.id
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn subject(&self) -> &str {
        &self.subject
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn entries(&self) -> impl Iterator<Item = &MetadataEntry> {
        self.entries.iter()
    }
}

impl Display for ProvenanceCertificate {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
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

    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(self, subject, provenance))
    )]
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
            subject.to_string(),
            provenance.metadata().collect(),
        );

        self.next_id += 1;
        self.issued.push(certificate.clone());

        certificate
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn issued(&self) -> impl Iterator<Item = &Self::Certificate> {
        self.issued.iter()
    }
}
