use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow detected")]
    OverflowDetected,
    #[msg("Underflow detected")]
    UnderflowDetected,
    #[msg("the token amount can't be zero")]
    InvalidTokenAmount,
    #[msg("the sol amount can't be zero")]
    InvalidSolAmount,
    #[msg("Invalis Inputs check the that either name, ticker, uri or description are not empty")]
    InvalidInputs,
    #[msg("insufficient funds in the account 'from' account")]
    InsufficientFunds,
    #[msg("the give token mint address is not owned by the bonding_curve")]
    InvalidOwner,
    #[msg("value didn't set MetadataFailed")]
    MetadataFailed,
}
