//! Hardware-accelerated SHA-1 implemented directly with CPU intrinsics so the
//! `[sha-ext]` label is truthful and auditable. The previous backend went
//! through `ring::digest::SHA1_FOR_LEGACY_USE_ONLY`, which on at least some
//! hosts measured at ~0.3 GB/s — incompatible with real SHA-NI dispatch and
//! consistent with ring leaving SHA-1 on its generic path because the digest
//! is deprecated upstream.
//!
//! The block-compression function is gated by `#[target_feature]` and is only
//! called after [`super::super::cpu::sha_ext_available`] confirms the host CPU
//! exposes the instruction set. On x86_64 that's SHA-NI; on aarch64 it's the
//! ARMv8 SHA1 extension (which Rust surfaces under the `sha2` aarch64 feature).

use crate::algorithms::cpu::sha_ext_available;
use crate::registry::{Algorithm, Category, OutputBits, Runner};
use std::hint::black_box;

#[cfg(target_arch = "aarch64")]
const HW_FEATURES: &[&str] = &["armv8-sha1"];
#[cfg(not(target_arch = "aarch64"))]
const HW_FEATURES: &[&str] = &["sha-ni"];

const INITIAL_STATE: [u32; 5] = [
    0x6745_2301,
    0xEFCD_AB89,
    0x98BA_DCFE,
    0x1032_5476,
    0xC3D2_E1F0,
];

