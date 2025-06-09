//! # System Error Testing: Insufficient Funds
//!
//! This test suite demonstrates the progression from verbose to elegant error assertions
//! when testing system program errors. All tests validate the same error scenario: attempting
//! to transfer more SOL than an account contains.
//!
//! ## API Progression: Good → Better → Best
//!
//! - **Good**: Full `TransactionError` construction (verbose but shows complete hierarchy)
//! - **Better**: `InstructionError` level (removes transaction nesting, cleaner)
//! - **Best**: `SystemError` level (type-safe, no casting, most ergonomic)
//! - **Best+**: `SystemError` with explicit index control (surgical precision)
//!
//! ## Testing Styles
//!
//! Each approach is demonstrated in two styles:
//! - **Direct**: Traditional function calls `demand_*(expected, result)`
//! - **Fluent**: Method chaining `result.demand_*(expected)`
//!
//! ## The Error Being Tested
//!
//! `SystemError::ResultWithNegativeLamports` occurs when a transfer would result in
//! an account having negative lamports (insufficient funds). This manifests as:
//! - Transaction level: `TransactionError::InstructionError(1, InstructionError::Custom(1))`
//! - Instruction level: `InstructionError::Custom(1)`
//! - System level: `SystemError::ResultWithNegativeLamports` (enum value = 1)
//!
//! **Note**: The transaction includes a compute budget instruction at index 0, so the
//! failing transfer instruction is at index 1. This demonstrates multi-instruction scenarios.

use litesvm_testing::{prelude::*, setup_svm_and_fee_payer};

use {
    litesvm::LiteSVM, //
    solana_compute_budget_interface::ComputeBudgetInstruction,
    solana_instruction::error::InstructionError,
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_system_interface::error::SystemError,
    solana_transaction::Transaction,
    solana_transaction_error::TransactionError,
};

/// Sets up a transaction scenario that will trigger `SystemError::ResultWithNegativeLamports`.
///
/// Creates a "poor" account with only 1000 lamports, then attempts to transfer 500,000 lamports
/// to a recipient. This will fail because the account doesn't have sufficient funds, triggering
/// the system program's insufficient funds validation.
///
fn setup_insufficient_funds_scenario() -> (LiteSVM, Transaction) {
    let (mut svm, fee_payer) = setup_svm_and_fee_payer();

    // Create a poor account with minimal SOL
    let poor_account = Keypair::new();
    svm.airdrop(&poor_account.pubkey(), 1000) // Only 1000 lamports
        .expect("airdrop failed");

    // Try to transfer more than the account has (guaranteed to fail)
    let recipient = Keypair::new();
    let transfer_amount = 500_000; // 500x more than the account has

    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(4000);

    let transfer_ix = solana_system_interface::instruction::transfer(
        &poor_account.pubkey(),
        &recipient.pubkey(),
        transfer_amount,
    );

    // Expected instruction layout:
    //   [0] ComputeBudgetInstruction::set_compute_unit_limit(4000)
    //   [1] transfer_ix (this should fail with SystemError::ResultWithNegativeLamports)
    let tx = Transaction::new_signed_with_payer(
        &[compute_budget_ix, transfer_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &poor_account],
        svm.latest_blockhash(),
    );

    (svm, tx)
}

/// **Good Approach A**: Direct function call with full transaction error construction.
///
/// Shows the complete Solana error hierarchy: TransactionError wrapping InstructionError
/// wrapping Custom code. Verbose but demonstrates the full error structure that Solana
/// actually returns. Useful for understanding the complete error context.
#[test]
fn demand_transaction_error_directly() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    let result = svm.send_transaction(tx);
    demand_transaction_error(
        TransactionError::InstructionError(
            1,
            InstructionError::Custom(SystemError::ResultWithNegativeLamports as u32),
        ),
        result,
    );
}

/// **Good Approach B**: Fluent method chaining with full transaction error construction.
///
/// Same error validation as Approach A, but using fluent method syntax. The error
/// construction is still verbose, but the call site is more readable for chaining
/// with other assertions.
#[test]
fn demand_transaction_error_fluently() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    svm.send_transaction(tx)
        .demand_transaction_error(TransactionError::InstructionError(
            1,
            InstructionError::Custom(SystemError::ResultWithNegativeLamports as u32),
        ));
}

/// **Better Approach A**: Direct function call at instruction error level.
///
/// Removes the TransactionError wrapping layer and focuses on the InstructionError.
/// Still requires manual casting of SystemError to u32, but cleaner than the full
/// transaction error construction. Good middle ground between explicitness and brevity.
#[test]
fn demand_instruction_error_directly() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    let result = svm.send_transaction(tx);
    demand_instruction_error(
        InstructionError::Custom(SystemError::ResultWithNegativeLamports as u32),
        result,
    );
}

/// **Better Approach B**: Fluent method chaining at instruction error level.
///
/// Same instruction-level validation as Approach A, with fluent syntax. Removes the
/// transaction nesting complexity while maintaining the casting requirement. More
/// concise than transaction-level assertions.
#[test]
fn demand_instruction_error_fluently() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    svm.send_transaction(tx)
        .demand_instruction_error(InstructionError::Custom(
            SystemError::ResultWithNegativeLamports as u32,
        ));
}

/// **Best Approach A**: Direct function call with type-safe system error.
///
/// The most ergonomic approach - works directly with SystemError enum values.
/// No manual casting, no error code knowledge required, no nesting complexity.
/// Type-safe and self-documenting. This is the "anywhere" version that matches
/// the system error regardless of instruction index.
#[test]
fn demand_system_error_directly() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    let result = svm.send_transaction(tx);
    demand_system_error(SystemError::ResultWithNegativeLamports, result);
}

/// **Best Approach B**: Fluent method chaining with type-safe system error.
///
/// Combines the ergonomics of SystemError enum usage with fluent chaining syntax.
/// Perfect for single assertions or as part of a larger assertion chain. Most
/// readable and maintainable approach for typical system error testing.
#[test]
fn demand_system_error_fluently() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    svm.send_transaction(tx)
        .demand_system_error(SystemError::ResultWithNegativeLamports);
}

/// **Best+ Approach A**: Direct function call with surgical precision (index control).
///
/// Same ergonomics as the "Best" approach, but with explicit instruction index validation.
/// This test uses a multi-instruction transaction (compute budget + transfer) to demonstrate
/// that we can precisely target which instruction should fail. The transfer at index 1 fails,
/// while the compute budget instruction at index 0 succeeds.
#[test]
fn demand_system_error_at_index_directly() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    let result = svm.send_transaction(tx);
    demand_system_error_at_index(1, SystemError::ResultWithNegativeLamports, result);
}

/// **Best+ Approach B**: Fluent method chaining with surgical precision (index control).
///
/// Fluent version of surgical error testing using the same multi-instruction transaction.
/// Demonstrates the power of combining fluent syntax with precise instruction targeting.
/// Perfect for complex scenarios where you need to validate specific instruction failures.
#[test]
fn demand_system_error_at_index_fluently() {
    let (mut svm, tx) = setup_insufficient_funds_scenario();
    svm.send_transaction(tx)
        .demand_system_error_at_index(1, SystemError::ResultWithNegativeLamports);
}
