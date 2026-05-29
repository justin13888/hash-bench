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

# Cross-implementation consistency tests (sw vs HW variants, BLAKE3 vs rayon, library manifest)
test:
    cargo test --all-features --release -- --nocapture

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
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Auto-apply clippy fixes to staged + unstaged changes
clippy-fix:
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged

# Run clippy with no default features (verifies the minimal build)
clippy-no-default:
    cargo clippy --no-default-features --all-targets -- -D warnings

# Verify a few single-family feature subsets compile in isolation
check-features:
    cargo check --no-default-features --features sha2
    cargo check --no-default-features --features blake3
    cargo check --no-default-features --features crc
    cargo check --no-default-features --features crc-sw
    cargo check --no-default-features --features "sha1,sha-hw"
    cargo check --no-default-features --features "sha2,sha-hw"

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
check-all: fmt-check clippy
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

# --- Mobile app (Tauri 2) ---

# Add the Rust std targets for Android (4) and iOS (3, macOS-only builds)
mobile-targets:
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# One-time setup: install JS deps and generate the native android/ios projects.
# Requires the Android SDK/NDK (and, for ios, macOS + Xcode). See README.
mobile-init:
    cd mobile && bun install
    cd mobile && bun run tauri android init
    cd mobile && bun run tauri ios init

# Run on a connected Android device or a running emulator (works on Linux)
mobile-android-dev:
    cd mobile && bun run tauri android dev

# Build Android APK/AAB artifacts
mobile-android-build:
    cd mobile && bun run tauri android build

# Run on the iOS simulator or device (macOS + Xcode only)
mobile-ios-dev:
    cd mobile && bun run tauri ios dev

# Build the iOS app (macOS + Xcode only)
mobile-ios-build:
    cd mobile && bun run tauri ios build

# Lint the mobile frontend (Biome), consistent with the web app
lint-mobile:
    cd mobile && bun run check

# Format the mobile frontend with Biome
fmt-mobile:
    cd mobile && bun run format
