use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::context::InstructionExecutionContext;

/// Type of benchmark being measured
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "benchmark_type", content = "benchmark_name")]
pub enum StatType {
    #[serde(rename = "instruction")]
    Instruction(String),
    #[serde(rename = "transaction")]
    Transaction(String),
}

/// Enhanced benchmark result with execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionBenchmarkResult {
    pub instruction_name: String,
    pub cu_estimate: ComputeUnitStats,
    pub execution_context: InstructionExecutionContext,
    pub generated_at: String,
    pub generated_by: String,
}

/// Confidence level for CU estimates, similar to Helius Priority Fee API levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ComputeUnitLevel {
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

/// CU usage statistics for a specific benchmark type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeUnitStats {
    /// Type and name of the benchmark
    #[serde(flatten)]
    pub stat_type: StatType,
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
}

impl ComputeUnitStats {
    /// Get CU estimate for the specified confidence level
    pub fn get_cu_for_level(&self, level: ComputeUnitLevel) -> u64 {
        match level {
            ComputeUnitLevel::Min => self.min,
            ComputeUnitLevel::Conservative => self.conservative,
            ComputeUnitLevel::Balanced => self.balanced,
            ComputeUnitLevel::Safe => self.safe,
            ComputeUnitLevel::VeryHigh => self.very_high,
            ComputeUnitLevel::UnsafeMax => self.unsafe_max,
            ComputeUnitLevel::Custom(cu) => cu,
            ComputeUnitLevel::Multiplier(mult) => (self.balanced as f32 * mult) as u64,
        }
    }

    /// Create estimate from a series of CU measurements
    pub fn from_measurements(stat_type: StatType, measurements: &[u64]) -> Self {
        let mut sorted = measurements.to_vec();
        sorted.sort_unstable();

        let len = sorted.len();
        let min = sorted[0];
        let unsafe_max = sorted[len - 1];

        // Calculate percentiles (use len-1 for proper indexing)
        let conservative = sorted[(len - 1) * 25 / 100];
        let balanced = sorted[(len - 1) * 50 / 100];
        let safe = sorted[(len - 1) * 75 / 100];
        let very_high = sorted[(len - 1) * 95 / 100];

        Self {
            stat_type,
            min,
            conservative,
            balanced,
            safe,
            very_high,
            unsafe_max,
            sample_size: len,
        }
    }
}

/// Database of CU estimates for different instruction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeUnitDatabase {
    pub estimates: HashMap<String, ComputeUnitStats>,
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
    pub fn get_estimate(&self, instruction_type: &str) -> Option<&ComputeUnitStats> {
        self.estimates.get(instruction_type)
    }

    /// Get CU estimate for instruction type at specified level
    pub fn get_cu_estimate(&self, instruction_type: &str, level: ComputeUnitLevel) -> Option<u64> {
        self.get_estimate(instruction_type)
            .map(|est| est.get_cu_for_level(level))
    }
}

impl Default for ComputeUnitDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentiles_simple_case() {
        // Test with 100 values: 1, 2, 3, ..., 100
        let measurements: Vec<u64> = (1..=100).collect();
        let stats = ComputeUnitStats::from_measurements(
            StatType::Instruction("test".to_string()),
            &measurements,
        );

        // For 1-100, percentiles should be:
        // 25th percentile: index (100-1)*25/100 = 99*25/100 = 24 -> value 25
        // 50th percentile: index (100-1)*50/100 = 99*50/100 = 49 -> value 50
        // 75th percentile: index (100-1)*75/100 = 99*75/100 = 74 -> value 75
        // 95th percentile: index (100-1)*95/100 = 99*95/100 = 94 -> value 95

