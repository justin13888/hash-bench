//! Hardware-accelerated CRC variants.
//!
//! - **CRC32 (IEEE)** via `crc32fast` — dispatches at runtime to x86 PCLMULQDQ
//!   or ARMv8 `crc32` instructions.
//! - **CRC32C (Castagnoli)** via `crc32c` — dispatches to x86 SSE4.2 `crc32`
//!   or ARMv8 `crc32c` instructions.
//! - **CRC64/XZ** via `crc64fast` — ECMA-182 polynomial with reflected
//!   input/output; dispatches to x86 PCLMULQDQ or ARMv8 PMULL.
//!
//! Each entry is filtered out on hosts without the matching CPU feature so the
//! `[clmul]` / `[crc-ext]` labels remain truthful — without that filter the
//! crates would silently fall back to table-based implementations.

use crate::algorithms::cpu::{clmul_available, crc_ext_available};
use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

#[cfg(target_arch = "aarch64")]
const CLMUL_FEATURES: &[&str] = &["pmull"];
#[cfg(not(target_arch = "aarch64"))]
const CLMUL_FEATURES: &[&str] = &["pclmulqdq"];

#[cfg(target_arch = "aarch64")]
const CRC_EXT_FEATURES: &[&str] = &["armv8-crc"];
#[cfg(not(target_arch = "aarch64"))]
const CRC_EXT_FEATURES: &[&str] = &["sse4.2-crc32"];

fn crc32(data: &[u8]) {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn crc32c(data: &[u8]) {
    black_box(crc32c::crc32c(data));
}

fn crc64(data: &[u8]) {
    let mut hasher = crc64fast::Digest::new();
    hasher.write(data);
    black_box(hasher.sum64());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "CRC32",
            variant: "clmul",
            crate_name: "crc32fast",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "IEEE polynomial; x86 PCLMULQDQ / ARMv8 `crc32` via `crc32fast`",
            runner: Runner::SingleStream(crc32),
            available: clmul_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: true,
            hardware_features: CLMUL_FEATURES,
        },
        Algorithm {
            name: "CRC32C",
            variant: "crc-ext",
            crate_name: "crc32c",
            output: OutputBits::Fixed(32),
            category: Category::NonCryptographic,
            notes: "Castagnoli polynomial; x86 SSE4.2 `crc32` / ARMv8 `crc32c` via `crc32c`",
            runner: Runner::SingleStream(crc32c),
            available: crc_ext_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: true,
            hardware_features: CRC_EXT_FEATURES,
        },
        Algorithm {
            name: "CRC64",
            variant: "clmul",
            crate_name: "crc64fast",
            output: OutputBits::Fixed(64),
            category: Category::NonCryptographic,
            notes: "CRC-64/XZ (ECMA-182 polynomial, reflected); x86 PCLMULQDQ / ARMv8 PMULL via `crc64fast`",
            runner: Runner::SingleStream(crc64),
            available: clmul_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: true,
            hardware_features: CLMUL_FEATURES,
        },
    ]
}
