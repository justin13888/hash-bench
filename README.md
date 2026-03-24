# hash-bench

This benchmark compares the runtime performance of various hashing algorithms in Rust. This was originally created for analyzing different hashing algorithms to choose the best one for the [Beam](https://github.com/justin13888/beam) project.

## Testing Methodology

The benchmark tests the runtime performance of each hashing algorithm with the following parameters:

- **Input Size**: The size of the input data to hash.
- **Iterations**: The number of times to hash the input data.
- **Number of Threads**: The number of threads to use for hashing.
- **Number of Simultaneous Hashes**: The number of simultaneous hashes to perform.

The benchmark measures the time taken to hash the input data preloaded into heap memory. The benchmark is run multiple times to get a statistical average of the runtime performance. Algorithms with multithreading (e.g. BLAKE3) and/or when ran in parallel are ran in pools managed by [rayon](https://github.com/rayon-rs/rayon). For more detail, refer to the code in `benches`.

### Algorithms

The following algorithms are benchmarked:

#### Cryptographic (24)

- [BLAKE3](https://crates.io/crates/blake3) — single-stream
- [BLAKE3 (rayon)](https://crates.io/crates/blake3) — multi-threaded single stream via `update_rayon`
- [BLAKE2b512](https://crates.io/crates/blake2)
- [BLAKE2b256](https://crates.io/crates/blake2)
- [BLAKE2s256](https://crates.io/crates/blake2)
- [SHA-1](https://crates.io/crates/sha1)
- [SHA-224](https://crates.io/crates/sha2)
- [SHA-256](https://crates.io/crates/sha2)
- [SHA-384](https://crates.io/crates/sha2)
- [SHA-512](https://crates.io/crates/sha2)
- [SHA-512/256](https://crates.io/crates/sha2)
- [SHA3-224](https://crates.io/crates/sha3)
- [SHA3-256](https://crates.io/crates/sha3)
- [SHA3-384](https://crates.io/crates/sha3)
- [SHA3-512](https://crates.io/crates/sha3)
- [Keccak-256](https://crates.io/crates/sha3)
- [MD5](https://crates.io/crates/md-5)
- [RIPEMD-160](https://crates.io/crates/ripemd)
- [SM3](https://crates.io/crates/sm3)
- [Streebog-256](https://crates.io/crates/streebog)
- [Streebog-512](https://crates.io/crates/streebog)
- [Tiger](https://crates.io/crates/tiger)
- [Tiger2](https://crates.io/crates/tiger)
- [Whirlpool](https://crates.io/crates/whirlpool)

#### Non-cryptographic (17)

- [CRC32](https://crates.io/crates/crc32fast)
- [XXH32/XXH64/XXH3](https://crates.io/crates/xxhash-rust)
- [SipHash-1-3](https://crates.io/crates/siphasher)
- [SipHash-2-4](https://crates.io/crates/siphasher)
- [AHash](https://crates.io/crates/ahash)
- [wyhash](https://crates.io/crates/wyhash)
- [FxHash](https://crates.io/crates/rustc-hash)
- [GxHash](https://crates.io/crates/gxhash)†
- [MurmurHash3](https://crates.io/crates/murmur3)
- [HighwayHash](https://crates.io/crates/highway)
- [MetroHash](https://crates.io/crates/metrohash)
- [FNV-1a](https://crates.io/crates/fnv)
- [SeaHash](https://crates.io/crates/seahash)
- [Adler32](https://crates.io/crates/adler)

†GxHash requires hardware AES support (AES-NI + SSE2 on x86_64, AES + NEON on aarch64) and will not compile on platforms without it.

## Results

Results are available for two platforms:

**AMD Ryzen 9 7900X (Desktop):**
- **OS**: Ubuntu 24.04.3 LTS (x86_64) (Linux 6.8.0)
- **CPU**: AMD Ryzen 9 7900X (24) @ 5.609GHz
- **RAM**: 128 GB DDR5 5600 MHZ CL32

**Apple M3 MacBook Pro:**
- **OS**: macOS (aarch64)
- **RAM**: 36 GB

IMPORTANT: When we say "single-threaded" we mean that the algorithm is run on a single data stream. Most hashing algorithms can only utilize a single core. The exception is BLAKE3, which is benchmarked in two modes: standard single-stream (`BLAKE3`) and with internal rayon parallelism (`BLAKE3 (rayon)`) that parallelizes hashing of a single buffer across all available threads.

### Single-threaded Hashing Performance

This graph shows the performance of each hashing algorithm when run in a single thread. We see algorithms such as BLAKE3 and SHA-256 outperforming the other algorithms. CRC32 is the fastest (about 2x faster than BLAKE3), but it is non-cryptographic.

<div style="background-color: white;">
    <img src="results/amd-7900x/1-threaded%20Hashing/report/violin.svg" alt="Single-threaded Hashing Performance" style="width: 100%;"/>
</div>

### Multi-threaded Hashing Performance

This graphs shows the performance of each hashing algorithm when run in 12 threads on a system with 12 physical cores. Results scaled similarly for all single-threaded algorithms when run in 64 threads. All algorithms (single-threaded ones in particular) utilized all cores simultaneously on arbitrary bits (i.e., non-specific document types). Non-cryptographic algorithms like CRC32 and xxhash family performed noticeably better than the next-best which is BLAKE3 (cryptographic).

<div style="background-color: white;">
    <img src="results/amd-7900x/12-threaded%20Hashing/report/violin.svg" alt="Multi-threaded (12) Hashing Performance" style="width: 100%;"/>
</div>

For more detailed results, see the [results](results/) directory. The reports saved as `index.html` files are particularly useful as summaries of the different metrics.

## Running the benchmark

### Prerequisites

- Rust installed via rustup

### Starting the benchmark

To run the benchmark, clone the repository and run the following command:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

The specific parameters for the benchmark can be adjusted in the `benches/hashmark.rs` file.

## Contributions

Feel free to run the benchmark on your own machine to obtain results for your specific setup. Open to PRs for additional hashing algorithms.

## License

This repository is licensed under the MIT license.
