#[allow(clippy::unused_imports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
#[allow(unused_imports)]
pub use error::*;
pub use utils::*;

declare_id!("FPf834XQpnVNgFTKtihkik9Bc9c57859SdXAMNrQ554Q");

#[program]
pub mod pump {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }
}
