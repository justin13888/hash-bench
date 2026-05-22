//! SHA-1 (broken — included for reference only).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use sha1::Digest;
use std::hint::black_box;

/// Hash data using SHA-1.
fn sha1(data: &[u8]) {
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "SHA-1",
        crate_name: "sha1",
        output: OutputBits::Fixed(160),
        category: Category::Cryptographic,
        notes: "Broken — included for reference only",
        runner: Runner::SingleStream(sha1),
    }]
}