/// One-shot SHA-1 over `data`, returning the 20-byte digest.
///
/// Caller must have verified [`sha_ext_available`] returns true.
#[inline]
pub fn sha1_hw(data: &[u8]) -> [u8; 20] {
    let mut state = INITIAL_STATE;

    // Process all complete 64-byte blocks.
    let full_blocks = data.len() / 64;
    let tail_start = full_blocks * 64;
    for i in 0..full_blocks {
        let block: &[u8; 64] = data[i * 64..i * 64 + 64].try_into().unwrap();
        // SAFETY: gated by `sha_ext_available()` at the registry level.
        unsafe { compress_block(&mut state, block) };
    }

    // Padding: append 0x80, zero-pad to 56 mod 64, then 8-byte BE bit length.
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let tail = &data[tail_start..];
    let mut buf = [0u8; 128];
    buf[..tail.len()].copy_from_slice(tail);
    buf[tail.len()] = 0x80;
    let pad_end = if tail.len() < 56 { 64 } else { 128 };
    buf[pad_end - 8..pad_end].copy_from_slice(&bit_len.to_be_bytes());

    // SAFETY: same gate.
    unsafe {
        let first: &[u8; 64] = buf[..64].try_into().unwrap();
        compress_block(&mut state, first);
        if pad_end == 128 {
            let second: &[u8; 64] = buf[64..128].try_into().unwrap();
            compress_block(&mut state, second);
        }
    }

    let mut out = [0u8; 20];
    for (i, w) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_be_bytes());
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// x86_64 SHA-NI block compression
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sha,sse2,ssse3,sse4.1")]
unsafe fn compress_block(state: &mut [u32; 5], block: &[u8; 64]) {
    use core::arch::x86_64::*;

    // Reverse all 16 bytes: this both byte-swaps each 32-bit word AND reverses
    // the lane order, so W[0] ends up in the high lane (which is what SHA-NI
    // expects as the first round's message input).
    let bswap_mask = _mm_set_epi64x(
        0x0001_0203_0405_0607u64 as i64,
        0x0809_0a0b_0c0d_0e0fu64 as i64,
    );

    // SHA-NI wants ABCD packed as [D, C, B, A]; load and reverse.
    let mut abcd = _mm_shuffle_epi32::<0x1B>(_mm_loadu_si128(state.as_ptr() as *const __m128i));
    let mut e0 = _mm_set_epi32(state[4] as i32, 0, 0, 0);

    let abcd_save = abcd;
    let e_save = e0;

    let blk = block.as_ptr() as *const __m128i;
    let mut m0 = _mm_shuffle_epi8(_mm_loadu_si128(blk.add(0)), bswap_mask);
    let mut m1 = _mm_shuffle_epi8(_mm_loadu_si128(blk.add(1)), bswap_mask);
    let mut m2 = _mm_shuffle_epi8(_mm_loadu_si128(blk.add(2)), bswap_mask);
    let mut m3 = _mm_shuffle_epi8(_mm_loadu_si128(blk.add(3)), bswap_mask);

    // Rounds 0-3
    e0 = _mm_add_epi32(e0, m0);
    let mut e1 = abcd;
    abcd = _mm_sha1rnds4_epu32::<0>(abcd, e0);

    // Rounds 4-7
    e1 = _mm_sha1nexte_epu32(e1, m1);
    e0 = abcd;
    abcd = _mm_sha1rnds4_epu32::<0>(abcd, e1);
    m0 = _mm_sha1msg1_epu32(m0, m1);

    // Rounds 8-11
    e0 = _mm_sha1nexte_epu32(e0, m2);
    e1 = abcd;
    abcd = _mm_sha1rnds4_epu32::<0>(abcd, e0);
    m1 = _mm_sha1msg1_epu32(m1, m2);
    m0 = _mm_xor_si128(m0, m2);

    // Rounds 12-15
    e1 = _mm_sha1nexte_epu32(e1, m3);
    e0 = abcd;
    m0 = _mm_sha1msg2_epu32(m0, m3);
    abcd = _mm_sha1rnds4_epu32::<0>(abcd, e1);
    m2 = _mm_sha1msg1_epu32(m2, m3);
    m1 = _mm_xor_si128(m1, m3);

    // Rounds 16-19
    e0 = _mm_sha1nexte_epu32(e0, m0);
    e1 = abcd;
    m1 = _mm_sha1msg2_epu32(m1, m0);
    abcd = _mm_sha1rnds4_epu32::<0>(abcd, e0);
    m3 = _mm_sha1msg1_epu32(m3, m0);
    m2 = _mm_xor_si128(m2, m0);

    // Rounds 20-23
    e1 = _mm_sha1nexte_epu32(e1, m1);
    e0 = abcd;
    m2 = _mm_sha1msg2_epu32(m2, m1);
    abcd = _mm_sha1rnds4_epu32::<1>(abcd, e1);
    m0 = _mm_sha1msg1_epu32(m0, m1);
    m3 = _mm_xor_si128(m3, m1);

    // Rounds 24-27
    e0 = _mm_sha1nexte_epu32(e0, m2);
    e1 = abcd;
    m3 = _mm_sha1msg2_epu32(m3, m2);
    abcd = _mm_sha1rnds4_epu32::<1>(abcd, e0);
    m1 = _mm_sha1msg1_epu32(m1, m2);
    m0 = _mm_xor_si128(m0, m2);

    // Rounds 28-31
    e1 = _mm_sha1nexte_epu32(e1, m3);
    e0 = abcd;
    m0 = _mm_sha1msg2_epu32(m0, m3);
    abcd = _mm_sha1rnds4_epu32::<1>(abcd, e1);
    m2 = _mm_sha1msg1_epu32(m2, m3);
    m1 = _mm_xor_si128(m1, m3);

    // Rounds 32-35
    e0 = _mm_sha1nexte_epu32(e0, m0);
    e1 = abcd;
    m1 = _mm_sha1msg2_epu32(m1, m0);
    abcd = _mm_sha1rnds4_epu32::<1>(abcd, e0);
    m3 = _mm_sha1msg1_epu32(m3, m0);
    m2 = _mm_xor_si128(m2, m0);

    // Rounds 36-39
    e1 = _mm_sha1nexte_epu32(e1, m1);
    e0 = abcd;
    m2 = _mm_sha1msg2_epu32(m2, m1);
    abcd = _mm_sha1rnds4_epu32::<1>(abcd, e1);
    m0 = _mm_sha1msg1_epu32(m0, m1);
    m3 = _mm_xor_si128(m3, m1);

    // Rounds 40-43
    e0 = _mm_sha1nexte_epu32(e0, m2);
    e1 = abcd;
    m3 = _mm_sha1msg2_epu32(m3, m2);
    abcd = _mm_sha1rnds4_epu32::<2>(abcd, e0);
    m1 = _mm_sha1msg1_epu32(m1, m2);
    m0 = _mm_xor_si128(m0, m2);

    // Rounds 44-47
    e1 = _mm_sha1nexte_epu32(e1, m3);
    e0 = abcd;
    m0 = _mm_sha1msg2_epu32(m0, m3);
    abcd = _mm_sha1rnds4_epu32::<2>(abcd, e1);
    m2 = _mm_sha1msg1_epu32(m2, m3);
    m1 = _mm_xor_si128(m1, m3);

    // Rounds 48-51
    e0 = _mm_sha1nexte_epu32(e0, m0);
    e1 = abcd;
    m1 = _mm_sha1msg2_epu32(m1, m0);
    abcd = _mm_sha1rnds4_epu32::<2>(abcd, e0);
    m3 = _mm_sha1msg1_epu32(m3, m0);
    m2 = _mm_xor_si128(m2, m0);

    // Rounds 52-55
    e1 = _mm_sha1nexte_epu32(e1, m1);
    e0 = abcd;
    m2 = _mm_sha1msg2_epu32(m2, m1);
    abcd = _mm_sha1rnds4_epu32::<2>(abcd, e1);
    m0 = _mm_sha1msg1_epu32(m0, m1);
    m3 = _mm_xor_si128(m3, m1);

    // Rounds 56-59
    e0 = _mm_sha1nexte_epu32(e0, m2);
    e1 = abcd;
    m3 = _mm_sha1msg2_epu32(m3, m2);
    abcd = _mm_sha1rnds4_epu32::<2>(abcd, e0);
    m1 = _mm_sha1msg1_epu32(m1, m2);
    m0 = _mm_xor_si128(m0, m2);

    // Rounds 60-63
    e1 = _mm_sha1nexte_epu32(e1, m3);
    e0 = abcd;
    m0 = _mm_sha1msg2_epu32(m0, m3);
    abcd = _mm_sha1rnds4_epu32::<3>(abcd, e1);
    m2 = _mm_sha1msg1_epu32(m2, m3);
    m1 = _mm_xor_si128(m1, m3);

    // Rounds 64-67
    e0 = _mm_sha1nexte_epu32(e0, m0);
    e1 = abcd;
    m1 = _mm_sha1msg2_epu32(m1, m0);
    abcd = _mm_sha1rnds4_epu32::<3>(abcd, e0);
    m3 = _mm_sha1msg1_epu32(m3, m0);
    m2 = _mm_xor_si128(m2, m0);

    // Rounds 68-71
    e1 = _mm_sha1nexte_epu32(e1, m1);
    e0 = abcd;
    m2 = _mm_sha1msg2_epu32(m2, m1);
    abcd = _mm_sha1rnds4_epu32::<3>(abcd, e1);
    m3 = _mm_xor_si128(m3, m1);

    // Rounds 72-75
    e0 = _mm_sha1nexte_epu32(e0, m2);
    e1 = abcd;
    m3 = _mm_sha1msg2_epu32(m3, m2);
    abcd = _mm_sha1rnds4_epu32::<3>(abcd, e0);

    // Rounds 76-79
    e1 = _mm_sha1nexte_epu32(e1, m3);
    e0 = abcd;
    abcd = _mm_sha1rnds4_epu32::<3>(abcd, e1);

    // Combine with saved state.
    e0 = _mm_sha1nexte_epu32(e0, e_save);
    abcd = _mm_add_epi32(abcd, abcd_save);

    let abcd_out = _mm_shuffle_epi32::<0x1B>(abcd);
    _mm_storeu_si128(state.as_mut_ptr() as *mut __m128i, abcd_out);
    state[4] = _mm_extract_epi32::<3>(e0) as u32;
}

