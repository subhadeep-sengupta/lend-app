use anchor_lang::prelude::*;

declare_id!("6bNZP92j3hUNdcLvPVN9YV2Fk86nGHQXD9bLs9o3gQ4N");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod lend_app {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_bank(ctx.bumps, liquidation_threshold, max_ltv)
    }
    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        ctx.accounts.init_user(ctx.bumps, usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
}
