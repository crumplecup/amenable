//! Emit derived Verus proof-token companions from the real registry.
//!
//! `GAAP_LEDGER_PLAN.md`'s Step 8: the same "codegen over hand-mirroring"
//! fix `creusot_export`/`verus_exchange_export` already applied to
//! captured method bodies, extended to a new capture target -- a token
//! type's own shape plus its `ProofToken`/`Establish` impls, mechanically
//! generated content that was previously hand-written a second time in
//! `amenable_verus::gallery::ledger_exchange` purely because `verus
//! --crate-type=lib` can resolve no external crate at all (not even a
//! proc-macro one, unlike `cargo creusot`), so neither the real token type
//! in `amenable_gaap` nor its real, backend-generic `Establish` impl can
//! ever be named from Verus's own gallery code. Reads `amenable_core::
//! ProofTokenMintRecord`, registered by `#[derive(amenable_derive::
//! ProofToken)]` and `#[amenable_derive::establish(..)]`'s verifier-less
//! form -- the same registry shape `creusot_export`/`verus_exchange_
//! export` already read for `ExchangeEdgeRecord`, applied here instead.
//!
//! **Explicit allowlist, matching `edge_group`'s own precedent exactly.**
//! `#[derive(ProofToken)]` is used pervasively across the workspace
//! (`stoplight.rs`'s `GreenToken`/`YellowToken`/`RedToken`, the much
//! larger `rust_std` corpus) -- every one of those registers a real
//! `ProofTokenMintRecord` too, since the derive can't tell which of its
//! callers will ever want a Verus companion. [`TOKENS`] names only the
//! three this gallery module actually needs (`PendingToken`/
//! `ValidatedToken`/`CommittedToken`), the same "an honest table beats a
//! formula that would happen to produce the wrong answer" reasoning
//! `verus_exchange_export::edge_group`'s own doc comment gives.
//!
//! **Two registrations can exist for the same token, by design.**
//! `#[derive(ProofToken)]` always registers one (`credential: None`);
//! `#[amenable_derive::establish(..)]`'s verifier-less form, when also
//! present on the same struct, registers a second, strictly richer one
//! (`credential: Some(..)`) -- real, hand-written `Establish` impls (a
//! root token like `PendingToken`, minted with no credential at all) only
//! ever produce the first. [`gather_tokens`] keeps the richer record when
//! both exist for the same `token` name.
//!
//! **Target verifier is hardcoded to `GalleryVerifier`, not derived from
//! the registry.** Every gallery case defines its own local verifier
//! marker (see `gallery::evidence_self_referential_root`'s own doc
//! comment for why); `ProofTokenMintRecord` carries no verifier field at
//! all (Kani's real tokens are backend-generic over any `V`), so the one
//! concrete name this file's own `Establish` impl needs to target is
//! supplied here, the identical thing `verus_exchange_export::render_
//! companion` already does for `verus_exchange!`'s own generated calls.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use amenable_core::ProofTokenMintRecord;

use crate::{AmenableError, AmenableResult};

/// The tokens this gallery module's generated companion covers -- see
/// this module's own doc comment for why an explicit table, not every
/// `ProofTokenMintRecord` in the registry. `RejectedFromPendingToken`/
/// `RejectedFromValidatedToken` joined in a later pass (`GAAP_LEDGER_
/// PLAN.md`'s Step 7, revisited), once `reject`/`rollback` themselves
/// were connected here too.
const TOKENS: &[&str] = &[
    "PendingToken",
    "ValidatedToken",
    "CommittedToken",
    "RejectedFromPendingToken",
    "RejectedFromValidatedToken",
];

/// One token's real shape, gathered from the registry -- see
/// [`gather_tokens`].
struct TokenSpec {
    token: String,
    proposition: String,
    credential: Option<String>,
}

/// Write one generated Verus token companion, covering every token named
/// in [`TOKENS`], to `path` (e.g. `amenable_verus/src/gallery/generated/
/// ledger_tokens.rs`). A token named in [`TOKENS`] with no registration at
/// all is a real error (a rename or a dropped derive this codegen should
/// catch, not silently skip), matching `creusot_export`'s/`verus_
/// exchange_export`'s own "no dead files, no silent guesses" discipline
/// in the opposite direction: those skip an edge with no allowlist entry;
/// this requires every allowlist entry to have a real registration.
pub fn write_verus_gaap_token_companion(path: &Path) -> AmenableResult<PathBuf> {
    let gathered = gather_tokens();
    let mut specs = Vec::with_capacity(TOKENS.len());
    for &token in TOKENS {
        let Some(spec) = gathered.get(token) else {
            return Err(AmenableError::invariant(format!(
                "no ProofTokenMintRecord registered for {token} (named in verus_gaap_tokens_export::TOKENS)"
            )));
        };
        specs.push(spec);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
    }
    let source = render_companion(&specs);
    fs::write(path, source).map_err(|error| AmenableError::io(path, error))?;
    Ok(path.to_owned())
}

