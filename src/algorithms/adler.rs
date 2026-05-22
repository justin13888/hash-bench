//! Adler32 — checksum used by zlib/gzip.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
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
        crate_name: "adler",
        output: OutputBits::Fixed(32),
        category: Category::NonCryptographic,
        notes: "Checksum used by zlib/gzip — included for reference",
        runner: Runner::SingleStream(adler32),
    }]
}
