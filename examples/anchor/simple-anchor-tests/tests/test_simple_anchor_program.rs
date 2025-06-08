use {
    anchor_lang::{InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    litesvm_testing::demand_logs_contain,
    simple_anchor_tests::load_simple_anchor_program,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

fn build_ix() -> Instruction {
    let ix_accounts = simple_anchor_program::accounts::Initialize {};
    let ix_data = simple_anchor_program::instruction::Initialize {};
    Instruction {
        program_id: simple_anchor_program::ID,
        accounts: ix_accounts.to_account_metas(None),
        data: ix_data.data(),
    }
}

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[test]
fn test_simple_anchor_program() {
    let mut svm = LiteSVM::new();
    load_simple_anchor_program(&mut svm);

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .expect("airdrop failed");

    let ix = build_ix();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok());

    demand_logs_contain(result, "Hello from anchor!");
}
