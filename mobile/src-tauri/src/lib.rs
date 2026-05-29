//! Thin Tauri shell around the `hash_bench` benchmark engine.
//!
//! Exposes a single [`run_bench`] command: it runs the suite on-device, writes a
//! schema-v3 `results.json` to the app cache dir, and returns the JSON plus the
//! file path so the frontend can hand it to the native share sheet.

use std::time::Duration;

use serde::Serialize;
use tauri::Manager;

const KIB: usize = 1024;
const MIB: usize = 1024 * 1024;

/// Result of [`run_bench`], serialized back to the frontend.
#[derive(Serialize)]
struct BenchOutput {
    /// Pretty-printed results.json (schema v3).
    json: String,
    /// Suggested file name, e.g. `pixel-8-results.json`.
    filename: String,
    /// Absolute path the report was written to, for the share step.
    cached_path: String,
}

/// Benchmark parameters for a preset.
struct Preset {
    sizes: Vec<usize>,
    concurrency: Vec<usize>,
    sample_count: usize,
    warmup: Duration,
}

/// Concurrency levels `[1, physical, logical]`, deduplicated — mirrors the
/// desktop CLI's `default_concurrency` so Full runs match the desktop matrix.
fn default_concurrency() -> Vec<usize> {
    let mut levels = vec![1, num_cpus::get_physical(), num_cpus::get()];
    levels.sort_unstable();
    levels.dedup();
    levels
}

/// Map a preset name to its config. Presets only shrink sizes / concurrency /
/// samples / warmup — they never filter the algorithm set, so a Full run stays
/// verify-clean against the desktop registry.
fn preset_config(name: &str) -> Result<Preset, String> {
    match name {
        // Fast single-threaded pass over small inputs: noisy, for quick checks.
        "quick" => Ok(Preset {
            sizes: vec![KIB, MIB],
            concurrency: vec![1],
            sample_count: 10,
            warmup: Duration::from_millis(500),
        }),
        // Full matrix at desktop-grade sample counts; committable to results/.
        // 100 MiB is intentionally dropped — the engine allocates the whole
        // buffer up front, which risks an OOM kill on phones.
        "full" => Ok(Preset {
            sizes: vec![64, 256, KIB, MIB, 10 * MIB],
            concurrency: default_concurrency(),
            sample_count: 30,
            warmup: Duration::from_millis(1500),
        }),
        other => Err(format!(
            "unknown preset {other:?} (expected \"quick\" or \"full\")"
        )),
    }
}

/// Keep a machine id safe as a file-name component.
fn sanitize(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// Run the benchmark suite on this device and write a schema-v3 results.json.
#[tauri::command]
async fn run_bench(
    app: tauri::AppHandle,
    machine_id: String,
    cpu_model: Option<String>,
    preset: String,
) -> Result<BenchOutput, String> {
    let machine_id = machine_id.trim().to_string();
    if machine_id.is_empty() {
        return Err("machine id must not be empty".into());
    }
    let cpu_model = cpu_model
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let preset = preset_config(&preset)?;

    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("could not resolve app cache dir: {e}"))?;

    // The benchmark is CPU-bound and runs many cells; keep it off the IPC thread
    // so the WebView stays responsive while it works.
    let id_for_run = machine_id.clone();
    let json = tauri::async_runtime::spawn_blocking(move || {
        let cfg = hash_bench::bench::BenchConfig {
            machine_id: id_for_run,
            cpu_model,
            sizes: preset.sizes,
            concurrency: preset.concurrency,
            sample_count: preset.sample_count,
            warmup: preset.warmup,
            filter: None,
        };
        let report = hash_bench::bench::run_benchmarks(&cfg);
        serde_json::to_string_pretty(&report).map_err(|e| format!("serialize report: {e}"))
    })
    .await
    .map_err(|e| format!("benchmark task failed: {e}"))??;

    let filename = format!("{}-results.json", sanitize(&machine_id));
    std::fs::create_dir_all(&cache_dir).map_err(|e| format!("create cache dir: {e}"))?;
    let path = cache_dir.join(&filename);
    std::fs::write(&path, json.as_bytes()).map_err(|e| format!("write {}: {e}", path.display()))?;

    Ok(BenchOutput {
        json,
        filename,
        cached_path: path.to_string_lossy().into_owned(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sharekit::init())
        .invoke_handler(tauri::generate_handler![run_bench])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
