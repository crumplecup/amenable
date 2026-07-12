//! `RustStdType`: interface and concrete registrations for Rust
//! standard-library carriers, defined together in one crate.
//!
//! `RustStdType` is implemented directly on foreign standard-library types
//! (`bool`, `i32`, `String`, ...), which Rust's orphan rules only permit
//! from the crate that defines the trait. So unlike the core constitutional
//! roles in `amenable`, this trait and its std-lib coverage live together
//! here rather than split across an interface crate and a downstream
//! consumer.

use std::{
    fs, io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use amenable_core::{OwnedProvenanceReport, Provenance as _, Registry, Standard};
use amenable_derive::Provenance;

/// Shared provenance for Rust-authored documented semantics.
#[derive(Debug, Clone, PartialEq, Eq, Provenance)]
#[provenance(crate = "amenable_core")]
pub struct RustLanguageProvenance {
    /// The class of authority this provenance record represents.
    authority_kind: String,
    /// The authorizing body for the documented semantics.
    authority: String,
    /// The Rust crate that normatively defines the type.
    source_crate: String,
    /// The Rust module path that normatively defines the type.
    source_module: String,
}

impl RustLanguageProvenance {
    /// Create a shared Rust-language provenance record.
    pub fn new(
        authority_kind: impl Into<String>,
        authority: impl Into<String>,
        source_crate: impl Into<String>,
        source_module: impl Into<String>,
    ) -> Self {
        Self {
            authority_kind: authority_kind.into(),
            authority: authority.into(),
            source_crate: source_crate.into(),
            source_module: source_module.into(),
        }
    }

    /// Provenance for Rust's primitive carriers as documented through `core`.
    pub fn core_primitive() -> Self {
        Self::new(
            "external_standard",
            "Rust Project Developers",
            "core",
            "core::primitive",
        )
    }

    /// Provenance for `String` as documented through `alloc`.
    pub fn alloc_string() -> Self {
        Self::new(
            "external_standard",
            "Rust Project Developers",
            "alloc",
            "alloc::string",
        )
    }
}

/// Structured provenance for a Rust standard-library-backed carrier.
#[derive(Debug, Clone, PartialEq, Eq, Provenance)]
#[provenance(crate = "amenable_core")]
pub struct RustStdProvenance {
    /// The shared Rust-language provenance this type-specific record relies on.
    rust: RustLanguageProvenance,
    /// The canonical documentation URL for the type.
    source_url: String,
    /// The fully-qualified Rust type name being certified.
    type_name: String,
    /// Concise summary of the semantic promise made by the standard library.
    semantic_summary: String,
}

impl RustStdProvenance {
    /// Create a provenance record for a concrete Rust standard-library type.
    pub fn new(
        rust: RustLanguageProvenance,
        source_url: impl Into<String>,
        type_name: impl Into<String>,
        semantic_summary: impl Into<String>,
    ) -> Self {
        Self {
            rust,
            source_url: source_url.into(),
            type_name: type_name.into(),
            semantic_summary: semantic_summary.into(),
        }
    }
}

/// Explicit standard-role wrapper for a Rust standard-library-backed type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustStdStandard<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> RustStdStandard<T> {
    /// Promote a Rust standard-library-backed type into the standard role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Standard for RustStdStandard<T>
where
    T: RustStdType,
{
    type Provenance = RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <T as RustStdType>::provenance()
    }
}

/// Provenance helper for Rust standard-library-backed carriers.
///
/// This trait names the authoritative Rust documentation surface and semantic
/// summary for concrete std or core types used as trusted roots.
pub trait RustStdType: 'static {
    /// Structured provenance for the documented semantics of this type.
    fn provenance() -> RustStdProvenance {
        RustStdProvenance::new(
            Self::rust_language_provenance(),
            Self::rust_doc_url(),
            std::any::type_name::<Self>(),
            Self::rust_semantics_summary(),
        )
    }

    /// Shared Rust-language provenance for this carrier family.
    fn rust_language_provenance() -> RustLanguageProvenance {
        RustLanguageProvenance::core_primitive()
    }

    /// Render the provenance report for this standard type.
    fn report() -> OwnedProvenanceReport<RustStdProvenance> {
        OwnedProvenanceReport::new(Self::provenance())
    }

    /// Issue a certificate for this trusted standard type through a registry.
    fn certification<R>(registry: &mut R) -> R::Certificate
    where
        Self: Sized,
        R: Registry,
    {
        let provenance = Self::provenance();

        provenance.certification(registry, std::any::type_name::<Self>())
    }

    /// The canonical documentation URL for the type.
    fn rust_doc_url() -> &'static str;

    /// Concise summary of the semantic promise made by the standard library.
    fn rust_semantics_summary() -> &'static str;
}

fn write_standard_certificate_artifact<T>(
    directory: &Path,
    registry: &mut crate::CertRegistry,
    file_stem: &str,
) -> io::Result<PathBuf>
where
    T: RustStdType,
{
    let certificate =
        RustStdStandard::<T>::new().certification(registry, std::any::type_name::<T>());
    let path = directory.join(format!("{file_stem}.provenance.txt"));

    fs::write(&path, certificate.to_string())?;

    Ok(path)
}

