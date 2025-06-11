/// Private helper function for building Solana programs with isolated temp directories.
///
/// This function handles the common logic for both Anchor and Pinocchio program builds:
/// - Sets up isolated temp directory to prevent file lock contention
/// - Cleans any existing artifacts to ensure fresh builds
/// - Runs `cargo build-sbf` with specified features
/// - Extracts workspace root from OUT_DIR environment variable  
/// - Copies all built .so files to workspace target directory
/// - Provides aggressive error handling with clear diagnostics
pub(crate) fn build_solana_program_internal<P: AsRef<std::path::Path>>(
    program_path: P,
    features: &[&str],
) {
    use std::{fs, process::Command};

    let program_manifest = program_path.as_ref().join("Cargo.toml");
    let program_src = program_path.as_ref().join("src");

    // Tell cargo to rerun this build script if the program source changes
    println!("cargo:rerun-if-changed={}", program_manifest.display());
    println!("cargo:rerun-if-changed={}", program_src.display());

    // Extract program name from Cargo.toml path
    let program_name = program_path
        .as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .expect("Failed to extract program name from path");

    // Determine target directory - use existing CARGO_TARGET_DIR or create temp
    let base_target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("litesvm-builds"));

    let temp_dir = base_target_dir.join(format!("program-{}", program_name));

    if let Err(e) = fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create build directory: {}", e);
        std::process::exit(1);
    }

    // Build the program in isolated directory
    // First clean to ensure no stale artifacts
    let clean_output = Command::new("cargo")
        .args([
            "clean",
            "--manifest-path",
            &program_manifest.to_string_lossy(),
        ])
        .env("CARGO_TARGET_DIR", &temp_dir)
        .output();

    match clean_output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to clean program:");
                eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cargo clean: {}", e);
            eprintln!("Make sure you have cargo installed and in your PATH");
            std::process::exit(1);
        }
    }

    // Now build the program
    let output = Command::new("cargo")
        .args([
            "build-sbf",
            "--manifest-path",
            &program_manifest.to_string_lossy(),
            "--features",
            &features.join(","),
        ])
        .env("CARGO_TARGET_DIR", &temp_dir)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to build program:");
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

    // Copy all built .so files to the workspace target directory
    let temp_so_dir = temp_dir.join("sbf-solana-solana/release");

    // Use OUT_DIR to find workspace target directory
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set in build scripts");

    // OUT_DIR pattern: /workspace/target/debug/build/crate-hash/out
    // Extract workspace root and construct target path
    let target_pos = out_dir.find("/target/").unwrap_or_else(|| {
        eprintln!("FATAL: Could not find '/target/' in OUT_DIR: {}", out_dir);
        eprintln!("Expected OUT_DIR pattern: /workspace/target/debug/build/crate-hash/out");
        eprintln!("This indicates a problem with the cargo build environment.");
        std::process::exit(1);
    });

    let workspace_root = &out_dir[..target_pos];
    let workspace_target = std::path::PathBuf::from(format!(
        "{}/target/sbf-solana-solana/release",
        workspace_root
    ));

    if let Err(e) = fs::create_dir_all(&workspace_target) {
        eprintln!("Failed to create workspace target directory: {}", e);
        std::process::exit(1);
    }

    // Find and copy all .so files
    let entries = fs::read_dir(&temp_so_dir).unwrap_or_else(|e| {
        eprintln!(
            "FATAL: Could not read temp build directory: {}",
            temp_so_dir.display()
        );
        eprintln!("Error: {}", e);
        eprintln!("This suggests the build failed or produced no output.");
        std::process::exit(1);
    });

    let mut copied_files = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "so") {
            let filename = path.file_name().expect("File should have a name");
            let target_path = workspace_target.join(filename);

            if let Err(e) = fs::copy(&path, &target_path) {
                eprintln!(
                    "FATAL: Failed to copy .so file from {} to {}: {}",
                    path.display(),
                    target_path.display(),
                    e
                );
                std::process::exit(1);
            }

            println!("Successfully built and copied: {}", target_path.display());
            copied_files += 1;
        }
    }

    if copied_files == 0 {
        eprintln!(
            "FATAL: No .so files found in build output directory: {}",
            temp_so_dir.display()
        );
        eprintln!("The program compilation succeeded but produced no deployable artifacts.");
        eprintln!("Check that the program builds correctly with 'cargo build-sbf'.");
        std::process::exit(1);
    }
}
