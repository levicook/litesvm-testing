use std::collections::HashMap;

use litesvm_testing::prelude::*;

use litesvm::LiteSVM;
use litesvm_testing::cu_bench::{benchmark_transaction, TransactionBenchmark};
use log::info;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_message::Message;
use solana_transaction::Transaction;
use spl_token::solana_program::program_pack::Pack;

/// Benchmark for a complete token setup transaction
/// This represents a realistic workflow: create mint + ATA + mint initial supply
struct TokenSetupTransactionBenchmark {
    mint_authority: Keypair,
    mint: Keypair,
    token_account_owner: Keypair,
    mint_amount: u64,
}

impl TokenSetupTransactionBenchmark {
    fn new() -> Self {
        let mint_authority = Keypair::new();
        let mint = Keypair::new();
        let token_account_owner = Keypair::new();

        Self {
            mint_authority,
            mint,
            token_account_owner,
            mint_amount: 1_000_000, // 1M tokens with 6 decimals
        }
    }
}

impl TransactionBenchmark for TokenSetupTransactionBenchmark {
    fn transaction_name(&self) -> &'static str {
        "token_setup_complete"
    }

    fn setup_svm(&self) -> LiteSVM {
        // Create and configure SVM with necessary accounts
        let mut svm = LiteSVM::new();

        // Airdrop SOL to accounts that need to pay fees/rent (generous amounts for many measurements)
        svm.airdrop(&self.mint_authority.pubkey(), 1_000_000_000)
            .unwrap(); // 10 SOL
        svm.airdrop(&self.token_account_owner.pubkey(), 1_000_000_000)
            .unwrap(); // 10 SOL

        svm
    }

    fn build_transaction(&mut self, svm: &mut LiteSVM) -> Transaction {
        // Use a fresh mint keypair for each transaction to avoid "account already exists" errors
        self.mint = Keypair::new();

        // Get fresh blockhash from the provided SVM
        svm.expire_blockhash();
        let recent_blockhash = svm.latest_blockhash();

        // Calculate rent for mint account
        let mint_rent = svm.minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN);

        // Get associated token account address
        let ata_address = spl_associated_token_account::get_associated_token_address(
            &self.token_account_owner.pubkey(),
            &self.mint.pubkey(),
        );

        // Build all instructions for the transaction
        let instructions = vec![
            // 1. Set compute unit limit
            ComputeBudgetInstruction::set_compute_unit_limit(150_000),
            // 2. Create mint account
            solana_system_interface::instruction::create_account(
                &self.mint_authority.pubkey(),
                &self.mint.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::ID,
            ),
            // 3. Initialize mint
            spl_token::instruction::initialize_mint(
                &spl_token::ID,
                &self.mint.pubkey(),
                &self.mint_authority.pubkey(),
                Some(&self.mint_authority.pubkey()),
                6, // decimals
            )
            .unwrap(),
            // 4. Create associated token account
            spl_associated_token_account::instruction::create_associated_token_account(
                &self.token_account_owner.pubkey(), // fee payer
                &self.token_account_owner.pubkey(), // wallet
                &self.mint.pubkey(),
                &spl_token::ID,
            ),
            // 5. Mint initial supply to the ATA
            spl_token::instruction::mint_to(
                &spl_token::ID,
                &self.mint.pubkey(),
                &ata_address,
                &self.mint_authority.pubkey(),
                &[&self.mint_authority.pubkey()],
                self.mint_amount,
            )
            .unwrap(),
        ];

        // Create and sign transaction
        let message = Message::new(&instructions, Some(&self.mint_authority.pubkey()));
        let mut transaction = Transaction::new_unsigned(message);
        transaction.message.recent_blockhash = recent_blockhash;

        // Sign with all required signers
        transaction.sign(
            &[&self.mint_authority, &self.mint, &self.token_account_owner],
            recent_blockhash,
        );

        transaction
    }

    fn address_book(&self) -> HashMap<Pubkey, String> {
        HashMap::from_iter(vec![
            (system_program::ID, "system_program".to_string()),
            (spl_token::ID, "spl_token".to_string()),
            (
                spl_associated_token_account::ID,
                "spl_associated_token_account".to_string(),
            ),
            (
                solana_compute_budget_interface::ID,
                "compute_budget".to_string(),
            ),
        ])
    }
}

fn main() {
    env_logger::init();
    info!("=== Token Setup Transaction CU Benchmark ===");

    let benchmark = TokenSetupTransactionBenchmark::new();
    let result = benchmark_transaction(benchmark, 100);

    info!(
        "Measured {} samples: {} CU ({}% variance)",
        result.cu_estimate.sample_size,
        result.cu_estimate.balanced,
        if result.cu_estimate.min == result.cu_estimate.unsafe_max {
            0
        } else {
            ((result.cu_estimate.unsafe_max - result.cu_estimate.min) * 100)
                / result.cu_estimate.balanced
        }
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("Failed to serialize")
    );
}
