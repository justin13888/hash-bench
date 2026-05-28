//! FxHash — the hasher used internally by `rustc`.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hash::Hasher;
use std::hint::black_box;

/// Hash data using FxHash.
fn fxhash(data: &[u8]) {
    let mut hasher = rustc_hash::FxHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "FxHash",
        variant: "sw",
        crate_name: "rustc-hash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Used internally by rustc",
        runner: Runner::SingleStream(fxhash),
        available: always_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: false,
        hardware_features: &[],
    }]
}
