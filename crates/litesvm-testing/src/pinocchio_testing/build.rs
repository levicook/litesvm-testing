//! Build utilities for Pinocchio programs in LiteSVM testing.
//!
//! This module provides build script utilities specifically designed for compiling Pinocchio
//! programs for use in LiteSVM tests. Pinocchio is a lightweight Solana program framework
//! that offers minimal boilerplate and direct BPF compilation.
//!
//! ## Key features:
//!
//! - **Automatic compilation**: Compiles Pinocchio programs during test build
//! - **Feature management**: Handles Pinocchio-specific feature flags like `bpf-entrypoint`
//! - **Build caching**: Only rebuilds when source files change
//! - **Error handling**: Clear error messages for common build issues
//!
//! ## Typical workflow:
//!
//! ```text
//! 1. Developer runs `cargo test`
//! 2. build.rs calls `build_pinocchio_program(program_path)`
//! 3. Function runs `cargo build-sbf --features bpf-entrypoint`
//! 4. Compiled .so file lands in target/deploy/
//! 5. Tests load program via `include_bytes!` and `svm.add_program()`
//! ```
//!
//! ## Pinocchio specifics:
//!
//! **Feature requirements**: Pinocchio programs need `bpf-entrypoint` feature for Solana deployment
//! **Minimal dependencies**: No IDL generation or complex build pipeline like Anchor
//! **Direct compilation**: Straight Rust-to-BPF compilation with minimal processing
//!
//! ## Example usage:
//!
//! In your test crate's `build.rs`:
//! ```rust,no_run
//! use litesvm_testing::pinocchio_testing::build_pinocchio_program;
//!
//! build_pinocchio_program("../my-pinocchio-program");
//! ```

use std::path::Path;

/// Build a Pinocchio program from a given path with the default features.
///
/// This is the standard entry point for compiling Pinocchio programs in test build scripts.
/// It automatically applies the `bpf-entrypoint` feature which is required for Solana deployment.
///
/// # Arguments
///
/// * `program_path` - The path to the Pinocchio program directory (contains Cargo.toml and src/ directory)
///
/// # Features applied
///
/// - `bpf-entrypoint` - Required for Solana BPF program compilation
///
/// # Example
///
/// ```rust,no_run
/// // In build.rs
/// use litesvm_testing::pinocchio_testing::build_pinocchio_program;
///
/// build_pinocchio_program("../simple-pinocchio-program");
/// ```
///
/// For custom feature configurations, use [`build_pinocchio_program_with_features`].
pub fn build_pinocchio_program<P: AsRef<Path>>(program_path: P) {
    build_pinocchio_program_with_features(program_path, &["bpf-entrypoint"]);
}

/// Build a Pinocchio program from a given path with specific features.
///
/// This function provides fine-grained control over which features are enabled during
/// Pinocchio program compilation. Use this when you need custom feature combinations
/// beyond the standard `bpf-entrypoint`.
///
/// # Arguments
///
/// * `program_path` - The path to the Pinocchio program directory (contains Cargo.toml and src/ directory)
/// * `features` - Array of feature names to enable during compilation
///
/// # Build process
///
/// 1. **Change detection**: Registers the Cargo.toml and src/ directory for rebuild triggers
/// 2. **Compilation**: Runs `cargo build-sbf` with specified features in a temp directory
/// 3. **Output**: Copies compiled `.so` file to `target/sbf-solana-solana/release/` directory
/// 4. **Error handling**: Provides detailed error messages for build failures
///
/// # Example
///
/// ```rust,no_run
/// // Custom features for specialized builds
/// use litesvm_testing::pinocchio_testing::build_pinocchio_program_with_features;
///
/// build_pinocchio_program_with_features(
///     "../my-program",
///     &["bpf-entrypoint", "custom-feature", "debug-mode"]
/// );
/// ```
///
/// # Error handling
///
/// The function will terminate the build process (`std::process::exit(1)`) if:
/// - `cargo build-sbf` command fails to execute
/// - Program compilation fails (non-zero exit status)
/// - Missing Solana CLI tools in PATH
///
/// Error output includes both stdout and stderr from the compilation process for debugging.
///
/// # Dependencies
///
/// Requires Solana CLI tools to be installed and available in PATH:
/// ```bash
/// sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
/// ```
pub fn build_pinocchio_program_with_features<P: AsRef<Path>>(program_path: P, features: &[&str]) {
    crate::build_solana_program_internal(program_path, features);
}
