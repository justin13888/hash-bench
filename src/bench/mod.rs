//! The benchmark engine: drives every enabled algorithm across the configured
//! input sizes and concurrency levels and produces a [`Report`].

pub mod harness;
pub mod report;

use std::hint::black_box;
use std::time::Duration;

use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use rayon::ThreadPool;

use crate::registry::{Algorithm, Runner};
use harness::{measure, HarnessConfig, Stats};
use report::{build_report, Report, ResultRow};

/// Fixed seed for reproducible benchmark data across runs and platforms.
const RNG_SEED: u64 = 0xDEAD_BEEF_CAFE_BABE;

/// Parameters for a benchmark run.
pub struct BenchConfig {
    /// Identifier for the machine under test (becomes the platform `id`).
    pub machine_id: String,
    /// Optional human-readable CPU model.
    pub cpu_model: Option<String>,
    /// Input sizes in bytes.
    pub sizes: Vec<usize>,
    /// Concurrency levels (thread counts) to sample.
    pub concurrency: Vec<usize>,
    /// Samples collected per measured cell.
    pub sample_count: usize,
    /// Warm-up duration per measured cell.
    pub warmup: Duration,
    /// Optional case-insensitive substring filter on algorithm names.
    pub filter: Option<String>,
}

/// Generate deterministic input data of `size` bytes.
fn generate_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    let mut rng = rand::rngs::StdRng::seed_from_u64(RNG_SEED);
    rng.fill(&mut data[..]);
    data
}

/// Build a rayon pool with a fixed thread count.
fn build_pool(threads: usize) -> ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .expect("failed to build rayon thread pool")
}

/// Bytes processed by one measured unit of `alg` at the given concurrency.
fn bytes_per_unit(alg: &Algorithm, size: usize, concurrency: usize) -> u64 {
    match alg.runner {
        // Single-stream: a unit hashes `concurrency` independent buffers.
        Runner::SingleStream(_) => size as u64 * concurrency as u64,
        // Internally parallel: a unit hashes one buffer across `concurrency` threads.
        Runner::ParallelStream(_) => size as u64,
    }
}

/// Measure one (algorithm, size, concurrency) cell.
fn measure_cell(
    harness: &HarnessConfig,
    alg: &Algorithm,
    data: &[u8],
    concurrency: usize,
    pool: &ThreadPool,
) -> Stats {
    match alg.runner {
        Runner::SingleStream(f) => {
            if concurrency <= 1 {
                measure(harness, || f(black_box(data)))
            } else {
                measure(harness, || {
                    pool.install(|| {
                        (0..concurrency)
                            .into_par_iter()
                            .for_each(|_| f(black_box(data)));
                    });
                })
            }
        }
        Runner::ParallelStream(f) => measure(harness, || f(black_box(data), pool)),
    }
}

/// Run the full benchmark matrix and return a standardized report.
pub fn run_benchmarks(cfg: &BenchConfig) -> Report {
    let filter = cfg.filter.as_ref().map(|f| f.to_lowercase());
    let algorithms: Vec<Algorithm> = crate::registry()
        .into_iter()
        .filter(|a| match &filter {
            Some(f) => a.name.to_lowercase().contains(f),
            None => true,
        })
        .collect();

    let datasets: Vec<(usize, Vec<u8>)> = cfg
        .sizes
        .iter()
        .map(|&size| (size, generate_data(size)))
        .collect();

    let harness = HarnessConfig {
        warmup: cfg.warmup,
        sample_count: cfg.sample_count,
    };

    let total_cells = cfg.concurrency.len() * datasets.len() * algorithms.len();
    eprintln!(
        "Benchmarking {} algorithm(s) × {} size(s) × {} concurrency level(s) = {} cells",
        algorithms.len(),
        datasets.len(),
        cfg.concurrency.len(),
        total_cells,
    );

    let mut results: Vec<ResultRow> = Vec::new();
    let mut cell = 0usize;
    for &concurrency in &cfg.concurrency {
        let pool = build_pool(concurrency);
        for (size, data) in &datasets {
            for alg in &algorithms {
                cell += 1;
                eprintln!(
                    "[{cell}/{total_cells}] {} @ {} B, {} thread(s)",
                    alg.name, size, concurrency,
                );
                let stats = measure_cell(&harness, alg, data, concurrency, &pool);
                let bytes = bytes_per_unit(alg, *size, concurrency);
                let throughput_bps = if stats.mean_ns > 0.0 {
                    bytes as f64 / (stats.mean_ns * 1e-9)
                } else {
                    0.0
                };
                results.push(ResultRow {
                    algorithm: alg.name.to_string(),
                    size_bytes: *size as u64,
                    threads: concurrency as u32,
                    mean_ns: stats.mean_ns,
                    median_ns: stats.median_ns,
                    stddev_ns: stats.stddev_ns,
                    ci_lower_ns: stats.ci_lower_ns,
                    ci_upper_ns: stats.ci_upper_ns,
                    throughput_bps,
                    samples: stats.samples as u32,
                });
            }
        }
    }

    build_report(cfg, results)
}
