# hash-bench

This benchmark compares the runtime performance of various hashing algorithms in Rust. This was originally created for analyzing different hashing algorithms to choose the best one for the [Beam](https://github.com/justin13888/beam) project.

## Testing Methodology

The benchmark tests the runtime performance of each hashing algorithm with the following parameters:

- **Algorithm**: The hashing algorithm to use.
- **Input Size**: The size of the input data to hash.
- **Iterations**: The number of times to hash the input data.
- **Number of Threads**: The number of threads to use for hashing.
- **Number of Simultaneous Hashes**: The number of simultaneous hashes to perform.

The benchmark measures the time taken to hash the input data preloaded into heap memory. The benchmark is run multiple times to get a statistical average of the runtime performance. Algorithms with multithreading (e.g. BLAKE3) and/or when ran in parallel are ran in pools managed by [rayon](https://github.com/rayon-rs/rayon).

### Algorithms

The following algorithms are benchmarked:

- [BLAKE3](https://crates.io/crates/blake3)
- [BLAKE2b512](https://crates.io/crates/blake2)
- [SHA-256](https://crates.io/crates/sha2)
- [SHA-512](https://crates.io/crates/sha2)
- [SHA3-256](https://crates.io/crates/sha3)
- [SHA3-512](https://crates.io/crates/sha3)
- [MD5](https://crates.io/crates/md-5)
- [Tiger2](https://crates.io/crates/tiger)
- [whirlpool](https://crates.io/crates/whirlpool)

*We also included CRC32, which is not a cryptographic, but relevant for file integrity checks.*

## Results

The following results were obtained on a desktop with the following specifications:

- **OS**: Fedora 39 Workstation (x86_64) (Linux 6.7.9)
- **CPU**: AMD Ryzen 7 7800X3D
- **RAM**: 32 GB DDR5 6400 MHZ CL32

IMPORTANT: When we say "single-threaded" we man that the algorithm is ran on a single data stream. Most hashing algorithms could only utilize a single core with the exception of BLAKE3, which is designed to run in parallel.

### Single-threaded Hashing Performance

This graph shows the performance of each hashing algorithm when run in a single thread. We see algorithms such as BLAKE3 and SHA-256 outperforming the other algorithms. CRC32 is the fastest (about 2x faster than BLAKE3), but it is not cryptographic. It's important to notes that BLAKE3 utilizes all cores

<div style="background-color: white;">
    <img src="results/desktop/1-threaded%20Hashing/report/lines.svg" alt="Multi-threaded (64) Hashing Performance" style="width: 100%;"/>
</div>

### Multi-threaded Hashing Performance

This graphs shows the performance of each hashing algorithm when run in 64 threads. Results scaled similarly for all single-threaded algorithms when run in 64 threads. All algorithms (single-threaded ones in particular) utilized all cores simultaneously. However, if you exclude CRC32 (non-cryptographic unlike all others in this benchmark), BLAKE3 is still the fastest algorithm.

<div style="background-color: white;">
    <img src="results/desktop/64-threaded%20Hashing/report/lines.svg" alt="Multi-threaded (64) Hashing Performance" style="width: 100%;"/>
</div>

For more detailed results, see the [results](results/) directory.

## Running the benchmark

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- `gnuplot` (optional, for plotting the results)

### Starting the benchmark

To run the benchmark, clone the repository and run the following command:

```bash
cargo bench
```

The specific parameters for the benchmark can be adjusted in the `benches/hashmark.rs` file.

## Contributions

Feel free to run the benchmark on your own machine to obtain results for your specific setup. Open to PRs for additional hashing algorithms.

## License

This repository is licensed under the MIT license.
