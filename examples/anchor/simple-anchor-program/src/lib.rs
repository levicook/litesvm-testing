use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod simple_anchor_program {
    use super::*;

    pub fn log_hello(ctx: Context<LogHello>) -> Result<()> {
        msg!("Hello from anchor! {}", ctx.program_id);
        Ok(())
    }

    pub fn fail_with_program_error(_ctx: Context<FailInstruction>, error_code: u64) -> Result<()> {
        let program_error = ProgramError::from(error_code);
        msg!("About to fail with {:?}", program_error);
        Err(program_error.into())
    }
}

#[derive(Accounts)]
pub struct LogHello {}

#[derive(Accounts)]
pub struct FailInstruction {}
