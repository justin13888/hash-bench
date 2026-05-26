//! CRC family — CRC32 (IEEE), CRC32C (Castagnoli), CRC64 (ECMA-182).
//!
//! Pure-Rust software variants via the `crc` crate (table-based) live in
//! `soft.rs` under the `crc-sw` feature. Hardware-accelerated variants via
//! `crc32fast` / `crc32c` / `crc64fast` (x86 PCLMULQDQ + SSE4.2 / ARMv8 CRC +
//! PMULL) live in `hw.rs` under the `crc` feature. The two sub-modules use
//! entirely different backing crates, so each entry's variant tag genuinely
//! describes the code path that ran.

use crate::registry::Algorithm;

#[cfg(feature = "crc-sw")]
pub mod soft;

#[cfg(feature = "crc")]
pub mod hw;

pub fn algorithms() -> Vec<Algorithm> {
    #[allow(unused_mut)]
    let mut algs: Vec<Algorithm> = Vec::new();
    #[cfg(feature = "crc-sw")]
    algs.extend(soft::algorithms());
    #[cfg(feature = "crc")]
    algs.extend(hw::algorithms());
    algs
}
