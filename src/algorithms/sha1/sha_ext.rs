//! Hardware-accelerated SHA-1 via `ring`, which dispatches to x86 SHA-NI or
//! ARMv8 SHA1 instructions at runtime when present. The entry is filtered out
//! on hosts without the matching CPU feature so the `[sha-ext]` label remains
//! truthful — without that filter, `ring` would silently fall back to a
//! non-SHA-extension implementation.

use crate::algorithms::cpu::sha_ext_available;
use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

fn sha1(data: &[u8]) {
    black_box(ring::digest::digest(
        &ring::digest::SHA1_FOR_LEGACY_USE_ONLY,
        data,
    ));
}

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "SHA-1",
        variant: "sha-ext",
        crate_name: "ring",
        output: OutputBits::Fixed(160),
        category: Category::Cryptographic,
        notes: "x86 SHA-NI / ARMv8 SHA1 via `ring`",
        runner: Runner::SingleStream(sha1),
        available: sha_ext_available,
    }]
}
