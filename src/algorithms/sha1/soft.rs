//! Pure-Rust SHA-1 via the `sha1` crate built with the `force-soft` feature, so
//! no x86 SHA-NI or ARMv8 SHA1 dispatch is compiled in.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use sha1::Digest;
use std::hint::black_box;

fn sha1(data: &[u8]) {
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "SHA-1",
        variant: "sw",
        crate_name: "sha1",
        output: OutputBits::Fixed(160),
        category: Category::Cryptographic,
        notes: "Broken — included for reference only; pure-Rust (force-soft)",
        runner: Runner::SingleStream(sha1),
        available: always_available,
    }]
}
