
/// Load the simple anchor program into LiteSVM
/// Note: build.rs ensures that the program is built before this is called
pub fn load_simple_anchor_program(svm: &mut litesvm::LiteSVM) {
    svm.add_program(
        simple_anchor_program::ID,
        include_bytes!(concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../../../target/deploy/simple_anchor_program.so"
        )),
    );
}
