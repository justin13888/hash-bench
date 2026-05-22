//! `hash-bench` — a modular benchmark suite for hashing algorithms in Rust.
//!
//! Each hash family lives in its own [`algorithms`] module behind its own Cargo
//! feature, carrying its metadata. [`registry`] assembles whichever families are
//! enabled into a single list. The [`bench`] engine measures them and emits a
//! versioned, schema-validated JSON report; [`metadata`] emits the algorithm
//! catalogue consumed by the web dashboard.
//!
//! The crate is a plain library — the benchmark runs from a normal binary, so it
//! can be cross-compiled and executed on targets without a Rust toolchain.

pub mod algorithms;
pub mod bench;
pub mod metadata;
pub mod registry;
pub mod verify;

pub use registry::{Algorithm, Category, OutputBits, Runner};

/// All algorithms enabled by the current feature set, in stable display order
/// (cryptographic families first, then non-cryptographic).
pub fn registry() -> Vec<Algorithm> {
    // `mut` is unused when the crate is built with no algorithm families enabled.
    #[allow(unused_mut)]
    let mut algs: Vec<Algorithm> = Vec::new();

    // ── Cryptographic ───────────────────────────────────────────────────────
    #[cfg(feature = "blake3")]
    algs.extend(algorithms::blake3::algorithms());
    #[cfg(feature = "blake2")]
    algs.extend(algorithms::blake2::algorithms());
    #[cfg(feature = "sha1")]
    algs.extend(algorithms::sha1::algorithms());
    #[cfg(feature = "sha2")]
    algs.extend(algorithms::sha2::algorithms());
    #[cfg(feature = "sha3")]
    algs.extend(algorithms::sha3::algorithms());
    #[cfg(feature = "md5")]
    algs.extend(algorithms::md5::algorithms());
    #[cfg(feature = "ripemd")]
    algs.extend(algorithms::ripemd::algorithms());
    #[cfg(feature = "sm3")]
    algs.extend(algorithms::sm3::algorithms());
    #[cfg(feature = "streebog")]
    algs.extend(algorithms::streebog::algorithms());
    #[cfg(feature = "whirlpool")]
    algs.extend(algorithms::whirlpool::algorithms());
    #[cfg(feature = "ascon")]
    algs.extend(algorithms::ascon::algorithms());

    // ── Non-cryptographic ───────────────────────────────────────────────────
    #[cfg(feature = "crc")]
    algs.extend(algorithms::crc::algorithms());
    #[cfg(feature = "xxhash")]
    algs.extend(algorithms::xxhash::algorithms());
    #[cfg(feature = "siphash")]
    algs.extend(algorithms::siphash::algorithms());
    #[cfg(feature = "ahash")]
    algs.extend(algorithms::ahash::algorithms());
    #[cfg(feature = "wyhash")]
    algs.extend(algorithms::wyhash::algorithms());
    #[cfg(feature = "fxhash")]
    algs.extend(algorithms::fxhash::algorithms());
    #[cfg(feature = "farmhash")]
    algs.extend(algorithms::farmhash::algorithms());
    #[cfg(feature = "murmur3")]
    algs.extend(algorithms::murmur3::algorithms());
    #[cfg(feature = "highway")]
    algs.extend(algorithms::highway::algorithms());
    #[cfg(feature = "fnv")]
    algs.extend(algorithms::fnv::algorithms());
    #[cfg(feature = "adler")]
    algs.extend(algorithms::adler::algorithms());

    algs
}
