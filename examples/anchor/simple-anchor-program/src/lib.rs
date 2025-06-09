use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod simple_anchor_program {
    use super::*;

    pub fn log_hello(ctx: Context<LogHello>) -> Result<()> {
        msg!("Hello from anchor! {}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogHello {}

#[derive(Accounts)]
pub struct FailInstruction {}
