//! FarmHash.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using FarmHash (64-bit).
fn farmhash(data: &[u8]) {
    black_box(farmhash::hash64(data));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "FarmHash",
        crate_name: "farmhash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Used by Google internally, TensorFlow",
        runner: Runner::SingleStream(farmhash),
    }]
}
