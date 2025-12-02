use anchor_lang::prelude::*;

declare_id!("3bBcHaSnvPxPeFmZCb7JRREMp3K8z5yDVzDRcJ85sqR9");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
