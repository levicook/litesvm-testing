//! # LiteSVM Testing Utilities
//!
//! This crate provides testing utilities for Solana programs using LiteSVM.
//! Currently focused on log assertions with support for multiple testing patterns.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! // Direct function call
//! use litesvm_testing::demand_logs_contain;
//! demand_logs_contain(result, "Hello from anchor!");
//!
//! // Fluent trait method  
//! use litesvm_testing::DemandFluency;
//! result.demand_logs_contain("Hello from anchor!");
//! ```
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

// "demanding solana"
// - transaction errors
// - instruction errors
// - anchor errors
// - anchor events
// - cu limits

use litesvm::types::TransactionResult;

/// Trait for fluent assertions on transaction results.
///
/// This trait extends [`TransactionResult`] with assertion methods that provide
/// detailed error messages when conditions are not met. The fluent API allows
/// for readable test code that chains naturally from transaction execution.
///
/// # Example
///
/// ```rust,no_run
/// use litesvm_testing::DemandFluency;
/// # use litesvm::LiteSVM; use solana_keypair::Keypair; use solana_transaction::Transaction;
/// # let mut svm = LiteSVM::new(); let payer = Keypair::new();
/// # let tx = Transaction::new_signed_with_payer(&[], Some(&payer.pubkey()), &[&payer], svm.latest_blockhash());
///
/// svm.send_transaction(tx)
///    .demand_logs_contain("Hello from anchor!");
/// ```
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
/// # Examples
///
/// ```rust,no_run
/// use litesvm::LiteSVM;
/// use litesvm_testing::demand_logs_contain;
///
/// let mut svm = LiteSVM::new();
/// // ... load your program and set up transaction ...
/// # let payer = Keypair::new();
/// # let tx = Transaction::new_signed_with_payer(&[], Some(&payer.pubkey()), &[&payer], svm.latest_blockhash());
///
/// let result = svm.send_transaction(tx);
/// assert!(result.is_ok());
///
/// // Assert that the logs contain your expected message
/// demand_logs_contain(result, "Hello from anchor!");
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
