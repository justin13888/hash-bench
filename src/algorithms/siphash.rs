//! SipHash family.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hash::Hasher;
use std::hint::black_box;

/// Hash data using SipHash-1-3.
fn siphash13(data: &[u8]) {
    let mut hasher = siphasher::sip::SipHasher13::new();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using SipHash-2-4.
fn siphash24(data: &[u8]) {
    let mut hasher = siphasher::sip::SipHasher24::new();
    hasher.write(data);
    black_box(hasher.finish());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "SipHash-1-3",
            crate_name: "siphasher",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "Used by Python's dict default hasher",
            runner: Runner::SingleStream(siphash13),
        },
        Algorithm {
            name: "SipHash-2-4",
            crate_name: "siphasher",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "Rust HashMap default (via DefaultHasher)",
            runner: Runner::SingleStream(siphash24),
        },
    ]
}
