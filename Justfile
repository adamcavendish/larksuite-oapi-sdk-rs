# List available recipes
default:
    @just --list

# ---------- Build ----------

# Build with default features
build:
    cargo build

# Build with all feature combinations
build-all:
    cargo build
    cargo build --features ws
    cargo build --features axum
    cargo build --all-features

# Check MSRV (1.95)
msrv:
    cargo +1.95.0 check --workspace
    cargo +1.95.0 check --workspace --all-features

# ---------- Lint ----------

# Run clippy across all feature combinations
clippy:
    cargo clippy --workspace --all-targets -- -D warnings
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Apply formatting
fmt:
    cargo fmt --all

# ---------- Test ----------

# Run tests with default features
test:
    cargo nextest run --workspace
    cargo test --doc --workspace

# Run tests with all feature combinations
test-all:
    cargo nextest run --workspace
    cargo nextest run --workspace --all-features
    cargo test --doc --workspace

# Verify the checked-in GoV397 metadata against the Go SDK tags
go-v397-check go_sdk:
    go run ./tools/generate_go_v397_metadata.go --go-sdk "{{go_sdk}}" --check

# Verify the checked-in Go service contract catalog against the Go SDK tag
go-service-catalog-check go_sdk:
    go run ./tools/generate_go_service_catalog.go --go-sdk "{{go_sdk}}" --check

# Verify the checked-in Go-to-Rust service parity report
go-rust-service-parity-check:
    go run ./tools/generate_go_rust_service_parity.go --check

# Verify Go tooling tests and all checked-in Go contract artifacts
go-contract-provenance-check go_sdk:
    go test tools/generate_go_service_catalog.go tools/generate_go_service_catalog_test.go
    go test tools/generate_go_rust_service_parity.go tools/generate_go_rust_service_parity_test.go
    just go-v397-check "{{go_sdk}}"
    just go-service-catalog-check "{{go_sdk}}"
    just go-rust-service-parity-check

# ---------- Docs ----------

# Build rustdoc (CI mode)
doc-check:
    RUSTDOCFLAGS="-Dwarnings" cargo doc --workspace --no-deps --all-features

# Build and open rustdoc
doc:
    RUSTDOCFLAGS="-Dwarnings" cargo doc --workspace --no-deps --all-features --open

# ---------- Coverage ----------

# Generate LCOV output for CI
coverage-lcov:
    cargo llvm-cov clean --workspace
    mkdir -p coverage
    cargo llvm-cov nextest --workspace --all-features --lcov --output-path coverage/lcov.info

# ---------- Publish ----------

# Dry-run publish to verify packaging
publish-dry-run:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# ---------- CI (run everything) ----------

# Run the full CI pipeline locally
ci: fmt-check clippy doc-check msrv test-all coverage-lcov
