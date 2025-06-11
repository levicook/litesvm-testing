# litesvm-testing

A comprehensive testing and benchmarking framework for Solana programs using [LiteSVM](https://github.com/LiteSVM/litesvm). Provides ergonomic, type-safe assertions for transaction results, logs, and all levels of Solana errors, plus systematic compute unit (CU) analysis for performance optimization.

> **⚠️ Development Status**: This library is currently in active development. The API may change before the first stable release.

## ✨ Features

### 🧪 Testing Framework

- 🎯 **Complete Error Testing**: Transaction, instruction, and system program errors with type safety
- 📋 **Log Assertions**: Detailed log content verification with helpful error messages
- 🔧 **Build System Integration**: Automatic program compilation for Anchor and Pinocchio
- ⚡ **Dual API Styles**: Direct function calls or fluent method syntax
- 🎪 **Precision Control**: "Anywhere" matching vs surgical instruction-index targeting
- 🛡️ **Type Safety**: Work with `SystemError` enums instead of raw error codes
- 📚 **Educational Examples**: Learn API progression from verbose to elegant

### 📊 CU Benchmarking Framework

- ⚡ **Systematic CU Analysis**: Measure compute unit usage with statistical accuracy
- 🔬 **Dual Paradigms**: Pure instruction benchmarking vs complete transaction workflows
- 📈 **Percentile-Based Estimates**: Min, conservative, balanced, safe, very high, and max CU usage
- 🎯 **Production-Ready**: Generate CU estimates for real-world fee planning
- 📝 **Rich Context**: Execution logs, program details, and SVM environment state
- 🔄 **Reproducible**: Consistent results across environments and runs

### 🚀 Framework Support

- **Testing**: Anchor, Pinocchio, with more coming
- **Benchmarking**: Universal framework for any Solana program

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
use litesvm_testing::prelude::*;

#[test]
fn test_my_program() {
    let (mut svm, fee_payer) = setup_svm_and_fee_payer();
    // ... load your program and create transaction ...

    let result = svm.send_transaction(tx);

    // Test successful execution
    result.demand_logs_contain("Hello from my program!");

    // Or test error conditions with type safety
    result.demand_system_error(SystemError::ResultWithNegativeLamports);
}
```

### Fluent API (Method Style)

```rust
use litesvm_testing::prelude::*;

#[test]
fn test_my_program_fluently() {
    let (mut svm, fee_payer) = setup_svm_and_fee_payer();
    // ... load your program and create transaction ...

    let result = svm.send_transaction(tx);

    // Call assertions as methods on the result
    result.demand_logs_contain("Hello from my program!");
    result.demand_instruction_error_at_index(1, InstructionError::Custom(42));
}
```

## 🎯 Error Testing Hierarchy

Test errors at every level of the Solana execution model:

### 🏗️ Transaction Level

Validation errors before execution:

```rust
result.demand_transaction_error(TransactionError::AlreadyProcessed);
```

### 📍 Instruction Level

Errors during instruction execution:

```rust
// "Anywhere" - don't care which instruction failed
result.demand_instruction_error(InstructionError::Custom(1));

// "Surgical" - specific instruction must fail
result.demand_instruction_error_at_index(1, InstructionError::Custom(1));
```

### ⚙️ System Program Level

Type-safe system program errors:

```rust
// "Anywhere" - system error occurred somewhere
result.demand_system_error(SystemError::ResultWithNegativeLamports);

// "Surgical" - specific instruction produced system error
result.demand_system_error_at_index(1, SystemError::AccountAlreadyInUse);
```

## 📚 Framework Support

### Anchor Programs

Complete build system integration with IDL support:

```toml
[build-dependencies]
litesvm-testing = { git = "...", features = ["anchor"] }
```

```rust
// build.rs
use litesvm_testing::anchor_testing::build_anchor_program;

fn main() {
    build_anchor_program("../my-anchor-program");
}
```

### Pinocchio Programs

Lightweight compilation with minimal boilerplate:

```toml
[build-dependencies]
litesvm-testing = { git = "...", features = ["pinocchio"] }
```

```rust
// build.rs
use litesvm_testing::pinocchio_testing::build_pinocchio_program;

fn main() {
    build_pinocchio_program("../my-pinocchio-program");
}
```

## 🎯 Complete Testing Utilities

### Log Assertions

```rust
// Search all logs for content
result.demand_logs_contain("Hello from my program!");

// Check specific log entry by index
result.demand_logs_contain_at_index("Specific message", 2);
```

### Transaction Error Testing

```rust
// Test transaction-level failures
result.demand_transaction_error(TransactionError::InsufficientFundsForFee);
result.demand_transaction_error(TransactionError::AccountNotFound);
```

### Instruction Error Testing

```rust
// Test any instruction failure
result.demand_instruction_error(InstructionError::Custom(1));
result.demand_instruction_error(InstructionError::InvalidAccountData);

// Test specific instruction failure
result.demand_instruction_error_at_index(1, InstructionError::Custom(42));
```

### System Error Testing (Type-Safe)

```rust
// Test system program errors anywhere
result.demand_system_error(SystemError::ResultWithNegativeLamports);
result.demand_system_error(SystemError::AccountAlreadyInUse);

// Test system errors at specific instruction
result.demand_system_error_at_index(0, SystemError::InsufficientFunds);
```

### Convenience Setup

```rust
// Quick SVM setup with funded fee payer
let (mut svm, fee_payer) = setup_svm_and_fee_payer();
```

## 🎪 API Styles: Choose Your Preference

**Direct Functions** (traditional):

```rust
demand_logs_contain("Hello!", result);
demand_system_error(SystemError::InsufficientFunds, result);
```

**Fluent Methods** (alternative syntax):

```rust
result.demand_logs_contain("Hello!");
result.demand_system_error(SystemError::InsufficientFunds);
```

Both styles provide identical functionality - choose what feels right for your team!

> **Note**: Chainable fluent methods (`DemandChaining`) are planned for a future release, which will enable `result.demand_x().demand_y().accept()` style chaining.

## 📖 Complete Examples

This repository includes comprehensive, documented examples:

### Testing Framework

#### Anchor Framework

- **Program**: [`examples/anchor/simple-anchor-program/`](examples/anchor/simple-anchor-program/)
- **Tests**: [`examples/anchor/simple-anchor-tests/`](examples/anchor/simple-anchor-tests/)
- **Features**: IDL integration, automatic compilation, complete build documentation

#### Pinocchio Framework

- **Program**: [`examples/pinocchio/simple-pinocchio-program/`](examples/pinocchio/simple-pinocchio-program/)
- **Tests**: [`examples/pinocchio/simple-pinocchio-tests/`](examples/pinocchio/simple-pinocchio-tests/)
- **Features**: Minimal boilerplate, lightweight setup, direct BPF compilation

#### Educational Test Suite

- **API Progression**: [`tests/test_system_error_insufficient_funds.rs`](crates/litesvm-testing/tests/test_system_error_insufficient_funds.rs)
- **Features**: Good → Better → Best → Best+ progression, demonstrates all API styles

### CU Benchmarking Framework

#### Instruction Benchmarks

- **SOL Transfer**: [`benches/cu_bench_sol_transfer_ix.rs`](crates/litesvm-testing/benches/cu_bench_sol_transfer_ix.rs) - Pure system program instruction (150 CU)
- **SPL Token Transfer**: [`benches/cu_bench_spl_transfer_ix.rs`](crates/litesvm-testing/benches/cu_bench_spl_transfer_ix.rs) - Complex multi-account instruction

#### Transaction Benchmarks

- **Token Setup Workflow**: [`benches/cu_bench_token_setup_tx.rs`](crates/litesvm-testing/benches/cu_bench_token_setup_tx.rs) - Complete 5-instruction workflow (28K-38K CU)

#### Documentation

- **Complete Guide**: [`BENCHMARKING.md`](crates/litesvm-testing/BENCHMARKING.md) - Comprehensive benchmarking documentation

## 🏃‍♂️ Running Examples

```bash
# Clone the repository
git clone https://github.com/levicook/litesvm-testing
cd litesvm-testing

# Run all tests with detailed output
cargo test --workspace --no-fail-fast -- --show-output

# Run specific framework tests
cargo test -p simple-anchor-tests -- --show-output
cargo test -p simple-pinocchio-tests -- --show-output

# Run educational test suite
cargo test -p litesvm-testing test_system_error -- --show-output

# Run CU benchmarks with progress logging
cd crates/litesvm-testing
RUST_LOG=info cargo bench --bench cu_bench_sol_transfer_ix --features cu_bench
RUST_LOG=info cargo bench --bench cu_bench_token_setup_tx --features cu_bench
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
│   └── litesvm-testing/           # Core library with comprehensive docs
│       ├── src/
│       │   ├── lib.rs             # Main API and documentation
│       │   ├── anchor_testing/    # Anchor build utilities
│       │   └── pinocchio_testing/ # Pinocchio build utilities
│       └── tests/                 # Educational test examples
├── examples/
│   ├── anchor/                    # Complete Anchor integration
│   │   ├── simple-anchor-program/
│   │   └── simple-anchor-tests/
│   └── pinocchio/                 # Complete Pinocchio integration
│       ├── simple-pinocchio-program/
│       └── simple-pinocchio-tests/
└── README.md                      # This file
```

## 🗺️ Roadmap

### ✅ Completed Features

- [x] **Core log assertion utilities**
- [x] **Complete error testing framework** (transaction, instruction, system)
- [x] **Type-safe system error handling**
- [x] **Anchor and Pinocchio build support** with comprehensive documentation
- [x] **Working examples for both frameworks** with educational progression
- [x] **Dual API styles** (direct functions + fluent method syntax)
- [x] **Precision control** ("anywhere" vs "surgical" assertions)
- [x] **CU benchmarking framework** with instruction and transaction paradigms
- [x] **Statistical CU analysis** with percentile-based estimates
- [x] **Rich benchmarking context** (execution logs, program details, SVM state)

### 🔄 In Progress

- [ ] **Steel framework support**
- [ ] **Additional testing utilities** (account state verification, etc.)
- [ ] **First stable release (v0.1.0) to crates.io**
- [ ] **Integration with popular Solana testing patterns**

## 🎓 Educational Value

This library is designed not just as a tool, but as a learning resource:

- **Progressive examples**: See how testing approaches evolve from verbose to elegant
- **Framework comparisons**: Understand trade-offs between Anchor and Pinocchio
- **Complete documentation**: Every function includes usage examples and context
- **Real error scenarios**: Test actual system program failures, not synthetic examples

## 📝 License

This project is dual licensed under GPL-3.0-or-later and CC BY-SA 4.0. See LICENSE for details.

## 🔗 Related Projects

- [LiteSVM](https://github.com/LiteSVM/litesvm) - Fast Solana VM for testing
- [Anchor](https://github.com/coral-xyz/anchor) - Solana development framework
- [Pinocchio](https://github.com/anza-xyz/pinocchio) - Lightweight Solana SDK
  test
