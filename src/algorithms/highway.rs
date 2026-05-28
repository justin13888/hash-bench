//! HighwayHash.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using HighwayHash-64.
fn highway64(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash64(data));
}

/// Hash data using HighwayHash-128.
fn highway128(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash128(data));
}

/// Hash data using HighwayHash-256.
fn highway256(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash256(data));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "HighwayHash-64",
            variant: "sw",
            crate_name: "highway",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "Google; SipHash alternative",
            runner: Runner::SingleStream(highway64),
            available: always_available,
            keyed: true,
            dos_resistant: true,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "HighwayHash-128",
            variant: "sw",
            crate_name: "highway",
            output: OutputBits::Fixed(128),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(highway128),
            available: always_available,
            keyed: true,
            dos_resistant: true,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "HighwayHash-256",
            variant: "sw",
            crate_name: "highway",
            output: OutputBits::Fixed(256),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(highway256),
            available: always_available,
            keyed: true,
            dos_resistant: true,
            hardware_required: false,
            hardware_features: &[],
        },
    ]
}
