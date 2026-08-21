//! Read-only reconstruction of a registered evidence chain and its proofs,
//! for auditing.
//!
//! Everything here is assembled purely by reading the statically-registered
//! [`EvidenceLink`]/[`ProofRecord`] descriptors that types submit alongside
//! their `Evidence`/`Witness` impls — nothing here ever constructs a value,
//! calls a calculation, or runs a verifier. That's the whole point: an
//! auditor should be able to inspect the chain of custody behind a claim
//! without re-deriving it.
//!
//! A piece of evidence's basis is not always a single prior link: a
//! calculation over more than one argument fans out into one
//! [`EvidenceLink`] per argument, all sharing the same `name`. So the
//! reconstructed evidence isn't a straight chain but a tree — one
//! [`ChainNode`] per link, with a `bases` entry per fan-out branch.
//!
//! A chain that's proven for `kani` at every node but has no `creusot`
//! proof past the root is not a legitimate `creusot` chain, even though
//! every individual node technically has *some* proof recorded against
//! it. Presenting mixed-completeness proofs in one tree would make an
//! incomplete chain look uniformly proven at a glance. So reconstruction
//! is all-or-nothing per requested verifier set: [`proof_chain`] and
//! [`proof_chain_for_verifiers`] return [`ChainError::Incomplete`],
//! listing every gap, rather than a partial [`ProofChainReport`].

use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

use crate::{EvidenceLink, ProofRecord};

/// One node in a reconstructed evidence tree: the evidence type's name,
/// every registered proof backing it (one per verifier with a
/// [`ProofRecord`] for it), and the node for each basis it rests on.
#[derive(Debug, Clone)]
pub struct ChainNode {
    /// This node's evidence type name.
    pub evidence: &'static str,
    /// Registered proofs for this node, as `(verifier, description)` pairs.
    pub proofs: Vec<(&'static str, String)>,
    /// The node for each basis this evidence rests on — empty at a root.
    pub bases: Vec<ChainNode>,
}

impl ChainNode {
    /// Whether this node is a root: it rests on no further basis.
    pub fn is_root(&self) -> bool {
        self.bases.is_empty()
    }
}

/// A reconstructed evidence tree, rooted at a named subject, complete for
/// every verifier in `verifiers`.
#[derive(Debug, Clone)]
pub struct ProofChainReport {
    /// The evidence type name the report was built from.
    pub subject: &'static str,
    /// Every verifier this report is complete for — each node in `root`
    /// is guaranteed to carry a proof for all of them.
    pub verifiers: Vec<String>,
    /// The subject's own node, and everything it rests on.
    pub root: ChainNode,
}

impl Display for ProofChainReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.verifiers.is_empty() {
            writeln!(
                f,
                "Proof chain for {} (no verifiers registered anywhere in this chain)",
                self.subject
            )?;
        } else {
            writeln!(
                f,
                "Proof chain for {} (complete for: {})",
                self.subject,
                self.verifiers.join(", ")
            )?;
        }
        writeln!(f)?;

        write_node(f, &self.root, 0)
    }
}

fn write_node(f: &mut Formatter<'_>, node: &ChainNode, depth: usize) -> fmt::Result {
    let indent = "  ".repeat(depth);

    write!(f, "{indent}{}", node.evidence)?;
    if node.is_root() {
        write!(f, " (root)")?;
    }
    writeln!(f)?;

    if !node.bases.is_empty() {
        let names = node
            .bases
            .iter()
            .map(|basis| basis.evidence)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, "{indent}  basis: {names}")?;
    }

    for (verifier, description) in &node.proofs {
        writeln!(f, "{indent}  proof [{verifier}]:")?;

        for line in description.lines() {
            writeln!(f, "{indent}    {line}")?;
        }
    }

    for basis in &node.bases {
        write_node(f, basis, depth + 1)?;
    }

    Ok(())
}

/// One missing proof: `evidence` has no [`ProofRecord`] registered for
/// `verifier`, even though the chain was asked to be complete for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainGap {
    /// The evidence type name missing a proof.
    pub evidence: &'static str,
    /// The verifier it has no proof registered for.
    pub verifier: String,
}

impl Display for ChainGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} has no proof registered for verifier {:?}",
            self.evidence, self.verifier
        )
    }
}

