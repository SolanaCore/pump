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
    },
};
// use anchor_lang::prelude::{Mint, TokenAccount}; // ✅ Fix: Use the correct types for Anchor compatibility

use crate::state::{BondingCurve, GlobalConfig};
use crate::constants::{ANCHOR_DISCRIMINATOR, BONDING_SEED};
// use crate::utils::*; // assumes mint_token and create_metadata_account_v3 are here

use anchor_spl::token::TokenAccount;
use anchor_spl::token::Mint;
#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub global_state: Account<'info, GlobalConfig>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = ["BONDING_SEED".as_bytes(), signer.key().as_ref()],
        space = ANCHOR_DISCRIMINATOR + BondingCurve::INIT_SPACE,
        bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
        init_if_needed,
        payer = signer,
        mint::decimals = 6,
        mint::authority = bonding_curve,
        mint::freeze_authority = bonding_curve,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub bonding_curve_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = ["bonding_curve_sol_escrow".as_bytes(), bonding_curve.key().as_ref()],
        bump,
    )]
    pub bonding_curve_sol_escrow: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metaplex>,
}

pub fn create_token(
    ctx: &mut Context<CreateToken>,
    bump: u8,
    sol_reserve: &u64,
    token_reserve: &u64,
    token_mint: &Pubkey,
    name: &str,
    ticker: &str,
    uri: &str,
) -> Result<()> {
    // Step 1: Initialize bonding curve state
    ctx.accounts.bonding_curve.init_bonding_curve(
        sol_reserve,
        token_reserve,
        token_mint,
        &bump,
    )?;

    // Step 2: Prepare signer seeds
    let bump_bytes = [bump];
    let seeds: &[&[u8]] = &[
        b"BONDING_SEED",
        ctx.accounts.signer.key.as_ref(),
        &bump_bytes,
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // Step 3: Create metadata account
    ctx.accounts.bonding_curve.create_metadata_account(
        name,
        ticker,
        uri,
        &ctx.accounts.token_metadata_program.to_account_info(),
        &ctx.accounts.signer.to_account_info(),
        &ctx.accounts.bonding_curve.to_account_info(),
        &ctx.accounts.mint.to_account_info(),
        &ctx.accounts.mint.to_account_info(),
        &ctx.accounts.bonding_curve.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.rent.to_account_info(),
        signer_seeds,
    );
    /*
    token_program: &AccountInfo<'info>,to: &AccountInfo<'info>, mint_authority: &AccountInfo<'info>, signer_seeds: &[&[&[u8]]],   mint_amount: u64
    */
    ctx.accounts.bonding_curve.mint_token(&ctx.accounts.token_program.to_account_info(), &ctx.accounts.bonding_curve_ata.to_account_info(), &ctx.accounts.mint.to_account_info(), &ctx.accounts.bonding_curve.to_account_info(), signer_seeds);

    Ok(())
}