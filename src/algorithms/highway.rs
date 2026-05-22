//! HighwayHash.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
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
            crate_name: "highway",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "Google; SipHash alternative",
            runner: Runner::SingleStream(highway64),
        },
        Algorithm {
            name: "HighwayHash-128",
            crate_name: "highway",
            output: OutputBits::Fixed(128),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(highway128),
        },
        Algorithm {
            name: "HighwayHash-256",
            crate_name: "highway",
            output: OutputBits::Fixed(256),
            category: Category::NonCryptographic,
            notes: "",
            runner: Runner::SingleStream(highway256),
        },
    ]
}
