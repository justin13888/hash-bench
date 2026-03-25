# hash-bench justfile

# List available recipes
default:
    @just --list

# Run all benchmarks and save results for the given machine
bench machine-id:
    #!/usr/bin/env bash
    set -euo pipefail
    RESULT_DIR="results/{{machine-id}}"
    if [ -d "$RESULT_DIR" ] && [ "$(ls -A "$RESULT_DIR" 2>/dev/null)" ]; then
      read -rp "Remove existing results for '{{machine-id}}'? [Y/n] " answer
      answer="${answer:-Y}"
      if [[ "$answer" =~ ^[Yy]$ ]]; then
        rm -rf "$RESULT_DIR"
        echo "Removed $RESULT_DIR"
      fi
    fi
    RUSTFLAGS="-C target-cpu=native" cargo bench
    echo "Copying results to $RESULT_DIR/"
    mkdir -p "$RESULT_DIR"
    cd target/criterion
    find . \( -path '*/new/estimates.json' -o -path '*/new/benchmark.json' \
      -o -path '*/new/sample.json' -o -path '*/new/tukey.json' \) | while read -r f; do
      mkdir -p "../../$RESULT_DIR/$(dirname "$f")"
      cp "$f" "../../$RESULT_DIR/$f"
    done
    cd ../..
    echo "Results saved to $RESULT_DIR/."
    read -rp "Commit results? [y/N] " commit
    commit="${commit:-N}"
    if [[ "$commit" =~ ^[Yy]$ ]]; then
      git add "$RESULT_DIR"
      git commit -m "Add benchmark results for {{machine-id}}"
      echo "Committed. Push with: git push"
    fi

# Run benchmarks matching a filter for the given machine
bench-filter machine-id filter:
    #!/usr/bin/env bash
    set -euo pipefail
    RESULT_DIR="results/{{machine-id}}"
    if [ -d "$RESULT_DIR" ] && [ "$(ls -A "$RESULT_DIR" 2>/dev/null)" ]; then
      read -rp "Remove existing results for '{{machine-id}}'? [Y/n] " answer
      answer="${answer:-Y}"
      if [[ "$answer" =~ ^[Yy]$ ]]; then
        rm -rf "$RESULT_DIR"
        echo "Removed $RESULT_DIR"
      fi
    fi
    RUSTFLAGS="-C target-cpu=native" cargo bench -- "{{filter}}"
    echo "Copying results to $RESULT_DIR/"
    mkdir -p "$RESULT_DIR"
    cd target/criterion
    find . \( -path '*/new/estimates.json' -o -path '*/new/benchmark.json' \
      -o -path '*/new/sample.json' -o -path '*/new/tukey.json' \) | while read -r f; do
      mkdir -p "../../$RESULT_DIR/$(dirname "$f")"
      cp "$f" "../../$RESULT_DIR/$f"
    done
    cd ../..
    echo "Results saved to $RESULT_DIR/."
    read -rp "Commit results? [y/N] " commit
    commit="${commit:-N}"
    if [[ "$commit" =~ ^[Yy]$ ]]; then
      git add "$RESULT_DIR"
      git commit -m "Add benchmark results for {{machine-id}}"
      echo "Committed. Push with: git push"
    fi

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

# Open the latest Criterion report in the browser
open-report:
    open target/criterion/report/index.html

# Start the web app dev server
dev:
    cd web && bun run dev

# Build the web app (processes results + Vite build)
build-web:
    cd web && bun run prebuild && bun run build