// ─────────────────────────────────────────────────────────────────────────────
// aarch64 ARMv8 SHA1 block compression
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "sha2")]
unsafe fn compress_block(state: &mut [u32; 5], block: &[u8; 64]) {
    use core::arch::aarch64::*;

    const K0: u32 = 0x5A82_7999;
    const K1: u32 = 0x6ED9_EBA1;
    const K2: u32 = 0x8F1B_BCDC;
    const K3: u32 = 0xCA62_C1D6;

    let k0v = vdupq_n_u32(K0);
    let k1v = vdupq_n_u32(K1);
    let k2v = vdupq_n_u32(K2);
    let k3v = vdupq_n_u32(K3);

    let abcd0 = vld1q_u32(state.as_ptr());
    let e0_init = state[4];

    // Byte-swap 16 big-endian u32 message words into NEON regs.
    let p = block.as_ptr() as *const u32;
    let mut w0 = vreinterpretq_u32_u8(vrev32q_u8(vreinterpretq_u8_u32(vld1q_u32(p))));
    let mut w1 = vreinterpretq_u32_u8(vrev32q_u8(vreinterpretq_u8_u32(vld1q_u32(p.add(4)))));
    let mut w2 = vreinterpretq_u32_u8(vrev32q_u8(vreinterpretq_u8_u32(vld1q_u32(p.add(8)))));
    let mut w3 = vreinterpretq_u32_u8(vrev32q_u8(vreinterpretq_u8_u32(vld1q_u32(p.add(12)))));

    let mut abcd = abcd0;
    let mut e = e0_init;
    let mut tw0 = vaddq_u32(w0, k0v);
    let mut tw1 = vaddq_u32(w1, k0v);

    // Rounds 0-3 (Ch)
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1cq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w2, k0v);
    w0 = vsha1su0q_u32(w0, w1, w2);

    // Rounds 4-7
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1cq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w3, k0v);
    w0 = vsha1su1q_u32(w0, w3);
    w1 = vsha1su0q_u32(w1, w2, w3);

    // Rounds 8-11
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1cq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w0, k0v);
    w1 = vsha1su1q_u32(w1, w0);
    w2 = vsha1su0q_u32(w2, w3, w0);

    // Rounds 12-15
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1cq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w1, k1v);
    w2 = vsha1su1q_u32(w2, w1);
    w3 = vsha1su0q_u32(w3, w0, w1);

    // Rounds 16-19
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1cq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w2, k1v);
    w3 = vsha1su1q_u32(w3, w2);
    w0 = vsha1su0q_u32(w0, w1, w2);

    // Rounds 20-23 (Parity)
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w3, k1v);
    w0 = vsha1su1q_u32(w0, w3);
    w1 = vsha1su0q_u32(w1, w2, w3);

    // Rounds 24-27
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w0, k1v);
    w1 = vsha1su1q_u32(w1, w0);
    w2 = vsha1su0q_u32(w2, w3, w0);

    // Rounds 28-31
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w1, k1v);
    w2 = vsha1su1q_u32(w2, w1);
    w3 = vsha1su0q_u32(w3, w0, w1);

    // Rounds 32-35
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w2, k2v);
    w3 = vsha1su1q_u32(w3, w2);
    w0 = vsha1su0q_u32(w0, w1, w2);

    // Rounds 36-39
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w3, k2v);
    w0 = vsha1su1q_u32(w0, w3);
    w1 = vsha1su0q_u32(w1, w2, w3);

    // Rounds 40-43 (Maj)
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1mq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w0, k2v);
    w1 = vsha1su1q_u32(w1, w0);
    w2 = vsha1su0q_u32(w2, w3, w0);

    // Rounds 44-47
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1mq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w1, k2v);
    w2 = vsha1su1q_u32(w2, w1);
    w3 = vsha1su0q_u32(w3, w0, w1);

    // Rounds 48-51
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1mq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w2, k2v);
    w3 = vsha1su1q_u32(w3, w2);
    w0 = vsha1su0q_u32(w0, w1, w2);

    // Rounds 52-55
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1mq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w3, k3v);
    w0 = vsha1su1q_u32(w0, w3);
    w1 = vsha1su0q_u32(w1, w2, w3);

    // Rounds 56-59
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1mq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w0, k3v);
    w1 = vsha1su1q_u32(w1, w0);
    w2 = vsha1su0q_u32(w2, w3, w0);

    // Rounds 60-63 (Parity)
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w1, k3v);
    w2 = vsha1su1q_u32(w2, w1);
    w3 = vsha1su0q_u32(w3, w0, w1);

    // Rounds 64-67
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e, tw0);
    tw0 = vaddq_u32(w2, k3v);
    w3 = vsha1su1q_u32(w3, w2);

    // Rounds 68-71
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);
    tw1 = vaddq_u32(w3, k3v);

    // Rounds 72-75
    let e1 = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e, tw0);

    // Rounds 76-79
    e = vsha1h_u32(vgetq_lane_u32::<0>(abcd));
    abcd = vsha1pq_u32(abcd, e1, tw1);

    let abcd_out = vaddq_u32(abcd0, abcd);
    let e_out = e0_init.wrapping_add(e);

    vst1q_u32(state.as_mut_ptr(), abcd_out);
    state[4] = e_out;
}

