use std::collections::HashMap;

use amenable_core::{
    ErasedEntry, Metadata, MetadataEntry, MetadataRecord, OwnedEntry, OwnedMetadataReport,
};

/// A distinct value type, to prove `get_as` recovers the real type rather
/// than a stringification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rpm(u32);

impl std::fmt::Display for Rpm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rpm", self.0)
    }
}

fn motor_specs() -> MetadataRecord {
    MetadataRecord::from_iter([
        OwnedEntry::new("max_rpm", Rpm(12_000)),
        OwnedEntry::new("part_number", "M-17".to_string()),
    ])
}

#[test]
fn owned_entry_exposes_rendered_and_structured_views() {
    amenable_core::init_tracing();
    let entry = OwnedEntry::new("max_rpm", Rpm(12_000));

    assert_eq!(entry.key(), "max_rpm");
    assert_eq!(entry.value().to_string(), "12000 rpm");
    assert_eq!(entry.value_any().downcast_ref::<Rpm>(), Some(&Rpm(12_000)));
    assert!(entry.value_any().downcast_ref::<u32>().is_none());
}

#[test]
fn metadata_record_answers_queries() {
    amenable_core::init_tracing();
    let specs = motor_specs();

    assert_eq!(specs.len(), 2);
    assert!(!specs.is_empty());
    assert!(specs.contains_key("max_rpm"));
    assert!(!specs.contains_key("absent"));
    assert_eq!(specs.keys(), vec!["max_rpm", "part_number"]);
    assert_eq!(specs.values(), vec!["12000 rpm", "M-17"]);
}

#[test]
fn get_as_recovers_the_concrete_value_type() {
    amenable_core::init_tracing();
    let specs = motor_specs();

    assert_eq!(specs.get_as::<Rpm>("max_rpm"), Some(Rpm(12_000)));
    assert_eq!(
        specs.get_as::<String>("part_number"),
        Some("M-17".to_string())
    );
    // wrong type, right key
    assert_eq!(specs.get_as::<u32>("max_rpm"), None);
    // right type, wrong key
    assert_eq!(specs.get_as::<Rpm>("absent"), None);
}

#[test]
fn prefixed_namespaces_a_nested_entry_without_touching_its_value() {
    amenable_core::init_tracing();
    let nested: Vec<OwnedEntry> = motor_specs()
        .snapshot()
        .into_iter()
        .map(|entry| entry.prefixed("motor"))
        .collect();
    let widget = MetadataRecord::from_iter(nested);

    assert_eq!(widget.keys(), vec!["motor.max_rpm", "motor.part_number"]);
    // the structured value survives the re-keying
    assert_eq!(widget.get_as::<Rpm>("motor.max_rpm"), Some(Rpm(12_000)));
}

#[test]
fn report_renders_one_line_per_entry() {
    amenable_core::init_tracing();
    let specs = motor_specs();

    assert_eq!(
        specs.report().to_string(),
        "max_rpm: 12000 rpm\npart_number: M-17"
    );
    assert_eq!(MetadataRecord::new().report().to_string(), "(no metadata)");
    assert_eq!(
        OwnedMetadataReport::new(motor_specs()).to_string(),
        "max_rpm: 12000 rpm\npart_number: M-17"
    );
}

#[test]
fn metadata_entry_freezes_a_live_entry() {
    amenable_core::init_tracing();
    let frozen: Vec<MetadataEntry> = motor_specs()
        .snapshot()
        .iter()
        .map(MetadataEntry::from)
        .collect();

    assert_eq!(frozen[0].key(), "max_rpm");
    assert_eq!(frozen[0].value(), "12000 rpm");
    assert_eq!(frozen[1], MetadataEntry::new("part_number", "M-17"));
}

/// A `HashMap`-backed record: proves `Metadata` is pluggable and that a
/// custom `get` override works.
#[derive(Debug, Default)]
struct MapMetadata {
    entries: HashMap<String, OwnedEntry>,
}

impl MapMetadata {
    fn insert(&mut self, entry: OwnedEntry) {
        self.entries.insert(entry.key().to_owned(), entry);
    }
}

impl Metadata for MapMetadata {
    fn snapshot(&self) -> Vec<OwnedEntry> {
        let mut out: Vec<OwnedEntry> = self.entries.values().cloned().collect();
        out.sort_by(|a, b| a.key().cmp(b.key()));
        out
    }

    fn get(&self, key: &str) -> Option<OwnedEntry> {
        self.entries.get(key).cloned()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

/// `#[derive(Metadata)]` on a plain spec struct — the widget use case: a
/// composite queried by fully-qualified key, structured values back, no
/// `impl Provenance`.
#[derive(Debug, Clone, Default, amenable_derive::Metadata)]
#[metadata(crate = "amenable_core")]
struct MotorSchema {
    max_rpm: u32,
    part_number: String,
}

#[derive(Debug, Clone, Default, amenable_derive::Metadata)]
#[metadata(crate = "amenable_core")]
struct WidgetSchema {
    serial: String,
    motor: MotorSchema,
}

#[test]
fn derive_metadata_composes_and_stays_out_of_the_provenance_role() {
    amenable_core::init_tracing();
    let widget = WidgetSchema {
        serial: "W-9".to_string(),
        motor: MotorSchema {
            max_rpm: 15_000,
            part_number: "M-3".to_string(),
        },
    };

    assert_eq!(
        widget.keys(),
        vec!["serial", "motor.max_rpm", "motor.part_number"]
    );
    assert_eq!(widget.get_as::<u32>("motor.max_rpm"), Some(15_000));
    assert_eq!(
        widget.get_as::<String>("motor.part_number"),
        Some("M-3".to_string())
    );

    // `#[derive(Metadata)]` does not make the type a `Provenance`.
    fn assert_metadata<M: Metadata>(_: &M) {}
    assert_metadata(&widget);
}

#[test]
fn a_map_backed_record_satisfies_metadata() {
    amenable_core::init_tracing();
    let mut record = MapMetadata::default();
    record.insert(OwnedEntry::new("max_rpm", Rpm(9_000)));
    record.insert(OwnedEntry::new("voltage", 48_u32));

    assert_eq!(record.len(), 2);
    assert_eq!(record.get_as::<Rpm>("max_rpm"), Some(Rpm(9_000)));
    assert_eq!(record.get_as::<u32>("voltage"), Some(48));
    assert_eq!(record.keys(), vec!["max_rpm", "voltage"]);
    assert!(record.get("absent").is_none());
}
