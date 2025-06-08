use std::{path::Path, process::Command};

/// Build an pinocchio program from a given path with the default features.
///
/// # Arguments
///
/// * `program_path` - The path to the pinocchio program. (contains Cargo.toml and src/ directory)
///
pub fn build_pinocchio_program<P: AsRef<Path>>(program_path: P) {
    build_pinocchio_program_with_features(program_path, &["bpf-entrypoint"]);
}

/// Build an pinocchio program from a given path with specific features.
///
/// # Arguments
///
/// * `program_path` - The path to the pinocchio program. (contains Cargo.toml and src/ directory)
/// * `features` - The features to build the pinocchio program with.
///
pub fn build_pinocchio_program_with_features<P: AsRef<Path>>(program_path: P, features: &[&str]) {
    let program_manifest = program_path.as_ref().join("Cargo.toml");
    let program_src = program_path.as_ref().join("src");

    // Tell cargo to rerun this build script if the program source changes
    println!("cargo:rerun-if-changed={}", program_manifest.display());
    println!("cargo:rerun-if-changed={}", program_src.display());

    // Build the pinocchio program
    let output = Command::new("cargo")
        .args(&[
            "build-sbf",
            "--manifest-path",
            &program_manifest.to_string_lossy(),
            "--features",
            &features.join(","),
        ])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to build pinocchio program:");
                eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cargo build-sbf: {}", e);
            eprintln!("Make sure you have the solana CLI tools installed and in your PATH");
            std::process::exit(1);
        }
    }
}