/// Emit provenance certificate artifacts for every built-in Rust standard type registration.
pub fn write_rust_std_certificate_artifacts(
    directory: impl AsRef<Path>,
) -> io::Result<Vec<PathBuf>> {
    let directory = directory.as_ref();
    let mut registry = crate::CertRegistry::new();
    let mut paths = Vec::new();

    fs::create_dir_all(directory)?;

    paths.push(write_standard_certificate_artifact::<bool>(
        directory,
        &mut registry,
        "bool",
    )?);
    paths.push(write_standard_certificate_artifact::<char>(
        directory,
        &mut registry,
        "char",
    )?);
    paths.push(write_standard_certificate_artifact::<i8>(
        directory,
        &mut registry,
        "i8",
    )?);
    paths.push(write_standard_certificate_artifact::<i16>(
        directory,
        &mut registry,
        "i16",
    )?);
    paths.push(write_standard_certificate_artifact::<i32>(
        directory,
        &mut registry,
        "i32",
    )?);
    paths.push(write_standard_certificate_artifact::<i64>(
        directory,
        &mut registry,
        "i64",
    )?);
    paths.push(write_standard_certificate_artifact::<i128>(
        directory,
        &mut registry,
        "i128",
    )?);
    paths.push(write_standard_certificate_artifact::<isize>(
        directory,
        &mut registry,
        "isize",
    )?);
    paths.push(write_standard_certificate_artifact::<u8>(
        directory,
        &mut registry,
        "u8",
    )?);
    paths.push(write_standard_certificate_artifact::<u16>(
        directory,
        &mut registry,
        "u16",
    )?);
    paths.push(write_standard_certificate_artifact::<u32>(
        directory,
        &mut registry,
        "u32",
    )?);
    paths.push(write_standard_certificate_artifact::<u64>(
        directory,
        &mut registry,
        "u64",
    )?);
    paths.push(write_standard_certificate_artifact::<u128>(
        directory,
        &mut registry,
        "u128",
    )?);
    paths.push(write_standard_certificate_artifact::<usize>(
        directory,
        &mut registry,
        "usize",
    )?);
    paths.push(write_standard_certificate_artifact::<f32>(
        directory,
        &mut registry,
        "f32",
    )?);
    paths.push(write_standard_certificate_artifact::<f64>(
        directory,
        &mut registry,
        "f64",
    )?);
    paths.push(write_standard_certificate_artifact::<String>(
        directory,
        &mut registry,
        "string",
    )?);

    fs::write(
        directory.join("registry-report.txt"),
        registry.report().to_string(),
    )?;

    Ok(paths)
}

macro_rules! impl_rust_std_primitive {
    ($ty:ty, $url:expr, $summary:expr) => {
        impl RustStdType for $ty {
            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

macro_rules! impl_rust_std_fixed_width_integer {
    ($(($ty:ty, $signedness:literal, $bits:literal)),* $(,)?) => {
        $(
            impl_rust_std_primitive!(
                $ty,
                concat!("https://doc.rust-lang.org/std/primitive.", stringify!($ty), ".html"),
                concat!(
                    "The ",
                    $signedness,
                    " ",
                    stringify!($bits),
                    "-bit integer carrier stores values in the ",
                    stringify!($ty),
                    " range defined by Rust."
                )
            );
        )*
    };
}

impl_rust_std_primitive!(
    bool,
    "https://doc.rust-lang.org/std/primitive.bool.html",
    "The boolean carrier admits exactly the truth values false and true."
);
impl_rust_std_primitive!(
    char,
    "https://doc.rust-lang.org/std/primitive.char.html",
    "The character carrier stores a Unicode scalar value."
);
impl_rust_std_fixed_width_integer!(
    (i8, "signed", 8),
    (i16, "signed", 16),
    (i32, "signed", 32),
    (i64, "signed", 64),
    (i128, "signed", 128),
    (u8, "unsigned", 8),
    (u16, "unsigned", 16),
    (u32, "unsigned", 32),
    (u64, "unsigned", 64),
    (u128, "unsigned", 128),
);
impl_rust_std_primitive!(
    isize,
    "https://doc.rust-lang.org/std/primitive.isize.html",
    "The pointer-sized signed integer carrier stores values in the isize range defined by Rust."
);
impl_rust_std_primitive!(
    usize,
    "https://doc.rust-lang.org/std/primitive.usize.html",
    "The pointer-sized unsigned integer carrier stores values in the usize range defined by Rust."
);
impl_rust_std_primitive!(
    f32,
    "https://doc.rust-lang.org/std/primitive.f32.html",
    "The 32-bit floating-point carrier follows Rust's f32 semantics."
);
impl_rust_std_primitive!(
    f64,
    "https://doc.rust-lang.org/std/primitive.f64.html",
    "The 64-bit floating-point carrier follows Rust's f64 semantics."
);

impl RustStdType for String {
    fn rust_language_provenance() -> RustLanguageProvenance {
        RustLanguageProvenance::alloc_string()
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/string/struct.String.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The String carrier stores owned UTF-8 text as defined by Rust's standard library."
    }
}
