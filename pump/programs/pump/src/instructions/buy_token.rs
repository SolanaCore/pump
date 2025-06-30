use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, MintTo, Token, transfer_checked, TransferChecked},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;
use crate::SwapAmount;
use crate::state::*;

#[derive(Accounts)]
pub struct BuyToken<'info>{
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
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve
    )]
    pub token_escrow: Account<'info, TokenAccount>,

    
    #[account(
        constraint = bonding_curve.token_mint == token_mint.key()
    )]
    pub token_mint: Account<'info, Mint>,

    
    #[account(
        mut,
        seeds = ["BONDING_CURVE".as_bytes(), token_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn buy_token(ctx: &mut Context<BuyToken>, max_sol:u64) -> Result<()> {
    let mut bonding_curve = &mut ctx.accounts.bonding_curve.clone();
    let swapAmount:SwapAmount = bonding_curve.buy_logic(max_sol)?;
    /*
    from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    */
    
    // User -> sol_escrow
    &bonding_curve.transfer_sol(&ctx.accounts.signer.to_account_info(), &bonding_curve.to_account_info(), swapAmount.max_sol,  &[],ctx.accounts.system_program.to_account_info());
    //token_escrow -> token_ata
    let binding = ctx.accounts.token_mint.key();
    let signer = &[b"BONDING_CURVE", binding.as_ref(), &[bonding_curve.bump.clone()]];
    let signer_seeds:&[&[&[u8]]] = &[&signer[..]];

    &bonding_curve.transfer_token(ctx.accounts.token_escrow.to_account_info(), ctx.accounts.token_ata.to_account_info(), signer_seeds, swapAmount.token_to_send, ctx.accounts.bonding_curve.to_account_info(),ctx.accounts.token_mint.to_account_info(), ctx.accounts.token_program.to_account_info());
    Ok(())
}