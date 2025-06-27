use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount, transfer as spl_transfer, Transfer as SplTransfer},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
};
use crate::error::ErrorCode;
use crate::constants::BONDING_SEED;

#[account]
#[derive(InitSpace)]
pub struct BondingCurve {
    pub virtual_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub token_sold: u64,
    pub token_mint: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct SwapAmount {
    pub token_to_send: u64,
    pub max_sol: u64,
}

impl<'info> BondingCurve {
    pub fn init_bonding_curve(
        &mut self,
        virtual_sol_reserve: &u64,
        virtual_token_reserve: &u64,
        token_mint: &Pubkey,
        bump: &u8,
    ) -> Result<()> {
        self.virtual_sol_reserve = *virtual_sol_reserve;
        self.virtual_token_reserve = *virtual_token_reserve;
        self.token_sold = 0;
        self.token_mint = *token_mint;
        self.is_active = true;
        self.bump = *bump;
        Ok(())
    }

    pub fn update_token_reserve(&mut self, new_token_reserve: u64) -> Result<()> {
        self.virtual_token_reserve = new_token_reserve;
        Ok(())
    }

    pub fn update_sol_reserve(&mut self, new_sol_reserve: u64) -> Result<()> {
        self.virtual_sol_reserve = new_sol_reserve;
        Ok(())
    }

    pub fn increment_token_sold(&mut self, amount: u64) -> Result<()> {
        self.token_sold = self
            .token_sold
            .checked_add(amount)
            .ok_or(ErrorCode::OverflowDetected)?;
        Ok(())
    }

    pub fn decrement_token_sold(&mut self, amount: u64) -> Result<()> {
        self.token_sold = self
            .token_sold
            .checked_sub(amount)
            .ok_or(ErrorCode::UnderflowDetected)?;
        Ok(())
    }

