//! BLAKE3 — modern cryptographic hash, both single-stream and rayon-parallel.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using BLAKE3 (single-threaded).
fn blake3(data: &[u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE3 with rayon parallelism (multi-threaded single stream).
///
/// The supplied pool's thread count is the concurrency level, so `update_rayon`
/// scales one stream across exactly that many cores — an apples-to-apples
/// counterpart to running that many independent single-stream hashes.
fn blake3_rayon(data: &[u8], pool: &rayon::ThreadPool) {
    let mut hasher = blake3::Hasher::new();
    pool.install(|| hasher.update_rayon(data));
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "BLAKE3",
            variant: "sw",
            crate_name: "blake3",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "Single-stream; runtime SIMD dispatch (SSE2/SSE4.1/AVX2/AVX-512/NEON)",
            runner: Runner::SingleStream(blake3),
            available: always_available,
        },
        Algorithm {
            name: "BLAKE3 (rayon)",
            variant: "sw",
            crate_name: "blake3",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "Multi-threaded single stream via update_rayon",
            runner: Runner::ParallelStream(blake3_rayon),
            available: always_available,
        },
    ]
}
