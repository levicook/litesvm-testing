use chrono::Utc;
use litesvm::LiteSVM;
use log::info;
use solana_message::Message;
use solana_transaction::Transaction;

use super::context::{
    discover_instruction_context, discover_transaction_context, TransactionExecutionContext,
};
use super::estimate::{ComputeUnitStats, InstructionBenchmarkResult, StatType};
use crate::cu_bench::{InstructionBenchmark, TransactionBenchmark};

/// Enhanced benchmark result for transactions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionBenchmarkResult {
    pub transaction_name: String,
    pub cu_estimate: ComputeUnitStats,
    pub execution_context: TransactionExecutionContext,
    pub generated_at: String,
    pub generated_by: String,
}

/// Universal benchmark runner for any instruction implementing InstructionBenchmark
pub fn benchmark_instruction<T: InstructionBenchmark>(
    benchmark: T,
    samples: usize,
) -> InstructionBenchmarkResult {
    // Set up SVM once - it will accumulate state across measurements
    let mut svm = benchmark.setup_svm();

    // Phase 1: Discover context through simulation
    let execution_context = discover_instruction_context(&benchmark, &mut svm);

    // Phase 2: Measure CU usage through actual execution
    let mut cu_measurements = Vec::new();
    for i in 0..samples {
        let cu_used = measure_instruction(&benchmark, &mut svm);
        cu_measurements.push(cu_used);

        if (i + 1) % 10 == 0 {
            info!("Completed {} measurements...", i + 1);
        }
    }

    // Create enhanced result
    InstructionBenchmarkResult {
        instruction_name: benchmark.instruction_name().to_string(),
        cu_estimate: ComputeUnitStats::from_measurements(
            StatType::Instruction(benchmark.instruction_name().to_string()),
            &cu_measurements,
        ),
        execution_context,
        generated_at: Utc::now().to_rfc3339(),
        generated_by: generated_by(),
    }
}

/// Universal benchmark runner for any transaction implementing TransactionBenchmark
pub fn benchmark_transaction<T: TransactionBenchmark>(
    mut benchmark: T,
    samples: usize,
) -> TransactionBenchmarkResult {
    // Set up SVM once using benchmark's configuration - it will accumulate state across measurements
    let mut svm = benchmark.setup_svm();

    // Phase 1: Discover context through simulation
    let context_tx = benchmark.build_transaction(&mut svm);
    let workflow_name = benchmark.transaction_name().to_string();
    let address_book = benchmark.address_book();
    let execution_context =
        discover_transaction_context(&context_tx, workflow_name, &mut svm, &address_book);

    // Phase 2: Measure CU usage through actual execution
    let mut cu_measurements = Vec::new();
    for i in 0..samples {
        let tx = benchmark.build_transaction(&mut svm);
        let cu_used = measure_transaction_cu(&tx, &mut svm);
        cu_measurements.push(cu_used);

        if (i + 1) % 10 == 0 {
            info!("Completed {} measurements...", i + 1);
        }
    }

    // Create enhanced result
    TransactionBenchmarkResult {
        transaction_name: benchmark.transaction_name().to_string(),
        cu_estimate: ComputeUnitStats::from_measurements(
            StatType::Transaction(benchmark.transaction_name().to_string()),
            &cu_measurements,
        ),
        execution_context,
        generated_at: Utc::now().to_rfc3339(),
        generated_by: generated_by(),
    }
}

/// Measure CU usage for a transaction using the provided SVM
fn measure_transaction_cu(transaction: &Transaction, svm: &mut LiteSVM) -> u64 {
    // Execute transaction and measure CU usage
    let result = svm.send_transaction(transaction.clone()).unwrap();
    result.compute_units_consumed
}

/// Measure CU usage for a single instruction
fn measure_instruction<T: InstructionBenchmark>(benchmark: &T, svm: &mut LiteSVM) -> u64 {
    // 1. Get target instruction and signer pubkeys from benchmark
    let (target_ix, signer_pubkeys) = benchmark.build_instruction(svm);

    // 2. Build unsigned transaction with just the target instruction
    // Get fresh blockhash for each measurement to avoid AlreadyProcessed
    svm.expire_blockhash();

    let message = Message::new(&[target_ix], Some(&signer_pubkeys[0]));
    let mut unsigned_tx = Transaction::new_unsigned(message);
    unsigned_tx.message.recent_blockhash = svm.latest_blockhash();

    // 3. Benchmark signs the transaction
    let signed_tx = benchmark.sign_transaction(unsigned_tx);

    // 4. Send transaction and measure CU usage
    let result = svm.send_transaction(signed_tx).unwrap();
    result.compute_units_consumed
}

fn generated_by() -> String {
    format!("{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
