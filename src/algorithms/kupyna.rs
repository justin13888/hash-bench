//! Kupyna — Ukrainian national standard (DSTU 7564:2014).

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use kupyna::Digest;
use std::hint::black_box;

/// Hash data using Kupyna-224.
fn kupyna224(data: &[u8]) {
    let mut hasher = kupyna::Kupyna224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Kupyna-256.
fn kupyna256(data: &[u8]) {
    let mut hasher = kupyna::Kupyna256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Kupyna-384.
fn kupyna384(data: &[u8]) {
    let mut hasher = kupyna::Kupyna384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Kupyna-512.
fn kupyna512(data: &[u8]) {
    let mut hasher = kupyna::Kupyna512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "Kupyna-224",
            variant: "sw",
            crate_name: "kupyna",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "Ukrainian national standard (DSTU 7564:2014)",
            runner: Runner::SingleStream(kupyna224),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Kupyna-256",
            variant: "sw",
            crate_name: "kupyna",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "Ukrainian national standard (DSTU 7564:2014)",
            runner: Runner::SingleStream(kupyna256),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Kupyna-384",
            variant: "sw",
            crate_name: "kupyna",
            output: OutputBits::Fixed(384),
            category: Category::Cryptographic,
            notes: "Ukrainian national standard (DSTU 7564:2014)",
            runner: Runner::SingleStream(kupyna384),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Kupyna-512",
            variant: "sw",
            crate_name: "kupyna",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "Ukrainian national standard (DSTU 7564:2014)",
            runner: Runner::SingleStream(kupyna512),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
    ]
}
