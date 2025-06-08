/// Load the simple pinocchio program into LiteSVM
/// Note: build.rs ensures that the program is built before this is called
pub fn load_simple_pinocchio_program(svm: &mut litesvm::LiteSVM) {
    svm.add_program(
        simple_pinocchio_program::ID.into(),
        include_bytes!(concat!(
            std::env!("CARGO_MANIFEST_DIR"),
            "/../../../target/deploy/simple_pinocchio_program.so"
        )),
    );
}
