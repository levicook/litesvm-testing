use std::collections::HashMap;

use litesvm::{types::SimulatedTransactionInfo, LiteSVM};
use serde::{Deserialize, Serialize};
use solana_hash::Hash;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

use crate::cu_bench::InstructionBenchmark;

/// Execution context discovered through simulation (for instructions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionExecutionContext {
    pub svm_context: SVMContext,
    pub program_context: ProgramContext,
    pub execution_stats: ExecutionStats,
}

/// Execution context discovered through simulation (for transactions/workflows)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionExecutionContext {
    pub svm_context: SVMContext,
    pub workflow_context: WorkflowContext,
    pub execution_stats: ExecutionStats,
}

/// SVM environment context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVMContext {
    pub current_slot: u64,
    #[serde(serialize_with = "serialize_hash")]
    pub latest_blockhash: Hash,
    // Future additions when available:
    // pub feature_set: Option<FeatureSetInfo>,
    // pub compute_budget: Option<ComputeBudget>,
    // pub rent_config: Option<Rent>,
}

/// Information about the primary program and its dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramContext {
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    pub program_name: String,
    pub cpi_count: usize,
}

/// Information about a multi-program workflow (for transactions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    pub workflow_name: String,
    pub involved_programs: Vec<ProgramInfo>,
    pub cpi_sequence: Vec<String>,
    pub total_cpi_calls: usize,
}

/// Information about a program involved in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInfo {
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    pub program_name: String,
    pub instruction_count: usize, // How many instructions call this program
}

/// Statistics about the instruction execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub logs: Vec<String>,
    pub simulated_cu: u64,
}

/// Discover execution context by simulating the pure instruction
pub fn discover_instruction_context<T: InstructionBenchmark>(
    benchmark: &T,
    svm: &mut LiteSVM,
) -> InstructionExecutionContext {
    let (target_ix, signer_pubkeys) = benchmark.build_instruction(svm);

    // Build transaction with just the target instruction (no CU budget)
    svm.expire_blockhash();

    let message = Message::new(&[target_ix], Some(&signer_pubkeys[0]));
    let mut unsigned_tx = Transaction::new_unsigned(message);
    unsigned_tx.message.recent_blockhash = svm.latest_blockhash();
    let signed_tx = benchmark.sign_transaction(unsigned_tx);

    // Simulate to extract context
    let simulation = svm.simulate_transaction(signed_tx.clone()).unwrap();
    let address_book = benchmark.address_book();

    InstructionExecutionContext {
        svm_context: SVMContext {
            current_slot: svm.get_sysvar::<solana_clock::Clock>().slot,
            latest_blockhash: svm.latest_blockhash(),
        },
        program_context: extract_program_context(&signed_tx, &simulation, &address_book),
        execution_stats: extract_execution_stats(&simulation),
    }
}

fn extract_program_context(
    transaction: &Transaction,
    simulation: &SimulatedTransactionInfo,
    address_book: &HashMap<Pubkey, String>,
) -> ProgramContext {
    let target_instruction = &transaction.message.instructions[0]; // Only instruction
    let program_id = transaction.message.account_keys[target_instruction.program_id_index as usize];

    ProgramContext {
        program_id,
        program_name: lookup_program_name(program_id, address_book),
        cpi_count: simulation.meta.inner_instructions.len(),
    }
}

fn extract_execution_stats(simulation: &SimulatedTransactionInfo) -> ExecutionStats {
    ExecutionStats {
        logs: simulation.meta.logs.clone(),
        simulated_cu: simulation.meta.compute_units_consumed,
    }
}

fn lookup_program_name(program_id: Pubkey, address_book: &HashMap<Pubkey, String>) -> String {
    address_book
        .get(&program_id)
        .cloned()
        .unwrap_or_else(|| program_id.to_string())
}

/// Discover execution context for a transaction workflow
pub fn discover_transaction_context(
    transaction: &Transaction,
    workflow_name: String,
    svm: &mut LiteSVM,
    address_book: &HashMap<Pubkey, String>,
) -> TransactionExecutionContext {
    // Simulate the transaction to extract context
    let simulation = svm.simulate_transaction(transaction.clone()).unwrap();

    // Extract workflow context from the transaction and simulation
    let workflow_context =
        extract_workflow_context(transaction, &simulation, workflow_name, address_book);

    TransactionExecutionContext {
        svm_context: SVMContext {
            current_slot: svm.get_sysvar::<solana_clock::Clock>().slot,
            latest_blockhash: svm.latest_blockhash(),
        },
        workflow_context,
        execution_stats: extract_execution_stats(&simulation),
    }
}

fn extract_workflow_context(
    transaction: &Transaction,
    simulation: &SimulatedTransactionInfo,
    workflow_name: String,
    address_book: &HashMap<Pubkey, String>,
) -> WorkflowContext {
    // Extract all unique programs involved
    let mut program_usage: HashMap<Pubkey, usize> = HashMap::new();
    let mut cpi_sequence: Vec<String> = Vec::new();

    // Count direct instruction calls
    for instruction in &transaction.message.instructions {
        let program_id = transaction.message.account_keys[instruction.program_id_index as usize];
        *program_usage.entry(program_id).or_insert(0) += 1;

        let program_name = lookup_program_name(program_id, address_book);
        cpi_sequence.push(program_name);
    }

    // Add CPI calls from simulation logs (extracted from inner instructions)
    for inner_instruction_set in &simulation.meta.inner_instructions {
        for inner_instruction in inner_instruction_set {
            let program_id = transaction.message.account_keys
                [inner_instruction.instruction.program_id_index as usize];
            *program_usage.entry(program_id).or_insert(0) += 1;

            let program_name = lookup_program_name(program_id, address_book);
            cpi_sequence.push(format!("{}_cpi", program_name));
        }
    }

    // Convert to program info list
    let involved_programs: Vec<ProgramInfo> = program_usage
        .into_iter()
        .map(|(program_id, instruction_count)| ProgramInfo {
            program_id,
            program_name: lookup_program_name(program_id, address_book),
            instruction_count,
        })
        .collect();

    WorkflowContext {
        workflow_name,
        involved_programs,
        cpi_sequence,
        total_cpi_calls: simulation.meta.inner_instructions.len(),
    }
}

// Custom serialization helpers for better display
fn serialize_hash<S>(hash: &Hash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&hash.to_string())
}

fn serialize_pubkey<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&pubkey.to_string())
}
