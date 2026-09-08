//! [`temporal_standard!`] — declares citation-only temporal contracts in
//! bulk, the `elicit_temporal` `structural_prop!` analog.
//!
//! Crate-internal (`#[macro_use]` in `lib.rs`), so it can lean on
//! `#[derive(amenable_derive::Standard)]` with a `crate::`-qualified
//! provenance type. If a downstream crate ever needs to declare its own
//! temporal standards this becomes `#[macro_export]` with full `$crate::`
//! qualification and hand-rolled trait impls — deferred until there is a
//! real second caller (`docs/AMENABLE_TIME_PLAN.md`, decision 2).

/// Declare one or more citation-only temporal contract types.
///
/// Each entry expands to a zero-sized `pub struct` that is
/// [`Standard`](amenable_core::Standard) + [`Evidence`](amenable_core::Evidence)
/// (via `#[derive(Standard)]`), a private `facts()` returning its
/// [`TemporalProvenance`](crate::TemporalProvenance), and an
/// `EvidenceLink` self-registration.
///
/// ```ignore
/// use crate::NormativeQuotation;
///
/// temporal_standard! {
///     /// An RFC 3339 timestamp uses a four-digit year.
///     Rfc3339UsesFourDigitYear => (
///         document: "RFC 3339",
///         section: "5.6",
///         body: Ietf,
///         status: Normative,
///         summary: "date-fullyear = 4DIGIT",
///         quotation: NormativeQuotation::verbatim("date-fullyear   = 4DIGIT"),
///         url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
///     );
/// }
/// ```
///
/// `quotation:` is any expression evaluating to a `NormativeQuotation` —
/// `ParaphraseOnly` for tier-C (paywalled) sources, `verbatim("…")` for
/// tiers A/B. `url:` and `cross_check: (doc, section, status)` repeat 0..n
/// and are optional.
macro_rules! temporal_standard {
    ( $(
        $(#[$doc:meta])*
        $name:ident => (
            document: $document:literal,
            section: $section:literal,
            body: $body:ident,
            status: $status:ident,
            summary: $summary:literal,
            quotation: $quotation:expr
            $(, url: $url:literal )?
            $(, cross_check: ( $cc_doc:literal, $cc_section:literal, $cc_status:ident ) )*
            $(,)?
        );
    )* ) => {
        $(
            $(#[$doc])*
            #[derive(
                Debug, Clone, Copy, Default, PartialEq, Eq, Hash,
                ::amenable_derive::Standard,
            )]
            #[standard(
                basis = "Self",
                provenance = "Self::facts()",
                provenance_type = "crate::TemporalProvenance"
            )]
            pub struct $name;

            impl $name {
                #[doc = concat!(
                    "The normative provenance record for the `",
                    stringify!($name),
                    "` contract."
                )]
                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn facts() -> $crate::TemporalProvenance {
                    let facts = $crate::TemporalProvenance::new(
                        $document,
                        $section,
                        $crate::NormativeStatus::$status,
                        $crate::StandardsBody::$body,
                        $summary,
                        $quotation,
                    );
                    $( let facts = facts.with_url(::amenable_std::SourceUrl::new($url)); )?
                    $(
                        let facts = facts.with_cross_check($crate::CrossCheck::new(
                            $cc_doc,
                            $cc_section,
                            $crate::NormativeStatus::$cc_status,
                        ));
                    )*
                    facts
                }
            }
        )*
    };
}
