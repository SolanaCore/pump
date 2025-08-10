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
use crate::error::ErrorCode;
// use crate::utils::*; // assumes mint_token and create_metadata_account_v3 are here
use crate::SwapAmount;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct SellToken<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

        #[account(
        mut,
        constraint = token_ata.mint == token_mint.key(),
        constraint = token_ata.owner == signer.key(),
        )]
    pub token_ata: Box<Account<'info, TokenAccount>>,

        #[account(
            mut,
        seeds = [BONDING_SEED.as_bytes(), token_mint.key().as_ref(), bonding_curve.key().as_ref()],
        bump,
    )]
    pub sol_escrow: SystemAccount<'info>,

    #[account(
    mut,
    constraint = token_escrow.mint == token_mint.key(),
    constraint = token_escrow.owner == bonding_curve.key(),
    )]
    pub token_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"BONDING_CURVE", token_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        constraint = bonding_curve.token_mint == token_mint.key()
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn sell_token(ctx: &mut Context<SellToken>, max_token: u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve; // ✅ MUTABLE borrow
    let swap_amount: SwapAmount = bonding_curve.sell_logic(max_token)?;

    // ✅ Check if PDA has enough lamports
    

    // ✅ Transfer lamports
    let sol_escrow = ctx.accounts.sol_escrow.to_account_info();
    let signer_info = ctx.accounts.signer.to_account_info();
    
    require!(
        **sol_escrow.lamports.borrow() >= swap_amount.max_sol,
        ErrorCode::InsufficientFunds
    );
    let binding = bonding_curve.token_mint.key();
    let bonding_curve_key = bonding_curve.key();
    let signer_seeds_sol: &[&[&[u8]]] = &[&[
        b"BONDING_CURVE", 
        binding.as_ref(), 
        bonding_curve_key.as_ref(), 
        &[ctx.bumps.sol_escrow]
    ]];

    &bonding_curve.transfer_sol(
        &ctx.accounts.sol_escrow.to_account_info(), 
        &signer_info.to_account_info(), 
        swap_amount.max_sol,  
        signer_seeds_sol, 
        ctx.accounts.system_program.to_account_info()
    );

    // ✅ Prepare signer seeds
    let binding = ctx.accounts.token_mint.key();
    let bump = bonding_curve.bump.clone();
    let signer = &[b"BONDING_CURVE", binding.as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[&signer[..]];


    // ✅ Token transfer (using mutable bonding_curve)
    let _ = &bonding_curve.transfer_token(
        ctx.accounts.token_ata.to_account_info(),
        ctx.accounts.token_escrow.to_account_info(),
        signer_seeds,
        swap_amount.token_to_send,
        ctx.accounts.signer.to_account_info().clone(),
        ctx.accounts.token_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
    )?;

    Ok(())
}
