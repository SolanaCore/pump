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

use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Account)]
pub struct BuyToken {
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

pub fn buy_token(ctx:Context<BuyToken>, maxSol:u64) -> Result<()> {
    let bonding_curve = &mut ctx.account.bonding_curve;
    let swapAmount = bonding_curve.buy_logic(max_sol);
    /*
    from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    */
    // User -> sol_escrow
    bonding_curve.transfer_token(ctx.acccounts.sol_ata.to_account_info(), ctx.acccounts.sol_escrow.to_account_info(), &[], swapAmount.max_sol, ctx.acccounts.signer.to_account_info(),ctx.acccounts.token_program.to_account_info());
    //token_escrow -> token_ata
    let signer = &[b"BONDING_CURVE", ctx.acccounts.token_mint.key().as_ref(), &[ctx.acccounts.bonding_curve.bump]];
    let signer_seeds:&[&[&[u8]]] = &[&signer[..]];

    bonding_curve.transfer_token(ctx.accounts.token_escrow.to_account_info(), ctx.accounts.token_ata.to_account_info, signer_seeds, swapAmount.token_to_send,ctx.acccounts.bonding_curve.to_account_info(), ctx.acccounts.token_program.to_account_info());
    Ok(())
}