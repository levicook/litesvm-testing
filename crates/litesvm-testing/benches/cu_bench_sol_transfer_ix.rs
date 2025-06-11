use litesvm::LiteSVM;
use litesvm_testing::cu_bench::{benchmark_instruction, InstructionBenchmark};
use litesvm_testing::prelude::*;
use log::info;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

/// SOL transfer benchmark using the new framework
struct SolTransferBenchmark {
    sender: Keypair,
    recipient: Keypair,
    transfer_amount: u64,
}

impl SolTransferBenchmark {
    fn new() -> Self {
        Self {
            sender: Keypair::new(),
            recipient: Keypair::new(),
            transfer_amount: 500_000, // Smaller transfer amount for multiple measurements
        }
    }
}

impl InstructionBenchmark for SolTransferBenchmark {
    fn instruction_name(&self) -> &'static str {
        "sol_transfer"
    }

    fn setup_svm(&self) -> LiteSVM {
        let mut svm = LiteSVM::new();

        // Fund the sender account with enough for 200+ transfers (including fees)
        svm.airdrop(&self.sender.pubkey(), 200_000_000).unwrap();

        svm
    }

    fn build_instruction(&self, _svm: &mut LiteSVM) -> (Instruction, Vec<Pubkey>) {
        let transfer_ix = solana_system_interface::instruction::transfer(
            &self.sender.pubkey(),
            &self.recipient.pubkey(),
            self.transfer_amount,
        );

        let signer_pubkeys = vec![self.sender.pubkey()];
        (transfer_ix, signer_pubkeys)
    }

    fn sign_transaction(&self, mut unsigned_tx: Transaction) -> Transaction {
        let signers = vec![&self.sender];
        unsigned_tx.sign(&signers, unsigned_tx.message.recent_blockhash);
        unsigned_tx
    }

    fn address_book(&self) -> std::collections::HashMap<Pubkey, String> {
        let mut book = std::collections::HashMap::new();
        book.insert(
            solana_system_interface::program::ID,
            "system_program".to_string(),
        );
        book.insert(self.sender.pubkey(), "sender".to_string());
        book.insert(self.recipient.pubkey(), "recipient".to_string());
        book
    }
}

fn main() {
    env_logger::init();
    info!("=== SOL Transfer CU Benchmark ===");

    let benchmark = SolTransferBenchmark::new();
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
