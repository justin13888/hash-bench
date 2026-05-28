//! Ascon-Hash256 — NIST SP 800-232 lightweight standard.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

/// Hash data using Ascon-Hash256.
fn ascon256(data: &[u8]) {
    // `ascon-hash` re-exports its own `Digest` trait; importing it locally keeps
    // it from clashing with the RustCrypto `Digest` used by sibling modules.
    use ascon_hash::Digest;
    let mut hasher = ascon_hash::AsconHash256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "Ascon-Hash256",
        variant: "sw",
        crate_name: "ascon-hash",
        output: OutputBits::Fixed(256),
        category: Category::Cryptographic,
        notes: "NIST SP 800-232 lightweight standard",
        runner: Runner::SingleStream(ascon256),
        available: always_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: false,
        hardware_features: &[],
    }]
}
