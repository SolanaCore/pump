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
    pub token_ata: Account<'info, TokenAccount>,

    #[account(
    mut,
    constraint = token_escrow.mint == token_mint.key(),
    constraint = token_escrow.owner == bonding_curve.key(),
    )]
    pub token_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"BONDING_CURVE", token_mint.key().as_ref()],
        bump,
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


pub fn sell_token(ctx: &mut Context<SellToken>, max_token: u64) -> Result<()> {
    
    let mut bonding_curve = &mut ctx.accounts.bonding_curve; // ✅ MUTABLE borrow
    let swap_amount: SwapAmount = bonding_curve.sell_logic(max_token)?;

    let bonding_curve_info = bonding_curve.to_account_info();
    let signer_info = ctx.accounts.signer.to_account_info();

    // ✅ Check if PDA has enough lamports
    require!(
        **bonding_curve_info.lamports.borrow() >= swap_amount.max_sol,
        ErrorCode::InsufficientFunds
    );

    // ✅ Transfer lamports
    **bonding_curve_info.try_borrow_mut_lamports()? -= swap_amount.max_sol;
    **signer_info.try_borrow_mut_lamports()? += swap_amount.max_sol;

    // ✅ Prepare signer seeds
    let binding = ctx.accounts.token_mint.key();
    let bump = bonding_curve.bump.clone();
    let signer = &[b"BONDING_CURVE", binding.as_ref(), &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[&signer[..]];

    // // ✅ Token transfer (using mutable bonding_curve)
    // &bonding_curve.transfer_token(
    //     ctx.accounts.token_ata.to_account_info(),
    //     ctx.accounts.token_escrow.to_account_info(),
    //     signer_seeds,
    //     swap_amount.token_to_send,
    //     bonding_curve.to_account_info().clone(),
    //     ctx.accounts.token_mint.to_account_info(),
    //     ctx.accounts.token_program.to_account_info(),
    // )?;

    Ok(())
}
