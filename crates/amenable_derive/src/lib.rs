//! Proc macros for the `amenable` constitutional trait family.

mod calculation;
mod capture_exchange_body;
mod establish;
mod evidence;
mod exchange;
mod harness;
mod kani_compose;
mod proof_token;
mod sidecar;
mod standard;
mod state_machine;
#[cfg(feature = "verus")]
mod verus_contract;
#[cfg(feature = "verus")]
mod verus_fragment;
mod witness;

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, Index, ItemFn, ItemImpl,
    ItemStruct, LitStr, Path, Type, Variant, WherePredicate, parse_macro_input, parse_quote,
};

use calculation::{CalculationArgs, expand_calculation};
use capture_exchange_body::{CaptureExchangeBodyArgs, expand_capture_exchange_body};
use establish::{EstablishArgs, expand_establish};
use evidence::{expand_evidence, expand_evidence_derive};
use exchange::{ExchangeArgs, expand_exchange};
use harness::expand_harness;
use kani_compose::expand_kani_compose;
use proof_token::expand_proof_token;
use sidecar::expand_sidecar;
use standard::expand_standard;
use state_machine::expand_state_machine;
use witness::expand_witness;

/// Define a `#[cfg(...)]`-gated proof harness item and, alongside it, an
/// always-available `&'static str` constant holding the harness's verbatim
/// source — whitespace and all.
///
/// `harness!(cfg_name, CONST_NAME, { item })` expands to `#[cfg(cfg_name)]
/// item` plus `const CONST_NAME: &str = "...";`. Both come from the same
/// braced group of tokens (captured via `Span::source_text`, which needs a
/// contiguous, human-authored span — this is why the item must be written
/// directly at the call site, not threaded through an intermediate
/// `macro_rules!` layer), so an audit report can show a proof exactly as
/// its author wrote it, and the two can never drift apart the way a
/// hand-maintained description could. Falls back to reconstructing the
/// source from tokens (losing original formatting) if `source_text` is
/// unavailable for the span.
#[proc_macro]
pub fn harness(input: TokenStream) -> TokenStream {
    match expand_harness(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(Provenance, attributes(provenance))]
pub fn derive_provenance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_provenance(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Turn a method into a chain link in the evidence graph: it knows it has a
/// method, knows it yields a token (named here), and registers itself.
#[proc_macro_attribute]
pub fn calculation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CalculationArgs);
    let input = parse_macro_input!(item as ItemFn);

    match expand_calculation(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generalizes the by-hand `Exchange` transition pattern proven in
/// `amenable_kani::stoplight` (see `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
/// Step 3 and Step 6): given `impl SelfType { fn method(&self, input:
/// Input) -> Result<Output, Error> { .. } }` — the real inherent method,
/// its body left exactly as authored — generates the `Witness<V>` impl for
/// `evidence` naming the given harness, the `ProofRecord` registration
/// backing it, the `Exchange<Input, Output, V>` impl that delegates to
/// `method`, and (Step 6) `method`'s own `#[cfg_attr(.., kani::ensures(..))]`
/// contract, calling through `evidence`'s own, separately-registered
/// `Ensures<V>` impl (`kani_ensures!`, still hand-written — the actual
/// predicate lives there, not here) rather than requiring it re-typed at
/// every call site. Deliberately does not touch or generate the
/// `harness! { .. }` invocation itself, since that macro's verbatim-source
/// capture only works when written directly at its own call site.
#[proc_macro_attribute]
pub fn exchange(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ExchangeArgs);
    let input = parse_macro_input!(item as ItemImpl);

    match expand_exchange(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `#[capture_exchange_body(evidence = "..", creusot_ensures = "..")]` on
/// `impl SelfType { fn method(&self, input: Input) -> Result<Output,
/// Error> { .. } }` — registers a real `ExchangeEdgeRecord` from the
/// method's own real body, verbatim, leaving the method itself
/// completely untouched. See `capture_exchange_body.rs`'s own doc
/// comment for why this is a separate, narrower macro from `#[exchange(
/// ..)]` rather than a mode of it: `GAAP_LEDGER_PLAN.md`'s Step 7 moved
/// `Ledger`'s own methods to a neutral crate with a fully generic
/// `Ensures<V>` bound, so there is no concrete verifier left for
/// `#[exchange(..)]`'s own contract/`Witness<V>`/`Exchange<..>` bundle to
/// name.
#[proc_macro_attribute]
pub fn capture_exchange_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CaptureExchangeBodyArgs);
    let input = parse_macro_input!(item as ItemImpl);

    match expand_capture_exchange_body(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Compute `is_root` for a hand-written `impl Evidence` block from its own
/// `Basis` declaration, at compile time — no `TypeId`, no `'static`. For a
/// hand-written impl with real, non-trivial `basis()`/`audit()` bodies; see
/// [`derive_evidence`] for the common trivial-root case, which needs no
/// hand-written impl at all.
#[proc_macro_attribute]
pub fn evidence(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    match expand_evidence(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate a real, provable-not-asserted root `Evidence` impl (`Basis` as
/// named, `Audit = ()`, `is_root()` computed the same way [`evidence`]
/// does) from a `#[evidence(basis = "..", basis_ctor = "..", bound =
/// "..")]` attribute, plus the same `EvidenceLink` auto-registration
/// [`derive_standard`] already does for its own root, non-generic case —
/// see `evidence.rs`'s own doc comment for the full rationale and the real
/// duplication (and missing registry entries) this closes.
#[proc_macro_derive(Evidence, attributes(evidence))]
pub fn derive_evidence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_evidence_derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate both `Standard` and `Evidence` impls from a `#[standard(...)]`
/// attribute, since they always share the same provenance value.
#[proc_macro_derive(Standard, attributes(standard))]
pub fn derive_standard(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_standard(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate `impl ProofToken for X { type Proposition = Y; }` from a
/// `#[proof_token(proposition = "Y")]` attribute -- see [`proof_token`]'s
/// own doc comment.
#[proc_macro_derive(ProofToken, attributes(proof_token))]
pub fn derive_proof_token(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_proof_token(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate the trivial-token-minting half of `impl Establish<C, V> for Y`
/// from a `#[establish(credential = .., verifier = .., proposition = ..)]`
/// attribute on the token struct -- see [`establish`]'s own doc comment
/// for why this is an attribute macro, not a derive.
#[proc_macro_attribute]
pub fn establish(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EstablishArgs);
    let input = parse_macro_input!(item as ItemStruct);

    match expand_establish(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate `impl Sidecar<V> for X { .. }` plus a `new(..)` constructor
/// from `#[sidecar(verifier = .., proposition = .., constructor = ..)]`
/// and `#[sidecar(primary)]`/`#[sidecar(token)]` field markers -- see
/// [`sidecar`]'s own doc comment.
#[proc_macro_derive(Sidecar, attributes(sidecar))]
pub fn derive_sidecar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_sidecar(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `#[derive(StateMachine)]` -- see [`state_machine`]'s own doc comment
/// for the full `#[state_machine(verifier = .., state(..), edge(..))]`
/// syntax. Step 1 of `docs/STATE_MACHINE_DERIVATION_PLAN.md`: emits only
/// the compiler-enforced static assertions, one per declared edge.
#[proc_macro_derive(StateMachine, attributes(state_machine))]
pub fn derive_state_machine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_state_machine(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(KaniCompose)]
pub fn derive_kani_compose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_kani_compose(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(Witness, attributes(provenance, witness))]
pub fn derive_witness(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_witness(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Expand a real Verus harness name to a `&'static [&'static str]` array
/// literal of its real `ensures` clauses, extracted from the real
/// carrier source at compile time -- a missing harness or clause is a
/// real compile error here, not a runtime failure discovered later.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_fragments(input: TokenStream) -> TokenStream {
    match verus_fragment::expand_verus_fragments(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_fragments!`], for a harness's real `requires`
/// clauses instead.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_fragments(input: TokenStream) -> TokenStream {
    match verus_fragment::expand_verus_fragments(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `verus_ensures_witness!(Type, evidence_expr, "harness")` generates a
/// real `impl Ensures<crate::VerusVerifier> for Type` (`Bound =
/// &'static [&'static str]`, one real clause per element) plus one
/// `ContractRecord` registration per clause -- see
/// [`verus_contract`]'s own doc comment for why `Bound` is a slice
/// and why the registration is generated rather than hand-written.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_witness(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_witness(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_witness!`], for `Requires` instead.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_witness(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_witness(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `verus_ensures_predicate!(Type, evidence_expr, "predicate_name")` --
/// like [`verus_ensures_witness!`], but for a claim that's a real,
/// named `pub open spec fn`'s own declaration (shared across several
/// different harnesses/carrier files) rather than any one harness's own
/// clause list. See [`verus_contract`]'s own doc comment.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_predicate(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_predicate_witness(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_predicate!`], for `Requires` instead.
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_predicate(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_predicate_witness(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct ProvenanceContainerOptions {
    crate_path: Path,
    tag: String,
}

impl Default for ProvenanceContainerOptions {
    fn default() -> Self {
        Self {
            crate_path: parse_quote!(amenable_core),
            tag: "variant".to_string(),
        }
    }
}

#[derive(Default)]
struct WitnessContainerOptions {
    verus_module: Option<String>,
}

#[derive(Default)]
struct MemberOptions {
    rename: Option<String>,
    skip: bool,
}

fn expand_provenance(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_provenance_container_options(&input.attrs)?;
    let name = &input.ident;
    let crate_path = &options.crate_path;
    let field_types = collect_field_types(&input.data)?;
    let mut generics = input.generics.clone();
    let where_clause = generics.make_where_clause();

    for field_type in field_types {
        let predicate: WherePredicate = parse_quote!(#field_type: ::#crate_path::Provenance);
        where_clause.predicates.push(predicate);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let metadata_body = match &input.data {
        Data::Struct(data) => expand_struct_metadata(crate_path, data)?,
        Data::Enum(data) => expand_enum_metadata(crate_path, data, &options)?,
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "Provenance can only be derived for structs and enums",
            ));
        }
    };

    Ok(quote! {
        impl #impl_generics ::#crate_path::Provenance for #name #ty_generics #where_clause {
            type MetadataIter = ::std::boxed::Box<dyn ::core::iter::Iterator<Item = ::#crate_path::MetadataEntry>>;

            fn metadata(&self) -> Self::MetadataIter {
                ::std::boxed::Box::new({ #metadata_body })
            }
        }
    })
}

fn expand_struct_metadata(
    crate_path: &Path,
    data: &DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_pushes = expand_struct_field_pushes(crate_path, &data.fields)?;

    Ok(quote! {
        let mut entries = ::std::vec::Vec::new();
        #(#field_pushes)*
        entries.into_iter()
    })
}

fn expand_enum_metadata(
    crate_path: &Path,
    data: &DataEnum,
    options: &ProvenanceContainerOptions,
) -> syn::Result<proc_macro2::TokenStream> {
    let tag = &options.tag;
    let arms = data
        .variants
        .iter()
        .map(|variant| expand_variant_arm(crate_path, variant, tag))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

fn expand_variant_arm(
    crate_path: &Path,
    variant: &Variant,
    tag: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_member_options(&variant.attrs)?;
    let variant_name = options.rename.unwrap_or_else(|| variant.ident.to_string());
    let ident = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_idents = fields
                .named
                .iter()
                .map(|field| {
                    field.ident.clone().ok_or_else(|| {
                        Error::new_spanned(field, "named enum-field expansion requires identifiers")
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let field_pushes = fields
                .named
                .iter()
                .map(|field| {
                    let field_options = parse_member_options(&field.attrs)?;
                    if field_options.skip {
                        return Ok(None);
                    }

                    let field_ident = field.ident.as_ref().ok_or_else(|| {
                        Error::new_spanned(field, "named enum-field expansion requires identifiers")
                    })?;
                    let field_name = field_options
                        .rename
                        .unwrap_or_else(|| field_ident.to_string());

                    Ok(Some(quote! {
                        for entry in ::#crate_path::Provenance::metadata(#field_ident) {
                            let key = if entry.key() == "value" {
                                ::std::borrow::ToOwned::to_owned(#field_name)
                            } else {
                                ::std::format!("{}.{}", #field_name, entry.key())
                            };

                            entries.push(::#crate_path::MetadataEntry::new(
                                key,
                                entry.value().to_owned(),
                            ));
                        }
                    }))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident { #(#field_idents),* } => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries.into_iter()
                }
            })
        }
        Fields::Unnamed(fields) => {
            let field_bindings = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _field)| format_ident!("__field_{index}"))
                .collect::<Vec<_>>();
            let field_pushes = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_options = parse_member_options(&field.attrs)?;
                    if field_options.skip {
                        return Ok(None);
                    }

                    let field_name = field_options.rename.unwrap_or_else(|| index.to_string());
                    let field_binding = &field_bindings[index];

                    Ok(Some(expand_field_entries(
                        crate_path,
                        field_name,
                        quote!(#field_binding),
                    )))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident(#(#field_bindings),*) => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries.into_iter()
                }
            })
        }
        Fields::Unit => Ok(quote! {
            Self::#ident => {
                let mut entries = ::std::vec::Vec::new();
                entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                entries.into_iter()
            }
        }),
    }
}

fn expand_struct_field_pushes(
    crate_path: &Path,
    fields: &Fields,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "named-field expansion requires field identifiers")
                })?;
                expand_struct_field_push(crate_path, field, None, quote!(&self.#field_ident))
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let tuple_index = Index::from(index);
                expand_struct_field_push(crate_path, field, Some(index), quote!(&self.#tuple_index))
            })
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

fn expand_struct_field_push(
    crate_path: &Path,
    field: &Field,
    position: Option<usize>,
    field_access: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_member_options(&field.attrs)?;
    if options.skip {
        return Ok(quote! {});
    }

    Ok(expand_field_entries(
        crate_path,
        field_name(field, position)?,
        field_access,
    ))
}

fn expand_field_entries(
    crate_path: &Path,
    field_name: String,
    field_access: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        for entry in ::#crate_path::Provenance::metadata(#field_access) {
            let key = if entry.key() == "value" {
                ::std::borrow::ToOwned::to_owned(#field_name)
            } else {
                ::std::format!("{}.{}", #field_name, entry.key())
            };

            entries.push(::#crate_path::MetadataEntry::new(
                key,
                entry.value().to_owned(),
            ));
        }
    }
}

fn collect_field_types(data: &Data) -> syn::Result<Vec<Type>> {
    match data {
        Data::Struct(data) => collect_field_types_from_fields(&data.fields),
        Data::Enum(data) => data
            .variants
            .iter()
            .map(|variant| collect_field_types_from_fields(&variant.fields))
            .collect::<syn::Result<Vec<_>>>()
            .map(|groups| groups.into_iter().flatten().collect()),
        Data::Union(data) => Err(Error::new_spanned(
            data.union_token,
            "Provenance can only be derived for structs and enums",
        )),
    }
}

fn collect_field_types_from_fields(fields: &Fields) -> syn::Result<Vec<Type>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(collect_field_type)
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .map(collect_field_type)
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

fn collect_field_type(field: &Field) -> syn::Result<Option<Type>> {
    let options = parse_member_options(&field.attrs)?;

    if options.skip {
        return Ok(None);
    }

    Ok(Some(field.ty.clone()))
}

fn field_name(field: &Field, position: Option<usize>) -> syn::Result<String> {
    let options = parse_member_options(&field.attrs)?;
    if let Some(rename) = options.rename {
        return Ok(rename);
    }

    match (&field.ident, position) {
        (Some(ident), _) => Ok(ident.to_string()),
        (None, Some(index)) => Ok(index.to_string()),
        (None, None) => Err(Error::new_spanned(
            field,
            "tuple fields require an explicit position",
        )),
    }
}

fn parse_provenance_container_options(
    attrs: &[syn::Attribute],
) -> syn::Result<ProvenanceContainerOptions> {
    let mut options = ProvenanceContainerOptions::default();

    for attr in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("provenance"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                options.crate_path = value.parse()?;
                return Ok(());
            }

            if meta.path.is_ident("tag") {
                let value: LitStr = meta.value()?.parse()?;
                options.tag = value.value();
                return Ok(());
            }

            Err(meta.error("unsupported provenance container attribute"))
        })?;
    }

    Ok(options)
}

fn parse_witness_container_options(
    attrs: &[syn::Attribute],
) -> syn::Result<WitnessContainerOptions> {
    let mut options = WitnessContainerOptions::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("witness")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("verus") {
                meta.parse_nested_meta(|meta| {
                    if meta.path.is_ident("module") {
                        let value: LitStr = meta.value()?.parse()?;
                        options.verus_module = Some(value.value());
                        return Ok(());
                    }

                    Err(meta.error("unsupported witness verus attribute"))
                })?;

                return Ok(());
            }

            Err(meta.error("unsupported witness container attribute"))
        })?;
    }

    Ok(options)
}

fn parse_member_options(attrs: &[syn::Attribute]) -> syn::Result<MemberOptions> {
    let mut options = MemberOptions::default();

    for attr in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("provenance"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                options.rename = Some(value.value());
                return Ok(());
            }

            if meta.path.is_ident("skip") {
                options.skip = true;
                return Ok(());
            }

            Err(meta.error("unsupported provenance field or variant attribute"))
        })?;
    }

    Ok(options)
}
