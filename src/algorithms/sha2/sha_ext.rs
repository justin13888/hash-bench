//! Hardware-accelerated SHA-256 via `ring`, which dispatches to x86 SHA-NI or
//! ARMv8 SHA2 instructions at runtime when present.
//!
//! Only SHA-256 has a HW entry: the x86 SHA extension ISA only covers SHA-1
//! and SHA-256; SHA-512 / SHA-384 have no x86 HW acceleration on commodity
//! CPUs, and ARMv8.2's SHA-512 extension is rare and not reliably exposed via
//! Rust's stable `is_aarch64_feature_detected!`. SHA-224 and SHA-512/* stay
//! pure-Rust via `sha2/soft.rs`.
//!
//! The entry is filtered out on hosts without the matching CPU feature so the
//! `[sha-ext]` label remains truthful — without that filter, `ring` would
//! silently fall back to a non-SHA-extension implementation (AVX2 / SSSE3).

use crate::algorithms::cpu::sha_ext_available;
use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

fn sha256(data: &[u8]) {
    black_box(ring::digest::digest(&ring::digest::SHA256, data));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "SHA-256",
        variant: "sha-ext",
        crate_name: "ring",
        output: OutputBits::Fixed(256),
        category: Category::Cryptographic,
        notes: "x86 SHA-NI / ARMv8 SHA2 via `ring`",
        runner: Runner::SingleStream(sha256),
        available: sha_ext_available,
    }]
}
