//! MD5 (broken — included for reference only).

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use md5::Digest;
use std::hint::black_box;

/// Hash data using MD5.
fn md5(data: &[u8]) {
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "MD5",
        variant: "sw",
        crate_name: "md-5",
        output: OutputBits::Fixed(128),
        category: Category::Cryptographic,
        notes: "Broken — included for reference only",
        runner: Runner::SingleStream(md5),
        available: always_available,
    }]
}
