# GitHub Workflows Setup Guide

This repository uses **Makefile-driven CI/CD** where the Makefile is the single source of truth for all build operations.

## 🏗️ **Architecture: Makefile as Single Source of Truth**

**Philosophy:** All build logic lives in the `Makefile`. GitHub workflows are just thin orchestration layers that call Makefile targets.

**Benefits:**

- ✅ **Consistency** - Same commands work locally and in CI
- ✅ **No drift** - Only one place to maintain build logic
- ✅ **Fast feedback** - Test locally with exact same commands as CI
- ✅ **Debugging** - Reproduce CI issues locally with `make ci-full`

## 🔧 **Workflows Overview**

### 1. **CI Workflow** (`.github/workflows/ci.yml`)

**Triggers:** Every push to `main` and all pull requests
**Purpose:** Continuous integration with parallel quality gates

**What it does:**

- ✅ **Publishable Crate Job** → `make ci-quick`
- ✅ **Full Workspace Job** → `make ci-full` (includes Solana programs)

**Key insight:** No custom build logic in workflow - just calls Makefile targets!

### 2. **Release Workflow** (`.github/workflows/release.yml`)

**Triggers:** Version tags (e.g., `v0.1.0`, `v1.2.3`)
**Purpose:** Automated publishing with quality gates

**What it does:**

1. **Pre-release checks** → `make ci-full` (full validation)
2. **Version validation** - Ensures tag matches `Cargo.toml`
3. **Publish to crates.io** - Uses `CARGO_REGISTRY_TOKEN` secret
4. **GitHub release** - Extracts notes from `CHANGELOG.md`

## 🎯 **Makefile Targets (The Real CI)**

```bash
# Development workflow
make check      # Fast workspace validation
make test       # All tests including Solana programs
make fmt        # Format code
make ci-local   # Full local CI with all checks

# CI targets (what workflows call)
make ci-quick   # Fast feedback - publishable crate only
make ci-full    # Complete validation - all packages
```

## ⚙️ **Setup Requirements**

### **Required GitHub Secrets**

Add these in **Settings → Secrets and variables → Actions**:

1. **`CARGO_REGISTRY_TOKEN`** (Required for publishing)

   ```bash
   # Get token from https://crates.io/me
   # Add as repository secret
   ```

2. **`GITHUB_TOKEN`** (Automatically provided)
   - Used for creating GitHub releases
   - No setup required

### **Required Branch Protection**

Configure **Settings → Branches → Add rule** for `main`:

- ✅ Require status checks: `Publishable Crate Quality`, `Full Workspace (inc. Solana Programs)`
- ✅ Require up-to-date branches
- ✅ Include administrators

## 🚀 **Publishing Workflow**

### **Automatic Publishing (Recommended)**

```bash
# 1. Update version in Cargo.toml
vim crates/litesvm-testing/Cargo.toml

# 2. Update CHANGELOG.md with new version section
vim CHANGELOG.md

# 3. Commit and push
git add -A && git commit -m "Release v0.1.0"
git push

# 4. Create and push tag
git tag v0.1.0
git push origin v0.1.0

# 5. Watch the magic happen ✨
# - CI validates everything
# - Publishes to crates.io
# - Creates GitHub release
```

### **Manual Publishing (Fallback)**

```bash
# Test locally first
make ci-full

# Publish manually
cargo publish --manifest-path crates/litesvm-testing/Cargo.toml
```

## 🐛 **Debugging CI Issues**

**The beauty of Makefile-driven CI:**

```bash
# Reproduce exact CI failure locally
make ci-full

# Or run specific CI subset
make ci-quick

# Debug individual steps
make check && make test
```

**No more "works on my machine" - if `make ci-full` passes locally, CI will pass!**

## 📊 **Local Development Workflow**

```bash
# Fast feedback loop
make check      # Quick validation
make test       # Run all tests

# Before pushing
make ci-full    # Complete CI simulation

# Code quality
make fmt        # Format code
make ci-local   # Everything including clippy
```

## 🎯 **Key Design Decisions**

1. **Makefile is canonical** - Workflows delegate to `make` targets
2. **Parallel CI jobs** - Fast feedback + comprehensive validation
3. **Solana toolchain isolation** - Different cache keys, proper setup
4. **Version validation** - Tag must match `Cargo.toml`
5. **Changelog integration** - Release notes from `CHANGELOG.md`

This architecture ensures your CI/CD is **maintainable, debuggable, and reliable**! 🎉

## 📋 Quality Standards

The workflows enforce these standards:

- **Zero clippy warnings** (using `-D warnings`)
- **Proper formatting** (using `cargo fmt`)
- **All tests pass**
- **Documentation builds** without errors
- **Version consistency** between tags and Cargo.toml
- **Dry-run publish success** before real publication

## 🔍 Monitoring

**CI Status:**

- Check the "Actions" tab for build status
- PR checks must pass before merging
- Main branch status visible in repository badges

**Release Status:**

- Failed releases are visible in Actions tab
- GitHub releases are created automatically on success
- crates.io publication status is logged in workflow

## 🛠️ Troubleshooting

**Common Issues:**

1. **"Tag version doesn't match Cargo.toml"**

   - Ensure the git tag matches the version in `crates/litesvm-testing/Cargo.toml`
   - Format: tag `v0.1.0` should match version `0.1.0`

2. **"crates.io token invalid"**

   - Check that `CRATES_IO_TOKEN` secret is set correctly
   - Ensure the token has publishing permissions

3. **"Clippy warnings blocking CI"**

   - Run `cargo clippy --all-features -- -D warnings` locally
   - Fix all warnings before pushing

4. **"Examples failing"**
   - Examples are non-blocking in CI
   - Check Solana CLI installation and toolchain setup

## 🎯 Benefits

This setup provides:

- **Quality assurance** - No broken code reaches main
- **Automated publishing** - Reduces manual errors
- **Consistent releases** - Standardized process
- **Visibility** - Clear status of builds and releases
- **Professional polish** - Shows attention to quality
