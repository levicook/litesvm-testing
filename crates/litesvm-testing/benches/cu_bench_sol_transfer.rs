use litesvm_testing::cu_bench::{ComputeUnitEstimate, CuLevel};
use litesvm_testing::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;

fn main() {
    println!("=== SOL Transfer CU Benchmark ===");

    // Run multiple measurements to collect data
    let mut cu_measurements = Vec::new();

    for i in 0..100 {
        // More samples for better statistics
        let cu_used = measure_sol_transfer();
        cu_measurements.push(cu_used);
        if (i + 1) % 10 == 0 {
            println!("Completed {} measurements...", i + 1);
        }
    }

    // Create structured estimate from our measurements
    let estimate = ComputeUnitEstimate::from_measurements(
        "sol_transfer".to_string(),
        &cu_measurements,
        vec!["litesvm".to_string()],
    );

    // Print basic stats like before
    let min = *cu_measurements.iter().min().unwrap();
    let max = *cu_measurements.iter().max().unwrap();
    let avg = cu_measurements.iter().sum::<u64>() / cu_measurements.len() as u64;

    println!("\n=== Raw Statistics ===");
    println!("Samples: {}", cu_measurements.len());
    println!("Min: {} CU", min);
    println!("Max: {} CU", max);
    println!("Avg: {} CU", avg);
    println!("Variance: {} CU", max - min);

    // Print our structured estimate
    println!("\n=== Structured Estimate ===");
    println!("Instruction: {}", estimate.instruction_type);
    println!(
        "Min (0th percentile): {} CU",
        estimate.get_cu_for_level(CuLevel::Min)
    );
    println!(
        "Conservative (25th): {} CU",
        estimate.get_cu_for_level(CuLevel::Conservative)
    );
    println!(
        "Balanced (50th): {} CU",
        estimate.get_cu_for_level(CuLevel::Balanced)
    );
    println!(
        "Safe (75th): {} CU",
        estimate.get_cu_for_level(CuLevel::Safe)
    );
    println!(
        "Very High (95th): {} CU",
        estimate.get_cu_for_level(CuLevel::VeryHigh)
    );
    println!(
        "Unsafe Max (100th): {} CU",
        estimate.get_cu_for_level(CuLevel::UnsafeMax)
    );

    // Show custom levels
    println!("\n=== Custom Levels ===");
    println!(
        "Custom(350): {} CU",
        estimate.get_cu_for_level(CuLevel::Custom(350))
    );
    println!(
        "Multiplier(1.2): {} CU",
        estimate.get_cu_for_level(CuLevel::Multiplier(1.2))
    );
    println!(
        "Multiplier(1.5): {} CU",
        estimate.get_cu_for_level(CuLevel::Multiplier(1.5))
    );

    // Output JSON for potential consumption
    println!("\n=== JSON Output ===");
    let json = serde_json::to_string_pretty(&estimate).expect("Failed to serialize");
    println!("{}", json);
}

fn measure_sol_transfer() -> u64 {
    // Set up fresh environment
    let (mut svm, fee_payer) = litesvm_testing::setup_svm_and_fee_payer();

    // Create sender with some SOL
    let sender = Keypair::new();
    svm.airdrop(&sender.pubkey(), 10_000_000).unwrap();

    // Create recipient
    let recipient = Keypair::new();

    // Build transaction with high CU limit for measurement
    let transfer_ix = solana_system_interface::instruction::transfer(
        &sender.pubkey(),
        &recipient.pubkey(),
        1_000_000, // 1M lamports
    );

    // Set high CU limit so we don't hit limits during measurement
    use solana_compute_budget_interface::ComputeBudgetInstruction;
    let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(200_000);

    let tx = solana_transaction::Transaction::new_signed_with_payer(
        &[cu_limit_ix, transfer_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &sender],
        svm.latest_blockhash(),
    );

    // Execute and extract CU usage
    let result = svm.send_transaction(tx);

    match result {
        Ok(meta) => {
            // CU usage is in the metadata
            meta.compute_units_consumed
        }
        Err(meta) => {
            panic!("Transaction failed: {:?}", meta);
        }
    }
}
