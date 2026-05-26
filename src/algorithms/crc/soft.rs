//! Pure-Rust CRC32 (IEEE), CRC32C (Castagnoli), and CRC64 (ECMA-182) via the
//! `crc` crate's table-driven implementations. No intrinsics, no PCLMULQDQ /
//! PMULL, no SSE4.2 / ARMv8 `crc32` instructions.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

const CRC32_IEEE: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
const CRC32_ISCSI: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
const CRC64_ECMA: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);

fn crc32(data: &[u8]) {
    let mut digest = CRC32_IEEE.digest();
    digest.update(data);
    black_box(digest.finalize());
}

fn crc32c(data: &[u8]) {
    let mut digest = CRC32_ISCSI.digest();
    digest.update(data);
    black_box(digest.finalize());
}

fn crc64(data: &[u8]) {
    let mut digest = CRC64_ECMA.digest();
    digest.update(data);
    black_box(digest.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "CRC32",
            variant: "sw",
            crate_name: "crc",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "IEEE polynomial; table-based pure-Rust",
            runner: Runner::SingleStream(crc32),
            available: always_available,
        },
        Algorithm {
            name: "CRC32C",
            variant: "sw",
            crate_name: "crc",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "Castagnoli polynomial; table-based pure-Rust",
            runner: Runner::SingleStream(crc32c),
            available: always_available,
        },
        Algorithm {
            name: "CRC64",
            variant: "sw",
            crate_name: "crc",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "ECMA-182; table-based pure-Rust",
            runner: Runner::SingleStream(crc64),
            available: always_available,
        },
    ]
}
