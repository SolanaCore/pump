use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    },
};
use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub fn mint_token<'info>(
    token_program: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    mint_amount: u64,
) -> Result<()> {
    require!(mint_amount > 0, ErrorCode::InvalidTokenAmount);

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.clone(),
        MintTo {
            mint: mint.clone(),
            to: to.clone(),
            authority: mint_authority.clone(),
        },
        signer_seeds,
    );

    mint_to(cpi_ctx, mint_amount)?;

    Ok(())
}
