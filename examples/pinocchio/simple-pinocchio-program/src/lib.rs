use pinocchio_pubkey::declare_id;

declare_id!("p1XPMdsz55y5Qn5Qh7XPBi7k2DdGbA1LP1SMD52Jqap");

#[cfg(feature = "bpf-entrypoint")]
mod entrypoint {
    use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult};
    use pinocchio_log::log;

    entrypoint!(process_instruction);

    pub fn process_instruction(
        program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        log!("Hello from pinocchio! {}", program_id);
        Ok(())
    }
}
