//! Certificate and registry roles for inspectable audit artifacts.

use std::fmt::{self, Display, Formatter};

use crate::{MetadataEntry, Provenance, Standard};

/// Issued audit artifact describing a certified trust claim.
pub trait Certificate: Display {
    /// Stable identifier type for this certificate family.
    type Id: Display;

    /// Borrow the certificate identifier.
    fn id(&self) -> &Self::Id;

    /// Borrow the certified subject name.
    fn subject(&self) -> &str;

    /// Iterate over the snapshotted metadata entries carried by this certificate.
    fn entries(&self) -> impl Iterator<Item = &MetadataEntry>;

    /// Look up a metadata entry by key.
    fn get(&self, key: &str) -> Option<&MetadataEntry> {
        self.entries().find(|entry| entry.key() == key)
    }

    /// Return whether this certificate carries a metadata entry with the given key.
    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Count the metadata entries snapshotted into this certificate.
    fn len(&self) -> usize {
        self.entries().count()
    }

    /// Return whether this certificate carries no metadata entries.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Issuing and tracking authority for audit certificates.
pub trait Registry {
    /// Concrete certificate family issued by this registry.
    type Certificate: Certificate;

    /// Issue a certificate from an explicit provenance record.
    fn issue_provenance_certificate<P>(
        &mut self,
        subject: impl Display,
        provenance: &P,
    ) -> Self::Certificate
    where
        P: Provenance + ?Sized;

    /// Issue a certificate from a provenance-backed standard.
    fn issue_standard_certificate<S>(
        &mut self,
        subject: impl Display,
        standard: &S,
    ) -> Self::Certificate
    where
        S: Standard + ?Sized,
    {
        self.issue_provenance_certificate(subject, &standard.provenance())
    }

    /// Iterate over every certificate this registry has issued.
    fn issued(&self) -> impl Iterator<Item = &Self::Certificate>;

    /// Number of issued certificates tracked by this registry.
    fn len(&self) -> usize {
        self.issued().count()
    }

    /// Return whether this registry has issued no certificates.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrow a generic report over every issued certificate.
    fn report(&self) -> RegistryReport<'_, Self>
    where
        Self: Sized,
    {
        RegistryReport::new(self)
    }
}

/// Borrowed report over every certificate issued by a registry.
#[derive(Debug, Clone, Copy)]
pub struct RegistryReport<'a, R>
where
    R: Registry + ?Sized,
{
    registry: &'a R,
}

impl<'a, R> RegistryReport<'a, R>
where
    R: Registry + ?Sized,
{
    /// Create a borrowed report over the registry log.
    pub const fn new(registry: &'a R) -> Self {
        Self { registry }
    }
}

impl<R> Display for RegistryReport<'_, R>
where
    R: Registry + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut issued = self.registry.issued();

        match issued.next() {
            Some(certificate) => {
                write!(f, "{certificate}")?;

                for certificate in issued {
                    write!(f, "\n\n{certificate}")?;
                }

                Ok(())
            }
            None => write!(f, "(no issued certificates)"),
        }
    }
}
