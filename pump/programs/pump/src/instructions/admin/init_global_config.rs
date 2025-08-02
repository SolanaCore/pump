use anchor_lang::prelude::*;
use crate::{GlobalConfig, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitGlobalConfig<'info> {
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + GlobalConfig::INIT_SPACE,
        seeds = [b"global_config"],
        bump,
    )] 
    pub global_config: Box<Account<'info, GlobalConfig>>,
    
    #[account(mut, address = crate::admin::id())]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

    pub fn init_global_config(ctx:&mut Context<InitGlobalConfig>) -> Result<()> {
        ctx.accounts.global_config.init_global_config(&ctx.bumps.global_config)?;
        Ok(())
    }
