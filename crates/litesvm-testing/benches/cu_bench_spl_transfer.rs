use litesvm_testing::cu_bench::{ComputeUnitEstimate, CuLevel};
use litesvm_testing::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_token::solana_program::program_pack::Pack;

fn main() {
    println!("=== SPL Token Transfer CU Benchmark ===");

    // Run multiple measurements to collect data
    let mut cu_measurements = Vec::new();

    for i in 0..100 {
        // More samples for better statistics
        let cu_used = measure_spl_token_transfer();
        cu_measurements.push(cu_used);
        if (i + 1) % 10 == 0 {
            println!("Completed {} measurements...", i + 1);
        }
    }

    // Create structured estimate from our measurements
    let estimate = ComputeUnitEstimate::from_measurements(
        "spl_token_transfer".to_string(),
        &cu_measurements,
        vec!["litesvm".to_string()],
    );

    // Print basic stats
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
        "Custom(5000): {} CU",
        estimate.get_cu_for_level(CuLevel::Custom(5000))
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

fn measure_spl_token_transfer() -> u64 {
    // Set up fresh environment
    let (mut svm, fee_payer) = litesvm_testing::setup_svm_and_fee_payer();

    // Create mint authority
    let mint_authority = Keypair::new();
    svm.airdrop(&mint_authority.pubkey(), 10_000_000).unwrap();

    // Create sender and recipient
    let sender = Keypair::new();
    let recipient = Keypair::new();
    svm.airdrop(&sender.pubkey(), 10_000_000).unwrap();
    svm.airdrop(&recipient.pubkey(), 10_000_000).unwrap();

    // Create mint
    let mint = Keypair::new();
    let create_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &mint_authority.pubkey(),
        None, // No freeze authority
        6,    // 6 decimals
    )
    .unwrap();

    let create_mint_account_ix = solana_system_interface::instruction::create_account(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        10_000_000, // Much more lamports to ensure rent exemption
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    // Create associated token accounts
    let sender_ata = spl_associated_token_account::get_associated_token_address(
        &sender.pubkey(),
        &mint.pubkey(),
    );
    let recipient_ata = spl_associated_token_account::get_associated_token_address(
        &recipient.pubkey(),
        &mint.pubkey(),
    );

    let create_sender_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &fee_payer.pubkey(),
            &sender.pubkey(),
            &mint.pubkey(),
            &spl_token::ID,
        );

    let create_recipient_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &fee_payer.pubkey(),
            &recipient.pubkey(),
            &mint.pubkey(),
            &spl_token::ID,
        );

    // Mint tokens to sender
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &sender_ata,
        &mint_authority.pubkey(),
        &[],
        1_000_000, // 1 token (with 6 decimals)
    )
    .unwrap();

    // Set up all accounts first
    let setup_tx = solana_transaction::Transaction::new_signed_with_payer(
        &[
            create_mint_account_ix,
            create_mint_ix,
            create_sender_ata_ix,
            create_recipient_ata_ix,
            mint_to_ix,
        ],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint, &mint_authority],
        svm.latest_blockhash(),
    );

    // Execute setup (not measured)
    svm.send_transaction(setup_tx).unwrap();

    // Now measure just the transfer
    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::ID,
        &sender_ata,
        &recipient_ata,
        &sender.pubkey(),
        &[],
        500_000, // 0.5 tokens (with 6 decimals)
    )
    .unwrap();

    // Set high CU limit so we don't hit limits during measurement
    use solana_compute_budget_interface::ComputeBudgetInstruction;
    let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(200_000);

    let transfer_tx = solana_transaction::Transaction::new_signed_with_payer(
        &[cu_limit_ix, transfer_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &sender],
        svm.latest_blockhash(),
    );

    // Execute and extract CU usage
    let result = svm.send_transaction(transfer_tx);

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
