//! Domain types for the two-account `Transfer` worked example: the
//! payload data (`AccountId`, `Amount`, `TransferPayload`) and the four
//! typestate markers (`Pending`/`Validated`/`Committed`/`Rejected`).
//! Mirrors `~/repos/elicitation/crates/elicit_server::ledger::typestate`'s
//! own shape (`Transfer<Pending -> Validated -> Committed>`, with a
//! `reject()`/`rollback()` branch to `Rejected`), re-expressed in
//! `amenable_core`'s own vocabulary.
//!
//! First case in this gallery where `Sidecar<V>::Primary` and
//! `Sidecar<V>::Proposition` genuinely diverge: `amenable_kani::
//! stoplight`'s `Established<T, Token>` uses the same type `T` for
//! both, since a traffic-light state IS its whole payload. A ledger
//! transfer carries real data (accounts, amount) alongside its state
//! marker, so `Primary` will be `TransferPayload` and `Proposition`
//! will be the state marker — wired up once a verifier's `Witness<V>`
//! impls exist (`GAAP_LEDGER_PLAN.md`'s Step 1 onward; no `Sidecar<V>`
//! impl exists yet in this file).
//!
//! Per-state associated data (`ValidatedData`'s captured balance-at-
//! validation-time, `CommittedData`'s before/after balances, mirroring
//! elicitation's `StateData<S>`) is deliberately not added here yet —
//! it belongs with the real transition logic that captures it (Step 1),
//! not speculated on before any transition exists to need it.

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;
use uuid::Uuid;

/// Identifies an account by a stable id, alongside a human-readable
/// name.
///
/// Equality, ordering, and hashing all compare `id` only, never `name`
/// — `amenable_kani::gallery::ledger_account_id_comparison`'s own
/// investigation (`GAAP_LEDGER_PLAN.md`'s Step 1) found that comparing
/// two independently-constructed `String`s for equality inside a
/// `#[kani::ensures]` closure is expensive for CBMC regardless of
/// content or length, while comparing a fixed-size value (an integer, a
/// UUID's 16 bytes) is cheap — and, critically, that a fixed-*capacity*
/// but still variable-*length* string (a bounded buffer plus a length
/// field) is exactly as expensive as an unbounded `String`, so
/// truncating/capping the name would not have helped. A `Uuid` was
/// chosen over a bare integer because it's a real, dedicated identity
/// type (globally unique without a central allocator) rather than a
/// proxy repurposing an arbitrary numeric type.
///
/// `id` must be supplied explicitly, not generated fresh on every call:
/// the same real-world account (e.g. two separate references to
/// "Alice" in different transfers) must always compare equal, which
/// only holds if its id is stable across reconstructions — the same
/// reason a real chart of accounts assigns an id once, at account
/// creation, rather than re-deriving one per lookup.
#[derive(Debug, Clone, Default, derive_getters::Getters, derive_new::new)]
pub struct AccountId {
    /// The account's stable id — what identity checks compare.
    #[getter(copy)]
    id: Uuid,
    /// The account's human-readable name — display only, never compared.
    #[new(into)]
    name: String,
}

impl PartialEq for AccountId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AccountId {}

impl Hash for AccountId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for AccountId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AccountId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

/// A transfer amount, in the ledger's smallest denomination.
///
/// See [`AccountId`] on why there's no constructor validation:
/// `AmountPositive`'s real check lands as a proof, not a runtime guard.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i64);

impl Amount {
    /// Wrap a raw amount.
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    /// The raw amount.
    pub fn value(&self) -> i64 {
        self.0
    }
}

/// The real payload carried alongside a transfer's state marker —
/// `Sidecar<V>::Primary`, once a verifier's impls exist. Distinct from
/// the state marker itself (`Sidecar<V>::Proposition`): a `Transfer` in
/// any state still names the same two accounts and amount.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    amenable_derive::Evidence,
    derive_getters::Getters,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct TransferPayload {
    /// The source account.
    from: AccountId,
    /// The destination account.
    to: AccountId,
    /// The transfer amount.
    #[getter(copy)]
    amount: Amount,
}

/// The transfer is awaiting validation — a root state claim, asserted
/// rather than derived from a prior transition (see `docs/
/// AMENABLE_PLAN.md`, "States Are Roots, Transitions Are Relations"):
/// every `Transfer` starts here, and starting here is asserted by
/// construction, not proven.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Pending;

impl Provenance for Pending {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, by construction (every Transfer starts Pending)",
        )]
        .into_iter()
    }
}

