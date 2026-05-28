//! Pure-Rust SHA-3 / SHAKE / Keccak via the `sha3` crate (no `asm` feature).

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use sha3::Digest;
use std::hint::black_box;

fn sha3_224(data: &[u8]) {
    let mut hasher = sha3::Sha3_224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha3_256(data: &[u8]) {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha3_384(data: &[u8]) {
    let mut hasher = sha3::Sha3_384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha3_512(data: &[u8]) {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn shake128(data: &[u8]) {
    use sha3::digest::{ExtendableOutput, Update, XofReader};
    let mut hasher = sha3::Shake128::default();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 32];
    reader.read(&mut result);
    black_box(result);
}

fn shake256(data: &[u8]) {
    use sha3::digest::{ExtendableOutput, Update, XofReader};
    let mut hasher = sha3::Shake256::default();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 64];
    reader.read(&mut result);
    black_box(result);
}

fn keccak224(data: &[u8]) {
    let mut hasher = sha3::Keccak224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn keccak256(data: &[u8]) {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn keccak384(data: &[u8]) {
    let mut hasher = sha3::Keccak384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn keccak512(data: &[u8]) {
    let mut hasher = sha3::Keccak512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "SHA3-224",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "NIST FIPS 202",
            runner: Runner::SingleStream(sha3_224),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "SHA3-256",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "NIST FIPS 202",
            runner: Runner::SingleStream(sha3_256),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "SHA3-384",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(384),
            category: Category::Cryptographic,
            notes: "NIST FIPS 202",
            runner: Runner::SingleStream(sha3_384),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "SHA3-512",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "NIST FIPS 202",
            runner: Runner::SingleStream(sha3_512),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "SHAKE128",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Xof { benched_bits: 256 },
            category: Category::Cryptographic,
            notes: "NIST FIPS 202; benchmarked at 256-bit output",
            runner: Runner::SingleStream(shake128),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "SHAKE256",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Xof { benched_bits: 512 },
            category: Category::Cryptographic,
            notes: "NIST FIPS 202; benchmarked at 512-bit output",
            runner: Runner::SingleStream(shake256),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Keccak-224",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "Pre-NIST Keccak",
            runner: Runner::SingleStream(keccak224),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Keccak-256",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "Pre-NIST Keccak; used by Ethereum",
            runner: Runner::SingleStream(keccak256),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Keccak-384",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(384),
            category: Category::Cryptographic,
            notes: "Pre-NIST Keccak",
            runner: Runner::SingleStream(keccak384),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
        Algorithm {
            name: "Keccak-512",
            variant: "sw",
            crate_name: "sha3",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "Pre-NIST Keccak",
            runner: Runner::SingleStream(keccak512),
            available: always_available,
            keyed: false,
            dos_resistant: false,
            hardware_required: false,
            hardware_features: &[],
        },
    ]
}
