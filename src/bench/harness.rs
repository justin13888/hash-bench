//! A small wall-clock measurement harness.
//!
//! Replaces Criterion so the benchmark can run from a plain binary on any
//! cross-compiled target. Statistics come from the `average` crate (mean,
//! standard error, median); the 95% confidence interval is the normal
//! approximation `mean ± 1.96 · standard_error`.

use std::time::{Duration, Instant};

use average::{Estimate, Quantile, Variance};

/// Minimum wall time a single timed batch should span; the inner repeat count
/// is auto-calibrated to reach it so per-call timings are not clock-limited.
const MIN_BATCH: Duration = Duration::from_micros(500);

/// Upper bound on the auto-calibrated inner repeat count, so a pathologically
/// fast unit cannot blow up calibration.
const MAX_INNER: u64 = 1 << 26;

/// Tunables for a measurement run.
#[derive(Clone, Copy)]
pub struct HarnessConfig {
    /// How long to spin the unit before collecting samples.
    pub warmup: Duration,
    /// Number of samples to collect.
    pub sample_count: usize,
}

/// Per-call timing statistics, all in nanoseconds.
#[derive(Clone, Copy)]
pub struct Stats {
    pub mean_ns: f64,
    pub median_ns: f64,
    pub stddev_ns: f64,
    pub ci_lower_ns: f64,
    pub ci_upper_ns: f64,
    pub samples: usize,
}

/// Auto-calibrate how many times `unit` must run for a batch to span [`MIN_BATCH`].
fn calibrate(unit: &mut impl FnMut()) -> u64 {
    let mut inner: u64 = 1;
    loop {
        let start = Instant::now();
        for _ in 0..inner {
            unit();
        }
        let elapsed = start.elapsed();
        if elapsed >= MIN_BATCH || inner >= MAX_INNER {
            return inner;
        }
        inner = if elapsed.is_zero() {
            inner.saturating_mul(8)
        } else {
            let factor = (MIN_BATCH.as_nanos() / elapsed.as_nanos()) as u64 + 1;
            inner.saturating_mul(factor.max(2))
        }
        .min(MAX_INNER);
    }
}

/// Measure `unit`: warm up, then collect `sample_count` per-call timings.
pub fn measure(cfg: &HarnessConfig, mut unit: impl FnMut()) -> Stats {
    let inner = calibrate(&mut unit);

    let warm_start = Instant::now();
    while warm_start.elapsed() < cfg.warmup {
        for _ in 0..inner {
            unit();
        }
    }

    let mut variance = Variance::new();
    let mut median = Quantile::new(0.5);
    for _ in 0..cfg.sample_count {
        let start = Instant::now();
        for _ in 0..inner {
            unit();
        }
        let per_call = start.elapsed().as_nanos() as f64 / inner as f64;
        variance.add(per_call);
        median.add(per_call);
    }

    let mean = variance.mean();
    let half = 1.96 * variance.error();
    Stats {
        mean_ns: mean,
        median_ns: median.quantile(),
        stddev_ns: variance.sample_variance().sqrt(),
        ci_lower_ns: (mean - half).max(0.0),
        ci_upper_ns: mean + half,
        samples: cfg.sample_count,
    }
}
