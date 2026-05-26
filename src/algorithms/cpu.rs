//! Runtime CPU-feature detection helpers used by the hardware-accelerated
//! algorithm variants to decide whether their entry should appear in the
//! registry on the current host. Keeping these in one place ensures every
//! `[sha-ext]`, `[clmul]`, `[crc-ext]` etc. label means the same thing across
//! families.

/// True when the host CPU exposes the SHA extension instruction set used for
/// SHA-1 / SHA-256 acceleration: x86 SHA-NI or ARMv8 SHA1/SHA2.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn sha_ext_available() -> bool {
    std::arch::is_x86_feature_detected!("sha")
}

#[cfg(target_arch = "aarch64")]
pub fn sha_ext_available() -> bool {
    std::arch::is_aarch64_feature_detected!("sha2")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
pub fn sha_ext_available() -> bool {
    false
}

/// True when the host CPU exposes carry-less multiply: x86 PCLMULQDQ or ARMv8
/// PMULL. Used by the CRC32 / CRC64 PCLMULQDQ-based implementations.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn clmul_available() -> bool {
    std::arch::is_x86_feature_detected!("pclmulqdq")
}

#[cfg(target_arch = "aarch64")]
pub fn clmul_available() -> bool {
    // PMULL is exposed under the `aes` feature flag on aarch64.
    std::arch::is_aarch64_feature_detected!("aes")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
pub fn clmul_available() -> bool {
    false
}

/// True when the host CPU exposes the CRC32 instruction used by CRC32C
/// (Castagnoli): x86 SSE4.2 `crc32` or ARMv8 `crc32c` / `crc32` instructions.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn crc_ext_available() -> bool {
    std::arch::is_x86_feature_detected!("sse4.2")
}

#[cfg(target_arch = "aarch64")]
pub fn crc_ext_available() -> bool {
    std::arch::is_aarch64_feature_detected!("crc")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
pub fn crc_ext_available() -> bool {
    false
}
