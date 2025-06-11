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