    pub fn buy_logic(&mut self, max_sol: u64) -> Result<SwapAmount> {
        require!(max_sol > 0, ErrorCode::InvalidSolAmount);

        let k = self
            .virtual_token_reserve
            .checked_mul(self.virtual_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let new_sol_reserve = self
            .virtual_sol_reserve
            .checked_add(max_sol)
            .ok_or(ErrorCode::OverflowDetected)?;

        let token_reserve_after_buy = k
            .checked_div(new_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let token_to_send = self
            .virtual_token_reserve
            .checked_sub(token_reserve_after_buy)
            .ok_or(ErrorCode::UnderflowDetected)?;

        require!(token_to_send > 0, ErrorCode::InvalidTokenAmount);

        self.update_token_reserve(token_reserve_after_buy)?;
        self.update_sol_reserve(new_sol_reserve)?;
        self.increment_token_sold(token_to_send)?;

        Ok(SwapAmount {
            token_to_send,
            max_sol,
        })
    }

    pub fn sell_logic(&mut self, max_token: u64) -> Result<SwapAmount> {
        require!(max_token > 0, ErrorCode::InvalidTokenAmount);

        let k = self
            .virtual_token_reserve
            .checked_mul(self.virtual_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let new_token_reserve = self
            .virtual_token_reserve
            .checked_add(max_token)
            .ok_or(ErrorCode::OverflowDetected)?;

        let sol_reserve_after_sell = k
            .checked_div(new_token_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let sol_to_send = self
            .virtual_sol_reserve
            .checked_sub(sol_reserve_after_sell)
            .ok_or(ErrorCode::UnderflowDetected)?;

        require!(sol_to_send > 0, ErrorCode::InvalidSolAmount);

        self.update_token_reserve(new_token_reserve)?;
        self.update_sol_reserve(sol_reserve_after_sell)?;
        self.decrement_token_sold(max_token)?;

        Ok(SwapAmount {
            token_to_send: max_token,
            max_sol: sol_to_send,
        })
    }

    pub fn create_metadata_account(
        &self,
        name: &str,
        ticker: &str,
        uri: &str,
        token_metadata_program: &AccountInfo<'info>,
        payer: &AccountInfo<'info>,
        update_authority: &AccountInfo<'info>,
        mint: &AccountInfo<'info>,
        metadata: &AccountInfo<'info>,
        mint_authority: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
        rent: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let token_data = DataV2 {
            name: name.to_string(),
            symbol: ticker.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            token_metadata_program.clone(),
            CreateMetadataAccountsV3 {
                metadata: metadata.clone(),
                mint: mint.clone(),
                mint_authority: mint_authority.clone(),
                update_authority: update_authority.clone(),
                payer: payer.clone(),
                system_program: system_program.clone(),
                rent: rent.clone(),
            },
            signer_seeds,
        );

        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;
        Ok(())
    }

    pub fn mint_token(
        &self,
        token_program: &AccountInfo<'info>,
        to: &AccountInfo<'info>,
        mint: &AccountInfo<'info>,
        mint_authority: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new_with_signer(
            token_program.clone(),
            MintTo {
                mint: mint.clone(),
                to: to.clone(),
                authority: mint_authority.clone(),
            },
            signer_seeds,
        );

        mint_to(cpi_ctx, 1_000_000_000)?;
        Ok(())
    }

        pub fn transfer_token(
            &self,
            from: AccountInfo<'info>,
            to: AccountInfo<'info>,
            signer: &[&[&[u8]]],
            amount: u64,
            authority: AccountInfo<'info>,
            token_program: AccountInfo<'info>,
        ) -> Result<()> {
            let cpi_ctx = CpiContext::new_with_signer(
                token_program,
                SplTransfer {
                    from: from.clone(),
                    to: to,
                    authority: authority.clone(),
                },
                signer,
            );
            spl_transfer(cpi_ctx, amount)?;
            Ok(())
        }

        pub fn transfer_sol(
            &self,
            from: AccountInfo<'info>,
            to: AccountInfo<'info>,
            signer_seeds: &[&[&[u8]]],
            amount: u64,
            system_program: AccountInfo<'info>,
        ) -> Result<()> {
            let cpi_ctx = CpiContext::new_with_signer(
                            system_program,
                            Transfer { from, to },
                            signer_seeds,
                        );
            transfer(cpi_ctx, amount)?;
            Ok(())
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bonding_curve_buy_and_sell_logic() {
        let mut bonding_curve = BondingCurve {
            virtual_sol_reserve: 30,
            virtual_token_reserve: 800_000_000,
            token_sold: 0,
            token_mint: Pubkey::default(),
            is_active: true,
            bump: 0,
        };

        let max_sol = 1;
        let k = 30 * 800_000_000;
        let expected_token_reserve_after_buy = k / (30 + max_sol);
        let token_to_send = 800_000_000 - expected_token_reserve_after_buy;

        let buy_result = bonding_curve.buy_logic(max_sol);
        assert!(buy_result.is_ok());
        assert_eq!(bonding_curve.virtual_sol_reserve, 31);
        assert_eq!(bonding_curve.virtual_token_reserve, expected_token_reserve_after_buy);
        assert_eq!(bonding_curve.token_sold, token_to_send);

        let max_token = 10_000_000;
        let k = expected_token_reserve_after_buy * 31;
        let new_token_reserve = expected_token_reserve_after_buy + max_token;
        let expected_sol_reserve_after_sell = k / new_token_reserve;
        let sol_to_send = 31 - expected_sol_reserve_after_sell;

        let sell_result = bonding_curve.sell_logic(max_token);
        assert!(sell_result.is_ok());
        assert_eq!(bonding_curve.virtual_token_reserve, new_token_reserve);
        assert_eq!(bonding_curve.virtual_sol_reserve, expected_sol_reserve_after_sell);
        assert_eq!(bonding_curve.token_sold, token_to_send - max_token);
    }
}
