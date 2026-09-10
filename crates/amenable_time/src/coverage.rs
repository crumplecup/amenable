//! Per-contract proof-coverage report over the temporal contract graph.
//!
//! Reads two statically-registered inventories — every temporal
//! contract's [`EvidenceLink`](amenable_core::EvidenceLink) (from
//! `#[derive(Standard)]`) and every backend's
//! [`ProofRecord`](amenable_core::ProofRecord) — and reports, per
//! contract, which verifier backends carry a machine-checked proof and
//! which are citation-only.
//!
//! A backend's `ProofRecord`s are only visible when that backend crate is
//! linked, so a meaningful report needs `amenable_kani` +
//! `amenable_creusot` (+ the `verus` feature) in the binary — the
//! `amenable` facade with `--features creusot,verus`, which is what
//! `just temporal-coverage` runs.

use std::fmt::{self, Display, Formatter};

use amenable_core::{EvidenceLink, ProofRecord};

/// The trailing `::`-segment of a path — a contract's bare type name,
/// which is unique across the temporal graph even though its
/// `EvidenceLink` carries the full module path and its `ProofRecord`
/// carries the crate-root path.
fn type_name(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

/// One contract's proof coverage across the three formal backends.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemporalCoverageRow {
    /// The contract's bare type name.
    contract: String,
    /// A Kani harness proves this contract.
    kani: bool,
    /// A Creusot contract function proves this contract.
    creusot: bool,
    /// A Verus spec function proves this contract.
    verus: bool,
}

impl TemporalCoverageRow {
    /// The contract's bare type name.
    #[must_use]
    pub fn contract(&self) -> &str {
        &self.contract
    }

    /// Whether Kani proves this contract.
    #[must_use]
    pub const fn kani(&self) -> bool {
        self.kani
    }

    /// Whether Creusot proves this contract.
    #[must_use]
    pub const fn creusot(&self) -> bool {
        self.creusot
    }

    /// Whether Verus proves this contract.
    #[must_use]
    pub const fn verus(&self) -> bool {
        self.verus
    }

    /// Whether any backend carries a machine-checked proof.
    #[must_use]
    pub const fn is_checked(&self) -> bool {
        self.kani || self.creusot || self.verus
    }
}

/// The whole temporal contract graph, one [`TemporalCoverageRow`] each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalCoverage {
    rows: Vec<TemporalCoverageRow>,
}

impl TemporalCoverage {
    /// Build the report from the linked binary's registries.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn snapshot() -> Self {
        let proofs: Vec<(&'static str, &'static str)> = inventory::iter::<ProofRecord>()
            .map(|record| (type_name(record.evidence()), record.verifier()))
            .collect();

        let proven_by = |contract: &str, verifier: &str| {
            proofs
                .iter()
                .any(|&(evidence, backend)| evidence == contract && backend == verifier)
        };

        // The citation-only contract `Standard`s all live under
        // `amenable_time::contracts::`; `#[derive(Evidence)]` also
        // registers `EvidenceLink`s for the composites, descriptors and
        // sidecars, which this report is not about.
        let mut rows: Vec<TemporalCoverageRow> = inventory::iter::<EvidenceLink>()
            .filter(|link| link.name().contains("amenable_time::contracts::"))
            .map(|link| {
                let contract = type_name(link.name()).to_owned();
                TemporalCoverageRow {
                    kani: proven_by(&contract, "kani"),
                    creusot: proven_by(&contract, "creusot"),
                    verus: proven_by(&contract, "verus"),
                    contract,
                }
            })
            .collect();

        rows.sort_by(|a, b| a.contract.cmp(&b.contract));
        rows.dedup_by(|a, b| a.contract == b.contract);
        Self { rows }
    }

    /// Every contract row, sorted by name.
    #[must_use]
    pub fn rows(&self) -> &[TemporalCoverageRow] {
        &self.rows
    }

    /// Total contracts in the graph.
    #[must_use]
    pub fn total(&self) -> usize {
        self.rows.len()
    }

    /// Contracts with at least one machine-checked proof.
    #[must_use]
    pub fn checked(&self) -> usize {
        self.rows.iter().filter(|row| row.is_checked()).count()
    }

    /// Contracts backed only by their normative citation.
    #[must_use]
    pub fn citation_only(&self) -> usize {
        self.total() - self.checked()
    }
}

impl Display for TemporalCoverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "temporal proof coverage")?;
        writeln!(f, "{:-<72}", "")?;
        writeln!(
            f,
            "{:<48} {:>5} {:>7} {:>5}",
            "contract", "kani", "creusot", "verus"
        )?;
        writeln!(f, "{:-<72}", "")?;

        let mark = |present: bool| if present { "yes" } else { "-" };
        for row in &self.rows {
            if row.is_checked() {
                writeln!(
                    f,
                    "{:<48} {:>5} {:>7} {:>5}",
                    row.contract,
                    mark(row.kani),
                    mark(row.creusot),
                    mark(row.verus),
                )?;
            }
        }

        writeln!(f, "{:-<72}", "")?;
        writeln!(
            f,
            "{} atomic contracts: {} machine-checked, {} citation-only",
            self.total(),
            self.checked(),
            self.citation_only(),
        )?;
        writeln!(
            f,
            "(the proof_composition aggregates compose these with the \
             structural leaves — see `temporal_composition_test`)"
        )
    }
}
