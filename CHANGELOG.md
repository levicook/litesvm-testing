# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Steel framework support (planned)
- Enhanced testing utilities (compute units, account state validation)
- Integration with popular Solana testing patterns

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
