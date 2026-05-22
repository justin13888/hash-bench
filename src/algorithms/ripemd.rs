//! RIPEMD family (ISO/IEC 10118-3).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use ripemd::Digest;
use std::hint::black_box;

/// Hash data using RIPEMD-128.
fn ripemd128(data: &[u8]) {
    let mut hasher = ripemd::Ripemd128::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-160.
fn ripemd160(data: &[u8]) {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-256.
fn ripemd256(data: &[u8]) {
    let mut hasher = ripemd::Ripemd256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-320.
fn ripemd320(data: &[u8]) {
    let mut hasher = ripemd::Ripemd320::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "RIPEMD-128",
            crate_name: "ripemd",
            output: OutputBits::Fixed(128),
            category: Category::Cryptographic,
            notes: "ISO/IEC 10118-3",
            runner: Runner::SingleStream(ripemd128),
        },
        Algorithm {
            name: "RIPEMD-160",
            crate_name: "ripemd",
            output: OutputBits::Fixed(160),
            category: Category::Cryptographic,
            notes: "ISO/IEC 10118-3; used by Bitcoin",
            runner: Runner::SingleStream(ripemd160),
        },
        Algorithm {
            name: "RIPEMD-256",
            crate_name: "ripemd",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "ISO/IEC 10118-3",
            runner: Runner::SingleStream(ripemd256),
        },
        Algorithm {
            name: "RIPEMD-320",
            crate_name: "ripemd",
            output: OutputBits::Fixed(320),
            category: Category::Cryptographic,
            notes: "ISO/IEC 10118-3",
            runner: Runner::SingleStream(ripemd320),
        },
    ]
}
