use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed,
    system_instruction,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to, Mint, MintTo, Token, TokenAccount,
        transfer_checked as spl_transfer_checked,
        TransferChecked as SplTransferChecked,
    },
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

    pub fn buy_logic(&mut self, max_sol: u64) -> Result<SwapAmount> {
        require!(max_sol > 0, ErrorCode::InvalidSolAmount);

        let virtual_token_reserve = self.virtual_token_reserve as u128;
        let virtual_sol_reserve = self.virtual_sol_reserve as u128;
        let max_sol = max_sol as u128;

        let k = virtual_token_reserve
            .checked_mul(virtual_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let new_sol_reserve = virtual_sol_reserve
            .checked_add(max_sol)
            .ok_or(ErrorCode::OverflowDetected)?;

        let token_reserve_after_buy = k
            .checked_div(new_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let token_to_send = virtual_token_reserve
            .checked_sub(token_reserve_after_buy)
            .ok_or(ErrorCode::UnderflowDetected)?;

        require!(token_to_send > 0, ErrorCode::InvalidTokenAmount);

        self.update_token_reserve(token_reserve_after_buy as u64)?;
        self.update_sol_reserve(new_sol_reserve as u64)?;

        Ok(SwapAmount {
            token_to_send: token_to_send as u64,
            max_sol: max_sol as u64,
        })
    }

    pub fn sell_logic(&mut self, max_token: u64) -> Result<SwapAmount> {
        require!(max_token > 0, ErrorCode::InvalidTokenAmount);

        let virtual_token_reserve = self.virtual_token_reserve as u128;
        let virtual_sol_reserve = self.virtual_sol_reserve as u128;
        let max_token = max_token as u128;

        let k = virtual_token_reserve
            .checked_mul(virtual_sol_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let new_token_reserve = virtual_token_reserve
            .checked_add(max_token)
            .ok_or(ErrorCode::OverflowDetected)?;

        let sol_reserve_after_sell = k
            .checked_div(new_token_reserve)
            .ok_or(ErrorCode::OverflowDetected)?;

        let sol_to_send = virtual_sol_reserve
            .checked_sub(sol_reserve_after_sell)
            .ok_or(ErrorCode::UnderflowDetected)?;

        require!(sol_to_send > 0, ErrorCode::InvalidSolAmount);

        self.update_token_reserve(new_token_reserve as u64)?;
        self.update_sol_reserve(sol_reserve_after_sell as u64)?;

        Ok(SwapAmount {
            token_to_send: max_token as u64,
            max_sol: sol_to_send as u64,
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

        mint_to(cpi_ctx, 1_000_000_000_000_000)?;
        Ok(())
    }

    pub fn transfer_token(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        signer: &[&[&[u8]]],
        amount: u64,
        authority: AccountInfo<'info>,
        token_mint: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
    ) -> Result<()> {
        let decimals: u8 = 6;

        if signer.is_empty() {
            let cpi_ctx = CpiContext::new(
                token_program,
                SplTransferChecked {
                    from: from.clone(),
                    to: to.clone(),
                    mint: token_mint.clone(),
                    authority: authority.clone(),
                },
            );
            spl_transfer_checked(cpi_ctx, amount, decimals)?;
        } else {
            let cpi_ctx = CpiContext::new_with_signer(
                token_program,
                SplTransferChecked {
                    from: from.clone(),
                    to: to.clone(),
                    mint: token_mint.clone(),
                    authority: authority.clone(),
                },
                signer,
            );
            spl_transfer_checked(cpi_ctx, amount, decimals)?;
        }

        Ok(())
    }

    pub fn transfer_sol(
        &self,
        from: &AccountInfo<'info>,
        to: &AccountInfo<'info>,
        amount: u64,
        signer_seeds:&[&[&[u8]]],
        //task: remove this system_program from the args as it is of no use.
        system_program: AccountInfo<'info>,
    ) -> Result<()> {
        let ix = system_instruction::transfer(&from.key(), &to.key(), amount);
        invoke_signed(&ix, &[from.clone(), to.clone()],signer_seeds)?;
        Ok(())
    }
    
    pub fn load_virtual_sol_reserve(&self) -> Result<u64> {
        Ok(self.virtual_sol_reserve)
    }
    
    pub fn load_virtual_token_reserve(&self) -> Result<u64> {
        Ok(self.virtual_token_reserve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
    const INITIAL_SOL: u64 = 30 * LAMPORTS_PER_SOL;
    const INITIAL_TOKEN: u64 = 800_000_000 * LAMPORTS_PER_SOL;

    fn get_default_curve() -> BondingCurve {
        BondingCurve {
            virtual_sol_reserve: INITIAL_SOL,
            virtual_token_reserve: INITIAL_TOKEN,
            token_sold: 0,
            token_mint: Pubkey::default(),
            is_active: true,
            bump: 0,
        }
    }

    #[test]
    fn test_buy_token_success() {
        let mut bonding_curve = get_default_curve();
        let max_sol = 1 * LAMPORTS_PER_SOL;

        let result = bonding_curve.buy_logic(max_sol);
        assert!(result.is_ok());

        let swap = result.unwrap();
        assert!(swap.token_to_send > 0);
        assert_eq!(swap.max_sol, max_sol);
        assert_eq!(bonding_curve.virtual_sol_reserve, INITIAL_SOL + max_sol);
    }

    #[test]
    fn test_sell_token_success() {
        let mut bonding_curve = get_default_curve();
        let max_token = 10_000_000 * LAMPORTS_PER_SOL;

        let result = bonding_curve.sell_logic(max_token);
        assert!(result.is_ok());

        let swap = result.unwrap();
        assert!(swap.max_sol > 0);
        assert_eq!(swap.token_to_send, max_token);
        assert_eq!(bonding_curve.virtual_token_reserve, INITIAL_TOKEN + max_token);
    }

    #[test]
    fn test_buy_token_zero_sol_should_fail() {
        let mut bonding_curve = get_default_curve();
        let result = bonding_curve.buy_logic(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_sell_token_zero_amount_should_fail() {
        let mut bonding_curve = get_default_curve();
        let result = bonding_curve.sell_logic(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_buy_token_minimum_value() {
        let mut bonding_curve = get_default_curve();
        let result = bonding_curve.buy_logic(1); // 1 lamport
        assert!(result.is_ok());
        let swap = result.unwrap();
        assert!(swap.token_to_send > 0);
    }

    #[test]
    fn test_sell_token_minimum_value() {
        let mut bonding_curve = get_default_curve();
        let result = bonding_curve.sell_logic(1); // 1 token in smallest unit
        assert!(result.is_ok());
        let swap = result.unwrap();
        assert!(swap.max_sol > 0);
    }

    #[test]
    fn test_buy_causes_overflow_should_fail() {
        let mut bonding_curve = BondingCurve {
            virtual_sol_reserve: u64::MAX,
            virtual_token_reserve: u64::MAX,
            ..get_default_curve()
        };
        let result = bonding_curve.buy_logic(1_000_000_000);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_sell_causes_underflow_should_fail() {
        let mut bonding_curve = BondingCurve {
            virtual_sol_reserve: 1,
            virtual_token_reserve: 1,
            ..get_default_curve()
        };
        let result = bonding_curve.sell_logic(u64::MAX);
        assert!(result.is_err());
    }
}
