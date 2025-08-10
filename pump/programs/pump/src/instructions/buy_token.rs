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
use crate::ErrorCode::*;
use crate::BONDING_SEED;

#[derive(Accounts)]
pub struct BuyToken<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

        #[account(
            mut,
            seeds = [BONDING_SEED.as_bytes(), token_mint.key().as_ref(), bonding_curve.key().as_ref()],
            bump,
        )]
        pub sol_escrow: SystemAccount<'info>,

        #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint  = token_mint,
        associated_token::authority = signer,
    )]
    pub token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve
    )]
    pub token_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = bonding_curve.token_mint == token_mint.key(),      
    )]
    pub token_mint: Box<Account<'info, Mint>>,   

    #[account(
        mut,
        seeds = ["BONDING_CURVE".as_bytes(), token_mint.key().as_ref()],
        bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn buy_token(ctx: &mut Context<BuyToken>, max_sol:u64) -> Result<()> {
    let bonding_curve_info = ctx.accounts.bonding_curve.to_account_info();
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let swap_amount:SwapAmount = bonding_curve.buy_logic(max_sol)?;
    /*
    from:  AccountInfo<'info>,
        to: AccountInfo<'info>,
        vault_signer_seeds: &[&[&[u8]]],
        amount: u64,
        token_program: &AccountInfo<'info>,
    */
    
    // User -> sol_escrow
    let sol_escrow = ctx.accounts.sol_escrow.to_account_info();
    let signer_info = ctx.accounts.signer.to_account_info();
    msg!("Transferring {} lamports from signer to sol_escrow", swap_amount.max_sol);

    // sol_escrow.add_lamports(swap_amount.max_sol)?;
    // signer_info.sub_lamports(swap_amount.max_sol)?;
    bonding_curve.transfer_sol(
        &signer_info, 
        &sol_escrow, 
        swap_amount.max_sol,  
        &[],
        ctx.accounts.system_program.to_account_info()
    )?;

    /*
       // ✅ Transfer lamports
    **bonding_curve_info.try_borrow_mut_lamports()? -= swap_amount.max_sol;
    **signer_info.try_borrow_mut_lamports()? += swap_amount.max_sol;
    */
    //token_escrow -> token_ata
    let binding = ctx.accounts.token_mint.key();
    let signer = &[b"BONDING_CURVE", binding.as_ref(), &[bonding_curve.bump]];
    let signer_seeds: &[&[&[u8]]] = &[&signer[..]];

    //transfer token
    bonding_curve.transfer_token(
        ctx.accounts.token_escrow.to_account_info(), 
        ctx.accounts.token_ata.to_account_info(), 
        signer_seeds, 
        swap_amount.token_to_send, 
        bonding_curve_info,  // authority parameter
        ctx.accounts.token_mint.to_account_info(), 
        ctx.accounts.token_program.to_account_info()
    )?;
    
    Ok(())
}
