//! Algorithm catalogue export — the single source of truth for the web
//! dashboard's category/metadata tables. See `schema/algorithms.v1.schema.json`.

use serde::Serialize;

/// Version of the algorithms JSON format. Bump on breaking changes.
pub const SCHEMA_VERSION: u32 = 1;

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
    pub notes: String,
}

/// Build the algorithm catalogue from the active [`crate::registry`].
pub fn metadata_report() -> MetadataReport {
    let algorithms = crate::registry()
        .into_iter()
        .map(|a| AlgorithmMeta {
            name: a.name.to_string(),
            crate_name: a.crate_name.to_string(),
            output_bits: a.output.bits(),
            output_kind: a.output.kind().to_string(),
            category: a.category.as_str().to_string(),
            internally_parallel: a.internally_parallel(),
            notes: a.notes.to_string(),
        })
        .collect();

    MetadataReport {
        schema_version: SCHEMA_VERSION,
        algorithms,
    }
}
