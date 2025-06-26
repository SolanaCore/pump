use anchor_lang::prelude::*;
#[derive(Account)]
pub struct BuyToken {
    #[account(mut)]
    pub signer: Signer<'info>

    pub signer_token_ata:Accoun
}