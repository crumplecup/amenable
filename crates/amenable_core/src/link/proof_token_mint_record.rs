//! A statically-registered fact: a real proof-token type exists, minting a
//! named proposition from an optional credential.

/// A statically-registered fact: a real proof-token type exists, minting a
/// named proposition from an optional credential. Registered once per
/// token by `#[derive(amenable_derive::ProofToken)]` (always) and by
/// `#[amenable_derive::establish(credential = .., proposition = ..)]`'s
/// *verifier-less* form (only when present, which supersedes the
/// `ProofToken`-only registration for the same token — see this record's
/// own `credential` field doc).
///
/// Exists for the identical reason [`crate::ExchangeEdgeRecord`] does, applied to
/// a different capture target: `verus --crate-type=lib` can resolve no
/// external crate at all, not even a proc-macro one (unlike `cargo
/// creusot`, which really does resolve ordinary Cargo dependencies) — so
/// neither the real token type nor its real, backend-generic `Establish`
/// impl (living in `amenable_gaap`, `GAAP_LEDGER_PLAN.md`'s Step 7) can
/// ever be named from Verus's own gallery code, and a hand-written local
/// mirror duplicates the same trivial shape `#[amenable_derive::
/// establish]` already exists to eliminate. A codegen tool reads this
/// registry and *writes* a real, checked-in, proc-macro-free companion —
/// the same shape [`crate::ExchangeEdgeRecord`]'s own consumers use, applied to a
/// type/impl shape instead of a captured method body.
///
/// Hand-written `const fn new`/getters, not derived, for the same real
/// reason [`crate::EvidenceLink`]/[`crate::ProofRecord`]/[`crate::ContractRecord`]/
/// [`crate::ExchangeEdgeRecord`] are: `inventory::submit!` requires a
/// `const`-evaluable value, and every real reader gets a
/// `ProofTokenMintRecord` back as `&'static` (from [`inventory::iter`]).
pub struct ProofTokenMintRecord {
    token: &'static str,
    proposition: &'static str,
    credential: Option<&'static str>,
}

impl ProofTokenMintRecord {
    /// Register a token type's mint, naming the proposition it proves
    /// and (if any) the credential it's established from.
    #[must_use]
    pub const fn new(
        token: &'static str,
        proposition: &'static str,
        credential: Option<&'static str>,
    ) -> Self {
        Self {
            token,
            proposition,
            credential,
        }
    }

    /// The token type's own name, as written (e.g. `"ValidatedToken"`).
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.token
    }

    /// The proposition this token proves, in the same naming convention as
    /// [`crate::EvidenceLink::name`].
    #[must_use]
    pub const fn proposition(&self) -> &'static str {
        self.proposition
    }

    /// The credential type this token is established from, if any —
    /// `None` for a root token.
    #[must_use]
    pub const fn credential(&self) -> Option<&'static str> {
        self.credential
    }
}

inventory::collect!(ProofTokenMintRecord);
