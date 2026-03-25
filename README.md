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

51 hashing algorithms are benchmarked across two categories.

#### Cryptographic (32)

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
| Ascon-Hash256 | [`ascon-hash`](https://crates.io/crates/ascon-hash) | 256 | NIST SP 800-232 lightweight standard |

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

**Live interactive results:** [hash.justinchung.net](https://hash.justinchung.net)

Raw benchmark data is stored in the [`results/`](results/) directory, organized by machine ID. The web dashboard at the link above is automatically deployed when results are updated.

## Running the benchmark

### Prerequisites

- Rust installed via [rustup](https://rustup.rs/)
- [just](https://github.com/casey/just) command runner (optional, but recommended)
- [Bun](https://bun.sh/) (for the web dashboard)

### Commands

All common commands are defined in the [`justfile`](justfile). Run `just` to see available recipes:

```bash
just                               # List available recipes
just bench <machine-id>            # Run all benchmarks and save results
just bench-filter <machine-id> "BLAKE3"  # Run filtered benchmarks
just dev                           # Start web dashboard dev server
just build-web                     # Build the web dashboard
just check                         # Check the project compiles
just lint                          # Run clippy lints
just fmt                           # Format code
just clean                         # Clean build artifacts
```

The specific parameters for the benchmark can be adjusted in the `benches/hashmark.rs` file.

### Contributing Results

1. Run benchmarks on your machine: `just bench my-machine-id`
2. Commit the results: `git add results/ && git commit -m "Add results for my-machine-id"`
3. Push — the web dashboard redeploys automatically via CI

## Limitations

- **Single-machine results**: Benchmarks reflect the specific hardware they were run on. Results are not directly comparable across machines due to differences in CPU microarchitecture, cache sizes, and available instruction sets (e.g. AES-NI, AVX2).
- **Throughput only**: The benchmark measures raw hashing throughput on pre-loaded heap memory. It does not capture latency for small/single-use hashes, memory allocation overhead, or streaming use cases.
- **Pure Rust implementations**: Results depend on the quality of each crate's implementation. Some algorithms may have faster implementations in C/C++ or via hardware intrinsics not yet exposed in their Rust crates.
- **No collision/security testing**: This benchmark is purely for performance. It does not evaluate collision resistance, cryptographic strength, or suitability for any particular security use case.

## Contributions

Open to PRs for additional hashing algorithms or benchmark results from new hardware.

## License

This repository is licensed under the MIT license.
