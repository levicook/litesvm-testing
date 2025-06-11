# GitHub Workflows Setup Guide

This repository uses **ultra-simple workflows** powered by **Docker + Makefile** architecture.

## 🏗️ **Architecture: Docker + Makefile = Simple + Powerful**

**Core insight**: Keep GitHub workflows **trivially simple** by moving all complexity into Makefile Docker targets.

```yaml
# GitHub workflows are dead simple:
steps:
  - uses: actions/checkout@v4
  - run: make ci-docker-full
```

```makefile
# Makefile handles Docker complexity:
ci-docker-full:
	docker run --rm -v $(PWD):/workspaces/project -w /workspaces/project \
		solanafoundation/anchor:v0.31.1 make ci-full
```

**Benefits:**

- ✅ **Dead-simple workflows** - Minimal YAML, maximum power
- ✅ **Local = CI** - Same Docker environment everywhere
- ✅ **Battle-tested tools** - Official Solana Foundation container
- ✅ **Zero drift** - One source of truth in Makefile
- ✅ **Easy debugging** - `make ci-docker-full` reproduces CI exactly

## 🔧 **The Two Workflows**

### 1. **CI Workflow** (`.github/workflows/ci.yml`)

**Triggers:** Push to `main`, all pull requests  
**Jobs:**

- **Quick Checks (Native)** → `make ci-quick` - Fast feedback on publishable crate
- **Docker Validation (Complete)** → `make ci-docker-full` - Full workspace in production environment

### 2. **Release Workflow** (`.github/workflows/release.yml`)

**Triggers:** Version tags (`v0.1.0`, `v1.2.3`, etc.)  
**Jobs:**

1. **Release Validation** → `make release-validation` - Complete validation in Docker
2. **Publish** → `make publish` - Push to crates.io
3. **GitHub Release** - Extract changelog and create release

## 🎯 **Docker Container: solanafoundation/anchor:v0.31.1**

This is the **official Anchor Docker image** used by `anchor build --verifiable`:

- ✅ **Solana CLI** (2.1.0)
- ✅ **Anchor** (0.31.1)
- ✅ **Rust/Cargo** (latest)
- ✅ **cargo-build-sbf** (Solana BPF builder)
- ✅ **Platform tools** (v1.43)

**No manual installation required** - everything just works!

## 🚀 **Makefile Targets**

```bash
# Development (native)
make check           # Fast workspace check
make test            # All tests
make fmt            # Format code
make ci-local       # Local CI with full tools

# Docker CI (matches production)
make ci-docker-quick    # Fast CI in Docker
make ci-docker-full     # Complete CI in Docker
make ci                 # Main CI target (Docker-based)

# Release
make release-validation # Complete release checks
make publish           # Publish to crates.io
```

## ⚙️ **Setup Requirements**

### **Required Tools**

**[Odometer (odo)](https://crates.io/crates/odometer)** - Workspace version management:

```bash
# Install odo for automated version management
cargo install odometer

# Verify installation
odo --version
```

**Key odo commands:**
```bash
odo show                        # Display current versions
odo roll minor --workspace     # Bump minor version for all workspace crates
odo roll major --workspace     # Bump major version for all workspace crates  
odo roll patch --workspace     # Bump patch version for all workspace crates
```

### **Required GitHub Secrets**

**Settings → Secrets and variables → Actions:**

1. **`CARGO_REGISTRY_TOKEN`** (Required for releases)

   ```bash
   # Get from: https://crates.io/me
   # Permissions: "Publish new crates" + "Publish updates"
   ```

2. **`GITHUB_TOKEN`** - Automatically provided, no setup needed

### **Branch Protection**

**Settings → Branches → Add rule** for `main`:

- ✅ Require status checks: `Quick Checks (Native)`, `Docker Validation (Complete)`
- ✅ Require up-to-date branches before merging
- ✅ Include administrators

## 🚀 **Publishing Process**

### **Automatic (Recommended)**

```bash
# 1. Update versions across workspace
odo roll minor --workspace     # or: odo roll major/patch --workspace
odo show                       # Verify all versions updated

# 2. Update changelog  
vim CHANGELOG.md

# 3. Create release branch & PR
git checkout -b release/0.2.0
git add -A && git commit -m "chore: release 0.2.0"
git push origin release/0.2.0
gh pr create --title "Release 0.2.0" --body "Version bump and changelog for 0.2.0 release"

# 4. After PR merged, tag the release
git checkout main && git pull
git tag v0.2.0 && git push origin v0.2.0

# 5. Watch automation ✨
# → Docker validation  
# → crates.io publishing
# → GitHub release creation
```

### **Why the PR Process?**

Branch protection on `main` requires:
- ✅ Status checks: `Quick Checks (Native)`, `Docker Validation (Complete)`
- ✅ Up-to-date branches before merging
- ✅ Applies to administrators too

This ensures **every release is validated** through CI before reaching `main`.

### **Local Testing (Before Release)**

```bash
# Test exact release process locally
TAG_VERSION="0.2.0" make release-validation

# Or full Docker CI
make ci-docker-full

# Verify versions before release
odo show
```

## 🐛 **Debugging CI Issues**

**The magic of Docker-based CI:**

```bash
# Reproduce CI failure exactly:
make ci-docker-full

# Or just the quick checks:
make ci-docker-quick

# Debug step by step:
docker run --rm -v $(pwd):/workspaces/project -w /workspaces/project \
  solanafoundation/anchor:v0.31.1 bash
# Then run individual commands inside container
```

**If `make ci-docker-full` passes locally, CI will pass!**

## 📊 **Local Development Workflow**

```bash
# Fast development loop (native tools)
make check          # Quick validation
make test           # Run tests
make fmt            # Format code

# Before pushing (Docker validation)
make ci-docker-full # Exact CI environment

# Quick Docker check
make ci-docker-quick # Fast Docker validation
```

## 🎯 **Key Design Principles**

1. **Workflows are thin wrappers** - Real logic in Makefile
2. **Docker for consistency** - Same environment everywhere
3. **Official containers** - Battle-tested, maintained by Solana Foundation
4. **Parallel jobs** - Fast feedback + thorough validation
5. **Version validation** - Tag must match `Cargo.toml`

## 🔍 **Quality Standards**

Enforced automatically:

- ✅ **Zero clippy warnings** (`-D warnings`)
- ✅ **Proper formatting** (`cargo fmt --check`)
- ✅ **All tests pass** (including Solana programs)
- ✅ **Documentation builds** cleanly
- ✅ **Publish dry-run** succeeds
- ✅ **Version consistency** (tag ↔ Cargo.toml)

## 🛠️ **Troubleshooting**

### **Common Issues**

**"Tag version doesn't match Cargo.toml"**

```bash
# Check versions match:
git describe --tags           # v0.2.0
odo show                     # All workspace versions
grep '^version = ' crates/litesvm-testing/Cargo.toml  # version = "0.2.0"
```

**"Docker container fails"**

```bash
# Test Docker setup locally:
docker pull solanafoundation/anchor:v0.31.1
make ci-docker-quick
```

**"crates.io token invalid"**

```bash
# Verify secret is set in GitHub:
# Settings → Secrets → CARGO_REGISTRY_TOKEN
```

### **Advanced Debugging**

```bash
# Interactive Docker debugging:
docker run -it --rm -v $(pwd):/workspaces/project -w /workspaces/project \
  solanafoundation/anchor:v0.31.1 bash

# Inside container:
make ci-full          # Run full CI
solana --version      # Check tool versions
cargo clippy          # Test individual commands
```

This architecture ensures **reliable, maintainable, and debuggable** CI/CD! 🎉
