/// Loads the compiled Pinocchio program into LiteSVM for testing.
///
/// This function demonstrates the standard pattern for testing Pinocchio programs with LiteSVM:
/// 1. **Build-time compilation**: `build.rs` automatically compiles the Pinocchio program using `cargo build-sbf`
/// 2. **Runtime loading**: This function loads the compiled `.so` file into the SVM runtime
/// 3. **Test execution**: Tests can then invoke the program using its program ID
///
/// ## How it works
///
/// The program binary is embedded at compile time using `include_bytes!`, which means:
/// - ✅ **No runtime file dependencies** - Tests are self-contained  
/// - ✅ **Automatic rebuilds** - Program changes trigger test recompilation
/// - ✅ **CI/CD friendly** - No external build artifacts to manage
/// - ✅ **Minimal overhead** - Direct binary embedding without IDL processing
///
/// ## Usage in tests
///
/// ```rust
/// use litesvm_testing::prelude::*;
///
/// #[test]
/// fn test_my_pinocchio_program() {
///     let (mut svm, fee_payer) = setup_svm_and_fee_payer();
///     
///     // Load the program before testing
///     load_simple_pinocchio_program(&mut svm);
///     
///     // Now you can test program instructions...
///     let result = svm.send_transaction(tx);
///     result.demand_logs_contain("Hello from pinocchio!");
/// }
/// ```
///
/// ## Pinocchio vs Anchor differences
///
/// **Compilation**:
/// - **Pinocchio**: Direct `cargo build-sbf --features bpf-entrypoint`
/// - **Anchor**: Multi-step build with IDL generation and additional processing
///
/// **Program loading**:
/// - **Pinocchio**: Simple binary embedding, no IDL required
/// - **Anchor**: Can include IDL metadata for enhanced tooling
///
/// **Testing simplicity**:
/// - **Pinocchio**: Minimal setup, direct instruction testing
/// - **Anchor**: More boilerplate but higher-level abstractions
///
/// ## Build system integration
///
/// This works in conjunction with:
/// - `build.rs` - Triggers program compilation via `litesvm_testing::pinocchio_testing::build_pinocchio_program`
/// - `simple_pinocchio_program::ID` - The program's declared public key
/// - `target/deploy/` - Solana's standard output location for compiled programs
/// - `bpf-entrypoint` feature - Required for Pinocchio BPF compilation
///
/// For more complex scenarios or custom features, see `litesvm_testing::pinocchio_testing` for the underlying build utilities.
pub fn load_simple_pinocchio_program(svm: &mut litesvm::LiteSVM) {
    svm.add_program(
        simple_pinocchio_program::ID.into(),
        include_bytes!(concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../../../target/deploy/simple_pinocchio_program.so"
        )),
    );
}
