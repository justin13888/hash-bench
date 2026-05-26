//! Pure-Rust SHA-2 family via the `sha2` crate built with the `force-soft`
//! feature, so no x86 SHA-NI or ARMv8 SHA2 dispatch is compiled in.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use sha2::Digest;
use std::hint::black_box;

fn sha224(data: &[u8]) {
    let mut hasher = sha2::Sha224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha256(data: &[u8]) {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha384(data: &[u8]) {
    let mut hasher = sha2::Sha384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha512(data: &[u8]) {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha512_224(data: &[u8]) {
    let mut hasher = sha2::Sha512_224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

fn sha512_256(data: &[u8]) {
    let mut hasher = sha2::Sha512_256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "SHA-224",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha224),
            available: always_available,
        },
        Algorithm {
            name: "SHA-256",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; used by Bitcoin, TLS; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha256),
            available: always_available,
        },
        Algorithm {
            name: "SHA-384",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(384),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; used by TLS; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha384),
            available: always_available,
        },
        Algorithm {
            name: "SHA-512",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha512),
            available: always_available,
        },
        Algorithm {
            name: "SHA-512/224",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha512_224),
            available: always_available,
        },
        Algorithm {
            name: "SHA-512/256",
            variant: "sw",
            crate_name: "sha2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; pure-Rust (force-soft)",
            runner: Runner::SingleStream(sha512_256),
            available: always_available,
        },
    ]
}
