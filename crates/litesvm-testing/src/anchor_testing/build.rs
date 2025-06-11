use std::path::Path;

/// Build an anchor program from a given path.
///
/// This is the standard entry point for compiling Anchor programs in test build scripts.
/// It uses default build settings without additional features.
///
/// # Arguments
///
/// * `program_path` - The path to the anchor program. (contains Anchor.toml, Cargo.toml and src/ directory)
///
/// For custom feature configurations, use [`build_anchor_program_with_features`].
pub fn build_anchor_program<P: AsRef<Path>>(program_path: P) {
    build_anchor_program_with_features(program_path, &[]);
}

/// Build an anchor program from a given path with specific features.
///
/// This function provides fine-grained control over which features are enabled during
/// Anchor program compilation.
///
/// # Arguments
///
/// * `program_path` - The path to the anchor program. (contains Anchor.toml, Cargo.toml and src/ directory)
/// * `features` - Array of feature names to enable during compilation
///
pub fn build_anchor_program_with_features<P: AsRef<Path>>(program_path: P, features: &[&str]) {
    crate::build_solana_program_internal(program_path, features);
}
