use std::{path::Path, process::Command};

/// Build an anchor program from a given path.
///
/// # Arguments
///
/// * `program_path` - The path to the anchor program. (contains Anchor.toml, Cargo.toml and src/ directory)
///
pub fn build_anchor_program<P: AsRef<Path>>(program_path: P) {
    let program_manifest = program_path.as_ref().join("Cargo.toml");
    let program_src = program_path.as_ref().join("src");

    // Tell cargo to rerun this build script if the program source changes
    println!("cargo:rerun-if-changed={}", program_manifest.display());
    println!("cargo:rerun-if-changed={}", program_src.display());

    // Build the anchor program
    let output = Command::new("cargo")
        .args(&[
            "build-sbf",
            "--manifest-path",
            &program_manifest.to_string_lossy(),
        ])
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to build anchor program:");
                eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cargo build-sbf: {}", e);
            eprintln!("Make sure you have the anchor CLI tools installed and in your PATH");
            std::process::exit(1);
        }
    }
}
