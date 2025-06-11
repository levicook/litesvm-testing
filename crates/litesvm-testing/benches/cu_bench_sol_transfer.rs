use litesvm::LiteSVM;
use litesvm_testing::cu_bench::{benchmark_instruction, InstructionBenchmark};
use litesvm_testing::prelude::*;
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
}

fn main() {
    println!("=== SOL Transfer CU Benchmark ===");

    let benchmark = SolTransferBenchmark::new();
    let estimate = benchmark_instruction(benchmark, 100);

    println!(
        "Measured {} samples: {} CU ({}% variance)",
        estimate.sample_size,
        estimate.balanced,
        if estimate.min == estimate.unsafe_max {
            0
        } else {
            ((estimate.unsafe_max - estimate.min) * 100) / estimate.balanced
        }
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&estimate).expect("Failed to serialize")
    );
}
