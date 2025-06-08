//! # Pinocchio Program Testing Examples
//!
//! This module demonstrates two ways to assert on transaction logs:
//!
//! 1. **Direct function call**: `demand_logs_contain(result, "message")`
//! 2. **Fluent trait method**: `result.demand_logs_contain("message")`
//!
//! Both approaches provide the same functionality with detailed error messages
//! when assertions fail. Choose the style that fits your testing preferences.

use {
    litesvm_testing::prelude::*, pinocchio::program_error::ProgramError,
    simple_pinocchio_tests::load_simple_pinocchio_program,
};

/// Test using the direct function call approach
#[test]
fn test_use_demand_logs_contain_directly() {
    use litesvm_testing::demand_logs_contain;
    let (mut svm, fee_payer) = setup();

    let tx = build_say_hello_tx(&svm, &fee_payer);

    let result = svm.send_transaction(tx);
    demand_logs_contain(result, "Hello from pinocchio!");
}

/// Test using the fluent trait method approach
#[test]
fn test_use_demand_logs_contain_fluently() {
    use litesvm_testing::DemandFluency;
    let (mut svm, fee_payer) = setup();

    let tx = build_say_hello_tx(&svm, &fee_payer);

    svm.send_transaction(tx)
        .demand_logs_contain("Hello from pinocchio!");
}

/// Test that error transactions work (temporary verification test)
#[test]
fn test_error_tx_works() {
    let (mut svm, fee_payer) = setup();

    let tx = build_program_error_tx(&svm, &fee_payer, ProgramError::InvalidAccountData);

    let result = svm.send_transaction(tx);
    assert!(result.is_err(), "Transaction should have failed");

    // Verify we get some error logs
    if let Err(meta) = result {
        println!("Error logs: {:?}", meta.meta.logs);
        assert!(!meta.meta.logs.is_empty(), "Should have some logs");
    }
}

// BOGUS / but had a useful learning (won't commit, but leaving here for now)
// /// Test the new demand_instruction_error function with Pinocchio
// #[test]
// fn test_demand_instruction_error() {
//     use litesvm_testing::demand_instruction_error;
//     let (mut svm, fee_payer) = setup();

//     let tx = build_program_error_tx(&svm, &fee_payer, ProgramError::InvalidAccountData);

//     let result = svm.send_transaction(tx);

//     // Use the new function to assert the exact error type and index
//     // ProgramError::InvalidAccountData converts to InstructionError::Custom(17179869184_u32)
//     let expected_error_code = u64::from(ProgramError::InvalidAccountData) as u32;
//     demand_instruction_error(result, 0, InstructionError::Custom(expected_error_code));
// }

// Test utilities:

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    load_simple_pinocchio_program(&mut svm);

    let fee_payer = Keypair::new();
    svm.airdrop(&fee_payer.pubkey(), 1_000 * LAMPORTS_PER_SOL)
        .expect("airdrop failed");

    (svm, fee_payer)
}

fn build_say_hello_tx(svm: &LiteSVM, fee_payer: &Keypair) -> Transaction {
    let ix = Instruction {
        program_id: simple_pinocchio_program::ID.into(),
        accounts: vec![],
        data: vec![simple_pinocchio_program::Instruction::LogHello as u8], // Use enum constant
    };

    Transaction::new_signed_with_payer(
        &[ix],
        Some(&fee_payer.pubkey()),
        &[fee_payer],
        svm.latest_blockhash(),
    )
}

fn build_program_error_tx(svm: &LiteSVM, fee_payer: &Keypair, err: ProgramError) -> Transaction {
    let error_code: u64 = err.into(); // Use .into() to convert ProgramError to u64

    let mut data = vec![simple_pinocchio_program::Instruction::FailWithProgramError as u8]; // Use enum constant
    data.extend_from_slice(&error_code.to_le_bytes()); // Add u64 as 8 bytes

    let ix = Instruction {
        program_id: simple_pinocchio_program::ID.into(),
        accounts: vec![],
        data,
    };

    Transaction::new_signed_with_payer(
        &[ix],
        Some(&fee_payer.pubkey()),
        &[fee_payer],
        svm.latest_blockhash(),
    )
}
