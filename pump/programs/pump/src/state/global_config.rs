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
    pub fn init_global_config(&mut self, bump: &u8) -> Result<()> {
        self.token_to_sell = 800_000_000; 
        self.token_to_mint = 1_000_000_000; 
        self.virtual_token_reserve = 800_000_000; 
        self.virtual_sol_reserve = 30_000_000_000; 
        self.bump = *bump; 
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

// ✅ CRITICAL: Implement the LazyGlobalConfig trait for LazyAccount compatibility
pub trait LazyGlobalConfig {
    fn load_virtual_token_reserve(&self) -> Result<u64>;
    fn load_virtual_sol_reserve(&self) -> Result<u64>;
}

impl LazyGlobalConfig for LazyAccount<'_, GlobalConfig> {
    fn load_virtual_token_reserve(&self) -> Result<u64> {
        Ok(self.virtual_token_reserve)
    }
    
    fn load_virtual_sol_reserve(&self) -> Result<u64> {
        Ok(self.virtual_sol_reserve)
    }
}
