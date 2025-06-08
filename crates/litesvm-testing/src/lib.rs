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
// - demand log contains

use litesvm::types::TransactionResult;

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