/// The transfer has been validated and is ready to commit — see
/// [`Pending`] for why this is a root claim, not a derived one, even
/// though in practice a transfer only reaches `Validated` via a proven
/// `Exchange<TransferPayload@Pending, TransferPayload@Validated>`
/// transition gated on `AmountPositive`/`SufficientFunds`/
/// `AccountsDistinct`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Validated;

impl Provenance for Validated {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, reachable only via a proven Pending -> Validated exchange",
        )]
        .into_iter()
    }
}

/// The transfer has been committed to the ledger — see [`Pending`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Committed;

impl Provenance for Committed {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, reachable only via a proven Validated -> Committed exchange",
        )]
        .into_iter()
    }
}

/// The transfer was rejected — validation failed (`Rejected<Pending>`),
/// or a validated transfer was manually rolled back before commit
/// (`Rejected<Validated>`). See [`Pending`] for why this is a root
/// claim, not a derived one.
///
/// Parameterized by the state it was rejected *from*, not flat — a
/// deliberate divergence from `~/repos/elicitation/crates/
/// elicit_server::ledger::typestate::Rejected` (flat, with a runtime
/// `RejectedData { reason }` field distinguishing why). `amenable_kani`'s
/// `#[amenable_derive::exchange]` mints exactly one `impl Witness<V> for
/// #evidence` per macro invocation: `reject()` (`Pending -> Rejected`)
/// and `rollback()` (`Validated -> Rejected`) both targeting a single
/// flat `Rejected` would be two impls of the same trait for the same
/// concrete type — a hard `E0119` conflicting-implementation error, not
/// a style choice to avoid. `Rejected<Pending>`/`Rejected<Validated>`
/// are genuinely distinct concrete types, so each edge earns its own
/// honest `Witness` proof for its own real claim — `reject()` mirrors
/// `validate`'s own failure conditions, `rollback()` doesn't obviously
/// prove the same thing, so collapsing both into one shared `Witness`
/// impl would either force an artificial unification or silently under-
/// represent one edge's real guarantee. See `amenable_kani::ledger`'s
/// `reject`/`rollback` doc comments for the two real proofs this backs.
// `#[derive(Standard)]` stays real, generic, single-invocation: `bound =
// "Self: Provenance"` (not a hardcoded `T = Pending | Validated` list)
// narrows the generated `Standard`/`Evidence` impls to exactly the `T`
// for which a `Provenance` impl actually exists below -- extensible if
// a future edge ever targets a third `Rejected<SomeOtherState>`, not a
// closed enumeration to keep in sync by hand.
//
// `Debug`/`Clone`/`Copy`/`PartialEq`/`Eq`/`Default` are plain, ordinary
// derives too, each conditional on `T: Trait` the way the built-in
// derive macros always generate for a struct with a generic parameter
// (even one that, like this one, only ever appears inside `PhantomData
// <T>`) -- previously hand-written here on the mistaken assumption that
// this conditional bound was itself the problem. It never was: `Pending`
// and `Validated` (the only two `T` this type is ever instantiated
// with) already derive every one of these traits, so the condition is
// always met in practice. The real gap was `#[derive(Standard)]` itself
// silently assuming `Self: Clone`/`#basis_ty: Default` inside its own
// generated impl body with no matching bound in that impl's own
// where-clause -- fixed at the macro (see its own doc comment), not
// routed around here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self", bound = "Self: Provenance")]
pub struct Rejected<T>(std::marker::PhantomData<T>);

impl Provenance for Rejected<Pending> {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, reachable only via a proven Pending -> Rejected exchange (validation failure)",
        )]
        .into_iter()
    }
}

impl Provenance for Rejected<Validated> {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, reachable only via a proven Validated -> Rejected exchange (manual rollback)",
        )]
        .into_iter()
    }
}

// `#[derive(Standard)]`'s auto `EvidenceLink` registration only covers
// non-generic roots (a generic basis has no single concrete name to
// register under) -- manual registration for each concrete
// instantiation, per that derive's own documented precedent.
::inventory::submit! {
    ::amenable_core::EvidenceLink::new(
        concat!(module_path!(), "::Rejected<Pending>"),
        concat!(module_path!(), "::Rejected<Pending>"),
        0,
    )
}

::inventory::submit! {
    ::amenable_core::EvidenceLink::new(
        concat!(module_path!(), "::Rejected<Validated>"),
        concat!(module_path!(), "::Rejected<Validated>"),
        0,
    )
}
