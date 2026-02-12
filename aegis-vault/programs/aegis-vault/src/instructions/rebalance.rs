use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::logic::{determine_vault_state, calculate_peg_bps, VaultState};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub keeper: Signer<'info>,
    
    /// CHECK: Pyth price feed account
    pub pyth_feed: AccountInfo<'info>,
    
    /// CHECK: Switchboard price feed account
    pub switchboard_feed: AccountInfo<'info>,
    
    // TODO: Add Kamino lending protocol accounts
    // pub kamino_reserve: AccountInfo<'info>,
    // pub kamino_obligation: AccountInfo<'info>,
    // etc.
}

pub fn handler(ctx: Context<Rebalance>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;
    
    // Check cooldown
    require!(
        clock.slot >= vault.last_rebalance_slot + vault.cooldown_slots,
        ErrorCode::RebalanceCooldown
    );
    
    // Fetch oracle data
    let oracle_data = crate::oracles::fetch_oracle_data(
        &ctx.accounts.pyth_feed,
        &ctx.accounts.switchboard_feed,
        vault.oracle_stale_slots,
        15, // 15 bps max deviation between oracles
    )?;
    
    // Validate oracle quality
    let oracle_quality_ok = crate::oracles::validate_oracle_quality(
        &ctx.accounts.pyth_feed,
        &ctx.accounts.switchboard_feed,
    )?;
    
    if !oracle_quality_ok {
        msg!("Oracle quality check failed");
    }
    
    // TODO: Get health factor from Kamino
    let health_factor_bps = 312u16; // Placeholder: 3.12
    
    // Determine state
    let state = determine_vault_state(
        oracle_data.peg_deviation_bps,
        oracle_data.is_stale,
        oracle_data.has_conflict,
        health_factor_bps,
        vault,
    );
    
    // Execute state-specific logic
    match state {
        VaultState::Loop => {
            // Check reexpansion delay
            require!(
                clock.unix_timestamp >= vault.reexpansion_unlocked_at,
                ErrorCode::ReexpansionCooldown
            );
            
            // TODO: Increase leverage toward max_leverage_bps
            // - Borrow more USDC from Kamino
            // - Supply borrowed USDC back to Kamino
            
            msg!("LOOP: Increasing leverage");
        },
        VaultState::Contract => {
            // TODO: Reduce leverage
            // - Withdraw USDC from Kamino
            // - Repay borrowed USDC
            
            // Set reexpansion delay
            vault.reexpansion_unlocked_at = clock.unix_timestamp + vault.reexpansion_delay_sec;
            
            msg!("CONTRACT: Reducing leverage");
        },
        VaultState::Exit => {
            // TODO: Emergency unwind
            // - Withdraw all supplied USDC
            // - Repay all borrowed USDC
            
            msg!("EXIT: Emergency unwind");
        },
        VaultState::Panic => {
            return Err(ErrorCode::VaultInPanicState.into());
        },
        VaultState::Idle => {
            msg!("IDLE: No action needed");
        },
    }
    
    // Update last rebalance slot
    vault.last_rebalance_slot = clock.slot;
    
    // TODO: Pay keeper fee (0.15% of rebalanced amount)
    
    Ok(())
}
