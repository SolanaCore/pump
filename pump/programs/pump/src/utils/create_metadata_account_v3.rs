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


pub fn create_metadata_account_v3<'info>(
    token_metadata_program: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    metadata: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent: &AccountInfo<'info>,
    name: &str,
    ticker: &str,
    uri: &str,
    description: &str,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let token_data = get_token_data(name, ticker, uri, description)?;
    let metadata_ctx = get_metadata_context(
        token_metadata_program,
        payer,
        update_authority,
        mint,
        metadata,
        mint_authority,
        system_program,
        rent,
        signer_seeds,
    )?;

    create_metadata_accounts_v3(
        metadata_ctx,
        token_data,
        false, // is_mutable
        true,  // update_authority_is_signer
        None,  // optional collection details
    )?;

    Ok(())
}
