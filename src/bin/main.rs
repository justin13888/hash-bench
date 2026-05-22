//! `hash-bench` CLI — runs the benchmark suite or exports algorithm metadata.
//!
//! Built as a standalone binary so it can be cross-compiled and executed on
//! targets without a Rust toolchain.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use hash_bench::bench::{run_benchmarks, BenchConfig};
use hash_bench::metadata::metadata_report;

/// Default input sizes: 64 B, 256 B, 1 KiB, 10 MiB, 100 MiB.
const DEFAULT_SIZES: [u64; 5] = [64, 256, 1024, 10 * 1024 * 1024, 100 * 1024 * 1024];

#[derive(Parser)]
#[command(
    name = "hash-bench",
    version,
    about = "Benchmark hashing algorithms in Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the benchmark suite and write a results JSON report.
    Run(RunArgs),
    /// Write the algorithm metadata catalogue as JSON.
    Metadata(MetadataArgs),
}

#[derive(Args)]
struct RunArgs {
    /// Identifier for the machine under test (becomes the platform id).
    #[arg(long)]
    machine_id: String,
    /// Output path (default: results/<machine-id>/results.json).
    #[arg(long)]
    output: Option<PathBuf>,
    /// Case-insensitive substring filter on algorithm names.
    #[arg(long)]
    filter: Option<String>,
    /// Comma-separated input sizes in bytes (default: 64,256,1024,10MiB,100MiB).
    #[arg(long, value_delimiter = ',')]
    sizes: Option<Vec<u64>>,
    /// Comma-separated concurrency levels (default: 1, physical, logical cores).
    #[arg(long, value_delimiter = ',')]
    concurrency: Option<Vec<usize>>,
    /// Samples collected per measured cell.
    #[arg(long, default_value_t = 30)]
    sample_count: usize,
    /// Warm-up duration per measured cell, in milliseconds.
    #[arg(long, default_value_t = 3000)]
    warmup: u64,
    /// Human-readable CPU model recorded in the report.
    #[arg(long)]
    cpu_model: Option<String>,
}

#[derive(Args)]
struct MetadataArgs {
    /// Output path (default: stdout).
    #[arg(long)]
    output: Option<PathBuf>,
}

/// Default concurrency levels: 1, physical cores, logical cores (deduplicated).
fn default_concurrency() -> Vec<usize> {
    let mut levels = vec![1, num_cpus::get_physical(), num_cpus::get()];
    levels.sort_unstable();
    levels.dedup();
    levels
}

/// Write `json` to `path`, creating parent directories.
fn write_file(path: &PathBuf, json: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, json)
}

fn run(args: RunArgs) -> Result<(), String> {
    let sizes: Vec<usize> = args
        .sizes
        .unwrap_or_else(|| DEFAULT_SIZES.to_vec())
        .into_iter()
        .map(|s| s as usize)
        .collect();
    let concurrency = args.concurrency.unwrap_or_else(default_concurrency);

    let cfg = BenchConfig {
        machine_id: args.machine_id.clone(),
        cpu_model: args.cpu_model,
        sizes,
        concurrency,
        sample_count: args.sample_count,
        warmup: Duration::from_millis(args.warmup),
        filter: args.filter,
    };

    let report = run_benchmarks(&cfg);
    let json = serde_json::to_string_pretty(&report)
        .map_err(|e| format!("failed to serialize report: {e}"))?;

    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("results/{}/results.json", args.machine_id)));
    write_file(&output, &json).map_err(|e| format!("failed to write {}: {e}", output.display()))?;

    eprintln!(
        "Wrote {} result rows to {}",
        report.results.len(),
        output.display()
    );
    Ok(())
}

fn metadata(args: MetadataArgs) -> Result<(), String> {
    let report = metadata_report();
    let json = serde_json::to_string_pretty(&report)
        .map_err(|e| format!("failed to serialize metadata: {e}"))?;

    match args.output {
        Some(path) => {
            write_file(&path, &json)
                .map_err(|e| format!("failed to write {}: {e}", path.display()))?;
            eprintln!(
                "Wrote metadata for {} algorithm(s) to {}",
                report.algorithms.len(),
                path.display()
            );
        }
        None => println!("{json}"),
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Run(args) => run(args),
        Command::Metadata(args) => metadata(args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
