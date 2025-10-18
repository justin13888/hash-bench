use std::{hint::black_box, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use humansize::{FormatSize, BINARY};
use rand::Rng;
use rayon::prelude::*;
use sha2::{Digest, Sha256};

/// Generate arbitrary data of a given size
fn generate_data(size: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    let mut rng = rand::rng();
    rng.fill(&mut data[..]);

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

/// Hash data using XXH32
fn hash_xxh32(data: &[u8]) {
    let _result: u32 = xxhash_rust::xxh32::xxh32(data, 0);
}

/// Hash data using XXH64
fn hash_xxh64(data: &[u8]) {
    let _result: u64 = xxhash_rust::xxh64::xxh64(data, 0);
}

/// Hash data using XXH3_64
fn hash_xxh3_64(data: &[u8]) {
    let _result: u64 = xxhash_rust::xxh3::xxh3_64(data);
}

/// Hash data using XXH3_128
fn hash_xxh3_128(data: &[u8]) {
    let _result: u128 = xxhash_rust::xxh3::xxh3_128(data);
}

fn hashmark(c: &mut Criterion) {
    // Detect number of CPU cores
    let physical_cpus = num_cpus::get_physical();
    println!("Detected {physical_cpus} physical CPU cores.");
    let logical_cpus = num_cpus::get();
    println!("Detected {logical_cpus} logical CPU cores.");

    // Sizes of files to hash
    let sizes = [
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
    let mut parallel_iterationss = vec![1, physical_cpus, logical_cpus];
    parallel_iterationss.dedup();
    println!(
        "Benchmarking parallel iterations: {:?}",
        parallel_iterationss
    );

    // Hashing algorithms to benchmark
    #[allow(clippy::type_complexity)]
    let hash_algs: [(String, fn(&[u8])); _] = [
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
        ("XXH32".to_string(), hash_xxh32),
        ("XXH64".to_string(), hash_xxh64),
        ("XXH3_64".to_string(), hash_xxh3_64),
        ("XXH3_128".to_string(), hash_xxh3_128),
    ];
    println!(
        "Benchmarking hashing algorithms: {:?}",
        hash_algs
            .iter()
            .map(|(name, _)| name)
            .collect::<Vec<&String>>()
    );

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
    parallel_iterationss: &[usize],
    hash_algs: &[(String, fn(&[u8]))],
) {
    let mut parallel_group_template = |parallel_iterations: usize| {
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
    config = Criterion::default().measurement_time(Duration::from_secs(30)).sample_size(20);
    targets = hashmark
}
criterion_main!(benches);
