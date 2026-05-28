//! FarmHash.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using FarmHash (64-bit).
fn farmhash(data: &[u8]) {
    black_box(farmhash::hash64(data));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "FarmHash",
        variant: "sw",
        crate_name: "farmhash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Used by Google internally, TensorFlow",
        runner: Runner::SingleStream(farmhash),
        available: always_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: false,
        hardware_features: &[],
    }]
}
