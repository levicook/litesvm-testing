use std::{env, path::PathBuf};

use litesvm_testing::anchor::build_anchor_program;

fn main() {
    let tests_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let tests_dir = PathBuf::from(&tests_dir);
    let anchor_dir = tests_dir.parent().unwrap(); // ../anchor/
    let program_dir = anchor_dir.join("simple-anchor-program");
    build_anchor_program(program_dir);
}
