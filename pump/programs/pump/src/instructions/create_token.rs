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
use crate::{
    error::ErrorCode,
};
// use anchor_lang::prelude::{Mint, TokenAccount}; // ✅ Fix: Use the correct types for Anchor compatibility

use crate::state::{BondingCurve, GlobalConfig};
use crate::constants::{ANCHOR_DISCRIMINATOR, BONDING_SEED};
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Mint;
use crate::state::global_config::LazyGlobalConfig;
#[derive(Accounts)]
pub struct CreateToken<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub global_state: LazyAccount<'info, GlobalConfig>,
    #[account(
        seeds = [BONDING_SEED.as_bytes(), mint.key().as_ref(), bonding_curve.key().as_ref()],
        bump,
    )]
    pub sol_escrow: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = ["BONDING_CURVE".as_bytes(), mint.key().as_ref()],
        space = ANCHOR_DISCRIMINATOR + BondingCurve::INIT_SPACE,
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        init_if_needed,
        payer = signer,
        mint::decimals = 6,
        mint::authority = bonding_curve,
        mint::freeze_authority = bonding_curve,
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub token_escrow: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = METAPLEX_ID)]
    pub token_metadata_program: Program<'info, Metaplex>,

    /// CHECK:New Metaplex Account being created
     #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}

pub fn create_token(
    ctx:Context<CreateToken>,
    sol_reserve: &u64,
    token_reserve: &u64,
    name: &str,
    ticker: &str,
    uri: &str,
) -> Result<()> {
    let virtual_token_reserve = ctx.accounts.global_state.load_virtual_token_reserve()?;
    let virtual_sol_reserve = ctx.accounts.global_state.load_virtual_sol_reserve()?;
    
    // Validation using lazy-loaded values
    require!(
        *token_reserve == *virtual_token_reserve, 
        ErrorCode::InvalidTokenAmount
    );
    require!(
        *sol_reserve == *virtual_sol_reserve, 
        ErrorCode::InvalidSolAmount
    );

    // Step 1: Initialize bonding curve state
    let result = ctx.accounts.bonding_curve.init_bonding_curve(
        sol_reserve,
        token_reserve,
        &ctx.accounts.mint.key(),
        &ctx.bumps.bonding_curve.clone(),
    )?;
    require!(ctx.accounts.bonding_curve.virtual_token_reserve == *token_reserve, ErrorCode::MetadataFailed);


    // Step 2: Prepare signer seeds
    let bump_bytes = [ctx.bumps.bonding_curve.clone()];
    let binding = ctx.accounts.mint.key();
    let seeds: &[&[u8]] = &[
        b"BONDING_CURVE",
        binding.as_ref(),
        &bump_bytes,
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // Step 3: Create metadata account
    let _ = ctx.accounts.bonding_curve.create_metadata_account(
        name,
        ticker,
        uri,
        &ctx.accounts.token_metadata_program.to_account_info(),
        &ctx.accounts.signer.to_account_info(),
        &ctx.accounts.bonding_curve.to_account_info(),
        &ctx.accounts.mint.to_account_info(),
        &ctx.accounts.metadata.to_account_info(),
        &ctx.accounts.bonding_curve.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.rent.to_account_info(),
        signer_seeds,
    );
    /*
    token_program: &AccountInfo<'info>,to: &AccountInfo<'info>, mint_authority: &AccountInfo<'info>, signer_seeds: &[&[&[u8]]],   mint_amount: u64
    */

    // Step 4: Mint tokens
    let _ = ctx.accounts.bonding_curve.mint_token(
        &ctx.accounts.token_program.to_account_info(), 
        &ctx.accounts.token_escrow.to_account_info(), 
        &ctx.accounts.mint.to_account_info(), 
        &ctx.accounts.bonding_curve.to_account_info(), 
        signer_seeds
    );

    Ok(())
}
