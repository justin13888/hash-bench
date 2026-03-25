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

50 hashing algorithms are benchmarked across two categories.

#### Cryptographic (31)

| Algorithm | Crate | Output (bits) | Notes |
|---|---|---|---|
| BLAKE3 | [`blake3`](https://crates.io/crates/blake3) | 256 | Single-stream |
| BLAKE3 (rayon) | [`blake3`](https://crates.io/crates/blake3) | 256 | Multi-threaded single stream via `update_rayon` |
| BLAKE2b512 | [`blake2`](https://crates.io/crates/blake2) | 512 | RFC 7693; used by WireGuard, Argon2 |
| BLAKE2b256 | [`blake2`](https://crates.io/crates/blake2) | 256 | RFC 7693 |
| BLAKE2s256 | [`blake2`](https://crates.io/crates/blake2) | 256 | RFC 7693; used by WireGuard |
| SHA-1 | [`sha1`](https://crates.io/crates/sha1) | 160 | Broken — included for reference only |
| SHA-224 | [`sha2`](https://crates.io/crates/sha2) | 224 | NIST FIPS 180-4 |
| SHA-256 | [`sha2`](https://crates.io/crates/sha2) | 256 | NIST FIPS 180-4; used by Bitcoin, TLS |
| SHA-384 | [`sha2`](https://crates.io/crates/sha2) | 384 | NIST FIPS 180-4; used by TLS |
| SHA-512 | [`sha2`](https://crates.io/crates/sha2) | 512 | NIST FIPS 180-4 |
| SHA-512/224 | [`sha2`](https://crates.io/crates/sha2) | 224 | NIST FIPS 180-4 |
| SHA-512/256 | [`sha2`](https://crates.io/crates/sha2) | 256 | NIST FIPS 180-4 |
| SHA3-224 | [`sha3`](https://crates.io/crates/sha3) | 224 | NIST FIPS 202 |
| SHA3-256 | [`sha3`](https://crates.io/crates/sha3) | 256 | NIST FIPS 202 |
| SHA3-384 | [`sha3`](https://crates.io/crates/sha3) | 384 | NIST FIPS 202 |
| SHA3-512 | [`sha3`](https://crates.io/crates/sha3) | 512 | NIST FIPS 202 |
| SHAKE128 | [`sha3`](https://crates.io/crates/sha3) | Variable (XOF) | NIST FIPS 202; benchmarked at 256-bit output |
| SHAKE256 | [`sha3`](https://crates.io/crates/sha3) | Variable (XOF) | NIST FIPS 202; benchmarked at 512-bit output |
| Keccak-224 | [`sha3`](https://crates.io/crates/sha3) | 224 | Pre-NIST Keccak |
| Keccak-256 | [`sha3`](https://crates.io/crates/sha3) | 256 | Pre-NIST Keccak; used by Ethereum |
| Keccak-384 | [`sha3`](https://crates.io/crates/sha3) | 384 | Pre-NIST Keccak |
| Keccak-512 | [`sha3`](https://crates.io/crates/sha3) | 512 | Pre-NIST Keccak |
| MD5 | [`md-5`](https://crates.io/crates/md-5) | 128 | Broken — included for reference only |
| RIPEMD-128 | [`ripemd`](https://crates.io/crates/ripemd) | 128 | ISO/IEC 10118-3 |
| RIPEMD-160 | [`ripemd`](https://crates.io/crates/ripemd) | 160 | ISO/IEC 10118-3; used by Bitcoin |
| RIPEMD-256 | [`ripemd`](https://crates.io/crates/ripemd) | 256 | ISO/IEC 10118-3 |
| RIPEMD-320 | [`ripemd`](https://crates.io/crates/ripemd) | 320 | ISO/IEC 10118-3 |
| SM3 | [`sm3`](https://crates.io/crates/sm3) | 256 | Chinese national standard (GB/T 32905-2016) |
| Streebog-256 | [`streebog`](https://crates.io/crates/streebog) | 256 | Russian standard (GOST R 34.11-2012) |
| Streebog-512 | [`streebog`](https://crates.io/crates/streebog) | 512 | Russian standard (GOST R 34.11-2012) |
| Whirlpool | [`whirlpool`](https://crates.io/crates/whirlpool) | 512 | ISO/IEC 10118-3; used by VeraCrypt |

#### Non-cryptographic (19)

| Algorithm | Crate | Output (bits) | Notes |
|---|---|---|---|
| CRC32 | [`crc32fast`](https://crates.io/crates/crc32fast) | 32 | Hardware-accelerated where available |
| CRC32C | [`crc32c`](https://crates.io/crates/crc32c) | 32 | Castagnoli polynomial; used by iSCSI, Btrfs, LevelDB, gRPC |
| CRC64 | [`crc64fast`](https://crates.io/crates/crc64fast) | 64 | ECMA-182; used by Redis, xz |
| XXH32 | [`xxhash-rust`](https://crates.io/crates/xxhash-rust) | 32 | |
| XXH64 | [`xxhash-rust`](https://crates.io/crates/xxhash-rust) | 64 | |
| XXH3_64 | [`xxhash-rust`](https://crates.io/crates/xxhash-rust) | 64 | Used by Linux kernel, rsync |
| XXH3_128 | [`xxhash-rust`](https://crates.io/crates/xxhash-rust) | 128 | |
| SipHash-1-3 | [`siphasher`](https://crates.io/crates/siphasher) | 64 | Used by Python's `dict` default hasher |
| SipHash-2-4 | [`siphasher`](https://crates.io/crates/siphasher) | 64 | Rust `HashMap` default (via `DefaultHasher`) |
| AHash | [`ahash`](https://crates.io/crates/ahash) | 64 | Rust `hashbrown` default; uses AES-NI when available |
| wyhash | [`wyhash`](https://crates.io/crates/wyhash) | 64 | Used by Go runtime (`maphash`), Zig |
| FxHash | [`rustc-hash`](https://crates.io/crates/rustc-hash) | 64 | Used internally by `rustc` |
| FarmHash | [`farmhash`](https://crates.io/crates/farmhash) | 64 | Used by Google internally, TensorFlow |
| MurmurHash3 | [`murmur3`](https://crates.io/crates/murmur3) | 128 | x64 128-bit variant; used by Cassandra, Elasticsearch |
| HighwayHash-64 | [`highway`](https://crates.io/crates/highway) | 64 | Google; SipHash alternative |
| HighwayHash-128 | [`highway`](https://crates.io/crates/highway) | 128 | |
| HighwayHash-256 | [`highway`](https://crates.io/crates/highway) | 256 | |
| FNV-1a | [`fnv`](https://crates.io/crates/fnv) | 64 | Go `hash/fnv` standard library |
| Adler32 | [`adler`](https://crates.io/crates/adler) | 32 | Checksum used by zlib/gzip — included for reference |

**Inclusion criteria:** Each algorithm family is included only if it is (1) formally standardized (e.g. NIST, ISO, RFC, national standard) or (2) used in at least one significant production system or tool. When a family is included, all standard output-size variants are benchmarked for completeness.

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

- Rust installed via [rustup](https://rustup.rs/)
- [just](https://github.com/casey/just) command runner (optional, but recommended)

### Commands

All common commands are defined in the [`justfile`](justfile). Run `just` to see available recipes:

```bash
just              # List available recipes
just bench        # Run all benchmarks with native CPU optimizations
just bench-filter "BLAKE3"  # Run benchmarks matching a filter
just build        # Build the project
just build-release # Build with native CPU optimizations (release)
just check        # Check the project compiles
just lint         # Run clippy lints
just fmt          # Format code
just fmt-check    # Check formatting
just clean        # Clean build artifacts
just open-report  # Open the latest benchmark report in the browser
```

Or equivalently, without `just`:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

The specific parameters for the benchmark can be adjusted in the `benches/hashmark.rs` file.

## Contributions

Feel free to run the benchmark on your own machine to obtain results for your specific setup. Open to PRs for additional hashing algorithms.

## License

This repository is licensed under the MIT license.
