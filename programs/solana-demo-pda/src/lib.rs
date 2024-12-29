use anchor_lang::prelude::*;

declare_id!("B32YT9RCQtFvLdCggdGdzp41z5YUaNkdTDchASqftjka");

#[program]
pub mod solana_demo_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
