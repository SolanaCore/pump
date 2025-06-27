#![allow(clippy::unused_imports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("FPf834XQpnVNgFTKtihkik9Bc9c57859SdXAMNrQ554Q");


pub mod admin {
    use anchor_lang::prelude::declare_id;

    #[cfg(feature = "devnet")]
    declare_id!("HYMbgwyWrMe98Wx3FSy5brmiBSEhagxtkbzhGA45PszD");

     #[cfg(not(feature = "devnet"))]
    declare_id!("HYMbgwyWrMe98Wx3FSy5brmiBSEhagxtkbzhGA45PszD");
}

#[program]
pub mod pump { 
    use super::*;

    pub fn init_global_config(ctx: &Context<InitGlobalConfig>) -> Result<()> {
        instructions::init_global_config(&mut ctx)?;
        Ok(())
    }

    pub fn init_bonding_curve(ctx: &Context<InitBondingCurve>) -> Result<()> {
        instructions::init_bonding_curve(&mut ctx)?;
        Ok(())
    }

    pub fn buy_token(ctx: &Context<BuyToken>, max_sol: u64) -> Result<()> {
        instructions::buy_token(&mut ctx, max_sol)?;
        Ok(())
    }

    pub fn sell_token(ctx: &Context<SellToken>, max_token: u64) -> Result<()> {
        instructions::sell_token(&mut ctx, max_token)?;
        Ok(())
    }
}
