//! Algorithm catalogue export — the single source of truth for the web
//! dashboard's category/metadata tables. See `schema/algorithms.v3.schema.json`.

use serde::Serialize;

/// Version of the algorithms JSON format. Bump on breaking changes.
///
/// v3 added structured use-case tags: `keyed`, `dos_resistant`,
/// `hardware_required`, `hardware_features`.
pub const SCHEMA_VERSION: u32 = 3;

/// The full algorithm catalogue for the enabled feature set.
#[derive(Serialize)]
pub struct MetadataReport {
    pub schema_version: u32,
    pub algorithms: Vec<AlgorithmMeta>,
}

/// Serializable metadata for one algorithm (the [`crate::Algorithm`] fields
/// minus the function pointer).
#[derive(Serialize)]
pub struct AlgorithmMeta {
    pub name: String,
    /// Implementation tag — matches `ResultRow::variant` in the results JSON.
    pub variant: String,
    #[serde(rename = "crate")]
    pub crate_name: String,
    /// Digest width in bits (the benchmarked width for XOFs).
    pub output_bits: u16,
    /// `"fixed"` or `"xof"`.
    pub output_kind: String,
    /// `"cryptographic"` or `"non-cryptographic"`.
    pub category: String,
    /// Whether the algorithm parallelises a single stream internally.
    pub internally_parallel: bool,
    /// Whether the benchmarked entry consumes a key.
    pub keyed: bool,
    /// Whether the entry is a keyed hash with documented HashDoS hardening.
    /// Implies `keyed = true`.
    pub dos_resistant: bool,
    /// Whether this variant requires a specific ISA feature to run.
    pub hardware_required: bool,
    /// ISA feature labels this variant relies on (empty for `sw`).
    pub hardware_features: Vec<String>,
    pub notes: String,
}

/// Build the algorithm catalogue from the active [`crate::registry`].
pub fn metadata_report() -> MetadataReport {
    let algorithms = crate::registry()
        .into_iter()
        .map(|a| AlgorithmMeta {
            name: a.name.to_string(),
            variant: a.variant.to_string(),
            crate_name: a.crate_name.to_string(),
            output_bits: a.output.bits(),
            output_kind: a.output.kind().to_string(),
            category: a.category.as_str().to_string(),
            internally_parallel: a.internally_parallel(),
            keyed: a.keyed,
            dos_resistant: a.dos_resistant,
            hardware_required: a.hardware_required,
            hardware_features: a.hardware_features.iter().map(|s| s.to_string()).collect(),
            notes: a.notes.to_string(),
        })
        .collect();

    MetadataReport {
        schema_version: SCHEMA_VERSION,
        algorithms,
    }
}
