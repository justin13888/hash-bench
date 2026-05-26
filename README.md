# hash-bench

This benchmark compares the runtime performance of various hashing algorithms in Rust. This was originally created for analyzing different hashing algorithms to choose the best one for the [Beam](https://github.com/justin13888/beam) project.

> Notes:
> - All implementations are use the best **Rust** implementations which usually are well-optimized for each platform and use platform-specific intrinsics when available. but equivalently, it is limited by the Rust implementation used.
> - If you are looking for the fastest cryptographic hash algorithm given limited power budget and for general use, you are probably looking for hardware-accelerated algorithms which basically leaves the SHA-2 family (and SHA-3 if your target devices have instructions for Keccak like via ARM v8.2).

## Testing Methodology

The benchmark tests the runtime performance of each hashing algorithm with the following parameters:

- **Input Size**: The size of the input data to hash.
- **Iterations**: The number of times to hash the input data.
- **Number of Threads**: The number of threads to use for hashing.
- **Number of Simultaneous Hashes**: The number of simultaneous hashes to perform.

The benchmark measures the time taken to hash the input data preloaded into heap memory. Each measured case is warmed up and then sampled multiple times to get a statistical average (mean, median, and a 95% confidence interval). The thread-count axis records how many threads were involved: for an ordinary algorithm that is the number of independent hashes run concurrently, while an internally-parallel algorithm (BLAKE3 in `rayon` mode) hashes a single stream across that many threads — sized via its own pool so the two are directly comparable. Parallel work is scheduled in pools managed by [rayon](https://github.com/rayon-rs/rayon). For more detail, refer to the code in [`src/`](src/).

### Algorithms

55 hashing algorithms are benchmarked across two categories.

#### Cryptographic (36)

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
| Kupyna-224 | [`kupyna`](https://crates.io/crates/kupyna) | 224 | Ukrainian national standard (DSTU 7564:2014) |
| Kupyna-256 | [`kupyna`](https://crates.io/crates/kupyna) | 256 | Ukrainian national standard (DSTU 7564:2014) |
| Kupyna-384 | [`kupyna`](https://crates.io/crates/kupyna) | 384 | Ukrainian national standard (DSTU 7564:2014) |
| Kupyna-512 | [`kupyna`](https://crates.io/crates/kupyna) | 512 | Ukrainian national standard (DSTU 7564:2014) |
| Whirlpool | [`whirlpool`](https://crates.io/crates/whirlpool) | 512 | ISO/IEC 10118-3; used by VeraCrypt |
| Ascon-Hash256 | [`ascon-hash`](https://crates.io/crates/ascon-hash) | 256 | NIST SP 800-232 lightweight standard |

#### Non-cryptographic (19)

| Algorithm | Crate | Output (bits) | Notes |
|---|---|---|---|
| CRC32 | [`crc32fast`](https://crates.io/crates/crc32fast) | 32 | Hardware-accelerated where available |
| CRC32C | [`crc32c`](https://crates.io/crates/crc32c) | 32 | Castagnoli polynomial; used by iSCSI, Btrfs, LevelDB, gRPC |
| CRC64 | [`crc64fast`](https://crates.io/crates/crc64fast) | 64 | CRC-64/XZ (ECMA-182 polynomial, reflected); used by Redis, xz |
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

Raw benchmark data is stored in the [`results/`](results/) directory, one
`results.json` per machine ID. Each report conforms to a versioned JSON Schema
([`schema/results.v2.schema.json`](schema/results.v2.schema.json)), so the format
is consistent across platforms and machine-parseable without any hardcoding. The
web dashboard at the link above is automatically deployed when results are updated.

## Project layout

`hash-bench` is a Rust **library** plus a standalone benchmark **binary**:

- Each algorithm family lives in its own module under
  [`src/algorithms/`](src/algorithms/), carrying its own metadata, behind its own
  Cargo feature. [`src/registry.rs`](src/registry.rs) defines the metadata types
  and `src/lib.rs` aggregates whichever families are enabled.
- Every hash crate is an optional dependency. A slim build selects only the
  families it needs, e.g. `cargo build --no-default-features --features sha2,blake3`
  — useful for bundling into desktop or thin mobile (Android/iOS) wrappers.
- The benchmark is a normal binary (not a `cargo bench` harness), so it can be
  cross-compiled and run on targets without a Rust toolchain installed.
- Algorithm metadata is the single source of truth: `hash-bench metadata` emits
  [`web/src/data/algorithms.json`](web/src/data/algorithms.json), which the web
  dashboard consumes for categories and labels.

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
just verify                        # Verify results files are complete
just gen-metadata                  # Regenerate web/src/data/algorithms.json
just dev                           # Start web dashboard dev server
just build-web                     # Build the web dashboard
just check                         # Check the project compiles
just lint                          # Run clippy lints
just fmt                           # Format code
just clean                         # Clean build artifacts
```

The benchmark binary can also be run directly for finer control over its
parameters:

```bash
cargo run --release -- run --machine-id my-machine \
  --sizes 64,1024,1048576 --concurrency 1,8 \
  --sample-count 30 --warmup 3000 --cpu-model "AMD Ryzen 9 7900X"
```

Run `cargo run --release -- run --help` for the full list of flags.

### Contributing Results

1. Run benchmarks on your machine: `just bench my-machine-id` (it offers to commit
   the resulting `results/my-machine-id/results.json` for you)
2. Verify the report is complete: `just verify` — it checks every
   `results/<id>/results.json` contains every expected algorithm × size × thread
   combination, and is run in CI on every pull request
3. Push — the web dashboard redeploys automatically via CI

## Development

### Git Hooks (lefthook)

This project uses [lefthook](https://github.com/evilmartians/lefthook) for pre-commit hooks. After cloning, install the hooks once:

```bash
lefthook install
```

The pre-commit hook runs in parallel:
- **Rust**: `cargo fmt --check` and `cargo clippy`
- **Web**: `biome check` on staged TypeScript/TSX files

### Lint & Format

```bash
just fmt-check      # Check Rust formatting
just lint           # Run clippy
just lint-web       # Run Biome lint + format check on web
just fmt            # Format Rust code
just fmt-web        # Format web source with Biome
just check-all      # Run all checks (Rust + web)
```

### CI

All pushes to `master` and pull requests run the [CI workflow](.github/workflows/ci.yml), which checks:
- Rust: `cargo fmt --check`, `cargo clippy --all-features`, `cargo check`, a minimal
  no-default-features build, and that `algorithms.json` is in sync with the registry
- Web: `biome check`, schema validation of results, TypeScript type checking

## Limitations

- **Single-machine results**: Benchmarks reflect the specific hardware they were run on. Results are not directly comparable across machines due to differences in CPU microarchitecture, cache sizes, and available instruction sets (e.g. AES-NI, AVX2).
- **Throughput only**: The benchmark measures raw hashing throughput on pre-loaded heap memory. It does not capture latency for small/single-use hashes, memory allocation overhead, or streaming use cases.
- **Pure Rust implementations**: Results depend on the quality of each crate's implementation. Some algorithms may have faster implementations in C/C++ or via hardware intrinsics not yet exposed in their Rust crates.
- **No collision/security testing**: This benchmark is purely for performance. It does not evaluate collision resistance, cryptographic strength, or suitability for any particular security use case.

## Contributions

Open to PRs for additional hashing algorithms or benchmark results from new hardware.

## License

This repository is licensed under the MIT license.
