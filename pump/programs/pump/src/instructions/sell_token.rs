use anchor_lang::prelude::*;
#[allow(unused_imports)]
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, MintTo, Token},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        Metadata as Metaplex,
        mpl_token_metadata::ID as METAPLEX_ID,
    },
};
// use anchor_lang::prelude::{Mint, TokenAccount}; // ✅ Fix: Use the correct types for Anchor compatibility

use crate::state::{BondingCurve, GlobalConfig};
use crate::constants::{ANCHOR_DISCRIMINATOR, BONDING_SEED};
// use crate::utils::*; // assumes mint_token and create_metadata_account_v3 are here

use anchor_spl::token::TokenAccount;
use anchor_spl::token::Mint;


#[derive(Account)]
pub struct SellToken {
    #[account(mut)]
    pub signer: Signer<'info>

    #[account(
    seeds = ["bonding_curve_sol_escrow".as_bytes(), bonding_curve.key().as_ref()],
    bump
     )]
    pub sol_ata: Account<'info, SystemAccount>

        #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint  = token_mint,
        associated_token::authority = signer,
    )]
    pub token_ata: Account<'info, TokenAccount>

    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve
    )]
    pub token_escrow: Account<'info, TokenAccount>

     #[account(
        seeds = ["bonding_curve_sol_escrow".as_bytes(), bonding_curve.key().as_ref()],
        bump,
    )]
    pub sol_escrow: SystemAccount<'info>

    #[account(
        seeds = [b"BONDING_CURVE", token_mint.key().as_ref()],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>

    #[account(
        constants = mint::decimal == 6,
        constants = bonding_curve.token_mint == token_mint.key()
    )]
    pub token_mint: Account<'info, Mint>

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
    pub associated_token_program: Program<'info, AssociatedToken>
}


pub fn sell_token(ctx: Context<SellToken>, max_token:u64) -> Result<()> {
    let signer = &["bonding_curve_sol_escrow".as_bytes(), ctx.acccounts.bonding_curve.key().as_ref(), &[ctx.bumps.sol_escrow]];
    let signer_seeds = &[&signer[..]];
    let bonding_curve = &mut ctx.acccounts.bonding_curve;   
    let swapAmount: SwapAmount = bonding_curve.sell_logic(max_token);

    //transfer logic
    
    /*
    from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    
    */

    bonding_curve.transfer_token(ctx.acccounts.token_ata.to_account_info(), ctx.acccounts.token_escrow.to_account_info(), &[], swapAmount.tokan_to_send,ctx.acccounts.signer.to_account_info() ctx.acccounts.token_program.to_account_info());
    bonding_curve.transfer_token(ctx.acccounts.sol_escrow.to_account_info(), ctx.acccounts.sol_ata.to_account_info,signer_seeds, swapAmount.max_sol,ctx.acccounts.bonding_curve.to_account_info() ctx.acccounts.token_program.to_account_info())
}