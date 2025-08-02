#![allow(clippy::unused_imports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FPf834XQpnVNgFTKtihkik9Bc9c57859SdXAMNrQ554Q");

pub mod admin {
    use anchor_lang::prelude::declare_id;

    // #[cfg(feature = "devnet")]
    declare_id!("52nvBaMXujpVYf6zBUvmQtHEZc4kAncRJccXG99F6yrg");

    // #[cfg(not(feature = "devnet"))]
    // declare_id!("HYMbgwyWrMe98Wx3FSy5brmiBSEhagxtkbzhGA45PszD");
}

#[program]
pub mod pump {
    use super::*;

    pub fn init_global_config(mut ctx: Context<InitGlobalConfig>) -> Result<()> {
        instructions::init_global_config(&mut ctx)?;
        Ok(())
    }

    pub fn create_token(
        mut ctx: Context<CreateToken>,
        sol_reserve: u64,
        token_reserve: u64,
        name: String,
        ticker: String,
        uri: String,
    ) -> Result<()> {
        instructions::create_token(
            &mut ctx,
            &sol_reserve,
            &token_reserve,
            &name,
            &ticker,
            &uri,
        )?;
        Ok(())
    }

    pub fn buy_token(mut ctx: Context<BuyToken>, max_sol: u64) -> Result<()> {
        instructions::buy_token(&mut ctx, max_sol)?;
        Ok(())
    }

    pub fn sell_token(mut ctx: Context<SellToken>, max_token: u64) -> Result<()> {
        instructions::sell_token(&mut ctx, max_token)?;
        Ok(())
    }
}


/*
error: More than one fallback function found
  --> programs/pump/src/lib.rs:30:5
   |
30 | /     pub fn init_global_config(mut ctx: &Context<InitGlobalConfig>) -> Result<()> {
31 | |         instructions::init_global_config(&mut ctx)?;
32 | |         Ok(())
33 | |     }
   | |_____^

   Due to marking the ctx ref in the args 
*/