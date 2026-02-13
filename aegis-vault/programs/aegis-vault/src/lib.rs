use anchor_lang::prelude::*;

pub mod state;
pub mod logic;
pub mod instructions;
pub mod oracles;
pub mod lending;
pub mod errors;

use instructions::*;

declare_id!("3yGKkTBvmbJCYcgfmFK3Sn94zBt1SL48Q1dBn4v76jEt");

#[program]
pub mod aegis_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        max_leverage_bps: u16,
        hf_floor_bps: u16,
    ) -> Result<()> {
        instructions::initialize_vault::handler(ctx, max_leverage_bps, hf_floor_bps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, shares)
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        instructions::rebalance::handler(ctx)
    }
}
