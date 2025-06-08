# litesvm-testing

A Rust library providing testing utilities for Solana programs using [LiteSVM](https://github.com/LiteSVM/litesvm).

> **⚠️ Development Status**: This library is currently in active development. The API may change before the first stable release. We plan to publish to [crates.io](https://crates.io) once the API stabilizes.

## ✨ Features

- 🔧 **Build utilities** for Anchor and Pinocchio programs
- 📋 **Log assertion helpers** with detailed error messages
- 🎯 **Dual API patterns** - choose direct functions or fluent trait methods
- 🚀 **Working examples** for multiple Solana frameworks
- 🎯 **Easy integration** with your existing test suite

## 🚀 Quick Start

### Current (Development)

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
litesvm-testing = { git = "https://github.com/levicook/litesvm-testing" }
litesvm = "0.6.1"
```

### Future (Stable Release)

Once published to crates.io, you'll be able to use:

```toml
[dev-dependencies]
litesvm-testing = "0.1.0"  # When available
litesvm = "0.6.1"
```

### Basic Usage

```rust
use litesvm::LiteSVM;
use litesvm_testing::demand_logs_contain;

#[test]
fn test_my_program() {
    let mut svm = LiteSVM::new();
    // ... load your program and create transaction ...

    let result = svm.send_transaction(tx);
    assert!(result.is_ok());

    // Assert log content with helpful error messages
    demand_logs_contain(result, "Hello from my program!");
}
```

### Fluent API (Alternative Style)

```rust
use litesvm::LiteSVM;
use litesvm_testing::DemandFluency;

#[test]
fn test_my_program_fluently() {
    let mut svm = LiteSVM::new();
    // ... load your program and create transaction ...

    // Chain assertions directly from the transaction result
    svm.send_transaction(tx)
        .demand_logs_contain("Hello from my program!");
}
```

## 📚 Framework Support

### Anchor Programs

```toml
[build-dependencies]
litesvm-testing = { git = "...", features = ["anchor"] }
```

```rust
// build.rs
use litesvm_testing::anchor::build_anchor_program;

fn main() {
    build_anchor_program("path/to/anchor/program");
}
```

### Pinocchio Programs

```toml
[build-dependencies]
litesvm-testing = { git = "...", features = ["pinocchio"] }
```

```rust
// build.rs
use litesvm_testing::pinocchio::build_pinocchio_program;

fn main() {
    build_pinocchio_program("path/to/pinocchio/program");
}
```

## 🎯 Testing Utilities

### `demand_logs_contain`

Asserts that transaction logs contain a specific string. Available in two styles:

**Direct function call:**

```rust
use litesvm_testing::demand_logs_contain;
demand_logs_contain(result, "Hello from my program!");
```

**Fluent trait method:**

```rust
use litesvm_testing::DemandFluency;
result.demand_logs_contain("Hello from my program!");
```

When assertions fail, you get detailed, helpful output:

```
❌ Log assertion failed!
Expected to find: "Hello from my program!" in one of 4 log entries:
  [0]: Program 11111111111111111111111111111111 invoke [1]
  [1]: Program log: Hello from anchor! Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
  [2]: Program 11111111111111111111111111111111 consumed 11832 of 200000 compute units
  [3]: Program 11111111111111111111111111111111 success
```

## 📖 Complete Examples

This repository includes working examples for multiple frameworks:

### Anchor

- **Program**: [`examples/anchor/simple-anchor-program/`](examples/anchor/simple-anchor-program/)
- **Tests**: [`examples/anchor/simple-anchor-tests/`](examples/anchor/simple-anchor-tests/)

### Pinocchio

- **Program**: [`examples/pinocchio/simple-pinocchio-program/`](examples/pinocchio/simple-pinocchio-program/)
- **Tests**: [`examples/pinocchio/simple-pinocchio-tests/`](examples/pinocchio/simple-pinocchio-tests/)

## 🏃‍♂️ Running Examples

```bash
# Clone the repository
git clone https://github.com/levicook/litesvm-testing
cd litesvm-testing

# Run all tests with detailed output
cargo test --workspace --no-fail-fast -- --show-output

# Run specific framework tests
cargo test -p simple-anchor-tests
cargo test -p simple-pinocchio-tests
```

## 🛠️ Prerequisites

- **Rust** (latest stable)
- **Solana CLI tools** for `cargo build-sbf` command

### Quick Installation (Recommended)

Install all Solana development dependencies with one command:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

This installs Rust, Solana CLI, Anchor CLI, Node.js, and Yarn all at once.

### Manual Installation

If the quick install doesn't work, install the Solana CLI individually:

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

**Note**: The Solana CLI is now maintained by Anza (formerly Solana Labs) and uses the Agave validator client.

### Verify Installation

Check that everything is installed correctly:

```bash
# Check Solana CLI
solana --version
# Expected: solana-cli 2.2.12 (src:0315eb6a; feat:1522022101, client:Agave)

# Check Rust
rustc --version
# Expected: rustc 1.86.0 (05f9846f8 2025-03-31)
```

## 🏗️ Project Structure

```
litesvm-testing/
├── crates/
│   └── litesvm-testing/     # Core library
├── examples/
│   ├── anchor/              # Anchor framework examples
│   │   ├── simple-anchor-program/
│   │   └── simple-anchor-tests/
│   └── pinocchio/           # Pinocchio framework examples
│       ├── simple-pinocchio-program/
│       └── simple-pinocchio-tests/
└── README.md
```

## 🗺️ Roadmap

- [x] **Core log assertion utilities**
- [x] **Anchor and Pinocchio build support**
- [x] **Working examples for both frameworks**
- [ ] **API stabilization and documentation review**
- [ ] **Additional testing utilities** (error assertions, compute unit checks, etc.)
- [ ] **Steel framework support**
- [ ] **First stable release (v0.1.0) to crates.io**
- [ ] **Integration with popular Solana testing patterns**

## 📝 License

This project is licensed under [your chosen license].

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔗 Related Projects

- [LiteSVM](https://github.com/LiteSVM/litesvm) - Fast Solana VM for testing
- [Anchor](https://github.com/coral-xyz/anchor) - Solana development framework
- [Pinocchio](https://github.com/anza-xyz/pinocchio) - Lightweight Solana SDK
