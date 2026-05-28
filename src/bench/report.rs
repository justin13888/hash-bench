//! The standardized benchmark report — the cross-language contract with the
//! web dashboard. See `schema/results.v3.schema.json`.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use super::BenchConfig;

/// Version of the results JSON format. Bump on breaking changes.
///
/// v3 added the optional `skipped_variants` array so consumers can distinguish
/// variants that were compiled in but filtered out at runtime (e.g. SHA-1
/// `[sha-ext]` on a host lacking SHA-NI) from variants that were never built.
pub const SCHEMA_VERSION: u32 = 3;

/// A complete per-platform benchmark report.
#[derive(Serialize)]
pub struct Report {
    pub schema_version: u32,
    /// Wall-clock time the report was produced, Unix epoch milliseconds.
    pub generated_at_unix_ms: u64,
    pub tool: ToolInfo,
    pub platform: PlatformInfo,
    pub config: ReportConfig,
    pub results: Vec<ResultRow>,
    /// Variants compiled in but excluded from this run (e.g. CPU lacks the
    /// advertised hardware feature). Empty when nothing was skipped.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skipped_variants: Vec<SkippedVariant>,
}

/// Identifies the binary that produced the report.
#[derive(Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub version: String,
}

/// Hardware/OS the benchmark ran on. `cpu_model` is best-effort and may be null.
#[derive(Serialize)]
pub struct PlatformInfo {
    pub id: String,
    pub cpu_model: Option<String>,
    pub physical_cores: u32,
    pub logical_cores: u32,
    pub os: String,
    pub arch: String,
}

/// The parameters the run was executed with.
#[derive(Serialize)]
pub struct ReportConfig {
    pub sizes_bytes: Vec<u64>,
    pub concurrency_levels: Vec<u32>,
    pub warmup_ms: u64,
    pub sample_count: u32,
}

/// One measured (algorithm, variant, size, thread-count) cell.
#[derive(Serialize)]
pub struct ResultRow {
    pub algorithm: String,
    /// Implementation tag — `"sw"` for pure-Rust, `"sha-ext"` for x86 SHA-NI /
    /// ARMv8 SHA2, `"aes-ext"` for AES-NI, `"clmul"` for PCLMULQDQ / PMULL,
    /// `"crc-ext"` for SSE4.2 / ARMv8 CRC instructions. Together with
    /// `algorithm`, it forms the unique row key.
    pub variant: String,
    pub size_bytes: u64,
    /// Threads involved: independent streams for single-stream algorithms, or
    /// internal pool size for internally-parallel ones.
    pub threads: u32,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub stddev_ns: f64,
    pub ci_lower_ns: f64,
    pub ci_upper_ns: f64,
    /// Bytes hashed per second using `threads` threads.
    pub throughput_bps: f64,
    pub samples: u32,
}

/// One variant that was filtered out before the matrix ran.
#[derive(Serialize, Clone)]
pub struct SkippedVariant {
    pub algorithm: String,
    pub variant: String,
    pub crate_name: String,
    /// Human-readable explanation, e.g.
    /// `"host CPU lacks required feature(s): sha-ni"`.
    pub reason: String,
    /// ISA feature labels the skipped variant relies on.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hardware_features: Vec<String>,
}

/// Assemble a [`Report`] from a finished run's config and measured rows.
pub fn build_report(
    cfg: &BenchConfig,
    results: Vec<ResultRow>,
    skipped_variants: Vec<SkippedVariant>,
) -> Report {
    let generated_at_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    Report {
        schema_version: SCHEMA_VERSION,
        generated_at_unix_ms,
        tool: ToolInfo {
            name: "hash-bench".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        platform: PlatformInfo {
            id: cfg.machine_id.clone(),
            cpu_model: cfg.cpu_model.clone(),
            physical_cores: num_cpus::get_physical() as u32,
            logical_cores: num_cpus::get() as u32,
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
        config: ReportConfig {
            sizes_bytes: cfg.sizes.iter().map(|&s| s as u64).collect(),
            concurrency_levels: cfg.concurrency.iter().map(|&c| c as u32).collect(),
            warmup_ms: cfg.warmup.as_millis() as u64,
            sample_count: cfg.sample_count as u32,
        },
        results,
        skipped_variants,
    }
}
