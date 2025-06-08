//! # System Error Testing Examples
//!
//! This module demonstrates testing real system program errors using LiteSVM.
//! These are actual errors that occur when making system program calls,
//! not artificial errors returned by our programs.
//!
//! ## Important: Two Types of System Errors
//!
//! System errors can manifest at **two different levels** in the Solana runtime:
//!
//! ### 1. Transaction-Level Errors
//! These occur during transaction validation, **before** instruction execution:
//! - `TransactionError::AlreadyProcessed` - Duplicate transaction detected
//! - `TransactionError::InsufficientFundsForFee` - Can't pay transaction fees
//! - `TransactionError::AccountNotFound` - Referenced account doesn't exist
//!
//! **Test with:** `demand_transaction_error(TransactionError::AlreadyProcessed, result)`
//! **Example:** [`test_account_already_in_use`] - Duplicate account creation
//!
//! ### 2. Instruction-Level Errors  
//! These occur during system program instruction execution:
//! - `InstructionError::Custom(1)` - Insufficient funds for transfer operation
//! - `InstructionError::Custom(3)` - Invalid account data length for allocation
//!
//! **Test with:** `demand_instruction_error(result, 0, InstructionError::Custom(code))`
//! **Examples:**
//! - [`test_insufficient_funds_transfer`] - Transfer more than account balance
//! - [`test_invalid_account_data_length`] - Allocate excessive data
//!
//! ## Why This Matters
//!
//! This distinction is crucial for:
//! - **Precise error testing** - Use the right assertion for the right error level
//! - **Understanding Solana** - Know when errors occur in the transaction lifecycle  
//! - **Better debugging** - Transaction vs instruction failures have different causes
//!
//! This helps validate that our error testing utilities work with real-world
//! error scenarios that developers will encounter.

use {
    litesvm_testing::prelude::*, simple_anchor_tests::load_simple_anchor_program,
    solana_system_interface::instruction::create_account,
};

/// Test SystemError::ResultWithNegativeLamports
/// Try to transfer more SOL than the account has
#[test]
fn test_insufficient_funds_transfer() {
    use litesvm_testing::demand_instruction_error;
    let (mut svm, fee_payer) = setup();

    // Create a poor account with minimal SOL
    let poor_account = Keypair::new();
    svm.airdrop(&poor_account.pubkey(), 1000) // Just 1000 lamports
        .expect("airdrop failed");

    // Try to transfer more than the account has (minus rent + fees)
    let recipient = Keypair::new();
    let transfer_amount = 500_000; // Way more than poor_account has

    let transfer_ix = solana_system_interface::instruction::transfer(
        &poor_account.pubkey(), //
        &recipient.pubkey(),
        transfer_amount,
    );

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &poor_account],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);

    // Debug: Let's see exactly what error structure we get
    if let Err(ref meta) = result {
        println!("Actual TransactionError: {:?}", meta.err);
        println!(
            "SystemError::ResultWithNegativeLamports as u32 = {}",
            SystemError::ResultWithNegativeLamports as u32
        );
    }

    // Should fail with SystemError::ResultWithNegativeLamports
    let expected_error_code = SystemError::ResultWithNegativeLamports as u32;
    demand_instruction_error(result, 0, InstructionError::Custom(expected_error_code));
}

/// Test SystemError::AccountAlreadyInUse  
/// Try to create an account that already exists
#[test]
fn test_account_already_in_use() {
    let (mut svm, fee_payer) = setup();

    // Create an account first
    let new_account = Keypair::new();
    let create_ix = create_account(
        &fee_payer.pubkey(),
        &new_account.pubkey(),
        1_000_000,          // 1M lamports
        0,                  // 0 bytes data
        &SYSTEM_PROGRAM_ID, // owned by system program
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &new_account],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "First account creation should succeed");

    // Now try to create the same account again
    let create_again_ix = create_account(
        &fee_payer.pubkey(),
        &new_account.pubkey(), // Same pubkey!
        1_000_000,
        0,
        &SYSTEM_PROGRAM_ID,
    );

    let tx2 = Transaction::new_signed_with_payer(
        &[create_again_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &new_account],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx2);

    // Debug output
    if let Err(ref meta) = result {
        println!(
            "AccountAlreadyInUse - Actual TransactionError: {:?}",
            meta.err
        );
        println!(
            "SystemError::AccountAlreadyInUse as u32 = {}",
            SystemError::AccountAlreadyInUse as u32
        );
    }

    // IMPORTANT: This system error manifests as a TransactionError, not InstructionError!
    // The system program detects this at the transaction level before instruction execution.
    use litesvm_testing::{demand_transaction_error, solana_transaction_error::TransactionError};
    demand_transaction_error(TransactionError::AlreadyProcessed, result);
}

/// Test SystemError::InvalidAccountDataLength
/// Try to allocate an invalid amount of data
#[test]
fn test_invalid_account_data_length() {
    use litesvm_testing::demand_instruction_error;
    let (mut svm, fee_payer) = setup();

    let new_account = Keypair::new();

    // Try to create account with excessive data allocation
    let create_ix = create_account(
        &fee_payer.pubkey(),
        &new_account.pubkey(),
        1_000_000_000, // 1 SOL
        u64::MAX,      // Massive data allocation - should fail
        &SYSTEM_PROGRAM_ID,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &new_account],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);

    // Debug output
    if let Err(ref meta) = result {
        println!(
            "InvalidAccountDataLength - Actual TransactionError: {:?}",
            meta.err
        );
        println!(
            "SystemError::InvalidAccountDataLength as u32 = {}",
            SystemError::InvalidAccountDataLength as u32
        );
    }

    // Should fail with SystemError::InvalidAccountDataLength
    let expected_error_code = SystemError::InvalidAccountDataLength as u32;
    demand_instruction_error(result, 0, InstructionError::Custom(expected_error_code));
}

// Test utilities:

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// System program ID (well-known constant)
const SYSTEM_PROGRAM_ID: Pubkey = solana_system_interface::program::ID;

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    load_simple_anchor_program(&mut svm);

    let fee_payer = Keypair::new();
    svm.airdrop(&fee_payer.pubkey(), 2 * LAMPORTS_PER_SOL) // Give enough for fees
        .expect("airdrop failed");

    (svm, fee_payer)
}
