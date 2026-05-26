//! Hash algorithm families, one module per backing crate.
//!
//! Each family module is gated behind its own Cargo feature and exposes
//! `algorithms() -> Vec<Algorithm>`. [`crate::registry`] aggregates whichever
//! families are enabled. Families that ship both software and
//! hardware-accelerated variants (sha1, sha2, crc) are directory modules with
//! one file per backing crate so the variants share no dependencies.

pub mod cpu;

#[cfg(feature = "adler")]
pub mod adler;
#[cfg(feature = "ahash")]
pub mod ahash;
#[cfg(feature = "ascon")]
pub mod ascon;
#[cfg(feature = "blake2")]
pub mod blake2;
#[cfg(feature = "blake3")]
pub mod blake3;
#[cfg(any(feature = "crc", feature = "crc-sw"))]
pub mod crc;
#[cfg(feature = "farmhash")]
pub mod farmhash;
#[cfg(feature = "fnv")]
pub mod fnv;
#[cfg(feature = "fxhash")]
pub mod fxhash;
#[cfg(feature = "highway")]
pub mod highway;
#[cfg(feature = "kupyna")]
pub mod kupyna;
#[cfg(feature = "md5")]
pub mod md5;
#[cfg(feature = "murmur3")]
pub mod murmur3;
#[cfg(feature = "ripemd")]
pub mod ripemd;
#[cfg(feature = "sha1")]
pub mod sha1;
#[cfg(feature = "sha2")]
pub mod sha2;
#[cfg(feature = "sha3")]
pub mod sha3;
#[cfg(feature = "siphash")]
pub mod siphash;
#[cfg(feature = "sm3")]
pub mod sm3;
#[cfg(feature = "streebog")]
pub mod streebog;
#[cfg(feature = "whirlpool")]
pub mod whirlpool;
#[cfg(feature = "wyhash")]
pub mod wyhash;
#[cfg(feature = "xxhash")]
pub mod xxhash;
