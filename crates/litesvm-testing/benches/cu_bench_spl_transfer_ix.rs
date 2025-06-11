use std::collections::HashMap;

use litesvm::LiteSVM;
use litesvm_testing::cu_bench::{benchmark_instruction, InstructionBenchmark};
use litesvm_testing::prelude::*;
use log::info;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction::create_account;
use solana_transaction::Transaction;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};
use spl_token::solana_program::program_pack::Pack;

/// SPL token transfer benchmark using the new framework
struct SplTokenTransferBenchmark {
    mint_authority: Keypair,
    mint: Keypair,
    sender: Keypair,
    recipient: Keypair,
    sender_ata: Pubkey,
    recipient_ata: Pubkey,
    transfer_amount: u64,
}

impl SplTokenTransferBenchmark {
    fn new() -> Self {
        let sender = Keypair::new();
        let recipient = Keypair::new();
        let mint = Keypair::new();

        let sender_ata = spl_associated_token_account::get_associated_token_address(
            &sender.pubkey(),
            &mint.pubkey(),
        );

        let recipient_ata = spl_associated_token_account::get_associated_token_address(
            &recipient.pubkey(),
            &mint.pubkey(),
        );

        Self {
            mint_authority: Keypair::new(),
            mint,
            sender,
            recipient,
            sender_ata,
            recipient_ata,
            transfer_amount: 100_000, // 0.1 tokens (with 6 decimals)
        }
    }
}

impl InstructionBenchmark for SplTokenTransferBenchmark {
    fn instruction_name(&self) -> &'static str {
        "spl_token_transfer"
    }

    fn setup_svm(&self) -> LiteSVM {
        let mut svm = LiteSVM::new();

        // Airdrop to accounts that need SOL
        svm.airdrop(&self.mint_authority.pubkey(), 50_000_000)
            .unwrap();
        svm.airdrop(&self.sender.pubkey(), 10_000_000).unwrap();
        svm.airdrop(&self.recipient.pubkey(), 10_000_000).unwrap();

        // Create mint account
        let create_mint_account_ix = create_account(
            &self.mint_authority.pubkey(),
            &self.mint.pubkey(),
            5_000_000, // Rent exemption
            spl_token::state::Mint::LEN as u64,
            &spl_token::ID,
        );

        // Initialize mint
        let create_mint_ix = initialize_mint(
            &spl_token::ID,
            &self.mint.pubkey(),
            &self.mint_authority.pubkey(),
            None, // No freeze authority
            6,    // 6 decimals
        )
        .unwrap();

        // Create associated token accounts
        let create_sender_ata_ix = create_associated_token_account(
            &self.mint_authority.pubkey(),
            &self.sender.pubkey(),
            &self.mint.pubkey(),
            &spl_token::ID,
        );

        let create_recipient_ata_ix = create_associated_token_account(
            &self.mint_authority.pubkey(),
            &self.recipient.pubkey(),
            &self.mint.pubkey(),
            &spl_token::ID,
        );

        // Mint tokens to sender (enough for 200+ transfers)
        let mint_to_ix = mint_to(
            &spl_token::ID,
            &self.mint.pubkey(),
            &self.sender_ata,
            &self.mint_authority.pubkey(),
            &[],
            50_000_000, // 50 tokens (with 6 decimals)
        )
        .unwrap();

        // Execute setup transaction
        let setup_tx = Transaction::new_signed_with_payer(
            &[
                create_mint_account_ix,
                create_mint_ix,
                create_sender_ata_ix,
                create_recipient_ata_ix,
                mint_to_ix,
            ],
            Some(&self.mint_authority.pubkey()),
            &[&self.mint_authority, &self.mint],
            svm.latest_blockhash(),
        );

        svm.send_transaction(setup_tx).unwrap();

        svm
    }

    fn build_instruction(&self, _svm: &mut LiteSVM) -> (Instruction, Vec<Pubkey>) {
        let transfer_ix = spl_token::instruction::transfer(
            &spl_token::ID,
            &self.sender_ata,
            &self.recipient_ata,
            &self.sender.pubkey(),
            &[],
            self.transfer_amount,
        )
        .unwrap();

        let signer_pubkeys = vec![self.sender.pubkey()];
        (transfer_ix, signer_pubkeys)
    }

    fn sign_transaction(&self, mut unsigned_tx: Transaction) -> Transaction {
        let signers = vec![&self.sender];
        unsigned_tx.sign(&signers, unsigned_tx.message.recent_blockhash);
        unsigned_tx
    }

    fn address_book(&self) -> HashMap<Pubkey, String> {
        HashMap::from_iter(vec![
            (spl_token::ID, "spl_token".to_string()),
            (
                spl_associated_token_account::ID,
                "spl_associated_token_account".to_string(),
            ),
            (self.mint.pubkey(), "test_mint".to_string()),
            (self.sender_ata, "sender_ata".to_string()),
            (self.recipient_ata, "recipient_ata".to_string()),
            (self.sender.pubkey(), "sender".to_string()),
            (self.recipient.pubkey(), "recipient".to_string()),
            (self.mint_authority.pubkey(), "mint_authority".to_string()),
            (
                solana_system_interface::program::ID,
                "system_program".to_string(),
            ),
        ])
    }
}

fn main() {
    env_logger::init();
    info!("=== SPL Token Transfer CU Benchmark ===");

    let benchmark = SplTokenTransferBenchmark::new();
    let result = benchmark_instruction(benchmark, 100);

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
