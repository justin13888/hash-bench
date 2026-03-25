# hash-bench justfile

# List available recipes
default:
    @just --list

# Run all benchmarks with native CPU optimizations
bench:
    RUSTFLAGS="-C target-cpu=native" cargo bench

# Run benchmarks matching a filter (e.g., `just bench-filter "BLAKE3"`)
bench-filter filter:
    RUSTFLAGS="-C target-cpu=native" cargo bench -- "{{filter}}"

# Build the project
build:
    cargo build

# Build with native CPU optimizations (release profile)
build-release:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

# Check the project compiles without building
check:
    cargo check

# Run clippy lints
lint:
    cargo clippy --benches -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean

# Open the latest benchmark report in the browser
open-report:
    open target/criterion/report/index.html