// ─────────────────────────────────────────────────────────────────────────────
// Other architectures: no-op stub. `sha_ext_available()` returns false there,
// so `algorithms()` returns an empty vec and this is never called.
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
unsafe fn compress_block(_state: &mut [u32; 5], _block: &[u8; 64]) {
    unreachable!("sha_ext path invoked on an architecture without SHA-NI / ARMv8 SHA1");
}

fn sha1(data: &[u8]) {
    black_box(sha1_hw(data));
}

pub fn algorithms() -> Vec<Algorithm> {
    // On architectures without a SHA-1 hardware extension we don't expose the
    // entry at all — the `available` predicate would do the same, but skipping
    // here keeps `objdump | grep sha1rnds4` clean on those targets too.
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        return Vec::new();
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    vec![Algorithm {
        name: "SHA-1",
        variant: "sha-ext",
        crate_name: "hash-bench",
        output: OutputBits::Fixed(160),
        category: Category::Cryptographic,
        notes: "x86 SHA-NI / ARMv8 SHA1 via in-tree intrinsics",
        runner: Runner::SingleStream(sha1),
        available: sha_ext_available,
        keyed: false,
        dos_resistant: false,
        hardware_required: true,
        hardware_features: HW_FEATURES,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(b: &[u8]) -> String {
        let mut s = String::with_capacity(b.len() * 2);
        for byte in b {
            s.push_str(&format!("{byte:02x}"));
        }
        s
    }

    fn run_if_available<F: FnOnce()>(f: F) {
        if sha_ext_available() {
            f();
        } else {
            eprintln!("skipping: host lacks SHA-NI / ARMv8 SHA1");
        }
    }

    #[test]
    fn kat_empty() {
        run_if_available(|| {
            assert_eq!(
                hex(&sha1_hw(b"")),
                "da39a3ee5e6b4b0d3255bfef95601890afd80709"
            );
        });
    }

    #[test]
    fn kat_abc() {
        run_if_available(|| {
            assert_eq!(
                hex(&sha1_hw(b"abc")),
                "a9993e364706816aba3e25717850c26c9cd0d89d"
            );
        });
    }

    #[test]
    fn kat_56_byte_boundary() {
        // 56-byte input forces the padding into a second block.
        run_if_available(|| {
            let msg = [b'a'; 56];
            assert_eq!(
                hex(&sha1_hw(&msg)),
                "c2db330f6083854c99d4b5bfb6e8f29f201be699"
            );
        });
    }

    #[test]
    fn kat_million_a() {
        run_if_available(|| {
            let msg = vec![b'a'; 1_000_000];
            assert_eq!(
                hex(&sha1_hw(&msg)),
                "34aa973cd4c4daa4f61eeb2bdbad27316534016f"
            );
        });
    }
}
