//! CRC checksums — CRC32, CRC32C, and CRC64.

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using CRC32.
fn crc32(data: &[u8]) {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using CRC32C.
fn crc32c(data: &[u8]) {
    black_box(crc32c::crc32c(data));
}

/// Hash data using CRC64.
fn crc64(data: &[u8]) {
    let mut hasher = crc64fast::Digest::new();
    hasher.write(data);
    black_box(hasher.sum64());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "CRC32",
            crate_name: "crc32fast",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "Hardware-accelerated where available",
            runner: Runner::SingleStream(crc32),
        },
        Algorithm {
            name: "CRC32C",
            crate_name: "crc32c",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "Castagnoli polynomial; used by iSCSI, Btrfs, LevelDB, gRPC",
            runner: Runner::SingleStream(crc32c),
        },
        Algorithm {
            name: "CRC64",
            crate_name: "crc64fast",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "ECMA-182; used by Redis, xz",
            runner: Runner::SingleStream(crc64),
        },
    ]
}
