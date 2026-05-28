//! aHash — Rust `hashbrown` default hasher.
//!
//! The AES-vs-fallback selection is **compile-time**: `ahash::AHasher`
//! re-exports `aes_hash::AHasher` when the binary is built with
//! `cfg(target_feature = "aes")` (on x86 / aarch64), otherwise it re-exports
//! `fallback_hash::AHasher` (pure-Rust). The fallback module is private to the
//! `ahash` crate so the two implementations cannot coexist in a single build
//! without forking the crate, which is out of scope.
//!
//! Variant tagging mirrors that selection: `[aes-ext]` when AES is on,
//! `[sw]` when not, so the dashboard reflects what actually ran.

use crate::registry::{always_available, Algorithm, Category, OutputBits, Runner};
use std::hash::Hasher;
use std::hint::black_box;

/// Hash data using AHash.
fn ahash(data: &[u8]) {
    let mut hasher = ahash::AHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
const VARIANT: &str = "aes-ext";
#[cfg(all(target_arch = "aarch64", target_feature = "aes"))]
const VARIANT: &str = "aes-ext";
#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    target_feature = "aes",
)))]
const VARIANT: &str = "sw";

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
const HW_REQUIRED: bool = true;
#[cfg(all(target_arch = "aarch64", target_feature = "aes"))]
const HW_REQUIRED: bool = true;
#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    target_feature = "aes",
)))]
const HW_REQUIRED: bool = false;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "aes",
))]
const HW_FEATURES: &[&str] = &["aes-ni"];
#[cfg(all(target_arch = "aarch64", target_feature = "aes"))]
const HW_FEATURES: &[&str] = &["armv8-aes"];
#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"),
    target_feature = "aes",
)))]
const HW_FEATURES: &[&str] = &[];

pub fn algorithms() -> Vec<Algorithm> {
    vec![Algorithm {
        name: "AHash",
        variant: VARIANT,
        crate_name: "ahash",
        output: OutputBits::Fixed(64),
        category: Category::NonCryptographic,
        notes: "Rust hashbrown default; AES-NI / ARMv8 AES when target_feature=\"aes\"",
        runner: Runner::SingleStream(ahash),
        available: always_available,
        keyed: true,
        dos_resistant: true,
        hardware_required: HW_REQUIRED,
        hardware_features: HW_FEATURES,
    }]
}
