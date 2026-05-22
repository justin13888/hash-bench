//! wyhash.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using wyhash.
fn wyhash(data: &[u8]) {
    black_box(wyhash::wyhash(data, 0));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "wyhash",
        crate_name: "wyhash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Used by Go runtime (maphash), Zig",
        runner: Runner::SingleStream(wyhash),
    }]
}
