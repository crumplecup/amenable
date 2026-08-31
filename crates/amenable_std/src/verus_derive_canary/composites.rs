//! Composite canaries built from [`super::leaves`]' atomic building
//! blocks, and the `emit_verus_witnesses!` registration that exports the
//! whole set to the Verus derive-witness composition pipeline.

use amenable_core::Provenance;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use super::leaves::{
    CheckedVerusExportLeaf, RawTemplateVerusExportLeaf, RequiresVerusExportLeaf,
    TrivialVerusExportLeaf, TrustedVerusExportLeaf,
};

type ConcreteVerusExportCanaryStruct = VerusExportCheckedPlusTrivialStruct<CheckedVerusExportLeaf>;
type ConcreteVerusExportTupleStruct =
    VerusExportTupleStruct<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;
type ConcreteVerusExportCanaryEnum =
    VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;

crate::emit_verus_witnesses!(
    ConcreteVerusExportCanaryStruct,
    ConcreteVerusExportTupleStruct,
    VerusExportRequiresStruct,
    VerusExportRawTemplateStruct,
    ConcreteVerusExportCanaryEnum,
    VerusExportMultiCheckedStruct,
    VerusExportNestedStruct,
    VerusExportMultiCheckedEnum,
);

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_requires_struct_witness"))]
struct VerusExportRequiresStruct {
    checked: RequiresVerusExportLeaf,
}

/// Two checked leaves with different real harnesses in one composite —
/// exercises the derive-witness composition renderer's multi-checked-
/// call tuple path (`-> (result: (T1, T2))`, `result.0`/`result.1`
/// citations) against the real `verus` tool for the first time: every
/// other checked-leaf canary so far has had exactly one checked field,
/// so this path was previously only exercised by `RefCell`'s own single
/// leaf, whose real return type already happens to be a tuple -- never
/// by two genuinely independent checked calls composed into one.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_multi_checked_struct_witness"))]
struct VerusExportMultiCheckedStruct {
    first: CheckedVerusExportLeaf,
    second: RequiresVerusExportLeaf,
}

/// A composite field that is itself another already-composed `Witness`
/// type, not a bare leaf -- confirms the derive macro and the
/// `amenable::verus_export` renderer both recurse correctly past depth
/// 1, which no canary so far has exercised (every existing composite's
/// own fields have all been leaves).
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_nested_struct_witness"))]
struct VerusExportNestedStruct {
    inner: VerusExportRequiresStruct,
    trusted: TrustedVerusExportLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_raw_template_struct_witness"))]
struct VerusExportRawTemplateStruct {
    checked: RawTemplateVerusExportLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(
    module = "crate::derived_witness::verus_export_checked_plus_trivial_struct_witness"
))]
struct VerusExportCheckedPlusTrivialStruct<TChecked: Provenance + Clone + Default> {
    checked: TChecked,
    marker: TrivialVerusExportLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_tuple_struct_witness"))]
struct VerusExportTupleStruct<
    TChecked: Provenance + Clone + Default,
    TTrusted: Provenance + Clone + Default,
>(
    TChecked,
    #[provenance(rename = "trusted")] TTrusted,
    #[provenance(rename = "marker")] TrivialVerusExportLeaf,
);

/// An enum-shaped derive-witness composition canary: one variant mixing
/// a checked and a trusted leaf, one variant carrying only a trusted
/// leaf (plus a skipped checked one), and one leaf-free variant —
/// exercises named-variant, tuple-variant, and unit-variant derive
/// codegen in a single registered export.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_canary_enum_witness"))]
pub enum VerusExportCanaryEnum<
    TChecked: Provenance + Clone + Default,
    TTrusted: Provenance + Clone + Default,
> {
    /// Mixes a generic checked leaf with a fixed trusted leaf.
    Balanced {
        /// The checked leaf.
        checked: TChecked,
        /// The trusted leaf.
        trusted: TrustedVerusExportLeaf,
    },
    /// Carries a generic trusted leaf and a fixed checked leaf (skipped
    /// from provenance metadata) — the fallback path.
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] CheckedVerusExportLeaf,
    ),
    /// Carries no leaves at all.
    #[default]
    Closed,
}

amenable_derive::verus_ensures_predicate!(
    ConcreteVerusExportCanaryEnum,
    "amenable_std::verus_derive_canary::VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>",
    "verus_export_canary_enum_witness_ensures_holds"
);

/// One enum variant carrying two checked leaves with different real
/// harnesses — exercises `render_enum_module`'s multi-checked-call
/// bind-name path (`r0`/`r1`, not the single-`r` path) against the real
/// `verus` tool for the first time: `VerusExportCanaryEnum::Balanced`
/// above has always had exactly one checked leaf per variant.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "state")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_multi_checked_enum_witness"))]
pub enum VerusExportMultiCheckedEnum {
    /// Two independent checked leaves in one variant.
    Active {
        /// The first checked leaf.
        first: CheckedVerusExportLeaf,
        /// The second checked leaf.
        second: RequiresVerusExportLeaf,
    },
    /// Carries no leaves at all.
    #[default]
    Idle,
}

amenable_derive::verus_requires_predicate!(
    VerusExportMultiCheckedEnum,
    "amenable_std::verus_derive_canary::VerusExportMultiCheckedEnum",
    "verus_export_multi_checked_enum_witness_requires_holds"
);

amenable_derive::verus_ensures_predicate!(
    VerusExportMultiCheckedEnum,
    "amenable_std::verus_derive_canary::VerusExportMultiCheckedEnum",
    "verus_export_multi_checked_enum_witness_ensures_holds"
);
