use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::*;

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = Bank::DISCRIMINATOR.len() + Bank::INIT_SPACE,
        seeds = [ mint.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        token::mint = mint,
        token::authority = bank_token_account,
        payer = signer,
        seeds = [b"treasury", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = User::DISCRIMINATOR.len() + User::INIT_SPACE,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitBank<'info> {
    pub fn init_bank(
        &mut self,
        bumps: InitBankBumps,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        self.bank.set_inner(Bank {
            authority: self.signer.key(),
            mint_address: self.mint.key(),
            total_deposits: 0,
            total_deposited_shares: 0,
            liquidation_threshold,
            liquidation_bonus: 0,
            liquidation_close_factor: 0,
            max_ltv,
            bump: bumps.bank,
            token_account_bump: bumps.bank_token_account,
        });

        Ok(())
    }
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: InitUserBumps) -> Result<()> {
        self.user_account.set_inner(User {
            owner: self.signer.key(),
            deposited_sol: 0,
            deposited_sol_shares: 0,
            borrowed_sol: 0,
            borrowed_sol_shares: 0,
            deposited_usdc: 0,
            borrowed_usdc: 0,
            borrowed_usdc_shares: 0,
            usdc_address: Pubkey::default(),
            last_updated: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }
}
