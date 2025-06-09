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

/// Test SystemError::InvalidAccountDataLength
/// Try to allocate an invalid amount of data
#[test]
fn test_invalid_account_data_length() {
    use litesvm_testing::demand_instruction_error_at_index;
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

    // Explicit match to decode the exact error structure
    let expected_error_code = SystemError::InvalidAccountDataLength as u32;

    match result {
        Ok(_) => panic!("Expected invalid data length allocation to fail"),
        Err(ref meta) => {
            println!("=== DETAILED ERROR ANALYSIS ===");
            println!("Raw error: {:?}", meta.err);
            println!(
                "Expected error code: {} (SystemError::InvalidAccountDataLength)",
                expected_error_code
            );

            // Match on the specific error structure
            match &meta.err {
                TransactionError::InstructionError(index, instruction_error) => {
                    println!("✅ CONFIRMED: InstructionError at index {}", index);

                    match instruction_error {
                        InstructionError::Custom(code) => {
                            println!("Custom error code: {}", code);
                            if *code == expected_error_code {
                                println!("✅ CONFIRMED: Code matches SystemError::InvalidAccountDataLength");
                            } else {
                                println!(
                                    "❌ UNEXPECTED: Code {} doesn't match expected {}",
                                    code, expected_error_code
                                );
                                panic!("Wrong custom error code");
                            }
                        }
                        other => {
                            println!(
                                "❌ UNEXPECTED: Got {:?}, expected Custom({})",
                                other, expected_error_code
                            );
                            panic!("Expected Custom error, got: {:?}", other);
                        }
                    }
                }
                other => {
                    println!("❌ UNEXPECTED ERROR TYPE: {:?}", other);
                    panic!("Expected InstructionError, got: {:?}", other);
                }
            }
        }
    }

    // Should fail with SystemError::InvalidAccountDataLength
    demand_instruction_error_at_index(0, InstructionError::Custom(expected_error_code), result);
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
