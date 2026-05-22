//! FNV-1a.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hash::Hasher;
use std::hint::black_box;

/// Hash data using FNV-1a.
fn fnv1a(data: &[u8]) {
    let mut hasher = fnv::FnvHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "FNV-1a",
        crate_name: "fnv",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Go hash/fnv standard library",
        runner: Runner::SingleStream(fnv1a),
    }]
}
