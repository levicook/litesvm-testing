//! Compute Unit benchmarking framework for Solana programs
//!
//! This module provides tools to measure and analyze compute unit (CU) usage
//! of Solana instructions, similar to how the Helius Priority Fee API analyzes
//! transaction fees.

use std::collections::HashMap;

use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

pub mod context;
pub mod estimate;
pub mod runner;

// Re-export main types for convenience
pub use context::{
    ExecutionStats, InstructionExecutionContext, ProgramContext, ProgramInfo, SVMContext,
    TransactionExecutionContext, WorkflowContext,
};
pub use estimate::{
    ComputeUnitDatabase, ComputeUnitLevel, ComputeUnitStats, InstructionBenchmarkResult, StatType,
};
pub use runner::{benchmark_instruction, benchmark_transaction, TransactionBenchmarkResult};

/// Trait for benchmarking the CU usage of specific instructions
pub trait InstructionBenchmark {
    /// Human-readable name for this instruction type
    fn instruction_name(&self) -> &'static str;

    /// Set up SVM with necessary programs and initial state (called once per benchmark run)
    fn setup_svm(&self) -> LiteSVM;

    /// Build the instruction to measure, returning instruction and required signer pubkeys
    fn build_instruction(&self, svm: &mut LiteSVM) -> (Instruction, Vec<Pubkey>);

    /// Sign the unsigned transaction containing the instruction
    fn sign_transaction(&self, unsigned_tx: Transaction) -> Transaction;

    /// Provide names for programs/accounts this benchmark interacts with
    fn address_book(&self) -> HashMap<Pubkey, String> {
        HashMap::new()
    }
}

/// Trait for benchmarking the CU usage of a transaction
pub trait TransactionBenchmark {
    /// Human-readable name for this transaction type
    fn transaction_name(&self) -> &'static str;

    /// Set up SVM with necessary programs and initial state (called once per benchmark run)
    fn setup_svm(&self) -> LiteSVM;

    /// Build the transaction to measure using the provided SVM
    fn build_transaction(&mut self, svm: &mut LiteSVM) -> Transaction;

    /// Provide names for programs/accounts this benchmark interacts with
    fn address_book(&self) -> HashMap<Pubkey, String> {
        HashMap::new()
    }
}
