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
use anchor_spl::token::Mint;
use crate::state::{BondingCurve, GlobalConfig};
use crate::constants::{ANCHOR_DISCRIMINATOR, BONDING_SEED};
// use crate::utils::*; // assumes mint_token and create_metadata_account_v3 are here
use crate::SwapAmount;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct SellToken<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

        #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint  = token_mint,
        associated_token::authority = signer,
    )]
    pub token_ata: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve
    )]
    pub token_escrow: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"BONDING_CURVE", token_mint.key().as_ref()],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
        constraint = bonding_curve.token_mint == token_mint.key()
    )]
    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn sell_token(ctx: &mut Context<SellToken>, max_token:u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;   
    let swapAmount: SwapAmount = bonding_curve.sell_logic(max_token)?;
    // let seed_2 = bonding_curve.key().clone();;
    // let seed_3 = 
    //     let signer = &["BONDING_CURVE".as_bytes(), seed_2.as_ref(), &[ctx.accounts.bonding_curve.bump]];
    // let signer_seeds = &[&signer[..]];

    //transfer logic
    
    /*
    from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    
    */
    let token_mint = ctx.accounts.token_mint.key().clone();
    let signer = &[b"BONDING_CURVE", token_mint.as_ref(), &[bonding_curve.bump.clone()]];
    let signer_seeds:&[&[&[u8]]] = &[&signer[..]];

    bonding_curve.transfer_token(ctx.accounts.token_ata.to_account_info(), ctx.accounts.token_escrow.to_account_info(), &[&[&[]]], swapAmount.token_to_send, ctx.accounts.signer.to_account_info(), ctx.accounts.token_program.to_account_info());
    bonding_curve.transfer_sol(bonding_curve.to_account_info().clone(), ctx.accounts.signer.to_account_info(), signer_seeds, swapAmount.max_sol, ctx.accounts.system_program.to_account_info())
} 