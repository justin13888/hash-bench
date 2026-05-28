//! Cross-implementation consistency check (`just test`).
//!
//! Two things are checked:
//!
//! 1. **Digest equality across paired variants.** For pairs of registry
//!    entries that must produce identical output (SHA-1 / SHA-256 `[sw]` vs
//!    `[sha-ext]`, the three CRCs `[sw]` vs `[clmul]`/`[crc-ext]`, and
//!    BLAKE3 single vs `update_rayon`), several inputs are hashed through
//!    both and the digests compared byte-for-byte. Catches a HW variant
//!    silently falling back to a different code path, and catches the rayon
//!    variant disagreeing with single-threaded BLAKE3.
//!
//! 2. **Library manifest.** For every `(name, variant)` in the live
//!    registry, the test looks up an expected backing-crate entry and
//!    asserts the registry's `crate_name` matches. Catches a label drifting
//!    from the actual library — the only practical check for singletons
//!    (algorithms with no paired variant) without hardcoded test vectors.

use hash_bench::registry;
use rand::{rngs::StdRng, RngCore, SeedableRng};

// ── inputs ──────────────────────────────────────────────────────────────────

/// Test buffers fed through every paired algorithm. The 1 MiB buffer matters
/// because some HW dispatchers (and BLAKE3's `update_rayon`) only kick in
/// past a length threshold, so small inputs alone could mask a fallback.
fn inputs() -> Vec<(&'static str, Vec<u8>)> {
    let mut v: Vec<(&'static str, Vec<u8>)> = vec![
        ("empty", Vec::new()),
        ("\"abc\"", b"abc".to_vec()),
        ("1 KiB zeros", vec![0u8; 1024]),
        ("1 KiB cycle", (0..1024).map(|i| (i % 256) as u8).collect()),
    ];
    let mut buf = vec![0u8; 1 << 20];
    StdRng::seed_from_u64(0xC0FFEE).fill_bytes(&mut buf);
    v.push(("1 MiB random", buf));
    v
}

// ── digest dispatcher (paired entries only) ─────────────────────────────────

/// Compute the digest of `data` using `(name, variant)` as the lookup key.
/// Only entries that appear in [`pairs`] need an arm here. The dispatcher is
/// deliberately independent of the registry's `Runner` function pointers,
/// which `black_box(hasher.finalize())` and so can't be reused for equality
/// checks.
#[allow(unused_variables)]
fn digest(name: &str, variant: &str, data: &[u8]) -> Vec<u8> {
    match (name, variant) {
        #[cfg(feature = "sha1")]
        ("SHA-1", "sw") => {
            use sha1::{Digest, Sha1};
            Sha1::digest(data).to_vec()
        }
        #[cfg(all(
            feature = "sha-hw",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ))]
        ("SHA-1", "sha-ext") => hash_bench::algorithms::sha1::sha_ext::sha1_hw(data).to_vec(),
        #[cfg(feature = "sha2")]
        ("SHA-256", "sw") => {
            use sha2::{Digest, Sha256};
            Sha256::digest(data).to_vec()
        }
        #[cfg(feature = "sha-hw")]
        ("SHA-256", "sha-ext") => ring::digest::digest(&ring::digest::SHA256, data)
            .as_ref()
            .to_vec(),
        #[cfg(feature = "crc-sw")]
        ("CRC32", "sw") => {
            let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
            let mut d = crc.digest();
            d.update(data);
            d.finalize().to_le_bytes().to_vec()
        }
        #[cfg(feature = "crc")]
        ("CRC32", "clmul") => {
            let mut h = crc32fast::Hasher::new();
            h.update(data);
            h.finalize().to_le_bytes().to_vec()
        }
        #[cfg(feature = "crc-sw")]
        ("CRC32C", "sw") => {
            let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
            let mut d = crc.digest();
            d.update(data);
            d.finalize().to_le_bytes().to_vec()
        }
        #[cfg(feature = "crc")]
        ("CRC32C", "crc-ext") => crc32c::crc32c(data).to_le_bytes().to_vec(),
        #[cfg(feature = "crc-sw")]
        ("CRC64", "sw") => {
            let crc = crc::Crc::<u64>::new(&crc::CRC_64_XZ);
            let mut d = crc.digest();
            d.update(data);
            d.finalize().to_le_bytes().to_vec()
        }
        #[cfg(feature = "crc")]
        ("CRC64", "clmul") => {
            let mut h = crc64fast::Digest::new();
            h.write(data);
            h.sum64().to_le_bytes().to_vec()
        }
        #[cfg(feature = "blake3")]
        ("BLAKE3", "sw") => blake3::hash(data).as_bytes().to_vec(),
        #[cfg(feature = "blake3")]
        ("BLAKE3 (rayon)", "sw") => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .expect("build rayon pool");
            let mut h = blake3::Hasher::new();
            pool.install(|| h.update_rayon(data));
            h.finalize().as_bytes().to_vec()
        }
        _ => panic!(
            "tests/consistency.rs: no digest dispatcher for {name:?} [{variant:?}]; \
             add an arm in `digest()` and a matching entry in `pairs()` before \
             relying on this test for the new variant"
        ),
    }
}

