//! # LiteSVM Testing Utilities
//! Copyright (C) 2024 LiteSVM Testing Framework Contributors - Licensed under GPL v3.0-or-later
//!
//! A comprehensive testing framework for Solana programs using LiteSVM. Provides ergonomic,
//! type-safe assertions for transaction results, logs, and all levels of Solana errors.
//!
//! ## Core Features
//!
//! - **📋 Log Assertions**: Verify program logs contain expected content
//! - **🎯 Error Testing**: Complete coverage of transaction, instruction, and system errors  
//! - **🔧 Dual API Styles**: Direct function calls and fluent method chaining
//! - **⚡ Precision Control**: "Anywhere" matching vs surgical instruction-index targeting
//! - **🛡️ Type Safety**: Work with SystemError enums instead of raw error codes
//! - **📚 Educational Examples**: Learn API progression from verbose to elegant
//!
//! ## API Styles
//!
//! **Direct Functions** (traditional):
//! ```text
//! demand_logs_contain("Hello!", result);
//! demand_system_error(SystemError::ResultWithNegativeLamports, result);
//! ```
//!
//! **Fluent Methods** (chainable):
//! ```text
//! result.demand_logs_contain("Hello!")
//!       .demand_system_error(SystemError::ResultWithNegativeLamports);
//! ```
//!
//! ## Error Testing Hierarchy
//!
//! **🏗️ Transaction Level**: Validation errors before execution
//! - `demand_transaction_error(TransactionError::AlreadyProcessed, result)`
//!
//! **📍 Instruction Level**: Errors during instruction execution  
//! - `demand_instruction_error(InstructionError::Custom(1), result)`
//! - `demand_instruction_error_at_index(1, InstructionError::Custom(1), result)`
//!
//! **⚙️ System Level**: Type-safe system program errors
//! - `demand_system_error(SystemError::ResultWithNegativeLamports, result)` (anywhere)
//! - `demand_system_error_at_index(1, SystemError::ResultWithNegativeLamports, result)` (surgical)
//!
//! ## Complete Examples
//!
//! **API Progression Tutorial**:
//! - [`test_system_error_insufficient_funds.rs`](crates/litesvm-testing/tests/test_system_error_insufficient_funds.rs) - Shows Good → Better → Best → Best+ approaches
//!
//! **Framework Integration**:
//! - **Anchor**: [`examples/anchor/simple-anchor-tests/`](examples/anchor/simple-anchor-tests/) - Complete Anchor program testing with IDL integration
//! - **Pinocchio**: [`examples/pinocchio/simple-pinocchio-tests/`](examples/pinocchio/simple-pinocchio-tests/) - Lightweight testing with minimal boilerplate

#[cfg(feature = "anchor")]
pub mod anchor_testing;

#[cfg(any(feature = "anchor", feature = "pinocchio"))]
mod build_internal;

#[cfg(feature = "cu_bench")]
pub mod cu_bench;

#[cfg(feature = "pinocchio")]
pub mod pinocchio_testing;

// #[cfg(feature = "token")]
// pub mod token_testing;

// #[cfg(feature = "steel")]
// pub mod steel_testing;

use num_traits::FromPrimitive;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_system_interface::error::SystemError;

/// Convenient re-exports for LiteSVM testing.
///
/// This prelude module provides all the commonly needed types and functions for testing
/// Solana programs with LiteSVM. Import everything with:
///
/// ```text
/// use litesvm_testing::prelude::*;
/// ```
///
/// ## What's included:
///
/// **Core Solana types**:
/// - `litesvm` - The LiteSVM runtime for testing
/// - `solana_*` - Transaction, instruction, keypair, and error types
///
/// **Testing assertions**:
/// - `demand_logs_contain` - Assert transaction logs contain expected content
/// - `demand_transaction_error` - Assert transaction-level errors  
/// - `demand_instruction_error` - Assert instruction-level errors
/// - `demand_system_error` - Assert system program errors (type-safe)
/// - `DemandFluency` - Trait for fluent method chaining
pub mod prelude {
    pub use litesvm;
    pub use solana_compute_budget_interface;
    pub use solana_instruction;
    pub use solana_keypair;
    pub use solana_pubkey;
    pub use solana_signer;
    pub use solana_system_interface;
    pub use solana_transaction;
    pub use solana_transaction_error;
    pub use spl_associated_token_account;
    pub use spl_token;

    pub use solana_keypair::Keypair;
    pub use solana_pubkey::Pubkey;
    pub use solana_signer::Signer;
    pub use solana_system_interface::program as system_program;

