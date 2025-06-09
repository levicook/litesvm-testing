//! Build script for Anchor program testing with LiteSVM.
//!
//! This build script automatically compiles the Anchor program before tests run,
//! ensuring that the compiled program binary is available for loading into LiteSVM.
//!
//! ## How it works:
//!
//! 1. **Locate the program**: Finds the `simple-anchor-program` directory relative to this test crate
//! 2. **Compile with Solana**: Uses `cargo build-sbf` to compile the Anchor program
//! 3. **Output to target/deploy/**: Solana toolchain places the `.so` file in the standard location
//! 4. **Embed in tests**: The compiled binary gets embedded via `include_bytes!` in `lib.rs`
//!
//! ## Directory structure:
//! ```text
//! examples/anchor/
//! ├── simple-anchor-program/     <- The Anchor program to compile
//! │   ├── Anchor.toml
//! │   ├── Cargo.toml  
//! │   └── src/lib.rs
//! └── simple-anchor-tests/       <- This test crate
//!     ├── build.rs               <- This file  
//!     ├── src/lib.rs
//!     └── tests/
//! ```
//!
//! This pattern allows tests to be completely self-contained while automatically
//! staying in sync with program changes.

use std::{env, path::PathBuf};

use litesvm_testing::anchor_testing::build_anchor_program;

fn main() {
    // Get the directory of this test crate
    let tests_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let tests_dir = PathBuf::from(&tests_dir);

    // Navigate to the anchor program directory
    let anchor_dir = tests_dir.parent().unwrap(); // ../anchor/
    let program_dir = anchor_dir.join("simple-anchor-program");

    // Build the anchor program using litesvm_testing utilities
    build_anchor_program(program_dir);
}
