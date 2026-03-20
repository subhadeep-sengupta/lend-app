use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::{Bank, User};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump = bank.bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"treasury", mint.key().as_ref()],
        bump = bank.token_account_bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.bank_token_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        let bank = &mut self.bank;

        if bank.total_deposits == 0 {
            bank.total_deposits = amount;
            bank.total_deposited_shares = amount;
        }
        let deposit_ratio = amount.checked_div(bank.total_deposits).unwrap();
        let user_shares = bank
            .total_deposited_shares
            .checked_mul(deposit_ratio)
            .unwrap();

        let user = &mut self.user_account;

        match self.mint.to_account_info().key() {
            key if key == user.usdc_address => {
                user.deposited_usdc += amount;
                user.deposited_usdc_shares += amount;
            }
            _ => {
                user.deposited_sol += amount;
                user.deposited_sol_shares += amount;
            }
        }

        bank.total_deposits += amount;
        bank.total_deposited_shares += user_shares;
        Ok(())
    }
}
