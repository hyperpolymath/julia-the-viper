# SPDX-License-Identifier: PMPL-1.0-or-later
# Justfile - Julia the Viper build system

# Default: show all available commands
default:
    @just --list

# Build all packages in workspace
build:
    @echo "🔨 Building workspace..."
    cargo build --workspace

# Build with release optimizations
build-release:
    @echo "🚀 Building release..."
    cargo build --workspace --release

# Build WASM target for web playground
build-wasm:
    @echo "🌐 Building WASM..."
    cd crates/jtv-core && wasm-pack build --target web --out-dir ../../pkg
    @echo "✅ WASM build complete: pkg/"

# Run all tests
test:
    @echo "🧪 Running tests..."
    cargo test --workspace

# Run tests with output
test-verbose:
    @echo "🧪 Running tests (verbose)..."
    cargo test --workspace -- --nocapture

# Run a specific test
test-one TEST:
    @echo "🧪 Running test: {{TEST}}"
    cargo test --workspace {{TEST}} -- --nocapture

# Run all lints (clippy)
lint:
    @echo "🔍 Running clippy..."
    cargo clippy --workspace -- -D warnings

# Format all code
fmt:
    @echo "✨ Formatting code..."
    cargo fmt --all

# Check formatting without changes
fmt-check:
    @echo "👀 Checking formatting..."
    cargo fmt --all -- --check

# Run all checks (lint + test + fmt-check)
check: lint test fmt-check
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    @echo "🧹 Cleaning..."
    cargo clean
    rm -rf pkg/
    rm -rf target/

# Install CLI binary
install:
    @echo "📦 Installing jtv CLI..."
    cargo install --path crates/jtv-cli

# Run the REPL
repl:
    @echo "🐍 Starting JtV REPL..."
    cargo run --bin jtv repl

# Run a JtV file
run FILE:
    @echo "▶️  Running {{FILE}}..."
    cargo run --bin jtv run {{FILE}}

# Run a file with trace enabled
run-trace FILE:
    @echo "🔬 Running {{FILE}} with trace..."
    cargo run --bin jtv run --trace {{FILE}}

# Parse a file and show AST
parse FILE:
    @echo "🌳 Parsing {{FILE}}..."
    cargo run --bin jtv parse {{FILE}}

# Check a file for errors
check-file FILE:
    @echo "🔍 Checking {{FILE}}..."
    cargo run --bin jtv check {{FILE}}

# Run all example programs
run-examples:
    @echo "🎯 Running all examples..."
    @for file in examples/basic/*.jtv; do \
        echo "\n▶️  $$file"; \
        cargo run --quiet --bin jtv run "$$file" || exit 1; \
    done
    @for file in examples/advanced/*.jtv; do \
        echo "\n▶️  $$file"; \
        cargo run --quiet --bin jtv run "$$file" || exit 1; \
    done
    @echo "\n✅ All examples ran successfully!"

# Run benchmarks
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench --workspace

# Check RSR compliance
rsr:
    @echo "📋 Checking RSR compliance..."
    cargo run --bin jtv rsr-check

# Generate documentation
doc:
    @echo "📚 Generating documentation..."
    cargo doc --workspace --no-deps --open

# Update dependencies
update:
    @echo "⬆️  Updating dependencies..."
    cargo update

# Audit dependencies for security issues
audit:
    @echo "🔒 Auditing dependencies..."
    cargo audit

# Prepare a release (requires VERSION argument)
release VERSION:
    @echo "🎉 Preparing release {{VERSION}}..."
    @echo "1. Updating Cargo.toml versions..."
    @sed -i 's/^version = .*/version = "{{VERSION}}"/' crates/jtv-core/Cargo.toml
    @sed -i 's/^version = .*/version = "{{VERSION}}"/' crates/jtv-cli/Cargo.toml
    @sed -i 's/^version = .*/version = "{{VERSION}}"/' Cargo.toml
    @echo "2. Running checks..."
    just check
    @echo "3. Building release..."
    just build-release
    @echo "4. Building WASM..."
    just build-wasm
    @echo "5. Running examples..."
    just run-examples
    @echo "✅ Release {{VERSION}} ready! Now:"
    @echo "   git add -A"
    @echo "   git commit -m 'chore: release v{{VERSION}}'"
    @echo "   git tag v{{VERSION}}"
    @echo "   git push && git push --tags"

# Watch for changes and run tests
watch:
    @echo "👀 Watching for changes..."
    cargo watch -x "test --workspace"

# Watch and run a specific example
watch-run FILE:
    @echo "👀 Watching {{FILE}}..."
    cargo watch -x "run --bin jtv run {{FILE}}"

# Validate project structure (Nix build)
validate:
    @echo "🔧 Validating with Nix..."
    nix flake check

# Enter Nix development shell
dev:
    @echo "🐚 Entering development shell..."
    nix develop

# CI workflow simulation
ci: fmt-check lint test build-wasm run-examples rsr
    @echo "✅ CI checks complete!"

# Show project statistics
stats:
    @echo "📊 Project Statistics:"
    @echo ""
    @echo "Lines of Rust code:"
    @find crates -name "*.rs" -exec wc -l {} + | tail -1
    @echo ""
    @echo "Example programs:"
    @find examples -name "*.jtv" | wc -l
    @echo ""
    @echo "Test count:"
    @grep -r "#\[test\]" crates/ | wc -l
    @echo ""
    @echo "Crates:"
    @ls -1 crates/ | wc -l

# Quick iteration: fmt + check
quick: fmt check
    @echo "⚡ Quick checks done!"
