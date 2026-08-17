//! The canonical `Stoplight` worked example's evidence markers —
//! `Green`/`Yellow`/`Red`, the traffic-light states `docs/AMENABLE_PLAN.md`'s
//! "States Are Roots, Transitions Are Relations" section already uses to
//! motivate the split between state claims (asserted) and transition claims
//! (proven).
//!
//! Live here, not in `amenable_kani::stoplight` (where they originally
//! lived) or in a new dedicated crate: both `amenable_kani` and
//! `amenable_creusot` need a real `Witness<TheirVerifier>` impl on these
//! *exact* types, and verifier backend crates never depend on each other
//! (`amenable_kani`'s own former Cargo edge to `amenable_creusot` was
//! removed for exactly this reason). Putting the markers in a neutral
//! crate both backends can depend on — the same split `amenable_gaap`
//! already uses for `Pending`/`Validated`/`Committed`/`Rejected<T>` — means
//! neither backend ever needs to depend on the other. `amenable_core` is
//! the neutral crate here rather than a new dedicated one (unlike GAAP,
//! which earns its own crate for real domain content): these three types
//! are about as simple as a worked example gets, and `Stoplight` is
//! already the canonical teaching example this crate's own trait family
//! is documented against.
//!
//! **What does *not* live here.** `amenable_kani::stoplight` still owns
//! everything backend-specific: the tokens (`GreenToken`/`YellowToken`/
//! `RedToken`), the `Established<T, Token>` sidecar wrapper, the real
//! `Stoplight` struct and its transition methods, and the real Kani
//! contracts — matching exactly how `Ledger`/`Transfer<S, Token>`/the
//! ledger tokens stay in `amenable_kani::ledger`, not `amenable_gaap`,
//! even though `Pending`/`Validated`/etc. do live in `amenable_gaap`.
//! Construction of a token is still gated by backend-private constructors
//! (`Established::new` stays private to `amenable_kani`) — moving the
//! bare evidence markers here doesn't touch that guarantee at all.
//!
//! **No `#[derive(Standard)]` here.** `amenable_core` cannot depend on
//! `amenable_derive` as a real (non-dev) dependency — `amenable_derive`
//! itself optionally depends back on `amenable_core` (its `verus`
//! feature), so a real edge the other direction would risk a genuine
//! Cargo cycle under workspace-wide feature unification. The `Standard`/
//! `Evidence` impls below are hand-written instead, replicating exactly
//! what `#[derive(Standard)] #[standard(basis = "Self")]` generates for a
//! root claim — this is the one place in the whole `amenable` family
//! where that derive genuinely cannot reach, not a lapse in "dogfood the
//! derives" discipline.

use crate::{Evidence, MetadataEntry, Provenance, Standard};

/// The light is green — a root state claim, asserted rather than derived
/// from a prior transition: the first assertion any running `Stoplight`
/// makes is not computed from anything, it's asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Green;

impl Provenance for Green {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new(
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, by design convention (power-on default)",
            )]
            .into_iter(),
        )
    }
}

impl Standard for Green {
    type Provenance = Self;

    fn provenance(&self) -> Self::Provenance {
        *self
    }
}

impl Evidence for Green {
    type Basis = Self;
    type Audit = Self;

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {
        *self
    }

    fn is_root() -> bool {
        true
    }
}

/// The light is yellow — see [`Green`] for why this is a root claim, not
/// a derived one, even though in practice a `Stoplight` only ever reaches
/// `Yellow` via a proven `Green -> Yellow` transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Yellow;

impl Provenance for Yellow {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new(
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, reachable only via a proven Green -> Yellow exchange",
            )]
            .into_iter(),
        )
    }
}

impl Standard for Yellow {
    type Provenance = Self;

    fn provenance(&self) -> Self::Provenance {
        *self
    }
}

impl Evidence for Yellow {
    type Basis = Self;
    type Audit = Self;

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {
        *self
    }

    fn is_root() -> bool {
        true
    }
}

/// The light is red — see [`Green`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Red;

impl Provenance for Red {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new(
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, reachable only via a proven Yellow -> Red exchange",
            )]
            .into_iter(),
        )
    }
}

impl Standard for Red {
    type Provenance = Self;

    fn provenance(&self) -> Self::Provenance {
        *self
    }
}

impl Evidence for Red {
    type Basis = Self;
    type Audit = Self;

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {
        *self
    }

    fn is_root() -> bool {
        true
    }
}
