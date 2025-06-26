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
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitGlobalConfig<'info> {
    pub fn init_global_config(&mut self, bump : u8) -> Result<()> {
        self.global_config.init_global_config(&bump)?;
        Ok(())
    }
}