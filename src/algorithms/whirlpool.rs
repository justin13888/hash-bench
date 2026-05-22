//! Whirlpool (ISO/IEC 10118-3).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;
use whirlpool::Digest;

/// Hash data using Whirlpool.
fn whirlpool(data: &[u8]) {
    let mut hasher = whirlpool::Whirlpool::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "Whirlpool",
        crate_name: "whirlpool",
        output: OutputBits::Fixed(512),
        category: Category::Cryptographic,
        notes: "ISO/IEC 10118-3; used by VeraCrypt",
        runner: Runner::SingleStream(whirlpool),
    }]
}
