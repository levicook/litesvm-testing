# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Steel framework support
- Enhanced testing utilities (account state validation)
- Integration with popular Solana testing patterns

## [0.2.0] - 2024-06-11

### Added - CU Benchmarking Framework

- **Systematic CU analysis framework** with instruction and transaction paradigms
- **Dual benchmarking modes**: Pure instruction measurement vs complete transaction workflows
- **Statistical analysis engine** with percentile-based estimates (min, conservative, balanced, safe, very_high, unsafe_max)
- **Rich execution context discovery** through simulation (SVM state, program details, CPI analysis)
- **Professional tooling integration** with env_logger and clean JSON output
- **Comprehensive unit tests** (324 lines) covering edge cases and percentile calculations

### Framework Features

- `InstructionBenchmark` trait for pure CU measurement without framework overhead
- `TransactionBenchmark` trait for multi-program workflow analysis
- Two-phase measurement: simulation for context + execution for statistics
- Address book system for human-readable program names
- Type-safe domain modeling with `StatType` enum

### Examples & Documentation

- **Working benchmarks**: SOL transfer (150 CU), SPL token transfer, token setup workflow (28K-38K CU)
- **Comprehensive guide**: [`BENCHMARKING.md`](crates/litesvm-testing/BENCHMARKING.md) with living documentation approach
- **Enhanced README**: Repositioned as testing + benchmarking platform

### Technical Improvements

- Fixed percentile calculation bugs that showed incorrect variance
- Removed automatic ComputeBudgetInstruction for measurement transparency
- SVM state accumulation for realistic vs isolated measurements
- Professional logging (quiet by default, optional progress via RUST_LOG)

### Breaking Changes

- None - all existing testing functionality preserved
- New benchmarking features require `--features cu_bench` opt-in

## [0.1.0] - 2025-01-30

### Added

- Complete error testing framework for transaction, instruction, and system errors
- Type-safe system error assertions using `SystemError` enums
- Log assertion utilities with detailed error messages
- Dual API styles: direct function calls and fluent method chaining
- Precision control: "anywhere" matching vs surgical instruction-index targeting
- Anchor framework integration with automatic compilation and IDL support
- Pinocchio framework integration with lightweight build utilities
- Educational examples showing API progression from verbose to elegant
- Comprehensive documentation with working examples
- Setup utilities for quick SVM and fee payer initialization

### Documentation

- Complete API documentation with examples
- Framework-specific integration guides
- Educational test suite demonstrating best practices
- Progressive examples showing Good → Better → Best → Best+ patterns

### Examples

- Working Anchor program integration
- Working Pinocchio program integration
- Educational test cases for common error scenarios
