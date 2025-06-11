//! Compute Unit benchmarking framework for Solana programs
//!
//! This module provides tools to measure and analyze compute unit (CU) usage
//! of Solana instructions, similar to how the Helius Priority Fee API analyzes
//! transaction fees.

use std::collections::HashMap;

#[cfg(feature = "cu_bench")]
use serde::{Deserialize, Serialize};

/// Confidence level for CU estimates, similar to Helius Priority Fee API levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CuLevel {
    /// Minimum observed CU usage (0th percentile) - absolute minimum
    Min,
    /// Conservative estimate (25th percentile) - safe for most cases  
    Conservative,
    /// Balanced estimate (50th percentile) - good default
    Balanced,
    /// Safe estimate (75th percentile) - high reliability
    Safe,
    /// Very high estimate (95th percentile) - very reliable
    VeryHigh,
    /// Maximum observed (100th percentile) - may be unnecessarily high
    UnsafeMax,
    /// Custom CU value for exact control
    Custom(u64),
    /// Apply multiplier to balanced estimate
    Multiplier(f32),
}

/// CU usage statistics for a specific instruction type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeUnitEstimate {
    /// Instruction type identifier
    pub instruction_type: String,
    /// Minimum observed CU usage (0th percentile)
    pub min: u64,
    /// Conservative estimate (25th percentile)
    pub conservative: u64,
    /// Balanced estimate (50th percentile)
    pub balanced: u64,
    /// Safe estimate (75th percentile)
    pub safe: u64,
    /// Very high estimate (95th percentile)
    pub very_high: u64,
    /// Maximum observed CU usage (100th percentile)
    pub unsafe_max: u64,
    /// Number of samples used to generate this estimate
    pub sample_size: usize,
    /// Testing environments used (e.g., ["litesvm", "mollusk"])
    pub environments: Vec<String>,
}

impl ComputeUnitEstimate {
    /// Get CU estimate for the specified confidence level
    pub fn get_cu_for_level(&self, level: CuLevel) -> u64 {
        match level {
            CuLevel::Min => self.min,
            CuLevel::Conservative => self.conservative,
            CuLevel::Balanced => self.balanced,
            CuLevel::Safe => self.safe,
            CuLevel::VeryHigh => self.very_high,
            CuLevel::UnsafeMax => self.unsafe_max,
            CuLevel::Custom(cu) => cu,
            CuLevel::Multiplier(mult) => (self.balanced as f32 * mult) as u64,
        }
    }

    /// Create estimate from a series of CU measurements
    pub fn from_measurements(
        instruction_type: String,
        measurements: &[u64],
        environments: Vec<String>,
    ) -> Self {
        let mut sorted = measurements.to_vec();
        sorted.sort_unstable();

        let len = sorted.len();
        let min = sorted[0];
        let unsafe_max = sorted[len - 1];

        // Calculate percentiles
        let conservative = sorted[len * 25 / 100];
        let balanced = sorted[len * 50 / 100];
        let safe = sorted[len * 75 / 100];
        let very_high = sorted[len * 95 / 100];

        Self {
            instruction_type,
            min,
            conservative,
            balanced,
            safe,
            very_high,
            unsafe_max,
            sample_size: len,
            environments,
        }
    }
}

/// Database of CU estimates for different instruction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeUnitDatabase {
    pub estimates: HashMap<String, ComputeUnitEstimate>,
    pub generated_at: String, // ISO timestamp
}

impl ComputeUnitDatabase {
    /// Create new empty database
    pub fn new() -> Self {
        Self {
            estimates: HashMap::new(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Get estimate for instruction type
    pub fn get_estimate(&self, instruction_type: &str) -> Option<&ComputeUnitEstimate> {
        self.estimates.get(instruction_type)
    }

    /// Get CU estimate for instruction type at specified level
    pub fn get_cu_estimate(&self, instruction_type: &str, level: CuLevel) -> Option<u64> {
        self.get_estimate(instruction_type)
            .map(|est| est.get_cu_for_level(level))
    }
}

impl Default for ComputeUnitDatabase {
    fn default() -> Self {
        Self::new()
    }
}

// // Core trait
// trait CuBenchInstruction { ... }

// // Runner
// struct CuBenchRunner { ... }

// // Database/estimates
// struct CuBenchDatabase { ... }
// struct CuBenchEstimate { ... }

// // TX builder integration
// let estimates = CuBenchDatabase::load();
// let tx_builder = TxBuilder::new()
//     .with_cubench_estimates(estimates);

// // benches/cu_measurements_sol_transfer.rs
// use litesvm_testing::*;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SolTransfer {
//     pub amount: u64,
//     pub from_balance: u64,
// }

// impl BenchmarkableInstruction for SolTransfer {
//     // trait implementation
// }

// fn main() {
//     let mut runner = BenchmarkRunner::new();
//     let results = runner.benchmark_instruction::<SolTransfer>();
//     results.write_reports("sol_transfer").unwrap();
// }
