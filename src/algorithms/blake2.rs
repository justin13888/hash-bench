//! BLAKE2 family (RFC 7693).

use crate::registry::{Algorithm, Category, OutputBits, Runner};
use blake2::Digest;
use std::hint::black_box;

/// Hash data using BLAKE2b512.
fn blake2b512(data: &[u8]) {
    let mut hasher = blake2::Blake2b512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE2b256.
fn blake2b256(data: &[u8]) {
    use blake2::digest::consts::U32;
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE2s256.
fn blake2s256(data: &[u8]) {
    let mut hasher = blake2::Blake2s256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "BLAKE2b512",
            crate_name: "blake2",
            output: OutputBits::Fixed(512),
            category: Category::Cryptographic,
            notes: "RFC 7693; used by WireGuard, Argon2",
            runner: Runner::SingleStream(blake2b512),
        },
        Algorithm {
            name: "BLAKE2b256",
            crate_name: "blake2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "RFC 7693",
            runner: Runner::SingleStream(blake2b256),
        },
        Algorithm {
            name: "BLAKE2s256",
            crate_name: "blake2",
            output: OutputBits::Fixed(256),
            category: Category::Cryptographic,
            notes: "RFC 7693; used by WireGuard",
            runner: Runner::SingleStream(blake2s256),
        },
    ]
}
