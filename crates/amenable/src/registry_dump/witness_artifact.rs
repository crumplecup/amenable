//! JSON-serializable shapes for [`crate::WitnessExportRecord`] and the
//! recursive [`crate::WitnessArtifactNode`] tree it carries.

use tracing::instrument;

/// One explicit [`crate::WitnessExportRecord`], owned for JSON
/// serialization.
#[derive(serde::Serialize)]
pub(super) struct WitnessExportRecordDump {
    pub(super) verifier: String,
    pub(super) evidence: String,
    pub(super) destination_module: String,
    pub(super) support_kind: String,
    pub(super) trivial: usize,
    pub(super) checked: usize,
    pub(super) trusted: usize,
    pub(super) opaque: usize,
    pub(super) artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact node, owned for JSON serialization.
#[derive(serde::Serialize)]
pub(super) struct WitnessArtifactNodeDump {
    shape: String,
    kind: String,
    tag: Option<String>,
    variant: Option<String>,
    detail: Option<String>,
    metadata: Vec<WitnessArtifactMetadataDump>,
    support_kind: String,
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
    members: Vec<WitnessArtifactMemberDump>,
    variants: Vec<WitnessArtifactVariantDump>,
}

/// One named witness artifact member, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMemberDump {
    label: String,
    artifact: WitnessArtifactNodeDump,
}

/// One named witness artifact variant, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactVariantDump {
    name: String,
    artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact metadata fact, owned for JSON
/// serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMetadataDump {
    key: String,
    value: String,
}

#[instrument(level = "debug", skip(node))]
pub(super) fn dump_witness_artifact(node: crate::WitnessArtifactNode) -> WitnessArtifactNodeDump {
    let (shape, support, kind, tag, variant, detail, metadata, members, variants) = node.dissolve();

    WitnessArtifactNodeDump {
        shape: shape.as_str().to_owned(),
        kind: kind.as_str().to_owned(),
        tag,
        variant,
        detail,
        metadata: metadata
            .into_iter()
            .map(|entry| WitnessArtifactMetadataDump {
                key: entry.key().to_owned(),
                value: entry.value().to_owned(),
            })
            .collect(),
        support_kind: support.kind().as_str().to_owned(),
        trivial: support.trivial(),
        checked: support.checked(),
        trusted: support.trusted(),
        opaque: support.opaque(),
        members: members
            .into_iter()
            .map(|member| {
                let (label, artifact) = member.dissolve();
                WitnessArtifactMemberDump {
                    label,
                    artifact: dump_witness_artifact(*artifact),
                }
            })
            .collect(),
        variants: variants
            .into_iter()
            .map(|variant| {
                let (name, artifact) = variant.dissolve();
                WitnessArtifactVariantDump {
                    name,
                    artifact: dump_witness_artifact(*artifact),
                }
            })
            .collect(),
    }
}
