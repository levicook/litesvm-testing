# LiteSVM Testing - Build and CI Tasks
# 
# This Makefile provides fast feedback loops for development and
# standardizes the exact commands that CI should run.

.PHONY: help check test fmt clean ci-quick ci-full ci-local ci-docker-quick ci-docker-full ci release-validation publish

# Default target
help:
	@echo "LiteSVM Testing Build Tasks"
	@echo ""
	@echo "Development:"
	@echo "  check      - Fast workspace check (what CI runs)"
	@echo "  test       - Run all tests including Solana programs"
	@echo "  fmt        - Format all code"
	@echo "  clean      - Clean all build artifacts"
	@echo ""
	@echo "CI Simulation:"
	@echo "  ci-quick   - Fast CI checks (publishable crate only)"
	@echo "  ci-full    - Complete workspace validation"
	@echo "  ci-local   - Full local CI simulation (requires Solana CLI)"
	@echo ""
	@echo "Docker CI (matches GitHub Actions exactly):"
	@echo "  ci-docker-quick - Fast CI in Docker container"
	@echo "  ci-docker-full  - Complete CI in Docker container"
	@echo "  ci              - Main CI target (Docker-based)"
	@echo ""
	@echo "Release:"
	@echo "  release-validation - Complete release validation"
	@echo "  publish            - Publish to crates.io"

# Fast workspace validation - exactly what works
check:
	@echo "🔍 Checking workspace..."
	cargo check --workspace
	@echo "✅ Workspace check passed"

# Complete test suite including Solana program compilation
test:
	@echo "🧪 Running all tests (including Solana programs)..."
	cargo test --workspace --verbose
	@echo "✅ All tests passed"

# Format all code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all
	@echo "✅ Code formatted"

# Check formatting
fmt-check:
	@echo "🎨 Checking code formatting..."
	cargo fmt --all -- --check
	@echo "✅ Code formatting OK"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete"

# Quick CI checks - focuses on publishable crate ONLY
ci-quick:
	@echo "⚡ Running quick CI checks (publishable crate)..."
	$(MAKE) fmt-check
	@echo "Core library check:"
	cargo check --manifest-path crates/litesvm-testing/Cargo.toml --all-features
	@echo "Core library clippy:"
	cargo clippy --manifest-path crates/litesvm-testing/Cargo.toml --all-features -- -D warnings
	@echo "Core library tests:"
	cargo test --manifest-path crates/litesvm-testing/Cargo.toml --all-features
	@echo "Documentation check:"
	cargo doc --manifest-path crates/litesvm-testing/Cargo.toml --all-features --no-deps
	@echo "Publish dry run:"
	cargo publish --manifest-path crates/litesvm-testing/Cargo.toml --dry-run --allow-dirty
	@echo "✅ Quick CI passed"

# Full CI validation - what can reliably work in CI
ci-full:
	@echo "🚀 Running full CI validation..."
	$(MAKE) fmt-check
	$(MAKE) check
	$(MAKE) test
	@echo "Core library publish check:"
	cargo publish --manifest-path crates/litesvm-testing/Cargo.toml --dry-run --allow-dirty
	@echo "✅ Full CI passed"

# Local development CI - includes everything possible
ci-local:
	@echo "🏠 Running local CI (includes all checks)..."
	$(MAKE) fmt-check
	$(MAKE) check
	$(MAKE) test
	@echo "Core library clippy (all features):"
	cargo clippy --manifest-path crates/litesvm-testing/Cargo.toml --all-features -- -D warnings
	@echo "Core library publish check:"
	cargo publish --manifest-path crates/litesvm-testing/Cargo.toml --dry-run --allow-dirty
	@echo "✅ Local CI passed"

# Docker-based CI targets (matches CI environment exactly)
ci-docker-quick:
	@echo "🐳 Running quick CI in Docker container..."
	@docker pull solanafoundation/anchor:v0.31.1 > /dev/null 2>&1 || true
	@mkdir -p ~/.cargo
	docker run --rm \
		-v $$(pwd):/workspaces/project \
		-v ~/.cargo:/home/solana/.cargo \
		-w /workspaces/project \
		solanafoundation/anchor:v0.31.1 make ci-quick

ci-docker-full:
	@echo "🐳 Running full CI in Docker container..."
	@docker pull solanafoundation/anchor:v0.31.1 > /dev/null 2>&1 || true
	@mkdir -p ~/.cargo
	docker run --rm \
		-v $$(pwd):/workspaces/project \
		-v ~/.cargo:/home/solana/.cargo \
		-w /workspaces/project \
		solanafoundation/anchor:v0.31.1 make ci-full

# Main CI target for GitHub Actions (uses Docker for reproducible environment)
ci: ci-docker-full

# Release validation - comprehensive checks before publishing
release-validation:
	@echo "🚀 Running release validation..."
	@echo "Verifying tag matches Cargo.toml version..."
	@if [ -n "$$TAG_VERSION" ] && [ -n "$$(grep '^version = ' crates/litesvm-testing/Cargo.toml | sed 's/version = "\(.*\)"/\1/')" ]; then \
		CARGO_VERSION=$$(grep '^version = ' crates/litesvm-testing/Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
		if [ "$$TAG_VERSION" != "$$CARGO_VERSION" ]; then \
			echo "❌ Tag version $$TAG_VERSION doesn't match Cargo.toml version $$CARGO_VERSION"; \
			exit 1; \
		fi; \
		echo "✅ Tag version matches Cargo.toml version: $$TAG_VERSION"; \
	fi
	$(MAKE) ci-docker-full
	@echo "✅ Release validation passed"

# Publish to crates.io (requires CARGO_REGISTRY_TOKEN)
publish:
	@echo "📦 Publishing to crates.io..."
	cargo publish --manifest-path crates/litesvm-testing/Cargo.toml --token $$CARGO_REGISTRY_TOKEN
	@echo "✅ Published to crates.io"

# Install tools needed for development
install-tools:
	@echo "🛠️  Installing development tools..."
	cargo install cargo-edit
	@command -v act >/dev/null 2>&1 || (echo "Consider installing 'act' for local GitHub Actions: brew install act")
	@echo "✅ Tools installed" 