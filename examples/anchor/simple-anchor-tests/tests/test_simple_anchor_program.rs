//! # Anchor Program Testing Examples
//!
//! This module demonstrates two ways to assert on transaction logs:
//!
//! 1. **Direct function call**: `demand_logs_contain(result, "message")`
//! 2. **Fluent trait method**: `result.demand_logs_contain("message")`
//!
//! Both approaches provide the same functionality with detailed error messages
//! when assertions fail. Choose the style that fits your testing preferences.

use litesvm_testing::prelude::*;

use {
    anchor_lang::{InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    simple_anchor_tests::load_simple_anchor_program,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

/// Test using the direct function call approach
#[test]
fn test_use_demand_logs_contain_directly() {
    let (mut svm, fee_payer) = setup();

    let tx = build_say_hello_tx(&svm, &fee_payer);

    let result = svm.send_transaction(tx);
    demand_logs_contain("Hello from anchor!", result);
}

/// Test using the fluent trait method approach
#[test]
fn test_use_demand_logs_contain_fluently() {
    let (mut svm, fee_payer) = setup();

    let tx = build_say_hello_tx(&svm, &fee_payer);

    svm.send_transaction(tx)
        .demand_logs_contain("Hello from anchor!");
}

// Test utilities:

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    load_simple_anchor_program(&mut svm);

    let fee_payer = Keypair::new();
    svm.airdrop(&fee_payer.pubkey(), 1_000 * LAMPORTS_PER_SOL)
        .expect("airdrop failed");

    (svm, fee_payer)
}

fn build_say_hello_tx(svm: &LiteSVM, fee_payer: &Keypair) -> Transaction {
    let ix_accounts = simple_anchor_program::accounts::LogHello {};
    let ix_data = simple_anchor_program::instruction::LogHello {};
    let ix = Instruction {
        program_id: simple_anchor_program::ID,
        accounts: ix_accounts.to_account_metas(None),
        data: ix_data.data(),
    };

    Transaction::new_signed_with_payer(
        &[ix],
        Some(&fee_payer.pubkey()),
        &[fee_payer],
        svm.latest_blockhash(),
    )
}
