//! SM3 — Chinese national standard (GB/T 32905-2016).

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use sm3::Digest;
use std::hint::black_box;

/// Hash data using SM3.
fn sm3(data: &[u8]) {
    let mut hasher = sm3::Sm3::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "SM3",
        variant: "sw",
        crate_name: "sm3",
        output: OutputBits::Fixed(256),
        category: Category::Cryptographic,
        notes: "Chinese national standard (GB/T 32905-2016)",
        runner: Runner::SingleStream(sm3),
        available: always_available,
    }]
}
