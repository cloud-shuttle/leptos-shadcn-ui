# Run all tests (native target – fast, default)
test:
    cargo nextest run --workspace --target aarch64-apple-darwin

# Run only native tests (explicitly)
test-native: test

# Run WASM tests only for client‑side component libraries
# (no --workspace, avoiding native‑only crates that pull in mio)
test-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    pkgs=""
    for dir in packages/leptos/*/; do
        if [ -f "${dir}Cargo.toml" ]; then
            pkgid=$(cargo pkgid --manifest-path "${dir}Cargo.toml" 2>/dev/null || true)
            if [ -n "$pkgid" ]; then
                pkgs="$pkgs --package $pkgid"
            fi
        fi
    done
    # Add the WASM entry point
    pkgid=$(cargo pkgid --manifest-path packages/leptos-shadcn-ui-wasm/Cargo.toml)
    pkgs="$pkgs --package $pkgid"
    cargo nextest run --target wasm32-unknown-unknown $pkgs
# Run all tests (native + WASM)
test-all: test-native test-wasm

# Run tests with coverage (native only)
coverage:
    cargo llvm-cov nextest --workspace --target aarch64-apple-darwin --html

# Quick check native compilation
check-native:
    cargo check --workspace --target aarch64-apple-darwin

# Quick check WASM compilation (for relevant crates)
check-wasm:
    cargo check --workspace --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm

# Format code
fmt:
    cargo fmt --all

# Run clippy lints
clippy:
    cargo clippy --workspace --target aarch64-apple-darwin -- -D warnings

# File watching (your original recipe)
wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"
