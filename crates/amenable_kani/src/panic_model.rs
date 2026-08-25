//! Kani-only accommodation models for focused panic-runtime laws.
//!
//! The direct `Location::caller()` and `catch_unwind` paths both reach
//! unsupported panic-runtime boundaries under Kani before the small
//! Rust-facing laws in this crate can be checked. This module keeps the
//! narrower observable contracts the production proofs actually claim.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of two tracked call sites in one source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::same_file_distinct_lines(\"proof.rs\", 10, 11)"
)]
pub struct KaniCallerLocationObservation {
    /// The shared file path.
    ///
    /// `#[getter(skip)]`, kept hand-written: `derive_getters` binds a
    /// `&'static`-typed field's getter to a `&'static self` receiver
    /// (confirmed via `cargo expand`: `pub fn file(&'static self) ->
    /// &'static str`), not a plain `&self` returning the field's own
    /// `'static` content the way the hand-written version does -- a real
    /// regression, not a style difference: it broke
    /// `demonstrate_immediate_call_site`'s `self.file()` call on an
    /// owned, short-lived `self` binding.
    #[getter(skip)]
    file: &'static str,
    /// The first immediate call-site line.
    #[getter(copy)]
    first_line: u32,
    /// The second immediate call-site line.
    #[getter(copy)]
    second_line: u32,
}

impl KaniCallerLocationObservation {
    /// Model two immediate call sites in the same file at different lines.
    #[must_use]
    pub fn same_file_distinct_lines(file: &'static str, first_line: u32, second_line: u32) -> Self {
        Self {
            file,
            first_line,
            second_line,
        }
    }

    /// Report the shared file path.
    #[must_use]
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// Report whether the two modeled lines are distinct.
    #[must_use]
    pub fn lines_differ(&self) -> bool {
        self.first_line != self.second_line
    }
}

impl Provenance for KaniCallerLocationObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "two calls to the same track_caller function from different lines in one file report that shared file and different immediate caller lines",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Location::caller path reaches Kani's unsupported caller_location boundary",
            ),
            MetadataEntry::new("file", self.file),
            MetadataEntry::new("first_line", self.first_line.to_string()),
            MetadataEntry::new("second_line", self.second_line.to_string()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of a panic hook capturing the panic's own payload text.
#[derive(Debug, Clone, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::message(\"captured panic message\")"
)]
pub struct KaniPanicHookObservation {
    message: String,
}

impl KaniPanicHookObservation {
    /// Model a panic hook that captured the panic payload message.
    #[must_use]
    pub fn message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Report the panic payload message.
    #[must_use]
    pub fn captured_message(&self) -> &str {
        &self.message
    }
}

impl Provenance for KaniPanicHookObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a panic hook installed for one panic observes that panic's own payload message exactly",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct panic-hook path reaches Kani's unsupported catch_unwind boundary before the payload check can complete",
            ),
            MetadataEntry::new("message", self.message.clone()),
        ]
        .into_iter()
        })
    }
}
