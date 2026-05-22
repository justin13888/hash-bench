//! SHA-2 family (NIST FIPS 180-4).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use sha2::Digest;
use std::hint::black_box;

/// Hash data using SHA-224.
fn sha224(data: &[u8]) {
    let mut hasher = sha2::Sha224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-256.
fn sha256(data: &[u8]) {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-384.
fn sha384(data: &[u8]) {
    let mut hasher = sha2::Sha384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512.
fn sha512(data: &[u8]) {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512/224.
fn sha512_224(data: &[u8]) {
    let mut hasher = sha2::Sha512_224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512/256.
fn sha512_256(data: &[u8]) {
    let mut hasher = sha2::Sha512_256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "SHA-224",
            crate_name: "sha2",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4",
            runner: Runner::SingleStream(sha224),
        },
        Algorithm {
            name: "SHA-256",
            crate_name: "sha2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; used by Bitcoin, TLS",
            runner: Runner::SingleStream(sha256),
        },
        Algorithm {
            name: "SHA-384",
            crate_name: "sha2",
            output: OutputBits::Fixed(384),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4; used by TLS",
            runner: Runner::SingleStream(sha384),
        },
        Algorithm {
            name: "SHA-512",
            crate_name: "sha2",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4",
            runner: Runner::SingleStream(sha512),
        },
        Algorithm {
            name: "SHA-512/224",
            crate_name: "sha2",
            output: OutputBits::Fixed(224),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4",
            runner: Runner::SingleStream(sha512_224),
        },
        Algorithm {
            name: "SHA-512/256",
            crate_name: "sha2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "NIST FIPS 180-4",
            runner: Runner::SingleStream(sha512_256),
        },
    ]
}
