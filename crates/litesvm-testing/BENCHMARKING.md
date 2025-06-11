# Compute Unit Benchmarking Guide

> **Systematic CU analysis for Solana programs using LiteSVM**

This guide shows you how to measure compute unit (CU) usage for Solana instructions and transactions with reproducible, statistical accuracy.

## Quick Start

```bash
# Run an instruction benchmark
RUST_LOG=info cargo bench --bench cu_bench_sol_transfer_ix --features cu_bench

# Run a transaction benchmark
RUST_LOG=info cargo bench --bench cu_bench_token_setup_tx --features cu_bench
```

## When to Use What

### Instruction Benchmarking

**Use when:** You want to measure the pure CU cost of a single instruction

- ✅ Research and analysis
- ✅ Understanding base program costs
- ✅ Comparing instruction efficiency
- ✅ Building CU cost models

**Example:** How much does a SOL transfer cost? → **150 CU**

### Transaction Benchmarking

**Use when:** You want to measure real-world workflow costs

- ✅ Production fee estimation
- ✅ Multi-instruction workflows
- ✅ Cross-program invocation analysis
- ✅ Complete user journeys

**Example:** How much does token setup cost? → **28,322-38,822 CU**

## Implementing Instruction Benchmarks

> **See [`benches/cu_bench_sol_transfer_ix.rs`](benches/cu_bench_sol_transfer_ix.rs) for a complete working example**

### 1. Implement the InstructionBenchmark Trait

```rust
use litesvm_testing::cu_bench::{benchmark_instruction, InstructionBenchmark};

struct SolTransferBenchmark {
    sender: Keypair,
    recipient: Keypair,
    transfer_amount: u64,
}

impl InstructionBenchmark for SolTransferBenchmark {
    fn instruction_name(&self) -> &'static str {
        "sol_transfer"
    }

    fn setup_svm(&self) -> LiteSVM {
        let mut svm = LiteSVM::new();
        svm.airdrop(&self.sender.pubkey(), 200_000_000).unwrap();
        svm
    }

    fn build_instruction(&self, _svm: &mut LiteSVM) -> (Instruction, Vec<Pubkey>) {
        let transfer_ix = solana_system_interface::instruction::transfer(
            &self.sender.pubkey(),
            &self.recipient.pubkey(),
            self.transfer_amount,
        );
        (transfer_ix, vec![self.sender.pubkey()])
    }

    fn sign_transaction(&self, mut unsigned_tx: Transaction) -> Transaction {
        unsigned_tx.sign(&[&self.sender], unsigned_tx.message.recent_blockhash);
        unsigned_tx
    }
}
```

### 2. Run the Benchmark

```rust
fn main() {
    env_logger::init();
    let benchmark = SolTransferBenchmark::new();
    let result = benchmark_instruction(benchmark, 100);

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
```

## Implementing Transaction Benchmarks

> **See [`benches/cu_bench_token_setup_tx.rs`](benches/cu_bench_token_setup_tx.rs) for a complete multi-program workflow example**

### 1. Implement the TransactionBenchmark Trait

```rust
use litesvm_testing::cu_bench::{benchmark_transaction, TransactionBenchmark};

impl TransactionBenchmark for TokenSetupTransactionBenchmark {
    fn transaction_name(&self) -> &'static str {
        "token_setup_complete"
    }

    fn setup_svm(&self) -> LiteSVM {
        let mut svm = LiteSVM::new();
        svm.airdrop(&self.mint_authority.pubkey(), 1_000_000_000).unwrap();
        svm
    }

    fn build_transaction(&mut self, svm: &mut LiteSVM) -> Transaction {
        // Create fresh mint to avoid collisions
        self.mint = Keypair::new();

        let instructions = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(150_000),
            // ... create mint, initialize, create ATA, mint tokens
        ];

        // Build and sign complete transaction
        // (see full example in benchmark file)
    }
}
```

## Understanding Results

### Percentile-Based Estimates

```json
{
  "cu_estimate": {
    "min": 150, // 0th percentile - absolute minimum
    "conservative": 150, // 25th percentile - safe for most cases
    "balanced": 150, // 50th percentile - good default
    "safe": 175, // 75th percentile - high reliability
    "very_high": 190, // 95th percentile - very reliable
    "unsafe_max": 200, // 100th percentile - maximum observed
    "sample_size": 100
  }
}
```

### Execution Context

Rich context about what happened during execution:

```json
{
  "execution_context": {
    "svm_context": {
      "current_slot": 0,
      "latest_blockhash": "..."
    },
    "program_context": {
      "program_id": "11111111111111111111111111111111",
      "program_name": "system_program",
      "cpi_count": 1
    },
    "execution_stats": {
      "logs": ["Program 11111111111111111111111111111111 invoke [1]", "..."],
      "simulated_cu": 150
    }
  }
}
```

## Best Practices

### 1. **Fresh State for Each Measurement**

- Use `svm.expire_blockhash()` to avoid `AlreadyProcessed` errors
- Generate fresh keypairs when needed to avoid account collisions
- Fund accounts generously for multiple measurements

### 2. **Realistic Scenarios**

- Mirror production account states and balances
- Include necessary setup instructions
- Use representative data sizes

### 3. **Statistical Rigor**

- Run 100+ samples for stable percentiles
- Use appropriate confidence levels:
  - `conservative` (25th) for safety margins
  - `balanced` (50th) for typical estimates
  - `safe` (75th) for high-reliability scenarios

### 4. **Environment Consistency**

- Pin Solana versions for reproducible results
- Document LiteSVM features that affect measurements
- Use consistent SVM configuration across benchmarks

## Advanced Examples

### Multi-Program Workflows

See [`benches/cu_bench_token_setup_tx.rs`](benches/cu_bench_token_setup_tx.rs) for:

- Cross-program invocations (CPI)
- Account creation and initialization
- Complex transaction construction
- Address book for human-readable program names

### Complex Instructions

See [`benches/cu_bench_spl_transfer_ix.rs`](benches/cu_bench_spl_transfer_ix.rs) for:

- Multi-step SVM setup
- Associated token account handling
- Mint and token account management

## Integration with Production Code

### Using Results for Fee Estimation

```rust
// Load benchmark results
let sol_transfer_result: InstructionBenchmarkResult =
    serde_json::from_str(include_str!("../results/sol_transfer.json"))?;

// Get conservative estimate for fee calculation
let cu_estimate = sol_transfer_result.cu_estimate.conservative;

// Build transaction with appropriate CU limit
let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu_estimate);
```

### Building CU Databases

```rust
use litesvm_testing::cu_bench::ComputeUnitDatabase;

let mut db = ComputeUnitDatabase::new();
// Add estimates from various benchmarks
// Save as JSON for application use
```

## Troubleshooting

### Common Issues

**"AlreadyProcessed" Errors**

- Ensure `svm.expire_blockhash()` before each measurement
- Don't reuse transactions across measurements

**"Account Already Exists" Errors**

- Generate fresh keypairs in `build_transaction()`
- Don't reuse account addresses across measurements

**"Insufficient Funds" Errors**

- Increase airdrop amounts in `setup_svm()`
- Account for fees across many measurements

**Inconsistent Results**

- Check for state accumulation effects
- Verify SVM setup consistency
- Ensure measurements are independent

## Further Reading

- **Benchmark Examples**: [`benches/`](benches/) directory
- **API Documentation**: [docs.rs/litesvm-testing](https://docs.rs/litesvm-testing)
- **Core Framework**: [`src/cu_bench/`](src/cu_bench/) module