/// Collect every registered `ProofTokenMintRecord`, keyed by token name --
/// keeping the richer (`credential: Some(..)`) record when two exist for
/// the same name, per this module's own doc comment.
fn gather_tokens() -> BTreeMap<&'static str, TokenSpec> {
    let mut gathered: BTreeMap<&'static str, TokenSpec> = BTreeMap::new();

    for record in inventory::iter::<ProofTokenMintRecord>() {
        let ProofTokenMintRecord {
            token,
            proposition,
            credential,
        } = *record;

        let richer =
            !matches!(gathered.get(token), Some(existing) if existing.credential.is_some());
        if richer {
            gathered.insert(
                token,
                TokenSpec {
                    token: token.to_string(),
                    proposition: proposition.to_string(),
                    credential: credential.map(str::to_string),
                },
            );
        }
    }

    gathered
}

/// Render the generated companion: its own `verus_builtin_macros::verus!
/// { .. }` wrapper (real, not decorative -- `verus! { .. }` is a
/// function-like proc-macro, so it only ever sees literal tokens written
/// directly inside its own braces; an `include!` sitting inside someone
/// *else's* `verus! { .. }` block hands it an opaque, unexpanded `include
/// ! (..)` call it cannot see through at all, confirmed against the real
/// toolchain: `cannot use type PendingToken which is ignored because it
/// is either declared outside the verus! macro`, even though the
/// generated struct really was textually spliced inside the block by the
/// time ordinary `rustc` macro expansion got to it -- `verus!`'s own scan
/// already ran and decided by then. Matching `crate::exchange_support::
/// verus_exchange!`'s/`verus_sidecar!`'s own precedent instead: wrap this
/// file's own content in its own `verus!` invocation, and `include!` the
/// whole thing at plain module scope, outside any hand-written `verus! {
/// .. }` block -- ordinary Rust item visibility doesn't care about
/// textual position within a module, so `Pending`/`Validated`/
/// `Committed`/`GalleryVerifier` (declared inside `gallery::
/// ledger_exchange`'s own hand-written block) stay reachable from here
/// regardless), for each token, its struct definition and `ProofToken`
/// impl (always), plus its `Establish<Credential, GalleryVerifier>` impl
/// (only when [`TokenSpec::credential`] is `Some` -- a root token like
/// `PendingToken` has none).
fn render_companion(specs: &[&TokenSpec]) -> String {
    let mut out = String::from(
        "// AUTO-GENERATED by `amenable emit-verus-gaap-tokens` -- do not\n\
         // hand-edit; regenerate with `just generate-verus-gaap-tokens`.\n\
         // Source: `amenable_gaap`'s own real token types, mechanically\n\
         // reconstructed from `amenable_core::ProofTokenMintRecord` -- see\n\
         // `amenable::verus_gaap_tokens_export`'s own doc comment. Plain\n\
         // `//`, not `//!`: this file is `include!`d mid-file into its own\n\
         // gallery module's scope, not its own module -- an inner doc\n\
         // comment there is a real compile error (E0753), the identical\n\
         // reason `creusot_export`/`verus_exchange_export` use plain `//`\n\
         // too.\n\n\
         verus_builtin_macros::verus! {\n\n",
    );

    for spec in specs {
        let proposition = tidy_stringified_type(&spec.proposition);
        out.push_str(&format!(
            "#[derive(Clone, Copy)]\npub struct {token};\n\nimpl ProofToken for {token} {{\n    type Proposition = {proposition};\n}}\n\n",
            token = spec.token,
        ));

        if let Some(credential) = &spec.credential {
            out.push_str(&format!(
                "impl Establish<{credential}, GalleryVerifier> for {proposition} {{\n    type Token = {token};\n\n    fn establish(_credential: {credential}) -> Self::Token {{\n        {token}\n    }}\n}}\n\n",
                credential = credential,
                token = spec.token,
            ));
        }
    }

    out.push_str("} // verus!\n");
    out
}

/// `stringify!(#ty)`'s one-space-per-token join (`Rejected < Pending >`)
/// survives untouched into the generated output otherwise -- the
/// identical real reason `creusot_export`'s/`verus_exchange_export`'s
/// own versions of this function document (`rustfmt` doesn't reformat
/// inside a macro invocation's own token tree, and this file's whole
/// generated body sits inside one `verus_builtin_macros::verus! { .. }`
/// call). Every proposition before `Rejected<Pending>`/`Rejected<
/// Validated>` joined `TOKENS` was a bare identifier (`Pending`/
/// `Validated`/`Committed`), so this was silently correct-by-coincidence
/// until a generic one arrived. Byte-for-byte the same fix as the other
/// two generators' own copies, duplicated rather than shared for the
/// identical reason theirs are: three structurally different renderers,
/// each with its own `AmenableResult` plumbing, not worth a shared
/// module yet.
fn tidy_stringified_type(stringified: &str) -> String {
    let mut tidied = String::with_capacity(stringified.len());
    let mut chars = stringified.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' if chars.peek() == Some(&'<') => {}
            '<' => {
                tidied.push(ch);
                while chars.peek() == Some(&' ') {
                    chars.next();
                }
            }
            ' ' if matches!(chars.peek(), Some(',') | Some('>')) => {}
            _ => tidied.push(ch),
        }
    }

    tidied
}
