//! SHA-3 / SHAKE / Keccak family (NIST FIPS 202 + pre-NIST Keccak).
//!
//! No hardware-accelerated variant in this pass: ARMv8.2's SHA-3 instructions
//! (EOR3 / RAX1 / XAR / BCAX) exist but no Rust crate dispatches to them
//! today, and there is no x86 SHA-3 ISA. `sha3 0.10`'s `asm` feature enables
//! generic ARMv8 Keccak assembly in the `keccak` crate, not SHA-3-specific
//! instructions, so it is not a true HW variant in the issue #3 sense.

use crate::registry::Algorithm;

pub mod soft;

pub fn algorithms() -> Vec<Algorithm> {
    soft::algorithms()
}
