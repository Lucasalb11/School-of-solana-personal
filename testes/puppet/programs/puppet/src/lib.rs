use anchor_lang::prelude::*;

declare_id!("MCk564kbjZVucdpWMaALpYoiiUmZ3QPpSiLDvBm3YYe");

#[program]
pub mod puppet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
