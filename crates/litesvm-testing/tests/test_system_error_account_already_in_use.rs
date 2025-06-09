// TODO remodel this after test_system_error_insufficient_funds.rs

use litesvm_testing::prelude::*;

use {
    litesvm::LiteSVM, //
    solana_instruction::error::InstructionError,
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_system_interface::error::SystemError,
    solana_system_interface::instruction::create_account,
    solana_transaction::Transaction,
    solana_transaction_error::TransactionError,
};

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

    // Now try to create the same account again (with different blockhash to avoid tx deduplication)
    svm.expire_blockhash(); // Advance blockhash to make transaction different
    let create_again_ix = create_account(
        &fee_payer.pubkey(),
        &new_account.pubkey(), // Same pubkey - this should fail!
        1_000_000,
        0,
        &SYSTEM_PROGRAM_ID,
    );

    let tx2 = Transaction::new_signed_with_payer(
        &[create_again_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &new_account], // Same signers but different blockhash
        svm.latest_blockhash(),      // New blockhash makes transaction different
    );

    let result = svm.send_transaction(tx2);

    // Explicit match to decode the exact error structure
    match result {
        Ok(_) => panic!("Expected duplicate account creation to fail"),
        Err(ref meta) => {
            println!("=== DETAILED ERROR ANALYSIS ===");
            println!("Raw error: {:?}", meta.err);

            // Match on the specific error structure
            match &meta.err {
                TransactionError::InstructionError(index, instruction_error) => {
                    println!("✅ CONFIRMED: InstructionError at index {}", index);

                    // If it's a Custom error, decode it
                    if let InstructionError::Custom(code) = instruction_error {
                        println!("Custom error code: {}", code);
                        if *code == SystemError::AccountAlreadyInUse as u32 {
                            println!("✅ CONFIRMED: Code matches SystemError::AccountAlreadyInUse");
                        } else {
                            println!("❌ UNEXPECTED: Code {} doesn't match SystemError::AccountAlreadyInUse ({})", 
                                code, SystemError::AccountAlreadyInUse as u32);
                            panic!("Wrong custom error code");
                        }
                    } else {
                        println!(
                            "❌ UNEXPECTED: Got {:?}, expected Custom({})",
                            instruction_error,
                            SystemError::AccountAlreadyInUse as u32
                        );
                        panic!("Expected Custom error");
                    }
                }
                TransactionError::AlreadyProcessed => {
                    panic!("Got TransactionError::AlreadyProcessed - transaction deduplication happened, need different signer");
                }
                other => {
                    println!("❌ UNEXPECTED ERROR TYPE: {:?}", other);
                    panic!(
                        "Expected InstructionError with SystemError::AccountAlreadyInUse, got: {:?}",
                        other
                    );
                }
            }
        }
    }

    // Demonstrate five approaches to testing the same error: manual → better → best → surgical → fluent

    // Manual: Full error structure, shows complete hierarchy
    demand_transaction_error(
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(SystemError::AccountAlreadyInUse as u32),
        ),
        result.clone(),
    );

    // Better: Manual casting, more concise but still requires error code knowledge
    demand_instruction_error_at_index(
        0,
        InstructionError::Custom(SystemError::AccountAlreadyInUse as u32),
        result.clone(),
    );

    // Best: Type-safe, no casting, encapsulates hierarchy knowledge ("anywhere" version)
    demand_system_error(SystemError::AccountAlreadyInUse, result.clone());

    // Surgical: Type-safe with explicit index control (useful for multi-instruction transactions)
    use litesvm_testing::demand_system_error_at_index;
    demand_system_error_at_index(0, SystemError::AccountAlreadyInUse, result.clone());

    // Fluent: Elegant method chaining for the same result
    result.demand_system_error(SystemError::AccountAlreadyInUse);
}

// Test utilities:

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// System program ID (well-known constant)
const SYSTEM_PROGRAM_ID: Pubkey = solana_system_interface::program::ID;

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();

    let fee_payer = Keypair::new();
    svm.airdrop(&fee_payer.pubkey(), 2 * LAMPORTS_PER_SOL) // Give enough for fees
        .expect("airdrop failed");

    (svm, fee_payer)
}
