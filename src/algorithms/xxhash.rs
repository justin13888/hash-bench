//! xxHash family.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using XXH32.
fn xxh32(data: &[u8]) {
    black_box(xxhash_rust::xxh32::xxh32(data, 0));
}

/// Hash data using XXH64.
fn xxh64(data: &[u8]) {
    black_box(xxhash_rust::xxh64::xxh64(data, 0));
}

/// Hash data using XXH3_64.
fn xxh3_64(data: &[u8]) {
    black_box(xxhash_rust::xxh3::xxh3_64(data));
}

/// Hash data using XXH3_128.
fn xxh3_128(data: &[u8]) {
    black_box(xxhash_rust::xxh3::xxh3_128(data));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "XXH32",
            crate_name: "xxhash-rust",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(xxh32),
        },
        Algorithm {
            name: "XXH64",
            crate_name: "xxhash-rust",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(xxh64),
        },
        Algorithm {
            name: "XXH3_64",
            crate_name: "xxhash-rust",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "Used by Linux kernel, rsync",
            runner: Runner::SingleStream(xxh3_64),
        },
        Algorithm {
            name: "XXH3_128",
            crate_name: "xxhash-rust",
            output: OutputBits::Fixed(128),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(xxh3_128),
        },
    ]
}
