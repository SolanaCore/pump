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

// ✅ Create a trait for selective field access
pub trait GlobalConfigLoader {
    fn get_virtual_token_reserve(&self) -> Result<u64>;
    fn get_virtual_sol_reserve(&self) -> Result<u64>;
}

// ✅ Implement the trait for LazyAccount
impl<'info> GlobalConfigLoader for LazyAccount<'info, GlobalConfig> {
    fn get_virtual_token_reserve(&self) -> Result<u64> {
        // Read only the specific 8 bytes for virtual_token_reserve
        // Offset: 8 (discriminator) + 8 (token_to_sell) + 8 (token_to_mint) = 24 bytes
        let account_info = self.to_account_info();
        let data = account_info.try_borrow_data()?;
        let bytes = &data[24..32]; // 8 bytes for u64
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
    
    fn get_virtual_sol_reserve(&self) -> Result<u64> {
        // Read only the specific 8 bytes for virtual_sol_reserve  
        // Offset: 8 (discriminator) + 8 (token_to_sell) + 8 (token_to_mint) + 8 (virtual_token_reserve) = 32 bytes
        let account_info = self.to_account_info();
        let data = account_info.try_borrow_data()?;
        let bytes = &data[32..40]; // 8 bytes for u64
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}
