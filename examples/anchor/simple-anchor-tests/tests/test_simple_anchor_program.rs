//! # Anchor Program Testing Examples
//!
//! This module demonstrates two ways to assert on transaction logs:
//!
//! 1. **Direct function call**: `demand_logs_contain(result, "message")`
//! 2. **Fluent trait method**: `result.demand_logs_contain("message")`
//!
//! Both approaches provide the same functionality with detailed error messages
//! when assertions fail. Choose the style that fits your testing preferences.

use {
    anchor_lang::{InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    simple_anchor_tests::load_simple_anchor_program,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

fn funded_keypair(svm: &mut LiteSVM, sol_amount: u64) -> Keypair {
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), sol_amount * LAMPORTS_PER_SOL)
        .expect("airdrop failed");

    payer
}

fn build_simple_ix() -> Instruction {
    let ix_accounts = simple_anchor_program::accounts::Initialize {};
    let ix_data = simple_anchor_program::instruction::Initialize {};
    Instruction {
        program_id: simple_anchor_program::ID,
        accounts: ix_accounts.to_account_metas(None),
        data: ix_data.data(),
    }
}

fn simple_setup() -> (LiteSVM, Transaction) {
    let mut svm = LiteSVM::new();
    load_simple_anchor_program(&mut svm);
    let payer = funded_keypair(&mut svm, 10);

    let ix = build_simple_ix();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    (svm, tx)
}

/// Test using the direct function call approach
#[test]
fn test_use_demand_logs_contain_directly() {
    use litesvm_testing::demand_logs_contain;

    let (mut svm, tx) = simple_setup();

    let result = svm.send_transaction(tx);

    demand_logs_contain(result, "Hello from anchor!");
}

/// Test using the fluent trait method approach
#[test]
fn test_use_demand_logs_contain_fluently() {
    use litesvm_testing::DemandFluency;

    let (mut svm, tx) = simple_setup();

    svm.send_transaction(tx)
        .demand_logs_contain("Hello from anchor!");
}
