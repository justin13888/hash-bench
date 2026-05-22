# hash-bench justfile

# List available recipes
default:
    @just --list

# Run the full benchmark suite and save results for the given machine
bench machine-id:
    #!/usr/bin/env bash
    set -euo pipefail
    RESULT_FILE="results/{{machine-id}}/results.json"
    RUSTFLAGS="-C target-cpu=native" cargo run --release -- \
      run --machine-id "{{machine-id}}" --output "$RESULT_FILE"
    echo "Results saved to $RESULT_FILE."
    read -rp "Commit results? [y/N] " commit
    commit="${commit:-N}"
    if [[ "$commit" =~ ^[Yy]$ ]]; then
      git add "$RESULT_FILE"
      git commit -m "Add benchmark results for {{machine-id}}"
      echo "Committed. Push with: git push"
    fi

# Run benchmarks matching a name filter for the given machine
bench-filter machine-id filter:
    RUSTFLAGS="-C target-cpu=native" cargo run --release -- \
      run --machine-id "{{machine-id}}" --filter "{{filter}}" \
      --output "results/{{machine-id}}/results.json"

# Regenerate the algorithm metadata catalogue consumed by the web app
gen-metadata:
    cargo run -- metadata --output web/src/data/algorithms.json

# Verify committed results files contain every expected combination
verify dir="results":
    cargo run --quiet -- verify --dir "{{dir}}"

# Build the project
build:
    cargo build

# Build with native CPU optimizations (release profile)
build-release:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

# Check the project compiles (all targets)
check:
    cargo check --all-targets

# Run clippy lints (all targets, all features)
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with no default features (verifies the minimal build)
lint-no-default:
    cargo clippy --no-default-features --all-targets -- -D warnings

# Verify a few single-family feature subsets compile in isolation
check-features:
    cargo check --no-default-features --features sha2
    cargo check --no-default-features --features blake3
    cargo check --no-default-features --features crc

# Format code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean

# Run all checks (Rust fmt, clippy, web lint + typecheck)
check-all: fmt-check lint
    cd web && bun run check && bun run typecheck

# Run web lint and format checks (Biome)
lint-web:
    cd web && bun run check

# Format web source files with Biome
fmt-web:
    cd web && bun run format

# Start the web app dev server
dev:
    cd web && bun run dev

# Build the web app (processes results + Vite build)
build-web:
    cd web && bun run build
