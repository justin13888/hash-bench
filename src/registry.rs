//! Algorithm metadata types and the master registry.
//!
//! Every hash family lives in its own module under [`crate::algorithms`] and
//! exposes a `algorithms() -> Vec<Algorithm>` function. [`crate::registry`]
//! aggregates them, conditionally on the enabled Cargo features, into a single
//! list that the benchmark engine and the `metadata` CLI subcommand consume.

/// Cryptographic classification of a hash algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Designed to resist collision/preimage attacks.
    Cryptographic,
    /// Built for speed (checksums, hash-table hashers); not collision-resistant.
    NonCryptographic,
}

impl Category {
    /// Stable lowercase token used by the web app and the metadata JSON.
    pub const fn as_str(self) -> &'static str {
        match self {
            Category::Cryptographic => "cryptographic",
            Category::NonCryptographic => "non-cryptographic",
        }
    }
}

/// Output size of a hash algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputBits {
    /// Fixed-width digest, e.g. `Fixed(256)` for SHA-256.
    Fixed(u16),
    /// Extendable-output function. Records the bit length used in the benchmark.
    Xof { benched_bits: u16 },
}

impl OutputBits {
    /// Stable token: `"fixed"` or `"xof"`.
    pub const fn kind(self) -> &'static str {
        match self {
            OutputBits::Fixed(_) => "fixed",
            OutputBits::Xof { .. } => "xof",
        }
    }

    /// The digest width in bits (the benchmarked width for XOFs).
    pub const fn bits(self) -> u16 {
        match self {
            OutputBits::Fixed(b) => b,
            OutputBits::Xof { benched_bits } => benched_bits,
        }
    }
}

/// How an algorithm consumes threads while hashing a single buffer.
#[derive(Clone, Copy)]
pub enum Runner {
    /// Hashes one buffer on the calling thread. The benchmark runs `concurrency`
    /// of these in parallel to measure aggregate throughput.
    SingleStream(fn(&[u8])),
    /// Internally parallel: hashes one buffer using the supplied rayon pool.
    /// The pool's thread count is the concurrency level, so a single-stream
    /// run scales across cores (e.g. BLAKE3's `update_rayon`).
    ParallelStream(fn(&[u8], &rayon::ThreadPool)),
}

/// One benchmarkable hash algorithm together with all of its metadata.
#[derive(Clone, Copy)]
pub struct Algorithm {
    /// Display name, e.g. `"SHA-256"`. Unique across the registry; used as the
    /// benchmark result key and the web app's algorithm identifier.
    pub name: &'static str,
    /// Backing crate as published on crates.io, e.g. `"sha2"`.
    pub crate_name: &'static str,
    /// Digest output size.
    pub output: OutputBits,
    /// Cryptographic vs non-cryptographic.
    pub category: Category,
    /// Human-readable notes (standards, provenance, production usage).
    pub notes: &'static str,
    /// How the algorithm is invoked and threaded.
    pub runner: Runner,
}

impl Algorithm {
    /// Whether the algorithm parallelises a single stream internally.
    pub fn internally_parallel(&self) -> bool {
        matches!(self.runner, Runner::ParallelStream(_))
    }
}
