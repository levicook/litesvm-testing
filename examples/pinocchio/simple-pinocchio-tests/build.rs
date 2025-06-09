//! Build script for Pinocchio program testing with LiteSVM.
//!
//! This build script automatically compiles the Pinocchio program before tests run,
//! ensuring that the compiled program binary is available for loading into LiteSVM.
//!
//! ## How it works:
//!
//! 1. **Locate the program**: Finds the `simple-pinocchio-program` directory relative to this test crate
//! 2. **Compile with Solana**: Uses `cargo build-sbf` to compile the Pinocchio program with `bpf-entrypoint` feature
//! 3. **Output to target/deploy/**: Solana toolchain places the `.so` file in the standard location
//! 4. **Embed in tests**: The compiled binary gets embedded via `include_bytes!` in `lib.rs`
//!
//! ## Directory structure:
//! ```text
//! examples/pinocchio/
//! ├── simple-pinocchio-program/  <- The Pinocchio program to compile
//! │   ├── Cargo.toml
//! │   └── src/lib.rs
//! └── simple-pinocchio-tests/    <- This test crate
//!     ├── build.rs               <- This file  
//!     ├── src/lib.rs
//!     └── tests/
//! ```
//!
//! ## Pinocchio vs Anchor differences:
//!
//! - **No Anchor.toml**: Pinocchio programs use only `Cargo.toml`
//! - **Feature flags**: Pinocchio requires `bpf-entrypoint` feature for Solana deployment
//! - **Simpler setup**: Less boilerplate compared to Anchor's IDL generation
//!
//! This pattern allows tests to be completely self-contained while automatically
//! staying in sync with program changes.

use std::{env, path::PathBuf};

use litesvm_testing::pinocchio_testing::build_pinocchio_program;

fn main() {
    // Get the directory of this test crate
    let tests_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let tests_dir = PathBuf::from(&tests_dir);

    // Navigate to the pinocchio program directory
    let pinocchio_dir = tests_dir.parent().unwrap(); // ../pinocchio/
    let program_dir = pinocchio_dir.join("simple-pinocchio-program");

    // Build the pinocchio program using litesvm_testing utilities
    build_pinocchio_program(program_dir);
}
