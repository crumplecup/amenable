use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of reading back the configured command arguments.
#[derive(Debug, Clone, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::configured(\"a\", \"b\")")]
pub struct KaniCommandArgsObservation {
    args: [String; 2],
}

impl KaniCommandArgsObservation {
    /// Model two configured command arguments in order.
    #[must_use]
    pub fn configured(first: impl Into<String>, second: impl Into<String>) -> Self {
        Self {
            args: [first.into(), second.into()],
        }
    }

    /// Report the modeled configured arguments.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn args(&self) -> [&str; 2] {
        [self.args[0].as_str(), self.args[1].as_str()]
    }
}

impl Provenance for KaniCommandArgsObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
                MetadataEntry::new(
                    "assumed",
                    "command builder argument introspection preserves the configured argument sequence",
                ),
                MetadataEntry::new(
                    "rationale",
                    "even direct Command construction reaches an unsupported CString strlen boundary under Kani",
                ),
                MetadataEntry::new("first_arg", self.args[0].clone()),
                MetadataEntry::new("second_arg", self.args[1].clone()),
            ]
            .into_iter()
        })
    }
}

/// Observable result of one environment override being visible to a spawned command.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::visible_override(\"AMENABLE_TEST_VAR\", \"configured-value\", \"configured-value\")"
)]
pub struct KaniCommandEnvObservation {
    /// The modeled override key.
    key: String,
    /// The modeled override value.
    value: String,
    /// The modeled child-visible stdout text.
    visible_stdout: String,
}

impl KaniCommandEnvObservation {
    /// Model one command environment override that is visible in child output.
    #[must_use]
    pub fn visible_override(
        key: impl Into<String>,
        value: impl Into<String>,
        visible_stdout: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            visible_stdout: visible_stdout.into(),
        }
    }
}

impl Provenance for KaniCommandEnvObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
                MetadataEntry::new(
                    "assumed",
                    "a command environment override is visible to the spawned child under the same key and value",
                ),
                MetadataEntry::new(
                    "rationale",
                    "the direct env-plus-spawn path combines unsupported command construction and real spawn boundaries under Kani",
                ),
                MetadataEntry::new("key", self.key.clone()),
                MetadataEntry::new("value", self.value.clone()),
                MetadataEntry::new("visible_stdout", self.visible_stdout.clone()),
            ]
            .into_iter()
        })
    }
}

/// Observable result of reading back one configured environment override.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::configured_override(\"SOME_KEY\", \"some_value\")"
)]
pub struct KaniCommandEnvsObservation {
    /// The modeled override key.
    key: String,
    /// The modeled override value.
    value: String,
}

impl KaniCommandEnvsObservation {
    /// Model one configured environment override stored on a command builder.
    #[must_use]
    pub fn configured_override(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Provenance for KaniCommandEnvsObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
                MetadataEntry::new(
                    "assumed",
                    "command builder environment introspection preserves configured key-value overrides",
                ),
                MetadataEntry::new(
                    "rationale",
                    "direct Command environment introspection still times out under Kani before the override law can be checked",
                ),
                MetadataEntry::new("key", self.key.clone()),
                MetadataEntry::new("value", self.value.clone()),
            ]
            .into_iter()
        })
    }
}