    pub use super::{
        demand_instruction_error, //
        demand_instruction_error_at_index,
        demand_logs_contain,
        demand_logs_contain_at_index,
        demand_system_error,
        demand_system_error_at_index,
        demand_transaction_error,
        DemandFluency,
    };
}

// "demanding solana"
// - transaction errors
// - instruction errors
// - custom errors (the special case instruction error)
// - anchor errors
// - anchor events
// - cu limits, etc, etc, etc

use litesvm::{types::TransactionResult, LiteSVM};
use solana_instruction::error::InstructionError;
use solana_transaction_error::TransactionError;

/// Trait for fluent assertions on transaction results.
///
/// This trait extends [`TransactionResult`] with assertion methods that provide
/// detailed error messages when conditions are not met. The fluent API allows
/// for readable test code that chains naturally from transaction execution.
///
/// See the working examples in the repository for complete usage patterns.
pub trait DemandFluency<T> {
    fn demand_instruction_error(self, expected_error: InstructionError);
    fn demand_instruction_error_at_index(
        self,
        expected_index: u8,
        expected_error: InstructionError,
    );
    fn demand_logs_contain(self, expected: &str);
    fn demand_system_error(self, expected_error: SystemError);
    fn demand_system_error_at_index(self, expected_index: u8, expected_error: SystemError);
    fn demand_transaction_error(self, expected_error: TransactionError);
}

impl DemandFluency<TransactionResult> for TransactionResult {
    fn demand_instruction_error(self, expected_error: InstructionError) {
        demand_instruction_error(expected_error, self);
    }

    fn demand_instruction_error_at_index(
        self,
        expected_index: u8,
        expected_error: InstructionError,
    ) {
        demand_instruction_error_at_index(expected_index, expected_error, self);
    }

    fn demand_logs_contain(self, expected: &str) {
        demand_logs_contain(expected, self);
    }

    fn demand_system_error(self, expected_error: SystemError) {
        demand_system_error(expected_error, self);
    }

    fn demand_system_error_at_index(self, expected_index: u8, expected_error: SystemError) {
        demand_system_error_at_index(expected_index, expected_error, self);
    }

    fn demand_transaction_error(self, expected_error: TransactionError) {
        demand_transaction_error(expected_error, self);
    }
}

// FUTURE IDEA: support for chaining methods on the result:
// pub trait DemandChaining<T> {
//     fn demand_logs_contain_and(self, expected: &str) -> Self;
//     fn demand_system_error_and(self, expected_error: SystemError) -> Self;
//     fn accept(self); // terminal method to consume the result
// }

