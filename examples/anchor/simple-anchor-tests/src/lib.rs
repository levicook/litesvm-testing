/// Loads the compiled Anchor program into LiteSVM for testing.
///
/// This function demonstrates the standard pattern for testing Anchor programs with LiteSVM:
/// 1. **Build-time compilation**: `build.rs` automatically compiles the Anchor program using `cargo build-sbf`
/// 2. **Runtime loading**: This function loads the compiled `.so` file into the SVM runtime
/// 3. **Test execution**: Tests can then invoke the program using its program ID
///
/// ## How it works
///
/// The program binary is embedded at compile time using `include_bytes!`, which means:
/// - ✅ **No runtime file dependencies** - Tests are self-contained  
/// - ✅ **Automatic rebuilds** - Program changes trigger test recompilation
/// - ✅ **CI/CD friendly** - No external build artifacts to manage
///
/// ## Usage in tests
///
/// ```rust
/// use litesvm_testing::prelude::*;
///
/// #[test]
/// fn test_my_anchor_program() {
///     let (mut svm, fee_payer) = setup_svm_and_fee_payer();
///     
///     // Load the program before testing
///     load_simple_anchor_program(&mut svm);
///     
///     // Now you can test program instructions...
///     let result = svm.send_transaction(tx);
///     result.demand_logs_contain("Hello from anchor!");
/// }
/// ```
///
/// ## Build system integration
///
/// This works in conjunction with:
/// - `build.rs` - Triggers program compilation via `litesvm_testing::anchor_testing::build_anchor_program`
/// - `simple_anchor_program::ID` - The program's declared public key
/// - `target/sbf-solana-solana/release/` - Solana's standard output location for compiled programs
///
/// For more complex scenarios, see `litesvm_testing::anchor_testing` for the underlying build utilities.
pub fn load_simple_anchor_program(svm: &mut litesvm::LiteSVM) {
    svm.add_program(
        simple_anchor_program::ID,
        include_bytes!(concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../../../target/sbf-solana-solana/release/simple_anchor_program.so"
        ),),
    );
}