        assert_eq!(stats.min, 1);
        assert_eq!(stats.conservative, 25);
        assert_eq!(stats.balanced, 50);
        assert_eq!(stats.safe, 75);
        assert_eq!(stats.very_high, 95);
        assert_eq!(stats.unsafe_max, 100);
        assert_eq!(stats.sample_size, 100);
    }

    #[test]
    fn test_percentiles_small_dataset() {
        // Test with 4 values: [10, 20, 30, 40]
        let measurements = vec![10, 20, 30, 40];
        let stats = ComputeUnitStats::from_measurements(
            StatType::Transaction("small_test".to_string()),
            &measurements,
        );

        // For 4 values, percentiles should be:
        // 25th percentile: index (4-1)*25/100 = 3*25/100 = 0 -> value 10
        // 50th percentile: index (4-1)*50/100 = 3*50/100 = 1 -> value 20
        // 75th percentile: index (4-1)*75/100 = 3*75/100 = 2 -> value 30
        // 95th percentile: index (4-1)*95/100 = 3*95/100 = 2 -> value 30

        assert_eq!(stats.min, 10);
        assert_eq!(stats.conservative, 10);
        assert_eq!(stats.balanced, 20);
        assert_eq!(stats.safe, 30);
        assert_eq!(stats.very_high, 30);
        assert_eq!(stats.unsafe_max, 40);
        assert_eq!(stats.sample_size, 4);
    }

    #[test]
    fn test_percentiles_single_value() {
        let measurements = vec![42];
        let stats = ComputeUnitStats::from_measurements(
            StatType::Instruction("single".to_string()),
            &measurements,
        );

        // All percentiles should be the same value
        assert_eq!(stats.min, 42);
        assert_eq!(stats.conservative, 42);
        assert_eq!(stats.balanced, 42);
        assert_eq!(stats.safe, 42);
        assert_eq!(stats.very_high, 42);
        assert_eq!(stats.unsafe_max, 42);
        assert_eq!(stats.sample_size, 1);
    }

    #[test]
    fn test_percentiles_duplicate_values() {
        // Test with duplicates: [5, 5, 5, 10, 10, 15, 20, 20, 20, 20]
        let measurements = vec![5, 5, 5, 10, 10, 15, 20, 20, 20, 20];
        let stats = ComputeUnitStats::from_measurements(
            StatType::Transaction("duplicates".to_string()),
            &measurements,
        );

        // Sorted: [5, 5, 5, 10, 10, 15, 20, 20, 20, 20]
        // Indices: 0, 1, 2,  3,  4,  5,  6,  7,  8,  9
        // 25th percentile: index (10-1)*25/100 = 9*25/100 = 2 -> value 5
        // 50th percentile: index (10-1)*50/100 = 9*50/100 = 4 -> value 10
        // 75th percentile: index (10-1)*75/100 = 9*75/100 = 6 -> value 20
        // 95th percentile: index (10-1)*95/100 = 9*95/100 = 8 -> value 20

        assert_eq!(stats.min, 5);
        assert_eq!(stats.conservative, 5);
        assert_eq!(stats.balanced, 10);
        assert_eq!(stats.safe, 20);
        assert_eq!(stats.very_high, 20);
        assert_eq!(stats.unsafe_max, 20);
        assert_eq!(stats.sample_size, 10);
    }

    #[test]
    fn test_percentiles_unsorted_input() {
        // Test that input gets sorted correctly
        let measurements = vec![100, 10, 50, 30, 80, 20, 90, 40, 70, 60];
        let stats = ComputeUnitStats::from_measurements(
            StatType::Instruction("unsorted".to_string()),
            &measurements,
        );

        // Should be sorted to: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
        // 25th percentile: index (10-1)*25/100 = 2 -> value 30
        // 50th percentile: index (10-1)*50/100 = 4 -> value 50
        // 75th percentile: index (10-1)*75/100 = 6 -> value 70
        // 95th percentile: index (10-1)*95/100 = 8 -> value 90

        assert_eq!(stats.min, 10);
        assert_eq!(stats.conservative, 30);
        assert_eq!(stats.balanced, 50);
        assert_eq!(stats.safe, 70);
        assert_eq!(stats.very_high, 90);
        assert_eq!(stats.unsafe_max, 100);
        assert_eq!(stats.sample_size, 10);
    }

    #[test]
    fn test_stat_type_serialization() {
        let instruction_stats = ComputeUnitStats::from_measurements(
            StatType::Instruction("test_instruction".to_string()),
            &[100, 200, 300],
        );

        let transaction_stats = ComputeUnitStats::from_measurements(
            StatType::Transaction("test_transaction".to_string()),
            &[100, 200, 300],
        );

        // Test that we can serialize/deserialize the stat types
        let instruction_json = serde_json::to_string(&instruction_stats).unwrap();
        let transaction_json = serde_json::to_string(&transaction_stats).unwrap();

        assert!(instruction_json.contains("\"benchmark_type\":\"instruction\""));
        assert!(instruction_json.contains("\"benchmark_name\":\"test_instruction\""));

        assert!(transaction_json.contains("\"benchmark_type\":\"transaction\""));
        assert!(transaction_json.contains("\"benchmark_name\":\"test_transaction\""));
    }

    #[test]
    fn test_get_cu_for_level() {
        let measurements = vec![10, 20, 30, 40, 50];
        let stats = ComputeUnitStats::from_measurements(
            StatType::Instruction("level_test".to_string()),
            &measurements,
        );

        assert_eq!(stats.get_cu_for_level(ComputeUnitLevel::Min), stats.min);
        assert_eq!(
            stats.get_cu_for_level(ComputeUnitLevel::Conservative),
            stats.conservative
        );
        assert_eq!(
            stats.get_cu_for_level(ComputeUnitLevel::Balanced),
            stats.balanced
        );
        assert_eq!(stats.get_cu_for_level(ComputeUnitLevel::Safe), stats.safe);
        assert_eq!(
            stats.get_cu_for_level(ComputeUnitLevel::VeryHigh),
            stats.very_high
        );
        assert_eq!(
            stats.get_cu_for_level(ComputeUnitLevel::UnsafeMax),
            stats.unsafe_max
        );
        assert_eq!(stats.get_cu_for_level(ComputeUnitLevel::Custom(999)), 999);

        // Test multiplier (2x balanced)
        let expected_multiplied = (stats.balanced as f32 * 2.0) as u64;
        assert_eq!(
            stats.get_cu_for_level(ComputeUnitLevel::Multiplier(2.0)),
            expected_multiplied
        );
    }
}
