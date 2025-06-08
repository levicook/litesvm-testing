//! # LiteSVM Testing Utilities
//!
//! This crate provides testing utilities for Solana programs using LiteSVM.
//! Currently focused on log assertions with support for multiple testing patterns.
//!
//! ## API Patterns
//!
//! Two ways to assert on transaction logs:
//! 1. **Direct function call**: `demand_logs_contain(result, "message")`
//! 2. **Fluent trait method**: `result.demand_logs_contain("message")`
//!
//! Both approaches provide the same functionality with detailed error messages
//! when assertions fail.
//!
//! ## Complete Examples
//!
//! See working examples with full setup:
//! - **Anchor**: `examples/anchor/simple-anchor-tests/tests/test_simple_anchor_program.rs`
//! - **Pinocchio**: `examples/pinocchio/simple-pinocchio-tests/tests/test_simple_pinocchio_program.rs`

#[cfg(feature = "anchor")]
pub mod anchor;

#[cfg(feature = "pinocchio")]
pub mod pinocchio;

// #[cfg(feature = "steel")]
// pub mod steel;

pub use solana_instruction;
pub use solana_keypair;
pub use solana_pubkey;
pub use solana_signer;
pub use solana_system_interface;
pub use solana_transaction;
pub use solana_transaction_error;

pub mod prelude {
    pub use litesvm;
    pub use solana_instruction;
    pub use solana_keypair;
    pub use solana_pubkey;
    pub use solana_signer;
    pub use solana_system_interface;
    pub use solana_transaction;

    pub use litesvm::LiteSVM;
    pub use solana_instruction::error::InstructionError;
    pub use solana_instruction::Instruction;
    pub use solana_keypair::Keypair;
    pub use solana_pubkey::Pubkey;
    pub use solana_signer::Signer;
    pub use solana_system_interface::error::SystemError;
    pub use solana_transaction::Transaction;
}

// "demanding solana"
// - transaction errors
// - instruction errors
// - anchor errors
// - anchor events
// - cu limits

use litesvm::types::TransactionResult;
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
    /// Assert that transaction logs contain a specific string.
    ///
    /// See [`demand_logs_contain`] for detailed behavior and error output.
    fn demand_logs_contain(self, expected: &str);
}

impl DemandFluency<TransactionResult> for TransactionResult {
    fn demand_logs_contain(self, expected: &str) {
        demand_logs_contain(self, expected);
    }
}

/// Asserts that a transaction's logs contain a specific string.
///
/// This function is designed for testing Solana programs with LiteSVM. It searches through
/// all log entries produced by a transaction and panics with a detailed error message if
/// the expected string is not found.
///
/// # Arguments
///
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
/// * `expected` - The string to search for within the transaction logs
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
/// demand_logs_contain(result, "Hello from anchor!");
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
/// ❌ Log assertion failed!
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
pub fn demand_logs_contain(result: TransactionResult, expected: &str) {
    let check_logs = |logs: &[String]| {
        if !logs.iter().any(|log| log.contains(expected)) {
            panic!(
                "\n❌ Log assertion failed!\n\
                 Expected to find: {:?} in one of {} log entries:\n{}\n",
                expected,
                logs.len(),
                logs.iter()
                    .enumerate()
                    .map(|(i, log)| format!("  [{}]: {}", i, log))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    };

    match &result {
        Ok(meta) => check_logs(&meta.logs),
        Err(meta) => check_logs(&meta.meta.logs),
    }
}

/// Asserts that a transaction's instruction error matches the expected error.
///
/// This function is designed for testing Solana programs with LiteSVM. It searches through
/// the transaction error metadata and panics with a detailed error message if the expected
/// error is not found.
///
/// # Arguments
///
/// * `result` - The result of executing a transaction via [`litesvm::LiteSVM::send_transaction`]
/// * `expected_index` - The index of the instruction that should have the expected error
/// * `expected_error` - The expected error
///
pub fn demand_instruction_error(
    result: TransactionResult,
    expected_index: u8,
    expected_error: InstructionError,
) {
    match result {
        Ok(_) => panic!("Expected error but got Ok"),
        Err(e) => {
            if let TransactionError::InstructionError(observed_index, observed_error) =
                e.err.clone()
            {
                if observed_index == expected_index && observed_error == expected_error {
                    return;
                }

                panic!(
                    "\n❌ Instruction error assertion failed!\n\
                    Expected {} error at index {}, got: {:?}",
                    expected_error, expected_index, e.err
                );
            } else {
                panic!("Expected InstructionError, got: {:?}", e.err);
            }
        }
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
    match result {
        Ok(_) => panic!("Expected error but got Ok"),
        Err(e) => {
            if e.err == expected {
                return;
            }
            panic!(
                "\n❌ Transaction error assertion failed!\n\
                Expected {}, got: {:?}",
                expected, e.err
            );
        }
    }
}

pub fn demand_system_error(
    _result: TransactionResult,
    _expected_error: solana_system_interface::error::SystemError,
) {
    todo!()
}
