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

use amenable_core::{Evidence, MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Identifies an account by name.
///
/// A bare newtype — no construction-time validation. The whole point of
/// this design is that checkable properties (like `AccountsDistinct`)
/// are proven at the `Exchange` edge, not baked into a constructor.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountId(String);

impl AccountId {
    /// Wrap an account name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrow the account name.
    pub fn name(&self) -> &str {
        &self.0
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransferPayload {
    from: AccountId,
    to: AccountId,
    amount: Amount,
}

impl TransferPayload {
    /// Construct a transfer payload naming its two accounts and amount.
    pub fn new(from: AccountId, to: AccountId, amount: Amount) -> Self {
        Self { from, to, amount }
    }

    /// The source account.
    pub fn from(&self) -> &AccountId {
        &self.from
    }

    /// The destination account.
    pub fn to(&self) -> &AccountId {
        &self.to
    }

    /// The transfer amount.
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

#[amenable_derive::evidence]
impl Evidence for TransferPayload {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self::default()
    }

    fn audit(&self) {}
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

/// The transfer was rejected — validation failed, or a validated
/// transfer was manually rolled back before commit. See [`Pending`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Rejected;

impl Provenance for Rejected {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new(
            "asserted",
            "transfer state, reachable from Pending (validation failure) or Validated (manual rollback)",
        )]
        .into_iter()
    }
}