/// No registered [`EvidenceLink`] matched the requested subject at all --
/// a real, `Error`-implementing leaf, not a bare `String`.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::Error)]
#[display("no registered evidence link matches {subject:?}")]
pub struct NotFoundSource {
    /// The subject that was searched for.
    #[error(ignore)]
    pub subject: String,
    /// Source line of the call site that produced this error.
    #[error(ignore)]
    pub line: u32,
    /// Source file of the call site that produced this error.
    #[error(ignore)]
    pub file: String,
}

impl NotFoundSource {
    /// Name the subject that was searched for, recording the caller's
    /// location.
    #[track_caller]
    fn new(subject: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            subject: subject.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// The subject was found, but at least one node in its tree has no proof
/// registered for at least one of the required verifiers.
#[derive(Debug, Clone, derive_more::Error)]
pub struct IncompleteSource {
    /// The evidence type name the lookup was rooted at.
    #[error(ignore)]
    pub subject: &'static str,
    /// Every verifier the chain was required to be complete for.
    #[error(ignore)]
    pub required: Vec<String>,
    /// Every `(evidence, verifier)` gap found, in tree order.
    #[error(ignore)]
    pub gaps: Vec<ChainGap>,
    /// Source line of the call site that produced this error. Public,
    /// like every other field in this module -- matching this file's
    /// own established convention (`ChainGap`/`ChainNode`/
    /// `ProofChainReport` are all plain public-field data types too).
    #[error(ignore)]
    pub line: u32,
    /// Source file of the call site that produced this error.
    #[error(ignore)]
    pub file: String,
}

impl IncompleteSource {
    /// Name the subject, the required verifiers, and every gap found,
    /// recording the caller's location.
    #[track_caller]
    fn new(subject: &'static str, required: Vec<String>, gaps: Vec<ChainGap>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            subject,
            required,
            gaps,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

impl Display for IncompleteSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "proof chain for {} is incomplete for verifier(s): {}",
            self.subject,
            self.required.join(", ")
        )?;
        writeln!(f)?;
        write!(f, "missing proofs:")?;

        for gap in &self.gaps {
            write!(f, "\n  {gap}")?;
        }

        Ok(())
    }
}

/// Why a proof chain lookup failed. Every variant is a clean 1-tuple
/// wrapping a real, named, `Error`-implementing source type.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ChainErrorKind {
    /// See [`NotFoundSource`].
    #[display("{_0}")]
    NotFound(NotFoundSource),
    /// See [`IncompleteSource`].
    #[display("{_0}")]
    Incomplete(IncompleteSource),
}