// ── paired entries to cross-check ───────────────────────────────────────────

/// `(name_a, variant_a, name_b, variant_b)` — digests must agree byte-for-byte.
/// Cross-`name` pairs (e.g. BLAKE3 vs BLAKE3 (rayon)) are intentional and
/// the reason this list is explicit rather than derived from `name`-matching.
fn pairs() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    #[allow(unused_mut)]
    let mut v: Vec<(&'static str, &'static str, &'static str, &'static str)> = Vec::new();
    #[cfg(all(
        feature = "sha1",
        feature = "sha-hw",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    v.push(("SHA-1", "sw", "SHA-1", "sha-ext"));
    #[cfg(all(feature = "sha2", feature = "sha-hw"))]
    v.push(("SHA-256", "sw", "SHA-256", "sha-ext"));
    #[cfg(all(feature = "crc-sw", feature = "crc"))]
    {
        v.push(("CRC32", "sw", "CRC32", "clmul"));
        v.push(("CRC32C", "sw", "CRC32C", "crc-ext"));
        v.push(("CRC64", "sw", "CRC64", "clmul"));
    }
    #[cfg(feature = "blake3")]
    v.push(("BLAKE3", "sw", "BLAKE3 (rayon)", "sw"));
    v
}

// ── library manifest ────────────────────────────────────────────────────────

