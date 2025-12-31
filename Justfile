# Justfile for Julia the Viper
# SPDX-License-Identifier: GPL-3.0-or-later

# Default recipe to display help information
default:
    @just --list

# Build all crates
build:
    @echo "Building JtV crates..."
    cargo build --release
    @echo "✓ All crates built"

# Build the CLI tool
build-cli:
    @echo "Building JtV CLI..."
    cargo build --release -p jtv-cli
    @echo "✓ CLI built at target/release/jtv"

# Build for WASM
build-wasm:
    @echo "Building WASM target..."
    wasm-pack build crates/jtv-core --target web --out-dir ../../dist/wasm
    @echo "✓ WASM build complete"

# Run tests for all crates
test:
    @echo "Running Rust tests..."
    cargo test --all
    @echo "✓ All tests passed"

# Run benchmarks
bench:
    @echo "Running benchmarks..."
    cargo bench -p jtv-core
    @echo "✓ Benchmarks complete"

# Format code
fmt:
    @echo "Formatting Rust code..."
    cargo fmt --all
    @echo "✓ Code formatted"

# Lint code
lint:
    @echo "Linting Rust code..."
    cargo clippy --all -- -D warnings
    @echo "✓ Linting complete"

# Run a JtV file
run file:
    @echo "Running {{file}}..."
    cargo run -p jtv-cli -- run {{file}}

# Parse a JtV file and display AST
parse file:
    @echo "Parsing {{file}}..."
    cargo run -p jtv-cli -- parse {{file}}

# Check a JtV file for errors
check-file file:
    @echo "Checking {{file}}..."
    cargo run -p jtv-cli -- check {{file}}

# Start the REPL
repl:
    cargo run -p jtv-cli -- repl

# Build documentation
docs:
    @echo "Building documentation..."
    cargo doc --no-deps --all --open
    @echo "✓ Documentation built"

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf dist/
    @echo "✓ Clean complete"

# Install development dependencies
install:
    @echo "Installing Rust toolchain..."
    rustup update stable
    rustup target add wasm32-unknown-unknown
    @echo "Installing wasm-pack..."
    cargo install wasm-pack
    @echo "✓ Dependencies installed"

# Run all checks (format, lint, test)
check: fmt lint test
    @echo "✓ All checks passed"

# Create a new release
release version:
    @echo "Creating release {{version}}..."
    # Build release
    just build
    just build-wasm
    # Tag release
    git tag -a v{{version}} -m "Release {{version}}"
    @echo "✓ Release {{version}} created"

# Watch for changes and rebuild
watch:
    @echo "Watching for changes..."
    cargo watch -x build

# Run example
example name:
    @echo "Running example: {{name}}"
    just run examples/basic/{{name}}.jtv

# Run smart contract example
contract name:
    @echo "Running contract: {{name}}"
    just run examples/contracts/{{name}}.jtv

# Generate code coverage
coverage:
    @echo "Generating code coverage..."
    cargo tarpaulin --all --out Html --output-dir coverage
    @echo "✓ Coverage report in coverage/index.html"

# Start development server for playground
dev-playground:
    @echo "Starting playground development server..."
    cd playground && deno task dev

# Build playground for production
build-playground:
    @echo "Building playground..."
    cd playground && deno task build
    @echo "✓ Playground built in playground/dist"

# Package for distribution
package: build build-wasm
    @echo "Creating distribution package..."
    mkdir -p dist/bin
    cp target/release/jtv dist/bin/jtv
    @echo "✓ Package created"

# Run all examples
run-all-examples:
    @echo "Running all basic examples..."
    just example 01_hello_addition
    just example 02_number_systems
    just example 03_functions
    just example 04_loops
    just example 05_conditionals
    @echo "Running all advanced examples..."
    just run examples/advanced/fibonacci.jtv
    just run examples/advanced/matrix_operations.jtv
    @echo "✓ All examples completed"

# Initialize git hooks
init-hooks:
    @echo "Installing git hooks..."
    echo '#!/bin/sh\njust check' > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    @echo "✓ Git hooks installed"

# Quick development cycle: format, lint, test, run
dev file: fmt lint test
    just run {{file}}

# Check RSR (Rhodium Standard Repository) compliance
rsr-check:
    @echo "Checking RSR compliance..."
    cargo run -p jtv-cli -- rsr-check
    @echo "✓ RSR compliance check complete"

# Validate repository meets RSR Gold standard
validate: rsr-check
    @echo "Validating repository structure..."
    @test -f README.adoc || (echo "❌ README.adoc missing" && exit 1)
    @test -f LICENSE.txt || (echo "❌ LICENSE.txt missing" && exit 1)
    @test -f GOVERNANCE.adoc || (echo "❌ GOVERNANCE.adoc missing" && exit 1)
    @test -f CONTRIBUTING.adoc || (echo "❌ CONTRIBUTING.adoc missing" && exit 1)
    @test -f CODE_OF_CONDUCT.adoc || (echo "❌ CODE_OF_CONDUCT.adoc missing" && exit 1)
    @test -f SECURITY.md || (echo "❌ SECURITY.md missing" && exit 1)
    @test -f CHANGELOG.md || (echo "❌ CHANGELOG.md missing" && exit 1)
    @echo "✓ Repository structure validated"

# Build Lean proofs
proofs:
    @echo "Building Lean proofs..."
    cd jtv_proofs && lake build
    @echo "✓ Lean proofs built"

# Full pre-release validation
pre-release: clean install build test lint validate proofs
    @echo "✓ Pre-release validation complete"

# Container build with nerdctl (primary) or podman (fallback)
container-build tag="jtv-dev":
    @echo "Building container image..."
    @command -v nerdctl >/dev/null 2>&1 && nerdctl build -t {{tag}} . || podman build -t {{tag}} .
    @echo "✓ Container built: {{tag}}"

# Run container
container-run tag="jtv-dev":
    @echo "Running container..."
    @command -v nerdctl >/dev/null 2>&1 && nerdctl run -it --rm {{tag}} || podman run -it --rm {{tag}}
