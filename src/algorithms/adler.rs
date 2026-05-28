//! Adler32 — checksum used by zlib/gzip.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using Adler32.
fn adler32(data: &[u8]) {
    let mut hasher = adler::Adler32::new();
    hasher.write_slice(data);
    black_box(hasher.checksum());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "Adler32",
        variant: "sw",
        crate_name: "adler",
        output: OutputBits::Fixed(32),
        category: Category::NonCryptographic,
        notes: "Checksum used by zlib/gzip — included for reference",
        runner: Runner::SingleStream(adler32),
        available: always_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: false,
        hardware_features: &[],
    }]
}
