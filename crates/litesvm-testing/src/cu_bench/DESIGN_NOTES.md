# CU Benchmarking Framework - Design Notes

## Core Principles

### Why Systematic CU Benchmarking?

- **Gap in Ecosystem**: Existing tools focus on RPC performance/tx success, not instruction-level CU optimization
- **Developer Pain Point**: Manual CU testing is ad-hoc, non-reproducible, and time-consuming
- **Economic Impact**: Better CU estimates → lower fees → more efficient network usage

### Design Philosophy

- **Helius-Inspired Confidence Levels**: Familiar model from priority fee API
- **Reproducible by Default**: Same benchmark should give same results across environments
- **Context-Aware**: Environment/program context affects CU usage significantly
- **Community-Extensible**: Easy for developers to add their own benchmarks

## Methodology Decisions

### Measurement Strategy

- **Stateful Accumulation**: SVM maintains state across measurements (more realistic)
- **Fresh Blockhashes**: Each measurement gets new blockhash to avoid AlreadyProcessed errors
- **Statistical Approach**: Multiple samples → percentile-based estimates
- **Framework Responsibility**: Framework handles tx construction, signing, and CU extraction

### Environment Standardization

- **Feature Set Tracking**: LiteSVM features affect CU (see feature list)
- **Version Pinning**: Solana version changes can affect CU usage
- **Program Dependencies**: Track what programs are loaded/interacted with

### Benchmark Separation

- **Instruction-Level**: Single instruction CU usage (current focus)
- **Transaction-Level**: Multiple instructions + interaction effects (future)
- **Scenario-Based**: Real-world usage patterns (future)

## Current API Design

### InstructionBenchmark Trait

```rust
pub trait InstructionBenchmark {
    fn instruction_name(&self) -> &'static str;
    fn setup_svm(&self) -> LiteSVM;
    fn build_instruction(&self, svm: &mut LiteSVM) -> (Instruction, Vec<Pubkey>);
    fn sign_transaction(&self, unsigned_tx: Transaction) -> Transaction;
}
```

**Key Design Decisions:**

- **Benchmark owns setup**: Each benchmark controls its SVM environment
- **Framework owns measurement**: Consistent CU extraction across all benchmarks
- **Separation of concerns**: Benchmark builds instruction, framework measures CU

## Future API Evolution

### TransactionBenchmark (Future)

```rust
pub trait TransactionBenchmark {
    fn transaction_name(&self) -> &'static str;
    fn setup_svm(&self) -> LiteSVM;
    fn build_transaction(&self, svm: &mut LiteSVM) -> (Transaction, Vec<Pubkey>);
    // Note: Different from instruction - builds full transaction
}
```

### Unified Benchmark Enum (Consideration)

```rust
pub enum BenchmarkType<'a> {
    Instruction(&'a dyn InstructionBenchmark),
    Transaction(&'a dyn TransactionBenchmark),
}
```

## Open Questions/TODOs

- [ ] How to handle CPI effects in instruction benchmarks?
- [ ] Should we validate benchmark reproducibility automatically?
- [ ] Database/sharing mechanism for community estimates?
- [ ] How to handle version evolution of programs?

## Feature Impact Considerations

Based on LiteSVM feature set, these could affect CU measurements:

- Signature verification settings
- Precompiled programs enabled
- SPL program availability
- Feature gates (curve25519, etc.)
