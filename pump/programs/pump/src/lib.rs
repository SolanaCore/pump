use anchor_lang::prelude::*;

declare_id!("4QKMkU8HQTSWEFTieQ1Hnd5SXRjJKJhJrXnyD7zFmRwL");

#[program]
pub mod pump {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