/// Expected `(name, variant) -> crate_name` for every registry entry. Each
/// entry is feature-gated so the manifest tracks whatever the current build
/// has compiled in. AHash lists both `[sw]` and `[aes-ext]` because the
/// variant is compile-time selected (only one ever appears in the registry).
fn manifest() -> Vec<(&'static str, &'static str, &'static str)> {
    #[allow(unused_mut)]
    let mut v: Vec<(&'static str, &'static str, &'static str)> = Vec::new();

    // ── Cryptographic ───────────────────────────────────────────────────────
    #[cfg(feature = "blake3")]
    {
        v.push(("BLAKE3", "sw", "blake3"));
        v.push(("BLAKE3 (rayon)", "sw", "blake3"));
    }
    #[cfg(feature = "blake2")]
    {
        v.push(("BLAKE2b512", "sw", "blake2"));
        v.push(("BLAKE2b256", "sw", "blake2"));
        v.push(("BLAKE2s256", "sw", "blake2"));
    }
    #[cfg(feature = "sha1")]
    v.push(("SHA-1", "sw", "sha1"));
    #[cfg(all(
        feature = "sha1",
        feature = "sha-hw",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    v.push(("SHA-1", "sha-ext", "hash-bench"));
    #[cfg(feature = "sha2")]
    {
        v.push(("SHA-224", "sw", "sha2"));
        v.push(("SHA-256", "sw", "sha2"));
        v.push(("SHA-384", "sw", "sha2"));
        v.push(("SHA-512", "sw", "sha2"));
        v.push(("SHA-512/224", "sw", "sha2"));
        v.push(("SHA-512/256", "sw", "sha2"));
    }
    #[cfg(all(feature = "sha2", feature = "sha-hw"))]
    v.push(("SHA-256", "sha-ext", "ring"));
    #[cfg(feature = "sha3")]
    {
        v.push(("SHA3-224", "sw", "sha3"));
        v.push(("SHA3-256", "sw", "sha3"));
        v.push(("SHA3-384", "sw", "sha3"));
        v.push(("SHA3-512", "sw", "sha3"));
        v.push(("SHAKE128", "sw", "sha3"));
        v.push(("SHAKE256", "sw", "sha3"));
        v.push(("Keccak-224", "sw", "sha3"));
        v.push(("Keccak-256", "sw", "sha3"));
        v.push(("Keccak-384", "sw", "sha3"));
        v.push(("Keccak-512", "sw", "sha3"));
    }
    #[cfg(feature = "md5")]
    v.push(("MD5", "sw", "md-5"));
    #[cfg(feature = "ripemd")]
    {
        v.push(("RIPEMD-128", "sw", "ripemd"));
        v.push(("RIPEMD-160", "sw", "ripemd"));
        v.push(("RIPEMD-256", "sw", "ripemd"));
        v.push(("RIPEMD-320", "sw", "ripemd"));
    }
    #[cfg(feature = "sm3")]
    v.push(("SM3", "sw", "sm3"));
    #[cfg(feature = "streebog")]
    {
        v.push(("Streebog-256", "sw", "streebog"));
        v.push(("Streebog-512", "sw", "streebog"));
    }
    #[cfg(feature = "kupyna")]
    {
        v.push(("Kupyna-224", "sw", "kupyna"));
        v.push(("Kupyna-256", "sw", "kupyna"));
        v.push(("Kupyna-384", "sw", "kupyna"));
        v.push(("Kupyna-512", "sw", "kupyna"));
    }
    #[cfg(feature = "whirlpool")]
    v.push(("Whirlpool", "sw", "whirlpool"));
    #[cfg(feature = "ascon")]
    v.push(("Ascon-Hash256", "sw", "ascon-hash"));

    // ── Non-cryptographic ───────────────────────────────────────────────────
    #[cfg(feature = "crc-sw")]
    {
        v.push(("CRC32", "sw", "crc"));
        v.push(("CRC32C", "sw", "crc"));
        v.push(("CRC64", "sw", "crc"));
    }
    #[cfg(feature = "crc")]
    {
        v.push(("CRC32", "clmul", "crc32fast"));
        v.push(("CRC32C", "crc-ext", "crc32c"));
        v.push(("CRC64", "clmul", "crc64fast"));
    }
    #[cfg(feature = "xxhash")]
    {
        v.push(("XXH32", "sw", "xxhash-rust"));
        v.push(("XXH64", "sw", "xxhash-rust"));
        v.push(("XXH3_64", "sw", "xxhash-rust"));
        v.push(("XXH3_128", "sw", "xxhash-rust"));
    }
    #[cfg(feature = "siphash")]
    {
        v.push(("SipHash-1-3", "sw", "siphasher"));
        v.push(("SipHash-2-4", "sw", "siphasher"));
    }
    #[cfg(feature = "ahash")]
    {
        v.push(("AHash", "sw", "ahash"));
        v.push(("AHash", "aes-ext", "ahash"));
    }
    #[cfg(feature = "wyhash")]
    v.push(("wyhash", "sw", "wyhash"));
    #[cfg(feature = "fxhash")]
    v.push(("FxHash", "sw", "rustc-hash"));
    #[cfg(feature = "farmhash")]
    v.push(("FarmHash", "sw", "farmhash"));
    #[cfg(feature = "murmur3")]
    v.push(("MurmurHash3", "sw", "murmur3"));
    #[cfg(feature = "highway")]
    {
        v.push(("HighwayHash-64", "sw", "highway"));
        v.push(("HighwayHash-128", "sw", "highway"));
        v.push(("HighwayHash-256", "sw", "highway"));
    }
    #[cfg(feature = "fnv")]
    v.push(("FNV-1a", "sw", "fnv"));
    #[cfg(feature = "adler")]
    v.push(("Adler32", "sw", "adler"));

    v
}

// ── tests ───────────────────────────────────────────────────────────────────

#[test]
fn cross_variant_digests_match() {
    let registry = registry();
    let inputs = inputs();
    let pairs = pairs();
    let in_registry = |n: &str, v: &str| registry.iter().any(|a| a.name == n && a.variant == v);

    let mut tested_pairs = 0usize;
    let mut comparisons = 0usize;
    let mut skipped: Vec<String> = Vec::new();

    for (name_a, variant_a, name_b, variant_b) in &pairs {
        if !in_registry(name_a, variant_a) || !in_registry(name_b, variant_b) {
            skipped.push(format!(
                "{name_a} [{variant_a}] vs {name_b} [{variant_b}] (variant filtered out by available() on this host)"
            ));
            continue;
        }
        tested_pairs += 1;
        for (input_label, data) in &inputs {
            let a = digest(name_a, variant_a, data);
            let b = digest(name_b, variant_b, data);
            assert_eq!(
                a,
                b,
                "digest mismatch — {name_a} [{variant_a}] vs {name_b} [{variant_b}] on {input_label} (len={})\n  {variant_a}: {}\n  {variant_b}: {}",
                data.len(),
                hex(&a),
                hex(&b),
            );
            comparisons += 1;
        }
    }

    println!(
        "consistency: {tested_pairs} pair(s) × {} input(s) = {comparisons} comparison(s) passed",
        inputs.len()
    );
    for s in &skipped {
        println!("  skipped: {s}");
    }
    assert!(
        !pairs.is_empty(),
        "no paired variants known — at least BLAKE3 (rayon vs single) should always be testable"
    );
}

#[test]
fn manifest_matches_registry() {
    let manifest = manifest();
    let registry = registry();

    assert!(
        !registry.is_empty(),
        "live registry is empty; --all-features expected"
    );

    for alg in &registry {
        let hit = manifest
            .iter()
            .find(|(n, v, _)| *n == alg.name && *v == alg.variant);
        match hit {
            Some((_, _, expected_crate)) => assert_eq!(
                alg.crate_name, *expected_crate,
                "{} [{}]: registry says crate={:?}, manifest expected crate={:?}",
                alg.name, alg.variant, alg.crate_name, expected_crate,
            ),
            None => panic!(
                "registry has {} [{}] (crate={}) but it's not in the manifest — \
                 add it in tests/consistency.rs::manifest()",
                alg.name, alg.variant, alg.crate_name
            ),
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}
