//! SHA-1 family — pure-Rust software (`sha1` crate with `force-soft`) plus
//! optional hardware-accelerated entries via `ring` (x86 SHA-NI / ARMv8 SHA1)
//! behind the `sha-hw` feature.
//!
//! See the family's variant sub-modules — each file is scoped to a single
//! backing crate so the dependency graph for one variant cannot leak into the
//! other.

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
