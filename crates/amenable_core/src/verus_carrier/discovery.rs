//! Find and parse real Verus carrier functions from sibling source files.

use std::{
    fs,
    path::{Path, PathBuf},
};

use verus_syn::parse::{Parse, ParseStream};

/// Mirrors `verus_builtin_macros::syntax::Items` (private to that
/// crate): a bare sequence of items, exactly what sits inside a
/// `verus! { ... }` macro body.
struct Items {
    items: Vec<verus_syn::Item>,
}

impl Parse for Items {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(input)))]
    fn parse(input: ParseStream) -> verus_syn::parse::Result<Items> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Items { items })
    }
}

/// Root directory real carrier files live under, resolved relative to
/// this crate's own manifest -- the same relative depth
/// `amenable::paths::verus_source_directory` already uses for the
/// sibling crate (`amenable_core`, like `amenable`, sits directly under
/// `crates/`).
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn carrier_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_verus/src")
}

/// Every `.rs` file under `root`, recursively.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }

    files
}

/// Parse one real carrier file's `verus! { ... }` body into a sequence
/// of `verus_syn` items.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(path)))]
fn parse_carrier_items(path: &Path) -> Option<Vec<verus_syn::Item>> {
    let source = fs::read_to_string(path).ok()?;
    let file: syn::File = syn::parse_file(&source).ok()?;

    let verus_macro_tokens = file.items.iter().find_map(|item| match item {
        syn::Item::Macro(item_macro) if item_macro.mac.path.is_ident("verus") => {
            Some(item_macro.mac.tokens.clone())
        }
        _ => None,
    })?;

    let items: Items = verus_syn::parse2(verus_macro_tokens).ok()?;
    Some(items.items)
}

/// Find the real, public `pub fn`/`pub open spec fn` named `name`
/// anywhere under `amenable_verus/src`, returning its defining file and
/// crate-relative module path.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub fn find_fn(name: &str) -> Option<(PathBuf, String, verus_syn::ItemFn)> {
    let root = carrier_root();

    for path in rust_files(&root) {
        let Some(items) = parse_carrier_items(&path) else {
            continue;
        };

        for item in items {
            if let verus_syn::Item::Fn(item_fn) = item
                && item_fn.sig.ident == name
                && matches!(item_fn.vis, verus_syn::Visibility::Public(_))
            {
                // `path` came from walking `root` itself (`rust_files(&root)`
                // above), so `module_path_for` can only fail here if that
                // walk is broken -- in which case there's no lawful module
                // path to report, so keep searching rather than trust an
                // unreachable branch never to fire.
                let Ok(module_path) = super::module_path_for(&root, &path, name) else {
                    continue;
                };
                return Some((path, module_path, item_fn));
            }
        }
    }

    None
}
