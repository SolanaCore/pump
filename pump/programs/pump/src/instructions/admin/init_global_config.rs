use anchor_lang::prelude::*;
use crate::GlobalConfig;
#[derive(Accounts)]
pub struct InitGlobalConfig<'info> {
    #[account(
        init,
        payer = signer,
        space = GlobalConfig::INIT_SPACE,
        seeds = [b"global_config"],
        bump,
    )] 
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut, address = crate::admin::id())]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

    pub fn init_global_config(ctx:&mut Context<InitGlobalConfig>, bump : u8) -> Result<()> {
        ctx.accounts.global_config.init_global_config(&bump)?;
        Ok(())
    }
