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

pub fn get_token_data(name: &str, ticker: &str, uri: &str, description: &str) -> Result<DataV2> {
    require!(
        !name.is_empty() && !uri.is_empty() && !ticker.is_empty(),
        ErrorCode::InvalidInputs
    );

    Ok(DataV2 {
        name: name.to_string(),
        symbol: ticker.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
        description: description.to_string(),
    })
}