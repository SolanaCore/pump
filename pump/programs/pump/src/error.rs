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
}
