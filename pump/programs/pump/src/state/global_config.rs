use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub token_to_sell: u64,
    pub token_to_mint: u64,
    pub virtual_token_reserve: u64,
    pub virtual_sol_reserve: u64,
    pub bump: u8,
}

impl GlobalConfig {
    pub fn init_global_config(&mut self, bump:&u8) -> Result<()> {
        self.token_to_sell = 800_000_000; // Example value, adjust as needed
        self.token_to_mint = 1_000_000_000; // Example value, adjust as needed
        self.virtual_token_reserve = 1_000_000_000; // Example value, adjust as needed
        self.virtual_sol_reserve = 30; // Example value, adjust as needed
        self.bump = *bump; // This should be set to the appropriate value later
        Ok(())
    }
    pub fn virtual_token_reserve(&self) -> u64 {
        self.virtual_token_reserve
    }
    pub fn virtual_sol_reserve(&self) -> u64 {
        self.virtual_sol_reserve
    }

    pub fn token_to_mint(&self) -> u64 {
        self.token_to_mint
    }
    pub fn token_to_sell(&self) -> u64 {
        self.token_to_sell
    }
    pub fn bump(&self) -> u8 {
        self.bump
    }
}