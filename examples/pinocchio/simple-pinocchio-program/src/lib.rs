use pinocchio_pubkey::declare_id;

declare_id!("p1XPMdsz55y5Qn5Qh7XPBi7k2DdGbA1LP1SMD52Jqap");

#[repr(u8)]
pub enum Instruction {
    LogHello = 0,
    FailWithProgramError = 1,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LogHello),
            1 => Ok(Self::FailWithProgramError),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "bpf-entrypoint")]
mod entrypoint {
    use super::Instruction;
    use pinocchio::{
        account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
        ProgramResult,
    };
    use pinocchio_log::log;

    entrypoint!(process_instruction);

    pub fn process_instruction(
        program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        if instruction_data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let (discriminator, remaining_data) = instruction_data.split_first().unwrap();

        match Instruction::try_from(*discriminator) {
            Ok(Instruction::LogHello) => log_hello(program_id, remaining_data),
            Ok(Instruction::FailWithProgramError) => fail_with_program_error(remaining_data),
            Err(_) => {
                log!("Unknown instruction discriminator: {}", *discriminator);
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }

    fn log_hello(program_id: &Pubkey, _data: &[u8]) -> ProgramResult {
        log!("Hello from pinocchio! {}", program_id);
        Ok(())
    }

    fn fail_with_program_error(data: &[u8]) -> ProgramResult {
        if data.len() != 8 {
            log!("Expected 8 bytes for u64 error code, got {}", data.len());
            return Err(ProgramError::InvalidInstructionData);
        }

        // Read u64 from remaining data
        let error_code = u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);

        let program_error = ProgramError::from(error_code);
        log!("About to fail with error code: {}", error_code);
        Err(program_error)
    }
}
