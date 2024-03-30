use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humansize::{FormatSize, BINARY};
use rand::{rngs::OsRng, RngCore};
use rayon::prelude::*;
use sha2::{Digest, Sha256};

/// Generate arbitrary data of a given size
fn generate_data(size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    OsRng.fill_bytes(&mut data);
    data
}

/// Hash data using BLAKE3
fn hash_blake3(data: &[u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update_rayon(data);
    let _result: [u8; 32] = hasher.finalize().into();
}

/// Hash data using BLAKE2b512
fn hash_blake2b(data: &[u8]) {
    let mut hasher = blake2::Blake2b512::new();
    hasher.update(data);
    let _result: [u8; 64] = hasher.finalize().into();
}

/// Hash data using SHA-512
fn hash_sha256(data: &[u8]) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let _result: [u8; 32] = hasher.finalize().into();
}

/// Hash data using SHA-512
fn hash_sha512(data: &[u8]) {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    let _result: [u8; 64] = hasher.finalize().into();
}

/// Hash data using SHA3-256
fn hash_sha3_256(data: &[u8]) {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(data);
    let _result: [u8; 32] = hasher.finalize().into();
}

/// Hash data using SHA3-512
fn hash_sha3_512(data: &[u8]) {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(data);
    let _result: [u8; 64] = hasher.finalize().into();
}

/// Hash data using MD5
fn hash_md5(data: &[u8]) {
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    let _result: [u8; 16] = hasher.finalize().into();
}

/// Hash data using Tiger2
fn hash_tiger2(data: &[u8]) {
    let mut hasher = tiger::Tiger::new();
    hasher.update(data);
    let _result: [u8; 24] = hasher.finalize().into();
}

/// Hash data using Whirlpool
fn hash_whirlpool(data: &[u8]) {
    let mut hasher = whirlpool::Whirlpool::new();
    hasher.update(data);
    let _result: [u8; 64] = hasher.finalize().into();
}

/// Hash data using CRC32
fn hash_crc32(data: &[u8]) {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    let _result: u32 = hasher.finalize();
}

fn hashmark(c: &mut Criterion) {
    // Sizes of files from 1KiB to 10GiB
    // e.g. [1024, 1024 * 1024, 10 * 1024 * 1024, 100 * 1024 * 1024, 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024];
    let sizes = [1024, 10 * 1024 * 1024, 100 * 1024 * 1024];

    // Number of files to hash in parallel
    // e.g. [1, 2, 4, 8, 16, 32, 64, 128];
    let parallel_iterationss = [1, 16, 64];

    // Hashing algorithms to benchmark
    #[allow(clippy::type_complexity)]
    let hash_algs: [(String, fn(&[u8])); 10] = [
        ("BLAKE3".to_string(), hash_blake3),
        ("BLAKE2b".to_string(), hash_blake2b),
        ("SHA-256".to_string(), hash_sha256),
        ("SHA-512".to_string(), hash_sha512),
        ("SHA3-256".to_string(), hash_sha3_256),
        ("SHA3-512".to_string(), hash_sha3_512),
        ("MD5".to_string(), hash_md5),
        ("Tiger2".to_string(), hash_tiger2),
        ("Whirlpool".to_string(), hash_whirlpool),
        ("CRC32".to_string(), hash_crc32),
    ];

    add_benchmarks(c, &sizes, &parallel_iterationss, &hash_algs);
}

/// Adds benchmarks for hashing algorithms
/// # Arguments
/// * `c` - Criterion instance
/// * `sizes` - Sizes of files to hash
/// * `parallel_iterationss` - Number of files to hash in parallel
/// * `hash_algs` - Hashing algorithms to benchmark
#[allow(clippy::type_complexity)]
fn add_benchmarks(
    c: &mut Criterion,
    sizes: &[usize],
    parallel_iterationss: &[u16],
    hash_algs: &[(String, fn(&[u8]))],
) {
    let mut parallel_group_template = |parallel_iterations: u16| {
        let mut parallel_group =
            c.benchmark_group(format!("{}-threaded Hashing", parallel_iterations));
        parallel_group.throughput(criterion::Throughput::Elements(sizes.len() as u64));
        for size in sizes.iter() {
            let size_str = size.format_size(BINARY);
            let data = generate_data(*size);
            parallel_group.throughput(criterion::Throughput::Bytes(*size as u64));

            hash_algs.iter().for_each(|(hash_name, hash_alg)| {
                parallel_group.bench_with_input(
                    BenchmarkId::new(hash_name, &size_str),
                    &data,
                    |b, data| {
                        b.iter(|| {
                            (0..parallel_iterations)
                                .into_par_iter()
                                .for_each(|_| hash_alg(black_box(data)))
                        })
                    },
                );
            });
        }
        parallel_group.finish();
    };

    parallel_iterationss
        .iter()
        .for_each(|&parallel_iterations| parallel_group_template(parallel_iterations));
}

// criterion_group!(benches, bench_hashes);
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).sample_size(10);
    targets = hashmark
}
criterion_main!(benches);
