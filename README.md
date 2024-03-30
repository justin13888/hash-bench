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

The following results were obtained on a 2019 MacBook Pro with a 2.6 GHz 6-Core Intel Core i7 processor and 16 GB of RAM.

<!-- TODO: Insert -->

## Running the benchmark

To run the benchmark, clone the repository and run the following command:

```bash
cargo bench
```

## License

This repository is licensed under the MIT license.