/// Proof-chain lookup error, carrying kind + call site location -- the
/// same `struct` boxing a `*Kind` enum shape `amenable::AmenableError`
/// uses.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("{kind} at {file}:{line}")]
pub struct ChainError {
    /// The specific error kind, boxed to keep `ChainError` itself small.
    #[error(source)]
    kind: Box<ChainErrorKind>,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ChainError {
    /// Construct an error from an already-classified kind, recording the
    /// caller's location.
    #[track_caller]
    fn new(kind: ChainErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }

    /// The specific error kind.
    pub fn kind(&self) -> &ChainErrorKind {
        &self.kind
    }

    /// Construct a [`ChainErrorKind::NotFound`] error naming the subject
    /// that was searched for.
    #[track_caller]
    fn not_found(subject: impl Into<String>) -> Self {
        Self::new(ChainErrorKind::NotFound(NotFoundSource::new(subject)))
    }

    /// Construct a [`ChainErrorKind::Incomplete`] error naming the
    /// subject, the required verifiers, and every gap found.
    #[track_caller]
    fn incomplete(subject: &'static str, required: Vec<String>, gaps: Vec<ChainGap>) -> Self {
        Self::new(ChainErrorKind::Incomplete(IncompleteSource::new(
            subject, required, gaps,
        )))
    }
}

/// Reconstruct the proof tree for the evidence type whose registered
/// [`EvidenceLink::name`] ends with `subject`, requiring a complete proof
/// at every node for every verifier that appears *anywhere* in the tree.
///
/// Returns [`ChainError::NotFound`] if no registered evidence link
/// matches `subject`, or [`ChainError::Incomplete`] — naming every gap —
/// if any node lacks a proof for any verifier present elsewhere in the
/// tree. Only ever reads statically-registered descriptors — never
/// constructs a value, evaluates a calculation, or runs a verifier.
pub fn proof_chain(subject: &str) -> Result<ProofChainReport, ChainError> {
    proof_chain_for_verifiers(subject, None)
}

/// Reconstruct the proof tree for `subject`, as [`proof_chain`] does, but
/// require completeness against exactly the named `verifiers` (e.g.
/// `&["kani"]`) rather than every verifier discovered in the tree — pass
/// `None` to match [`proof_chain`]'s auto-discovered behavior.
pub fn proof_chain_for_verifiers(
    subject: &str,
    verifiers: Option<&[&str]>,
) -> Result<ProofChainReport, ChainError> {
    let links: Vec<&EvidenceLink> = inventory::iter::<EvidenceLink>().collect();
    let root_name = links
        .iter()
        .find(|link| link.name.ends_with(subject))
        .ok_or_else(|| ChainError::not_found(subject))?
        .name;
    let mut visiting = HashSet::new();
    let full_root = build_node(root_name, &links, &mut visiting);

    let required: Vec<String> = match verifiers {
        Some(list) => list.iter().map(|v| (*v).to_owned()).collect(),
        None => discover_verifiers(&full_root),
    };

    let mut gaps = Vec::new();
    collect_gaps(&full_root, &required, &mut gaps);

    if !gaps.is_empty() {
        return Err(ChainError::incomplete(root_name, required, gaps));
    }

    let root = filter_node(full_root, &required);

    Ok(ProofChainReport {
        subject: root_name,
        verifiers: required,
        root,
    })
}

/// Narrow a node's (already-complete) proofs down to just `required`, so
/// a chain explicitly scoped to one verifier doesn't also print proofs
/// for others that happen to be registered alongside it.
fn filter_node(node: ChainNode, required: &[String]) -> ChainNode {
    ChainNode {
        evidence: node.evidence,
        proofs: node
            .proofs
            .into_iter()
            .filter(|(verifier, _)| required.iter().any(|r| r == verifier))
            .collect(),
        bases: node
            .bases
            .into_iter()
            .map(|basis| filter_node(basis, required))
            .collect(),
    }
}

fn build_node(
    name: &'static str,
    links: &[&EvidenceLink],
    visiting: &mut HashSet<&'static str>,
) -> ChainNode {
    let proofs = inventory::iter::<ProofRecord>()
        .filter(|record| record.evidence == name)
        .map(|record| (record.verifier, (record.describe)()))
        .collect();

    let mut bases = Vec::new();

    if visiting.insert(name) {
        let mut matching: Vec<&&EvidenceLink> =
            links.iter().filter(|link| link.name == name).collect();
        // `inventory`'s iteration order isn't guaranteed to match
        // registration order, so a calculation's arguments could otherwise
        // come out in an arbitrary order rather than `add(a, b)`'s a-then-b.
        matching.sort_by_key(|link| link.index);

        for link in matching {
            // A root registers a link to itself; that's a marker, not a
            // real basis to recurse into.
            if link.basis != link.name {
                bases.push(build_node(link.basis, links, visiting));
            }
        }
        visiting.remove(name);
    }

    ChainNode {
        evidence: name,
        proofs,
        bases,
    }
}

/// Every distinct verifier with a registered proof anywhere in the tree.
fn discover_verifiers(node: &ChainNode) -> Vec<String> {
    let mut found = Vec::new();
    collect_verifiers(node, &mut found);
    found
}

fn collect_verifiers(node: &ChainNode, found: &mut Vec<String>) {
    for (verifier, _) in &node.proofs {
        if !found.iter().any(|seen| seen == verifier) {
            found.push((*verifier).to_owned());
        }
    }

    for basis in &node.bases {
        collect_verifiers(basis, found);
    }
}

fn collect_gaps(node: &ChainNode, required: &[String], gaps: &mut Vec<ChainGap>) {
    for verifier in required {
        if !node.proofs.iter().any(|(v, _)| v == verifier) {
            gaps.push(ChainGap {
                evidence: node.evidence,
                verifier: verifier.clone(),
            });
        }
    }

    for basis in &node.bases {
        collect_gaps(basis, required, gaps);
    }
}
