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

pub fn get_metadata_context<'info>(
    token_metadata_program: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    metadata: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>>> {
    let accounts = CreateMetadataAccountsV3 {
        payer: payer.clone(),
        update_authority: update_authority.clone(),
        mint: mint.clone(),
        metadata: metadata.clone(),
        mint_authority: mint_authority.clone(),
        system_program: system_program.clone(),
        rent: rent.clone(),
    };

    Ok(CpiContext::new_with_signer(
        token_metadata_program.clone(),
        accounts,
        signer_seeds,
    ))
}