/// Asserts that a transaction's logs contain a specific string.
///
/// This function is designed for testing Solana programs with LiteSVM. It searches through
/// all log entries produced by a transaction and panics with a detailed error message if
/// the expected string is not found.
///
/// # Arguments
///
/// * `expected` - The string to search for within the transaction logs
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
/// # Panics
///
/// Panics if `expected` is not found in any of the transaction logs. The panic message
/// includes:
/// - The string that was expected but not found
/// - The total number of log entries searched
/// - All log entries with their indices for debugging
///
/// # Usage
///
/// Direct function call:
/// ```text
/// demand_logs_contain("Hello from anchor!", result);
/// ```
///
/// Fluent trait method:
/// ```text
/// result.demand_logs_contain("Hello from anchor!");
/// ```
///
/// For complete working examples, see:
/// - **Anchor**: `examples/anchor/simple-anchor-tests/tests/test_simple_anchor_program.rs`
/// - **Pinocchio**: `examples/pinocchio/simple-pinocchio-tests/tests/test_simple_pinocchio_program.rs`
///
/// These examples show the full setup including build scripts, program loading, and test structure.
///
/// ## Error output example
///
/// When the assertion fails, you'll see output like:
///
/// ```text
/// Expected to find: "Hello from my program!" in one of 4 log entries:
///   [0]: Program 11111111111111111111111111111111 invoke [1]
///   [1]: Program log: Hello from pinocchio! [12, 10, 227, ...]
///   [2]: Program 11111111111111111111111111111111 consumed 3258 of 200000 compute units
///   [3]: Program 11111111111111111111111111111111 success
/// ```
///
/// # Note
///
/// This function works with both successful and failed transactions. For failed transactions,
/// it searches through the logs in the error metadata.
pub fn demand_logs_contain(expected: &str, result: TransactionResult) {
    let logs = match &result {
        Ok(meta) => &meta.logs,
        Err(meta) => &meta.meta.logs,
    };

    if logs.iter().any(|log| log.contains(expected)) {
        return;
    }

    panic!(
        "Expected {:?} among {} log entries: {}",
        expected,
        logs.len(),
        logs.iter()
            .enumerate()
            .map(|(i, log)| format!("[{}]: {}", i, log))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// Asserts that a specific log entry contains an expected string.
///
/// Unlike [`demand_logs_contain`] which searches all logs, this function checks
/// only the log at the specified index. Useful when you need to verify specific
/// program log ordering or validate that a particular instruction produces
/// expected log output.
///
/// # Arguments
///
/// * `expected` - The string to search for within the specific log entry
/// * `expected_index` - The index of the log entry to check (0-based)
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
/// # Panics
///
/// Panics if:
/// - The log index is out of bounds (not enough log entries)
/// - The log entry at the specified index doesn't contain the expected string
///
/// # Example
///
/// ```text
/// // Verify the second log entry contains specific content
/// demand_logs_contain_at_index("Hello from instruction 1", 1, result);
/// ```
pub fn demand_logs_contain_at_index(
    expected: &str,
    expected_index: usize,
    result: TransactionResult,
) {
    let logs = match &result {
        Ok(meta) => &meta.logs,
        Err(meta) => &meta.meta.logs,
    };

    let Some(log_entry) = logs.get(expected_index) else {
        panic!(
            "Log index {} out of bounds, only {} entries available",
            expected_index,
            logs.len()
        );
    };

    if !log_entry.contains(expected) {
        panic!(
            "Expected {:?} at log index {} but found: {:?}",
            expected, expected_index, log_entry
        );
    }
}

/// Asserts that a transaction fails with a specific instruction error, regardless of index.
///
/// This is the "anywhere" version of instruction error testing - it matches the specified
/// instruction error without caring which instruction in the transaction produced it.
/// Perfect for single-instruction transactions or when you just need to verify the error type.
///
/// For precise instruction-index control, use [`demand_instruction_error_at_index`].
///
/// # Arguments
///
/// * `expected_error` - The expected instruction error
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
/// # Panics
///
/// Panics if:
/// - The transaction succeeds (no error)
/// - The error is not an instruction error
/// - The instruction error doesn't match the expected error
///
/// # Example
///
/// ```text
/// demand_instruction_error(
///     InstructionError::Custom(1),
///     result
/// );
/// ```
pub fn demand_instruction_error(expected_error: InstructionError, result: TransactionResult) {
    let Err(e) = result else {
        panic!("Expected {} but transaction succeeded", expected_error);
    };

    let TransactionError::InstructionError(_, observed_error) = &e.err else {
        panic!("Expected {} but got: {}", expected_error, e.err);
    };

    if *observed_error != expected_error {
        panic!("Expected {} but got {}", expected_error, observed_error);
    }
}

/// Asserts that a specific instruction fails with a specific error.
///
/// This is the "surgical" version of instruction error testing - it validates both the
/// error type AND which instruction produced it. Use this for multi-instruction transactions
/// where you need to verify that a specific instruction fails with a specific error.
///
/// For "anywhere" matching (don't care about index), use [`demand_instruction_error`].
///
/// # Arguments
///
/// * `expected_index` - The index of the instruction that should fail (0-based)
/// * `expected_error` - The expected instruction error
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
/// # Panics
///
/// Panics if:
/// - The transaction succeeds (no error)
/// - The error is not an instruction error
/// - The error occurs at a different instruction index
/// - The instruction error doesn't match the expected error
///
/// # Example
///
/// ```text
/// // Expect the second instruction (index 1) to fail with Custom(42)
/// demand_instruction_error_at_index(
///     1,
///     InstructionError::Custom(42),
///     result
/// );
/// ```
pub fn demand_instruction_error_at_index(
    expected_index: u8,
    expected_error: InstructionError,
    result: TransactionResult,
) {
    let Err(e) = result else {
        panic!(
            "Expected {} at index {} but transaction succeeded",
            expected_error, expected_index
        );
    };

    let TransactionError::InstructionError(observed_index, observed_error) = &e.err else {
        panic!(
            "Expected {} at index {} but got: {}",
            expected_error, expected_index, e.err
        );
    };

    if *observed_index != expected_index {
        panic!(
            "Expected {} at index {} but got error at index {}",
            expected_error, expected_index, observed_index
        );
    }

    if *observed_error != expected_error {
        panic!(
            "Expected {} at index {} but got {} at index {}",
            expected_error, expected_index, observed_error, observed_index
        );
    }
}

/// Asserts that a transaction error matches the expected error.
///
/// This function tests for transaction-level errors that occur before instruction execution,
/// such as `AlreadyProcessed`, `InsufficientFundsForFee`, `AccountNotFound`, etc.
///
/// # Important Distinction
///
/// **Transaction-level errors** occur during transaction validation/processing:
/// - `TransactionError::AlreadyProcessed` - Transaction already seen
/// - `TransactionError::InsufficientFundsForFee` - Can't pay transaction fees  
/// - `TransactionError::AccountNotFound` - Referenced account doesn't exist
///
/// **Instruction-level errors** occur during instruction execution:
/// - `InstructionError::Custom(1)` - System program insufficient funds for transfer
/// - `InstructionError::Custom(3)` - System program invalid data length
///
/// Use [`demand_instruction_error`] for instruction-level errors.
///
/// # Arguments
///
/// * `expected` - The expected transaction error
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
pub fn demand_transaction_error(expected: TransactionError, result: TransactionResult) {
    let Err(e) = result else {
        panic!("Expected {} but transaction succeeded", expected);
    };

    if e.err != expected {
        panic!("Expected {} but got {}", expected, e.err);
    }
}

/// Asserts that a system error occurs, regardless of which instruction index produced it.
///
/// This is the "anywhere" version that matches system errors without caring about instruction index.
/// When a transaction fails, there's exactly one error - this function checks if that error matches
/// the expected system error, ignoring which instruction caused it.
///
/// **When to use**: Use this when you know the error should occur but don't care which instruction
/// produced it. Perfect for single-instruction transactions or when testing general error conditions.
///
/// **When not to use**: When you need to verify the error occurs at a specific instruction index
/// to test precise error handling - use [`demand_system_error_at_index`] instead.
///
/// # Arguments
///
/// * `expected_error` - The expected system error
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
pub fn demand_system_error(expected_error: SystemError, result: TransactionResult) {
    let Err(e) = &result else {
        panic!("Expected {} but transaction succeeded", expected_error);
    };

    let TransactionError::InstructionError(_, InstructionError::Custom(observed_code)) = &e.err
    else {
        panic!("Expected {} but got: {}", expected_error, e.err);
    };

    let Some(observed_error) = SystemError::from_u64(*observed_code as u64) else {
        panic!(
            "Expected {} but got invalid code {}",
            expected_error, observed_code
        );
    };

    if observed_error != expected_error {
        panic!("Expected {} but got: {}", expected_error, observed_error);
    }
}

/// Asserts that a system error occurs at a specific instruction index.
///
/// This is the "surgical" version for when you need precise control over which
/// instruction should produce the error. Use this for multi-instruction transactions
/// or when you want to be explicit about the instruction index.
///
/// # Arguments
///
/// * `expected_index` - The index of the instruction that should produce the error
/// * `expected_error` - The expected system error  
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
///
pub fn demand_system_error_at_index(
    expected_index: u8,
    expected_error: SystemError,
    result: TransactionResult,
) {
    let Err(e) = &result else {
        panic!(
            "Expected {} at index {} but transaction succeeded",
            expected_error, expected_index
        );
    };

    let TransactionError::InstructionError(observed_index, InstructionError::Custom(observed_code)) =
        &e.err
    else {
        panic!(
            "Expected {} at index {} but got: {:?}",
            expected_error, expected_index, e.err
        );
    };

    if *observed_index != expected_index {
        panic!(
            "Expected {} at index {} but got error at index {}",
            expected_error, expected_index, observed_index
        );
    }

    let Some(observed_error) = SystemError::from_u64(*observed_code as u64) else {
        panic!(
            "Expected {} at index {} but got invalid code {} at index {}",
            expected_error, expected_index, observed_code, observed_index
        );
    };

    if observed_error != expected_error {
        panic!(
            "Expected {} at index {} but got {} at index {}",
            expected_error, expected_index, observed_error, observed_index
        );
    }
}

/// Sets up a fresh LiteSVM instance with a funded fee payer account.
///
/// This is a convenience function for getting started quickly with LiteSVM testing.
/// It creates a new SVM runtime and funds a fee payer account with 100 SOL, which
/// should be sufficient for most testing scenarios.
///
/// # Returns
///
/// A tuple containing:
/// - `LiteSVM` - A fresh SVM runtime instance
/// - `Keypair` - A fee payer account funded with 100 SOL
///
/// # Example
///
/// ```text
/// let (mut svm, fee_payer) = setup_svm_and_fee_payer();
///
/// // Use svm for testing...
/// let result = svm.send_transaction(tx);
/// ```
///
/// # Note
///
/// This function is primarily intended for examples and getting started. For production
/// tests, you may want more control over the setup process.
pub fn setup_svm_and_fee_payer() -> (LiteSVM, Keypair) {
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    let mut svm = LiteSVM::new();

    let fee_payer = Keypair::new();
    svm.airdrop(&fee_payer.pubkey(), 100 * LAMPORTS_PER_SOL)
        .expect("airdrop failed");

    (svm, fee_payer)
}
