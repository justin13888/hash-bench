//! SHA-2 family (NIST FIPS 180-4) — pure-Rust software (`sha2` crate with
//! `force-soft`) plus optional hardware-accelerated entries via `ring` (x86
//! SHA-NI / ARMv8 SHA2) behind the `sha-hw` feature.
//!
//! Each variant lives in its own sub-module with its own backing crate, so a
//! `[sw]` row is genuinely soft and a `[sha-ext]` row genuinely uses the SHA
//! extension instructions on hosts that have them.

use crate::registry::Algorithm;

pub mod soft;

#[cfg(feature = "sha-hw")]
pub mod sha_ext;

pub fn algorithms() -> Vec<Algorithm> {
    #[allow(unused_mut)]
    let mut algs = soft::algorithms();
    #[cfg(feature = "sha-hw")]
    algs.extend(sha_ext::algorithms());
    algs
}
