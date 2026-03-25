use std::{hint::black_box, time::Duration};

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration, PlottingBackend,
};
use humansize::{FormatSize, BINARY};
use rand::Rng;
use rayon::prelude::*;
use sha2::Digest;
use std::hash::Hasher;
use std::io::Cursor;

/// Generate arbitrary data of a given size
fn generate_data(size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    let mut rng = rand::rng();
    rng.fill(&mut data[..]);

    data
}

// ── Cryptographic ───────────────────────────────────────────────────────────

/// Hash data using BLAKE3 (single-threaded)
fn hash_blake3(data: &[u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE3 with rayon parallelism (multi-threaded single stream)
fn hash_blake3_rayon(data: &[u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update_rayon(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE2b512
fn hash_blake2b(data: &[u8]) {
    let mut hasher = blake2::Blake2b512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE2b256
fn hash_blake2b256(data: &[u8]) {
    use blake2::digest::consts::U32;
    let mut hasher = blake2::Blake2b::<U32>::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using BLAKE2s256
fn hash_blake2s(data: &[u8]) {
    let mut hasher = blake2::Blake2s256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-1
fn hash_sha1(data: &[u8]) {
    let mut hasher = sha1::Sha1::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-224
fn hash_sha224(data: &[u8]) {
    let mut hasher = sha2::Sha224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-256
fn hash_sha256(data: &[u8]) {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-384
fn hash_sha384(data: &[u8]) {
    let mut hasher = sha2::Sha384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512
fn hash_sha512(data: &[u8]) {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512/224
fn hash_sha512_224(data: &[u8]) {
    let mut hasher = sha2::Sha512_224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA-512/256
fn hash_sha512_256(data: &[u8]) {
    let mut hasher = sha2::Sha512_256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA3-224
fn hash_sha3_224(data: &[u8]) {
    let mut hasher = sha3::Sha3_224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA3-256
fn hash_sha3_256(data: &[u8]) {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA3-384
fn hash_sha3_384(data: &[u8]) {
    let mut hasher = sha3::Sha3_384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHA3-512
fn hash_sha3_512(data: &[u8]) {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SHAKE128 (256-bit output)
fn hash_shake128(data: &[u8]) {
    use sha3::digest::{ExtendableOutput, Update, XofReader};
    let mut hasher = sha3::Shake128::default();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 32];
    reader.read(&mut result);
    black_box(result);
}

/// Hash data using SHAKE256 (512-bit output)
fn hash_shake256(data: &[u8]) {
    use sha3::digest::{ExtendableOutput, Update, XofReader};
    let mut hasher = sha3::Shake256::default();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 64];
    reader.read(&mut result);
    black_box(result);
}

/// Hash data using Keccak-224
fn hash_keccak224(data: &[u8]) {
    let mut hasher = sha3::Keccak224::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Keccak-256
fn hash_keccak256(data: &[u8]) {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Keccak-384
fn hash_keccak384(data: &[u8]) {
    let mut hasher = sha3::Keccak384::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Keccak-512
fn hash_keccak512(data: &[u8]) {
    let mut hasher = sha3::Keccak512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using MD5
fn hash_md5(data: &[u8]) {
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-128
fn hash_ripemd128(data: &[u8]) {
    let mut hasher = ripemd::Ripemd128::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-160
fn hash_ripemd160(data: &[u8]) {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-256
fn hash_ripemd256(data: &[u8]) {
    let mut hasher = ripemd::Ripemd256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using RIPEMD-320
fn hash_ripemd320(data: &[u8]) {
    let mut hasher = ripemd::Ripemd320::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using SM3
fn hash_sm3(data: &[u8]) {
    let mut hasher = sm3::Sm3::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Streebog-256
fn hash_streebog256(data: &[u8]) {
    let mut hasher = streebog::Streebog256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Streebog-512
fn hash_streebog512(data: &[u8]) {
    let mut hasher = streebog::Streebog512::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Whirlpool
fn hash_whirlpool(data: &[u8]) {
    let mut hasher = whirlpool::Whirlpool::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using Ascon-Hash256 (NIST SP 800-232 lightweight standard)
fn hash_ascon256(data: &[u8]) {
    use ascon_hash::Digest;
    let mut hasher = ascon_hash::AsconHash256::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

// ── Non-cryptographic ───────────────────────────────────────────────────────

/// Hash data using CRC32
fn hash_crc32(data: &[u8]) {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    black_box(hasher.finalize());
}

/// Hash data using CRC32C
fn hash_crc32c(data: &[u8]) {
    black_box(crc32c::crc32c(data));
}

/// Hash data using CRC64
fn hash_crc64(data: &[u8]) {
    let mut hasher = crc64fast::Digest::new();
    hasher.write(data);
    black_box(hasher.sum64());
}

/// Hash data using XXH32
fn hash_xxh32(data: &[u8]) {
    black_box(xxhash_rust::xxh32::xxh32(data, 0));
}

/// Hash data using XXH64
fn hash_xxh64(data: &[u8]) {
    black_box(xxhash_rust::xxh64::xxh64(data, 0));
}

/// Hash data using XXH3_64
fn hash_xxh3_64(data: &[u8]) {
    black_box(xxhash_rust::xxh3::xxh3_64(data));
}

/// Hash data using XXH3_128
fn hash_xxh3_128(data: &[u8]) {
    black_box(xxhash_rust::xxh3::xxh3_128(data));
}

/// Hash data using SipHash-1-3
fn hash_siphash13(data: &[u8]) {
    let mut hasher = siphasher::sip::SipHasher13::new();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using SipHash-2-4
fn hash_siphash24(data: &[u8]) {
    let mut hasher = siphasher::sip::SipHasher24::new();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using AHash
fn hash_ahash(data: &[u8]) {
    let mut hasher = ahash::AHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using wyhash
fn hash_wyhash(data: &[u8]) {
    black_box(wyhash::wyhash(data, 0));
}

/// Hash data using FxHash
fn hash_fxhash(data: &[u8]) {
    let mut hasher = rustc_hash::FxHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using FarmHash (64-bit)
fn hash_farmhash(data: &[u8]) {
    black_box(farmhash::hash64(data));
}

/// Hash data using MurmurHash3 (x64, 128-bit)
fn hash_murmur3(data: &[u8]) {
    black_box(
        murmur3::murmur3_x64_128(&mut Cursor::new(data), 0)
            .expect("murmur3 on in-memory cursor never fails"),
    );
}

/// Hash data using HighwayHash-64
fn hash_highway(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash64(data));
}

/// Hash data using HighwayHash-128
fn hash_highway128(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash128(data));
}

/// Hash data using HighwayHash-256
fn hash_highway256(data: &[u8]) {
    use highway::HighwayHash;
    black_box(highway::HighwayHasher::default().hash256(data));
}

/// Hash data using FNV-1a
fn hash_fnv1a(data: &[u8]) {
    let mut hasher = fnv::FnvHasher::default();
    hasher.write(data);
    black_box(hasher.finish());
}

/// Hash data using Adler32
fn hash_adler32(data: &[u8]) {
    let mut hasher = adler::Adler32::new();
    hasher.write_slice(data);
    black_box(hasher.checksum());
}

fn hashmark(c: &mut Criterion) {
    // Detect number of CPU cores
    let physical_cpus = num_cpus::get_physical();
    println!("Detected {physical_cpus} physical CPU cores.");
    let logical_cpus = num_cpus::get();
    println!("Detected {logical_cpus} logical CPU cores.");

    // Sizes of files to hash
    let sizes = [
        64,                // 64 B  — per-call overhead / hash-table key size
        256,               // 256 B — small message
        1024,              // 1 KiB
        10 * 1024 * 1024,  // 10 MiB
        100 * 1024 * 1024, // 100 MiB
    ];
    println!(
        "Benchmarking file sizes: {:?}",
        sizes
            .iter()
            .map(|s| s.format_size(BINARY))
            .collect::<Vec<String>>()
    );

    // Number of files to hash in parallel
    let mut thread_counts = vec![1, physical_cpus, logical_cpus];
    thread_counts.sort();
    thread_counts.dedup();
    println!("Benchmarking parallel iterations: {:?}", thread_counts);

    // Hashing algorithms to benchmark
    #[allow(clippy::type_complexity)]
    let hash_algs: [(String, fn(&[u8])); _] = [
        // Cryptographic
        ("BLAKE3".to_string(), hash_blake3),
        ("BLAKE3 (rayon)".to_string(), hash_blake3_rayon),
        ("BLAKE2b512".to_string(), hash_blake2b),
        ("BLAKE2b256".to_string(), hash_blake2b256),
        ("BLAKE2s256".to_string(), hash_blake2s),
        ("SHA-1".to_string(), hash_sha1),
        ("SHA-224".to_string(), hash_sha224),
        ("SHA-256".to_string(), hash_sha256),
        ("SHA-384".to_string(), hash_sha384),
        ("SHA-512".to_string(), hash_sha512),
        ("SHA-512/224".to_string(), hash_sha512_224),
        ("SHA-512/256".to_string(), hash_sha512_256),
        ("SHA3-224".to_string(), hash_sha3_224),
        ("SHA3-256".to_string(), hash_sha3_256),
        ("SHA3-384".to_string(), hash_sha3_384),
        ("SHA3-512".to_string(), hash_sha3_512),
        ("SHAKE128".to_string(), hash_shake128),
        ("SHAKE256".to_string(), hash_shake256),
        ("Keccak-224".to_string(), hash_keccak224),
        ("Keccak-256".to_string(), hash_keccak256),
        ("Keccak-384".to_string(), hash_keccak384),
        ("Keccak-512".to_string(), hash_keccak512),
        ("MD5".to_string(), hash_md5),
        ("RIPEMD-128".to_string(), hash_ripemd128),
        ("RIPEMD-160".to_string(), hash_ripemd160),
        ("RIPEMD-256".to_string(), hash_ripemd256),
        ("RIPEMD-320".to_string(), hash_ripemd320),
        ("SM3".to_string(), hash_sm3),
        ("Streebog-256".to_string(), hash_streebog256),
        ("Streebog-512".to_string(), hash_streebog512),
        ("Whirlpool".to_string(), hash_whirlpool),
        ("Ascon-Hash256".to_string(), hash_ascon256),
        // Non-cryptographic
        ("CRC32".to_string(), hash_crc32),
        ("CRC32C".to_string(), hash_crc32c),
        ("CRC64".to_string(), hash_crc64),
        ("XXH32".to_string(), hash_xxh32),
        ("XXH64".to_string(), hash_xxh64),
        ("XXH3_64".to_string(), hash_xxh3_64),
        ("XXH3_128".to_string(), hash_xxh3_128),
        ("SipHash-1-3".to_string(), hash_siphash13),
        ("SipHash-2-4".to_string(), hash_siphash24),
        ("AHash".to_string(), hash_ahash),
        ("wyhash".to_string(), hash_wyhash),
        ("FxHash".to_string(), hash_fxhash),
        ("FarmHash".to_string(), hash_farmhash),
        ("MurmurHash3".to_string(), hash_murmur3),
        ("HighwayHash-64".to_string(), hash_highway),
        ("HighwayHash-128".to_string(), hash_highway128),
        ("HighwayHash-256".to_string(), hash_highway256),
        ("FNV-1a".to_string(), hash_fnv1a),
        ("Adler32".to_string(), hash_adler32),
    ];
    println!(
        "Benchmarking hashing algorithms: {:?}",
        hash_algs
            .iter()
            .map(|(name, _)| name)
            .collect::<Vec<&String>>()
    );

    add_benchmarks(c, &sizes, &thread_counts, &hash_algs);
}

/// Adds benchmarks for hashing algorithms
/// # Arguments
/// * `c` - Criterion instance
/// * `sizes` - Sizes of files to hash
/// * `thread_counts` - Number of files to hash in parallel
/// * `hash_algs` - Hashing algorithms to benchmark
#[allow(clippy::type_complexity)]
fn add_benchmarks(
    c: &mut Criterion,
    sizes: &[usize],
    thread_counts: &[usize],
    hash_algs: &[(String, fn(&[u8]))],
) {
    // Pre-generate data once per size so every thread-count group hashes
    // identical buffers, removing a source of run-to-run variance.
    let datasets: Vec<(usize, String, Vec<u8>)> = sizes
        .iter()
        .map(|&size| {
            let label = size.format_size(BINARY);
            let data = generate_data(size);
            (size, label, data)
        })
        .collect();

    for &parallel_iterations in thread_counts {
        let mut group =
            c.benchmark_group(format!("{}-threaded Hashing", parallel_iterations));
        group.plot_config(
            PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic),
        );

        for (size, size_str, data) in &datasets {
            group.throughput(criterion::Throughput::Bytes(
                *size as u64 * parallel_iterations as u64,
            ));

            for (hash_name, hash_alg) in hash_algs {
                // NOTE: When parallel_iterations > 1, BLAKE3 (rayon)'s inner
                // `update_rayon` competes with the outer par_iter for the same
                // rayon thread pool. Its multi-threaded results therefore
                // understate peak single-stream throughput in this mode.
                group.bench_with_input(
                    BenchmarkId::new(hash_name, size_str),
                    data,
                    |b, data| {
                        b.iter(|| {
                            (0..parallel_iterations)
                                .into_par_iter()
                                .for_each(|_| hash_alg(black_box(data)))
                        })
                    },
                );
            }
        }
        group.finish();
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(30))
        .sample_size(20)
        .plotting_backend(PlottingBackend::Plotters)
        .with_plots();
    targets = hashmark
}
criterion_main!(benches);
