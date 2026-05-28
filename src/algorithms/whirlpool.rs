//! Whirlpool (ISO/IEC 10118-3).

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
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
        variant: "sw",
        crate_name: "whirlpool",
        output: OutputBits::Fixed(512),
        category: Category::Cryptographic,
        notes: "ISO/IEC 10118-3; used by VeraCrypt",
        runner: Runner::SingleStream(whirlpool),
        available: always_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: false,
        hardware_features: &[],
    }]
}
