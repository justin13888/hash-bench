//! Streebog — Russian standard (GOST R 34.11-2012).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;
use streebog::Digest;

/// Hash data using Streebog-256.
fn streebog256(data: &[u8]) {
    let mut hasher = streebog::Streebog256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Streebog-512.
fn streebog512(data: &[u8]) {
    let mut hasher = streebog::Streebog512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "Streebog-256",
            crate_name: "streebog",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "Russian standard (GOST R 34.11-2012)",
            runner: Runner::SingleStream(streebog256),
        },
        Algorithm {
            name: "Streebog-512",
            crate_name: "streebog",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "Russian standard (GOST R 34.11-2012)",
            runner: Runner::SingleStream(streebog512),
        },
    ]
}
