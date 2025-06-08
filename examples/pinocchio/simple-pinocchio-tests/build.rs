use std::{env, path::PathBuf};

use litesvm_testing::pinocchio::build_pinocchio_program;

fn main() {
    let tests_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let tests_dir = PathBuf::from(&tests_dir);
    let pinocchio_dir = tests_dir.parent().unwrap(); // ../pinocchio/
    let program_dir = pinocchio_dir.join("simple-pinocchio-program");
    build_pinocchio_program(program_dir);
}
