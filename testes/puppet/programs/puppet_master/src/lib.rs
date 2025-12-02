use anchor_lang::prelude::*;

declare_id!("28wZ5grFph3ft4ViC8Rm5NcwbyZMVyLHHGSBjV8UufkD");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
