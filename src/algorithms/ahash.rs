//! aHash — Rust `hashbrown` default hasher.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hash::Hasher;
use std::hint::black_box;

/// Hash data using AHash.
fn ahash(data: &[u8]) {
    let mut hasher = ahash::AHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "AHash",
        crate_name: "ahash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Rust hashbrown default; uses AES-NI when available",
        runner: Runner::SingleStream(ahash),
    }]
}
